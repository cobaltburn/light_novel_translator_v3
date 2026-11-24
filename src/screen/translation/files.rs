//! Opening and saving the tab's epub, glossary, translation and recovery file.

use crate::{
    dialogs::{self, complete, save_file, select_epub, select_glossary, select_recovery},
    epub::get_pages,
    error::{Error, TaskExt as _},
    message::SendAsync as _,
    page::{Page, absolute_page_path, check_all_pages, page_markdown, save_translation},
    screen::{Translation, translation::message::Message},
};
use gpui_kit::{AppContext as _, Context, ExternalPaths, Window};
use smol::fs;
use std::{collections::HashMap, mem, path::PathBuf};

impl Translation {
    pub(super) fn set_pages(&mut self, path: impl Into<PathBuf>, pages: Vec<Page>) {
        self.current_page = 0;
        self.file_path = Some(path.into());
        self.pages = pages;
        self.glossary = None;
    }

    pub(super) fn select_epub(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        cx.spawn(async move |this, cx| {
            if let Some(handle) = select_epub().await {
                let buffer = handle.read().await;
                let pages = cx.background_spawn(async { get_pages(buffer) }).await?;
                this.send(cx, Message::EpubLoaded(handle.path().into(), pages))?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to open epub", window, cx);
    }

    pub(super) fn drop_epub(
        &self,
        paths: &ExternalPaths,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_running() || paths.paths().len() != 1 {
            return;
        }
        if let Some(path) = paths.paths().first().cloned()
            && path.is_file()
            && path.extension().is_some_and(|p| p == "epub")
        {
            cx.spawn(async move |this, cx| {
                let buffer = fs::read(&path).await?;
                let pages = cx.background_spawn(async { get_pages(buffer) }).await?;
                this.send(cx, Message::EpubLoaded(path, pages))?;
                Ok(())
            })
            .detach_and_prompt_err("Failed to open epub", window, cx);
        };
    }

    pub(super) fn select_glossary(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        cx.spawn(async move |this, cx| {
            if let Some(handle) = select_glossary().await {
                let glossary = String::from_utf8(handle.read().await)?;
                this.send(cx, Message::GlossaryLoaded(handle.path().into(), glossary))?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to open glossary", window, cx);
    }

    pub(super) fn save(&self, window: &mut Window, cx: &mut Context<Self>) {
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

    pub(super) fn recover(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        cx.spawn(async move |this, cx| {
            let handle = select_recovery().await;
            if let Some(handle) = handle {
                let json = handle.read().await;
                let pages: Vec<Page> = serde_json::from_slice(&json)?;
                this.send(cx, Message::Recovered(pages))?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to load recovery", window, cx);
    }

    pub(super) fn recover_pages(&mut self, pages: Vec<Page>) {
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

    /// Saves page `page` as markdown, wherever the user picks.
    pub(super) fn save_page(&self, page: usize, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(page) = self.pages.get(page) {
            let page = page.clone();
            cx.background_spawn(async move {
                let Some(name) = page.file_stem() else {
                    return Ok(());
                };

                if let Some(handle) = save_file(format!("{name}.md")).await {
                    let content = page_markdown(&page);
                    handle.write(content.as_ref()).await?;
                }
                Ok::<_, Error>(())
            })
            .detach_and_prompt_err("Failed to save page", window, cx);
        };
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
