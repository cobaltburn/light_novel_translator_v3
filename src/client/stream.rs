use crate::error::{Error, Result};
use futures::{Stream, StreamExt as _, future};
use rig::{
    Agent, ErrorKind,
    agent::MultiTurnStreamItem,
    completion::FinishReason,
    message::Message,
    run::{CompletionCall, PromptError},
    streaming::{Item, StreamEvent},
};

pub enum Reply {
    Text(String),
    Truncated,
}

/// Prompts `agent` and yields its reply as it arrives.
pub fn reply_stream(agent: &Agent, prompt: Message) -> impl Stream<Item = Result<Reply>> + use<> {
    agent
        .prompt(prompt)
        .stream()
        .filter_map(|item| future::ready(reply(item)))
}

fn reply(item: std::result::Result<MultiTurnStreamItem, PromptError>) -> Option<Result<Reply>> {
    match item {
        Ok(MultiTurnStreamItem::StreamAssistantItem(Item::Event(StreamEvent::Text {
            text,
            ..
        }))) if !text.is_empty() => Some(Ok(Reply::Text(text))),
        Ok(MultiTurnStreamItem::CompletionCall(call)) if is_truncated(&call) => {
            Some(Ok(Reply::Truncated))
        }
        Ok(_) => None,
        Err(error) => Some(Err(error.into())),
    }
}

fn is_truncated(call: &CompletionCall) -> bool {
    call.finish_reason
        .as_ref()
        .is_some_and(FinishReason::truncated_output)
}

/// An error in the model's reply to one prompt, rather than in reaching the
/// server, so the run can carry on past it.
pub fn is_response_report(error: &Error) -> bool {
    let Error::PromptError(error) = error else {
        return false;
    };
    matches!(&**error, PromptError::Report(report) if report.kind == ErrorKind::Response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rig::completion::Usage;
    use serde_json::Value;

    fn finished(reason: Option<FinishReason>) -> CompletionCall {
        let mut call = CompletionCall::new(0, Usage::default(), Value::Null);
        call.finish_reason = reason;
        call
    }

    #[test]
    fn a_reply_cut_off_by_the_context_limit_is_truncated() {
        assert!(is_truncated(&finished(Some(FinishReason::Length))));
    }

    #[test]
    fn a_reply_that_ended_or_gave_no_reason_is_not_truncated() {
        assert!(!is_truncated(&finished(Some(FinishReason::Stop))));
        assert!(!is_truncated(&finished(None)));
    }
}
