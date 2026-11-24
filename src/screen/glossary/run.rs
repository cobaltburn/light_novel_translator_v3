//! Starting, stopping and running glossary passes. Each pass reads one chapter
//! and rewrites the glossary built so far, so it lands in that chapter whole.

use crate::{
    client::{Reply, glossary_prompt, reply_stream},
    error::{Error, Result},
    page::Activity,
    screen::Glossary,
};
use futures::StreamExt as _;
use gpui_kit::{AsyncWindowContext, ClickEvent, Context, WeakEntity, Window};
use rig::{Agent, message::Message};
use std::collections::VecDeque;

const GLOSSARY_CUT_OFF: &str = "The model hit its limit partway through a glossary pass, so the \
     run stopped and kept the last complete pass. Try a model with a larger context window.";

impl Glossary {
    pub fn run(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let pending = (self.current_chapter..self.chapters.len()).collect();
        self.start(pending, window, cx);
    }

    /// Starts a run over the `pending` chapters, replacing the one in progress. The
    /// run is held in the server's handles, so [`Self::stop`] cancels it by dropping it.
    pub fn start(&mut self, pending: VecDeque<usize>, window: &mut Window, cx: &mut Context<Self>) {
        if let Err(error) = self.server.check_ready(cx) {
            error.prompt("Can't run glossary", window, cx);
            return;
        }
        if pending.is_empty() {
            Error::Error("No text to read in this epub").prompt("Can't run glossary", window, cx);
            return;
        }
        self.stop();
        self.pending = pending;

        let run = cx.spawn_in(window, async move |this, cx| {
            let result = run_chapters(&this, cx).await;
            this.update_in(cx, |this, window, cx| {
                this.stop();
                match result {
                    Ok(true) => this.show_complete(cx),
                    Ok(false) => {}
                    Err(error) => error.prompt("Glossary failed", window, cx),
                }
                cx.notify();
            })?;
            Ok(())
        });
        self.server.handles.push(run);
        cx.notify();
    }

    /// Cancels the run, if any. A pass cut short gives way to the last complete
    /// glossary, because each pass rewrites it whole.
    pub fn stop(&mut self) {
        let chapter = self
            .chapters
            .iter_mut()
            .find(|c| matches!(c.activity, Activity::Active(_)));
        if let Some(chapter) = chapter {
            chapter.activity = Activity::Incomplete;
        }
        self.pending.clear();
        self.server.abort();
    }

    /// Readies a pass over `chapter`: the agent, and the prompt holding the
    /// chapter's text and the glossary the chapter before it ended with.
    fn prepare_chapter(
        &mut self,
        chapter: usize,
        cx: &mut Context<Self>,
    ) -> Result<(Agent, String)> {
        let model = self.server.check_ready(cx)?;
        let current = self
            .chapters
            .get(chapter)
            .ok_or(Error::Error("No such chapter"))?;
        let existing = chapter
            .checked_sub(1)
            .and_then(|prior| self.chapters.get(prior))
            .map(|prior| prior.content.as_str());
        let prompt = glossary_prompt(existing, &current.text);
        let agent = self
            .server
            .client
            .glossary_agent(&model, &self.server.settings)?;

        let current = &mut self.chapters[chapter];
        current.content.clear();
        current.activity = Activity::Active(1);
        cx.notify();
        Ok((agent, prompt))
    }

    fn append_content(&mut self, chapter: usize, text: &str, cx: &mut Context<Self>) {
        if let Some(chapter) = self.chapters.get_mut(chapter) {
            chapter.content.push_str(text);
            cx.notify();
        }
    }

    fn complete_chapter(&mut self, chapter: usize, cx: &mut Context<Self>) {
        if let Some(chapter) = self.chapters.get_mut(chapter) {
            chapter.activity = Activity::Complete;
            cx.notify();
        }
    }
}

/// Runs a pass over each queued chapter in turn until none are left. True when
/// the last pass was over the book's last chapter, finishing the glossary.
async fn run_chapters(this: &WeakEntity<Glossary>, cx: &mut AsyncWindowContext) -> Result<bool> {
    let mut finished = false;
    while let Some(chapter) = this.update(cx, |this, _| this.pending.pop_front())? {
        let (agent, prompt) = this.update(cx, |this, cx| this.prepare_chapter(chapter, cx))??;
        stream_chapter(this, &agent, prompt, chapter, cx).await?;
        finished = this.update(cx, |this, cx| {
            this.complete_chapter(chapter, cx);
            chapter + 1 == this.chapters.len()
        })?;
    }
    Ok(finished)
}

/// Streams one pass's glossary into `chapter`. A pass cut off by the model's
/// limit is a half-written glossary, so it ends the run like any error.
async fn stream_chapter(
    this: &WeakEntity<Glossary>,
    agent: &Agent,
    prompt: String,
    chapter: usize,
    cx: &mut AsyncWindowContext,
) -> Result<()> {
    let mut replies = reply_stream(agent, Message::user(prompt));
    while let Some(reply) = replies.next().await {
        match reply? {
            Reply::Text(text) => {
                this.update(cx, |this, cx| this.append_content(chapter, &text, cx))?
            }
            Reply::Truncated => return Err(Error::Error(GLOSSARY_CUT_OFF)),
        }
    }
    Ok(())
}
