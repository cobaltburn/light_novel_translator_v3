//! Streams each part's translation into its section, retrying a failed reply once.

use crate::{
    client::{
        Reply, SharedHistory, compact_history, history_len, is_response_report, reply_stream,
        rewind_history, seed_history,
    },
    error::{Error, Result},
    page::Activity,
    screen::Translation,
    server::TranslationRun,
};
use futures::StreamExt as _;
use gpui_kit::{AsyncWindowContext, WeakEntity};
use rig::{Agent, message::Message};

/// Streams the translation of each part of `run` into `page`, in order.
pub(super) async fn translate_parts(
    this: &WeakEntity<Translation>,
    page: usize,
    run: TranslationRun,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    let TranslationRun {
        agent,
        history,
        seed,
        parts,
    } = run;
    // A history that fails to update is reported, and the run carries on.
    if let Some(history) = &history
        && let Err(error) = seed_history(history, seed).await
    {
        error.display_error("Failed to seed history", cx).await;
    }

    for (part, japanese) in parts {
        this.update(cx, |this, cx| {
            this.set_activity(page, Activity::Active(part + 1), cx)
        })?;
        if let Some(history) = &history
            && let Err(error) = compact_history(history).await
        {
            error.display_error("Failed to compact history", cx).await;
        }
        let result =
            translate_part(this, &agent, history.as_ref(), &japanese, page, part, cx).await;
        // TODO think about adding saving back up after every complete section
        this.update(cx, |this, cx| this.clean_section(page, part, cx))?;
        result?;
    }
    Ok(())
}

/// How one attempt at a part's reply ended.
enum Attempt {
    Complete,
    Truncated,
    Failed(Box<Error>),
}

/// Streams a part's translation into its section. A reply that fails or is
/// cut off is thrown away, along with its turn in the history, and tried once more.
async fn translate_part(
    this: &WeakEntity<Translation>,
    agent: &Agent,
    history: Option<&SharedHistory>,
    japanese: &str,
    page: usize,
    part: usize,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    // Where the history stood before the first attempt.
    let mark = match history {
        Some(history) => match history_len(history).await {
            Ok(len) => Some((history, len)),
            Err(error) => {
                error.display_error("Failed to read history", cx).await;
                None
            }
        },
        None => None,
    };

    let mut retried = false;
    loop {
        let reason = match stream_part(this, agent, japanese, page, part, retried, cx).await? {
            Attempt::Complete => return Ok(()),
            Attempt::Truncated if retried => return Ok(()),
            Attempt::Failed(error) if retried => return Err(*error),
            Attempt::Truncated => "was cut off".to_string(),
            Attempt::Failed(error) => format!("failed: {error}"),
        };
        log::warn!("Page {page} part {} {reason}, retrying", part + 1);
        retried = true;

        this.update(cx, |this, cx| this.clear_section(page, part, cx))?;
        if let Some((history, len)) = mark
            && let Err(error) = rewind_history(history, len).await
        {
            error.display_error("Failed to rewind history", cx).await;
        }
    }
}

/// Streams one attempt at a part's reply into its section. Only the `last`
/// attempt reports a bad chunk and reads on past it; an earlier one gives up
/// on any error, to be retried.
async fn stream_part(
    this: &WeakEntity<Translation>,
    agent: &Agent,
    japanese: &str,
    page: usize,
    part: usize,
    last: bool,
    cx: &mut AsyncWindowContext,
) -> Result<Attempt> {
    let mut replies = reply_stream(agent, Message::user(japanese));
    let mut truncated = false;
    while let Some(reply) = replies.next().await {
        match reply {
            Ok(Reply::Text(text)) => {
                this.update(cx, |this, cx| this.append_content(page, part, &text, cx))?
            }
            Ok(Reply::Truncated) => truncated = true,
            Err(error) if last && is_response_report(&error) => {
                error.display_error("Translation error", cx).await
            }
            Err(error) => return Ok(Attempt::Failed(Box::new(error))),
        }
    }
    Ok(if truncated {
        Attempt::Truncated
    } else {
        Attempt::Complete
    })
}
