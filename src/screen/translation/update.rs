use crate::{
    client::{
        Client, Reply, Settings, SharedHistory, compact_history, history_len, is_response_report,
        reply_stream, rewind_history, seed_history,
    },
    dialogs::{
        complete_dialog, save_translation_dialog, select_epub_dialog, select_glossary_dialog,
        select_recovery_dialog,
    },
    epub::get_pages,
    error::{Error, Result, TaskExt},
    page::{
        Activity, Job, Page, Section, absolute_page_path, check_all_pages, check_page_at,
        save_translation,
    },
    screen::{DisplayType, Translation},
    server::{ServerUpdate, TranslationRun},
};
use futures::StreamExt as _;
use gpui_kit::{AppContext as _, AsyncWindowContext, ClickEvent, Context, WeakEntity, Window};
use rig::{Agent, message::Message};
use std::{collections::HashMap, mem, path::PathBuf};

impl Translation {
    pub fn set_display(&mut self, display: DisplayType, cx: &mut Context<Self>) {
        self.display = display;
        cx.notify();
    }

    pub fn set_page(&mut self, page: usize, cx: &mut Context<Self>) {
        self.current_page = page;
        cx.notify();
    }

    fn set_epub(&mut self, path: impl Into<PathBuf>, pages: Vec<Page>) {
        self.current_page = 0;
        self.file_path = Some(path.into());
        self.pages = pages;
        self.glossary = None;
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

    pub fn load_models(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.server.connected() {
            return;
        }
        let client = self.server.client.clone();
        cx.spawn_in(window, async move |this, cx| {
            let models = client.get_models().await?;
            this.update_in(cx, |this, window, cx| {
                this.server.set_models(models, window, cx);
                cx.notify();
            })?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to load models", window, cx);
    }

    pub fn select_epub(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        cx.spawn(async move |this, cx| {
            if let Some(handle) = select_epub_dialog().await {
                let buffer = handle.read().await;
                let pages = cx.background_spawn(async { get_pages(buffer) }).await?;

                this.update(cx, |this, cx| {
                    if this.is_running() {
                        return;
                    }
                    this.set_epub(handle.path(), pages);
                    cx.notify();
                })?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to open epub", window, cx);
    }

    pub fn select_glossary(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        cx.spawn(async move |this, cx| {
            if let Some(handle) = select_glossary_dialog().await {
                let glossary = String::from_utf8(handle.read().await)?;
                this.update(cx, |view, cx| {
                    if view.is_running() {
                        return;
                    }
                    view.glossary = Some((handle.path().into(), glossary));
                    cx.notify();
                })?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to open glossary", window, cx);
    }

    pub fn save(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(name) = self.file_path.as_ref().and_then(|p| p.file_prefix()) else {
            return;
        };
        let name = name.to_string_lossy().into_owned();
        let pages = self.pages.clone();
        cx.background_spawn(async move {
            if let Some(handle) = save_translation_dialog(name).await {
                save_translation(&pages, handle.path()).await?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to save translation", window, cx)
    }

    pub fn recover(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        cx.spawn(async move |this, cx| {
            let handle = select_recovery_dialog().await;
            if let Some(handle) = handle {
                let json = handle.read().await;
                let pages: Vec<Page> = serde_json::from_slice(&json)?;
                this.update(cx, |this, cx| {
                    if this.is_running() {
                        return;
                    }
                    this.recover_pages(pages);
                    cx.notify();
                })?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to load recovery", window, cx);
    }

    pub fn recover_pages(&mut self, pages: Vec<Page>) {
        let mut sections: HashMap<PathBuf, _> = pages
            .into_iter()
            .map(|p| (absolute_page_path(p.path), p.sections))
            .collect();

        for page in self.pages.iter_mut() {
            if let Some(current) = sections.get_mut(&page.path) {
                for (x, y) in page.sections.iter_mut().zip(current) {
                    mem::swap(&mut x.content, &mut y.content);
                }
            }
        }

        self.pages
            .iter_mut()
            .for_each(|p| p.sections.iter_mut().for_each(|s| s.clean()));

        check_all_pages(&mut self.pages);
    }

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

    /// Readies a whole page: its requests, the history it continues, and the
    /// page itself, cleared for the new translation.
    fn prepare_page(&mut self, page: usize, cx: &mut Context<Self>) -> Result<TranslationRun> {
        let model = self.server.check_ready(cx)?;
        let glossary = self
            .glossary
            .as_ref()
            .map(|(_, glossary)| glossary.as_str());
        let pages = self
            .pages
            .get(..=page)
            .ok_or(Error::Error("No such page"))?;
        // Only a page that directly follows the history's can continue it.
        let carried = self
            .history
            .take()
            .filter(|(last, _)| last + 1 == page)
            .map(|(_, history)| history);
        let run = self.server.translate(pages, &model, glossary, carried)?;
        self.history = run.history.clone().map(|history| (page, history));
        self.pages[page].clear();
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
        let glossary = self
            .glossary
            .as_ref()
            .map(|(_, glossary)| glossary.as_str());
        let pages = self
            .pages
            .get(..=page)
            .ok_or(Error::Error("No such page"))?;
        let run = self.server.translate_part(pages, &model, part, glossary)?;
        self.history = None;
        let current = &mut self.pages[page];
        current.sections[part].content.clear();
        current.errors.clear();
        cx.notify();
        Ok(run)
    }

    fn set_activity(&mut self, page: usize, activity: Activity, cx: &mut Context<Self>) {
        if let Some(page) = self.pages.get_mut(page) {
            page.activity = activity;
            cx.notify();
        }
    }

    fn append_content(&mut self, page: usize, part: usize, text: &str, cx: &mut Context<Self>) {
        if let Some(section) = self.section_mut(page, part) {
            section.content.push_str(text);
            cx.notify();
        }
    }

    fn clean_section(&mut self, page: usize, part: usize, cx: &mut Context<Self>) {
        if let Some(section) = self.section_mut(page, part) {
            section.clean();
            cx.notify();
        }
    }

    fn clear_section(&mut self, page: usize, part: usize, cx: &mut Context<Self>) {
        if let Some(section) = self.section_mut(page, part) {
            section.content.clear();
            cx.notify();
        }
    }

    fn section_mut(&mut self, page: usize, part: usize) -> Option<&mut Section> {
        self.pages.get_mut(page)?.sections.get_mut(part)
    }

    /// Checks a translated page, then backs the book up next to the epub.
    fn complete_page(&mut self, page: usize, window: &Window, cx: &mut Context<Self>) {
        check_page_at(&mut self.pages, page);
        self.backup(window, cx);
        cx.notify();
    }

    /// Writes the pages next to the epub, for `recover` to load after a crash.
    fn backup(&self, window: &Window, cx: &mut Context<Self>) {
        let Some(path) = self.file_path.as_ref().map(|p| p.with_extension("json")) else {
            return;
        };
        let json = serde_json::to_string_pretty(&self.pages);
        cx.background_spawn(async move {
            smol::fs::write(path, json?).await?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to save recovery", window, cx);
    }

    fn show_complete(&self, cx: &mut Context<Self>) {
        let file_name = self.epub_name().unwrap_or_default().into_owned();
        cx.spawn(async move |_, _| complete_dialog(file_name).await)
            .detach();
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
        this.update_in(cx, |this, window, cx| this.complete_page(page, window, cx))?;
    }
    Ok(())
}

/// Streams the translation of each part of `run` into `page`, in order.
async fn translate_parts(
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
        this.update(cx, |this, cx| this.clean_section(page, part, cx))?;
        result?;
    }
    Ok(())
}

/// How one attempt at a part's reply ended.
enum Attempt {
    Complete,
    Truncated,
    Failed(Error),
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
            Attempt::Failed(error) if retried => return Err(error),
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
            // A bad chunk of the reply, but the server is fine: report it, pausing
            // the run until it's dismissed, then read the rest of the reply.
            Err(error) if last && is_response_report(&error) => {
                error.display_error("Translation error", cx).await
            }
            Err(error) => return Ok(Attempt::Failed(error)),
        }
    }
    Ok(if truncated {
        Attempt::Truncated
    } else {
        Attempt::Complete
    })
}

impl ServerUpdate for Translation {
    fn set_think(&mut self, index: &usize) {
        self.server.set_think(index);
    }

    fn set_method(&mut self, index: &usize) {
        self.server.set_method(index);
    }

    fn settings_mut(&mut self) -> &mut Settings {
        &mut self.server.settings
    }

    fn connect(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) {
        cx.spawn_in(window, async move |this, cx| {
            let client = Client::ollama();
            let models = client.get_models().await?;

            this.update_in(cx, |this, window, cx| {
                this.server.client = client;
                this.server.set_models(models, window, cx);
                cx.notify();
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to connect to server", window, cx);
    }
}
