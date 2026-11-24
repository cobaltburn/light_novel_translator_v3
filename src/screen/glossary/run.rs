//! Starting, stopping and running glossary passes. Each pass reads one chapter
//! and rewrites the glossary built so far, so it lands in that chapter whole.
//!
//! A run takes its chapters from the front of the queue one at a time: each
//! pass is streamed by a task of its own, which ends by sending
//! [`Message::ChapterDone`], and `update` then starts the next.

use super::actions::Run;
use crate::{
    client::{Reply, glossary_prompt, reply_stream},
    error::{Error, Result},
    message::SendAsync as _,
    page::Activity,
    screen::glossary::{Glossary, message::Message},
};
use futures::StreamExt as _;
use gpui_kit::{AsyncWindowContext, Context, WeakEntity, Window};
use rig::{Agent, message::Message as ChatMessage};
use std::collections::VecDeque;

const GLOSSARY_CUT_OFF: &str = "The model hit its limit partway through a glossary pass, so the \
     run stopped and kept the last complete pass. Try a model with a larger context window.";

impl Glossary {
    /// Starts the run `run` asks for, unless a run is in progress.
    pub(super) fn run(&mut self, run: Run, window: &mut Window, cx: &mut Context<Self>) {
        // Starting would cancel the run in progress.
        if self.is_running() {
            return;
        }
        let pending = match run {
            Run::All(i) => (i..self.chapters.len()).collect(),
            Run::Chapter(i) => VecDeque::from([i]),
        };
        self.start(pending, window, cx);
    }

    /// Starts a run over the `pending` chapters, replacing the one in progress.
    /// The task of the pass in progress is held in the server's handles, so
    /// [`Self::stop`] cancels the run by dropping it.
    pub(super) fn start(
        &mut self,
        pending: VecDeque<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
        self.next_chapter(window, cx);
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

    /// Starts a pass over the next queued chapter, or ends the run when none are left.
    fn next_chapter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(chapter) = self.pending.pop_front() else {
            return self.stop();
        };
        let (agent, prompt) = match self.prepare_chapter(chapter, cx) {
            Ok(prepared) => prepared,
            Err(error) => return self.fail(error, window, cx),
        };
        let pass = cx.spawn_in(window, async move |this, cx| {
            let result = stream_chapter(&this, &agent, prompt, chapter, cx).await;
            // This ends the pass, and its handling drops this task, so nothing
            // after it may await.
            this.send(cx, Message::ChapterDone { chapter, result })
        });
        // Dropping the last pass's task, which is the one that just ended.
        self.server.handles = vec![pass];
    }

    /// Completes the pass over `chapter` when it succeeded and moves on to the
    /// next, or ends the run when it failed. A run that ends on the book's last
    /// chapter has finished the glossary.
    pub(super) fn chapter_done(
        &mut self,
        chapter: usize,
        result: Result<()>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Err(error) = result {
            return self.fail(error, window, cx);
        }
        self.complete_chapter(chapter);
        if !self.pending.is_empty() {
            return self.next_chapter(window, cx);
        }
        self.stop();
        if chapter + 1 == self.chapters.len() {
            self.show_complete(cx);
        }
    }

    /// Ends the run, reporting the `error` that ended it.
    fn fail(&mut self, error: Error, window: &mut Window, cx: &mut Context<Self>) {
        self.stop();
        error.prompt("Glossary failed", window, cx);
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
        Ok((agent, prompt))
    }

    pub(super) fn append_content(&mut self, chapter: usize, text: &str) {
        if let Some(chapter) = self.chapters.get_mut(chapter) {
            chapter.content.push_str(text);
        }
    }

    fn complete_chapter(&mut self, chapter: usize) {
        if let Some(chapter) = self.chapters.get_mut(chapter) {
            chapter.activity = Activity::Complete;
        }
    }
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
    let mut replies = reply_stream(agent, ChatMessage::user(prompt));
    while let Some(reply) = replies.next().await {
        match reply? {
            Reply::Text(text) => this.send(cx, Message::AppendContent { chapter, text })?,
            Reply::Truncated => return Err(Error::Error(GLOSSARY_CUT_OFF)),
        }
    }
    Ok(())
}
