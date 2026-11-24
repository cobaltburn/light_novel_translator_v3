//! Streams each part's translation into its section.

use crate::{
    client::{Reply, compact_history, is_response_report, reply_stream, seed_history},
    error::Result,
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
        let result = translate_part(this, &agent, &japanese, page, part, cx).await;
        this.update(cx, |this, cx| this.clean_section(page, part, cx))?;
        result?;
        this.update_in(cx, |this, window, cx| this.backup(window, cx))?;
    }
    Ok(())
}

/// Streams a part's translation into its section. A bad chunk is reported and
/// read past; any other error ends the run.
async fn translate_part(
    this: &WeakEntity<Translation>,
    agent: &Agent,
    japanese: &str,
    page: usize,
    part: usize,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    let mut replies = reply_stream(agent, Message::user(japanese));
    while let Some(reply) = replies.next().await {
        match reply {
            Ok(Reply::Text(text)) => {
                this.update(cx, |this, cx| this.append_content(page, part, &text, cx))?
            }
            Ok(Reply::Truncated) => log::warn!("Page {page} part {} was cut off", part + 1),
            Err(error) if is_response_report(&error) => {
                error.display_error("Translation error", cx).await
            }
            Err(error) => return Err(error),
        }
    }
    Ok(())
}
