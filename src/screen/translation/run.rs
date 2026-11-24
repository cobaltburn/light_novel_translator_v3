//! Starting, stopping and queueing translation runs.
//!
//! A run takes its jobs from the front of the queue one at a time: each job
//! is streamed by a task of its own, which ends by sending
//! [`Message::JobDone`], and `update` then starts the next.

use super::stream::translate_parts;
use crate::{
    error::{Error, Result},
    message::SendAsync as _,
    page::{Activity, Job, Page, check_page_at},
    screen::translation::{Translate, Translation, message::Message},
    server::TranslationRun,
};
use gpui_kit::{Context, Window};

impl Translation {
    /// Starts the run `translate` asks for, unless a run is in progress.
    pub(super) fn translate(
        &mut self,
        translate: Translate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Starting would cancel the run in progress.
        if self.is_running() {
            return;
        }
        let jobs = match translate {
            Translate::All(page) => Job::run_from(page, self.pages.len()).collect(),
            Translate::Page(page) => vec![Job::Page(page)],
            Translate::Part { page, part } => vec![Job::Part { page, part }],
            Translate::Errors => self.error_jobs(),
            Translate::PageErrors(page) => self.pages(page).map_or_default(|p| p.error_jobs(page)),
        };
        self.start(jobs, window, cx);
    }

    /// Starts a run over `jobs`, replacing the one in progress. The task of the
    /// job in progress is held in the server's handles, so [`Self::stop`]
    /// cancels the run by dropping it.
    pub(super) fn start(
        &mut self,
        jobs: impl IntoIterator<Item = Job>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Err(error) = self.server.check_ready(cx) {
            error.prompt("Can't translate", window, cx);
            return;
        }
        self.stop();
        self.pending = jobs.into_iter().collect();
        self.next_job(window, cx);
    }

    /// Cancels the run, if any.
    pub fn stop(&mut self) {
        self.pending.clear();
        self.history = None;
        self.server.abort();
        if let Some(page) = self.pages.iter().position(Page::active) {
            check_page_at(&mut self.pages, page);
        }
    }

    /// Starts the next queued job, or ends the run when none are left.
    fn next_job(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (page, prepared) = loop {
            match self.pending.pop_front() {
                None => return self.stop(),
                Some(Job::Finish) => self.show_complete(cx),
                Some(Job::Page(page)) => break (page, self.prepare_page(page, cx)),
                Some(Job::Part { page, part }) => {
                    break (page, self.prepare_part(page, part, cx));
                }
            }
        };
        let run = match prepared {
            Ok(run) => run,
            Err(error) => return self.fail(error, window, cx),
        };
        let job = cx.spawn_in(window, async move |this, cx| {
            let result = translate_parts(&this, page, run, cx).await;
            // This ends the job, and its handling drops this task, so nothing
            // after it may await.
            this.send(cx, Message::JobDone { page, result })
        });
        // Dropping the last job's task, which is the one that just ended.
        self.server.handles = vec![job];
    }

    /// Completes the job over `page` when it succeeded and moves on to the
    /// next, or ends the run when it failed.
    pub(super) fn job_done(
        &mut self,
        page: usize,
        result: Result<()>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match result {
            Ok(()) => {
                check_page_at(&mut self.pages, page);
                self.next_job(window, cx);
            }
            Err(error) => self.fail(error, window, cx),
        }
    }

    /// Ends the run, reporting the `error` that ended it.
    fn fail(&mut self, error: Error, window: &mut Window, cx: &mut Context<Self>) {
        self.stop();
        error.prompt("Translation failed", window, cx);
    }

    /// Readies a whole page: its requests, the history it continues, and the
    /// page itself, cleared for the new translation.
    fn prepare_page(&mut self, page: usize, cx: &mut Context<Self>) -> Result<TranslationRun> {
        let model = self.server.check_ready(cx)?;
        // Only a page that directly follows the history's can continue it.
        let carried = self
            .history
            .take()
            .filter(|(last, _)| last + 1 == page)
            .map(|(_, history)| history);
        let run = self.server.translate(
            self.pages_through(page)?,
            &model,
            self.glossary_text(),
            carried,
        )?;
        self.history = run.history.clone().map(|history| (page, history));
        self.pages[page].clear();
        self.mark_active(page, &run);
        Ok(run)
    }

    /// Readies one part of a page, as [`Self::prepare_page`] does a whole one.
    fn prepare_part(
        &mut self,
        page: usize,
        part: usize,
        cx: &mut Context<Self>,
    ) -> Result<TranslationRun> {
        let model = self.server.check_ready(cx)?;
        let run = self.server.translate_part(
            self.pages_through(page)?,
            &model,
            part,
            self.glossary_text(),
        )?;
        self.history = None;
        let current = &mut self.pages[page];
        current.sections[part].content.clear();
        current.errors.retain(|e| !e.depends_on(part));
        self.mark_active(page, &run);
        Ok(run)
    }

    /// Marks `page` active at the first of `run`'s parts as soon as the page is
    /// cleared, so [`Self::stop`] re-checks it even if the run is cancelled
    /// before that part starts streaming (while the history is being seeded).
    fn mark_active(&mut self, page: usize, run: &TranslationRun) {
        if let Some(&(part, _)) = run.parts.first() {
            self.pages[page].activity = Activity::Active(part + 1);
        }
    }
}
