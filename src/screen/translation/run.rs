//! Starting, stopping and queueing translation runs.

use super::stream::translate_parts;
use crate::{
    error::Result,
    page::{Activity, Job, Page, check_page_at},
    screen::Translation,
    server::TranslationRun,
};
use gpui_kit::{AsyncWindowContext, ClickEvent, Context, WeakEntity, Window};

impl Translation {
    pub fn translate(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let jobs = Job::run_from(self.current_page, self.pages.len());
        self.start(jobs, window, cx);
    }

    /// Starts a run over `jobs`, replacing the one in progress. The run is
    /// held in the server's handles, so [`Self::stop`] cancels it by dropping it.
    pub fn start(
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

        let run = cx.spawn_in(window, async move |this, cx| {
            let result = run_jobs(&this, cx).await;
            this.update_in(cx, |this, window, cx| {
                // This drops the run, so nothing after it may await.
                this.stop();
                if let Err(error) = result {
                    error.prompt("Translation failed", window, cx);
                }
                cx.notify();
            })?;
            Ok(())
        });
        self.server.handles.push(run);
        cx.notify();
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
        cx.notify();
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
        cx.notify();
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

    /// Checks a translated page.
    fn complete_page(&mut self, page: usize, cx: &mut Context<Self>) {
        check_page_at(&mut self.pages, page);
        cx.notify();
    }
}

/// Runs the queued jobs one at a time until none are left.
async fn run_jobs(this: &WeakEntity<Translation>, cx: &mut AsyncWindowContext) -> Result<()> {
    while let Some(job) = this.update(cx, |this, _| this.pending.pop_front())? {
        let (page, run) = match job {
            Job::Finish => {
                this.update(cx, |this, cx| this.show_complete(cx))?;
                continue;
            }
            Job::Page(page) => (
                page,
                this.update(cx, |this, cx| this.prepare_page(page, cx))??,
            ),
            Job::Part { page, part } => (
                page,
                this.update(cx, |this, cx| this.prepare_part(page, part, cx))??,
            ),
        };
        translate_parts(this, page, run, cx).await?;
        this.update(cx, |this, cx| this.complete_page(page, cx))?;
    }
    Ok(())
}
