//! Opening and saving the tab's epub, glossary, translation and recovery file.

use crate::{
    dialogs::{self, complete, select_epub, select_glossary, select_recovery},
    epub::get_pages,
    error::TaskExt as _,
    page::{Page, absolute_page_path, check_all_pages, save_translation},
    screen::Translation,
};
use gpui_kit::{AppContext as _, ClickEvent, Context, Window};
use std::{collections::HashMap, mem, path::PathBuf};

impl Translation {
    fn set_epub(&mut self, path: impl Into<PathBuf>, pages: Vec<Page>) {
        self.current_page = 0;
        self.file_path = Some(path.into());
        self.pages = pages;
        self.glossary = None;
    }

    pub fn select_epub(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        cx.spawn(async move |this, cx| {
            if let Some(handle) = select_epub().await {
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
            if let Some(handle) = select_glossary().await {
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
            if let Some(handle) = dialogs::save_translation(name).await {
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
            let handle = select_recovery().await;
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

    /// Writes the pages next to the epub, for `recover` to load after a crash.
    pub(super) fn backup(&self, window: &Window, cx: &mut Context<Self>) {
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

    pub(super) fn show_complete(&self, cx: &mut Context<Self>) {
        let file_name = self.epub_name().unwrap_or_default().into_owned();
        cx.spawn(async move |_, _| complete(file_name).await)
            .detach();
    }
}
