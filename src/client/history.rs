use crate::{error::Result, text::remove_think_tags};
use rig::{
    conversation::{Author, Conversation, Entry},
    memory::{
        MemoryStore, apply_summary, load_async, save_async, summarized_prefix, summary_state,
    },
    message::{AssistantContent, AssistantMessage, Message, UserContent},
};

pub const CONVERSATION_ID: &str = "translation";
const SUMMARY_MAX_BYTES: usize = 8 * 1024;
const TRUNCATED: &str = "[…truncated…]\n";
const AUTHOR: &str = "history";

/// A conversation the agent loads before and saves after every run. Messages
/// older than the window are folded into a summary by [`compact_history`].
#[derive(Clone)]
pub struct SharedHistory {
    pub store: MemoryStore,
    pub window: usize,
}

impl std::fmt::Debug for SharedHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedHistory")
            .field("window", &self.window)
            .finish_non_exhaustive()
    }
}

/// `context_window` counts user/assistant pairs, so the window holds twice as many messages.
pub fn shared_history(context_window: usize) -> SharedHistory {
    SharedHistory {
        store: MemoryStore::default(),
        window: context_window * 2,
    }
}

pub async fn seed_history(history: &SharedHistory, messages: Vec<Message>) -> Result<()> {
    update_store(history, |conversation, window| {
        conversation.extend(messages);
        compact(conversation, window);
    })
    .await
}

/// Folds messages that fell out of the window into the summary before the next run.
pub async fn compact_history(history: &SharedHistory) -> Result<()> {
    update_store(history, compact).await
}

/// How many messages the history holds, to [`rewind_history`] back to.
pub async fn history_len(history: &SharedHistory) -> Result<usize> {
    let conversation = load_async(&history.store, CONVERSATION_ID).await?;
    Ok(conversation.messages().len())
}

/// Drops every message after the first `len`, such as a failed attempt's turn.
pub async fn rewind_history(history: &SharedHistory, len: usize) -> Result<()> {
    update_store(history, |conversation, _| {
        conversation.splice(len.., Vec::<Entry>::new(), Author::plugin(AUTHOR));
    })
    .await
}

async fn update_store(
    history: &SharedHistory,
    update: impl FnOnce(&mut Conversation, usize),
) -> Result<()> {
    let mut conversation = load_async(&history.store, CONVERSATION_ID).await?;
    update(&mut conversation, history.window);
    save_async(&history.store, CONVERSATION_ID, &mut conversation).await?;
    Ok(())
}

/// Strips the model's reasoning from the turns, then replaces everything but the
/// newest `window` messages with a plain-text summary of them, carrying over the
/// previous summary and capped at [`SUMMARY_MAX_BYTES`].
fn compact(conversation: &mut Conversation, window: usize) {
    strip_reasoning(conversation);
    summarize_overflow(conversation, window);
}

/// Leaves each assistant turn only its answer text, as a seeded history holds
/// it. The agent saves a turn whole, and the Ollama provider sends its reasoning
/// back as `thinking`, so every turn in the window would otherwise resend its
/// reasoning with every request.
fn strip_reasoning(conversation: &mut Conversation) {
    conversation.edit_each(Author::plugin(AUTHOR), |entry| match entry.message {
        Message::Assistant(turn) => answer_only(turn).map(Message::Assistant),
        _ => None,
    });
}

/// `turn` with only its answer text, `None` when it has nothing else.
fn answer_only(turn: &AssistantMessage) -> Option<AssistantMessage> {
    let text = turn.text();
    let answer = remove_think_tags(&text);
    let text_only = turn
        .content
        .iter()
        .all(|part| matches!(part, AssistantContent::Text(_)));
    if text_only && answer == text {
        return None;
    }
    let mut turn = turn.clone();
    turn.content = vec![AssistantContent::text(answer)];
    Some(turn)
}

fn summarize_overflow(conversation: &mut Conversation, window: usize) {
    let from = summarized_prefix(conversation);
    let messages = conversation.messages();
    let upto = messages.len().saturating_sub(window);
    if upto <= from {
        return;
    }

    let mut state = summary_state(conversation);
    let mut summary = state.summary;
    for line in messages[from..upto]
        .iter()
        .filter_map(|m| render_message(m))
    {
        if !summary.is_empty() {
            summary.push('\n');
        }
        summary.push_str(&line);
    }
    state.summary = truncate_summary(summary);
    let replaces = messages[..upto].to_vec();
    apply_summary(conversation, &replaces, &state);
}

/// Keeps the newest [`SUMMARY_MAX_BYTES`] of `summary`, marking the cut.
fn truncate_summary(summary: String) -> String {
    if summary.len() <= SUMMARY_MAX_BYTES {
        return summary;
    }
    let body = summary.strip_prefix(TRUNCATED).unwrap_or(&summary);
    let mut cut = body
        .len()
        .saturating_sub(SUMMARY_MAX_BYTES - TRUNCATED.len());
    while !body.is_char_boundary(cut) {
        cut += 1;
    }
    format!("{TRUNCATED}{}", &body[cut..])
}

fn render_message(message: &Message) -> Option<String> {
    let (role, text) = match message {
        Message::System { content } => ("system", content.clone()),
        Message::User { content } => {
            let parts: Vec<_> = content
                .iter()
                .filter_map(|part| match part {
                    UserContent::Text(text) => Some(text.text.as_str()),
                    _ => None,
                })
                .collect();
            ("user", parts.join(" "))
        }
        Message::Assistant(turn) => ("assistant", turn.text()),
    };
    (!text.is_empty()).then(|| format!("{role}: {text}"))
}

#[cfg(test)]
mod test {
    use super::{
        CONVERSATION_ID, SUMMARY_MAX_BYTES, TRUNCATED, answer_only, compact, history_len,
        rewind_history, seed_history, shared_history, truncate_summary,
    };
    use futures::executor::block_on;
    use rig::{
        conversation::Conversation,
        memory::{load_async, save_async},
        message::{AssistantContent, AssistantMessage, Message},
    };

    fn pairs(range: std::ops::Range<usize>) -> Vec<Message> {
        range
            .flat_map(|i| {
                [
                    Message::user(format!("ja {i}")),
                    Message::assistant(format!("en {i}")),
                ]
            })
            .collect()
    }

    fn thinking_turn(thought: &str, answer: &str) -> Message {
        Message::Assistant(AssistantMessage::new(vec![
            AssistantContent::reasoning(thought),
            AssistantContent::text(answer),
        ]))
    }

    fn summary_text(conversation: &Conversation) -> String {
        format!("{:?}", conversation.messages()[0])
    }

    #[test]
    fn compact_keeps_window_and_summarizes_rest() {
        let mut conversation = Conversation::from_messages(pairs(0..3));
        compact(&mut conversation, 2);
        let messages = conversation.messages();
        assert_eq!(messages.len(), 3);
        assert_eq!(*messages[1], Message::user("ja 2"));
        let Message::User { content } = &*messages[0] else {
            panic!("summary is a user message");
        };
        let summary = format!("{content:?}");
        assert!(summary.contains("user: ja 0") && summary.contains("assistant: en 1"));

        conversation.extend(pairs(3..4));
        compact(&mut conversation, 2);
        let messages = conversation.messages();
        assert_eq!(messages.len(), 3);
        assert_eq!(*messages[1], Message::user("ja 3"));
        let summary = format!("{:?}", messages[0]);
        assert!(summary.contains("user: ja 0") && summary.contains("assistant: en 2"));
    }

    #[test]
    fn compact_within_window_is_noop() {
        let mut conversation = Conversation::from_messages(pairs(0..2));
        compact(&mut conversation, 4);
        assert_eq!(conversation.messages().len(), 4);
    }

    #[test]
    fn compact_strips_reasoning_from_kept_turns() {
        let mut conversation = Conversation::from_messages([
            Message::user("ja 0"),
            thinking_turn("long reasoning", "en 0"),
        ]);
        compact(&mut conversation, 4);
        let messages = conversation.messages();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[1].as_ref(), &Message::assistant("en 0"));
    }

    #[test]
    fn compact_strips_think_tags_from_kept_turns() {
        let mut conversation = Conversation::from_messages([
            Message::user("ja 0"),
            Message::assistant("<think>long reasoning</think>\nen 0"),
        ]);
        compact(&mut conversation, 4);
        assert_eq!(
            conversation.messages()[1].as_ref(),
            &Message::assistant("en 0")
        );
    }

    #[test]
    fn summary_leaves_out_reasoning() {
        let mut conversation = Conversation::from_messages([
            Message::user("ja 0"),
            thinking_turn("long reasoning", "en 0"),
            Message::user("ja 1"),
            Message::assistant("<think>more reasoning</think>en 1"),
            Message::user("ja 2"),
            Message::assistant("en 2"),
        ]);
        compact(&mut conversation, 2);
        let summary = summary_text(&conversation);
        assert!(summary.contains("assistant: en 0") && summary.contains("assistant: en 1"));
        assert!(!summary.contains("reasoning"));
    }

    #[test]
    fn plain_turns_are_left_alone() {
        assert_eq!(
            answer_only(&AssistantMessage::new(vec![AssistantContent::text("en")])),
            None
        );
    }

    #[test]
    fn summary_stays_capped_as_history_grows() {
        let long = "訳".repeat(200);
        let mut conversation = Conversation::new();
        for i in 0..200 {
            conversation.extend([
                Message::user(format!("{i} {long}")),
                Message::assistant(format!("{i} {long}")),
            ]);
            compact(&mut conversation, 4);
            assert!(conversation.messages().len() <= 5);
        }
        let summary = summary_text(&conversation);
        assert!(summary.contains(TRUNCATED.trim_end()));
        // The summary message wraps the capped text in a short preamble.
        assert!(summary.len() < SUMMARY_MAX_BYTES + 512, "{}", summary.len());
        assert!(
            summary.contains("assistant: 195 "),
            "keeps the newest turns"
        );
    }

    #[test]
    fn rewinding_drops_a_failed_turn() {
        block_on(async {
            let history = shared_history(4);
            seed_history(&history, pairs(0..2)).await.unwrap();
            let len = history_len(&history).await.unwrap();

            // The agent saves the failed attempt's turn, as it would a good one.
            let mut conversation = load_async(&history.store, CONVERSATION_ID).await.unwrap();
            conversation.extend(pairs(2..3));
            save_async(&history.store, CONVERSATION_ID, &mut conversation)
                .await
                .unwrap();

            rewind_history(&history, len).await.unwrap();
            let conversation = load_async(&history.store, CONVERSATION_ID).await.unwrap();
            let kept: Vec<_> = conversation
                .messages()
                .iter()
                .map(|m| (**m).clone())
                .collect();
            assert_eq!(kept, pairs(0..2));
        });
    }

    #[test]
    fn truncate_keeps_newest_bytes() {
        let summary = "あ".repeat(SUMMARY_MAX_BYTES);
        let truncated = truncate_summary(summary);
        assert!(truncated.starts_with(TRUNCATED));
        assert!(truncated.len() <= SUMMARY_MAX_BYTES);
        assert_eq!(truncate_summary(truncated.clone()), truncated);
    }
}
