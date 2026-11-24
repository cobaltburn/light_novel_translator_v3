use crate::{
    dialogs::{glossary_complete, save_file, select_epub},
    epub::get_chapters,
    error::{Error, TaskExt as _},
    message::SendAsync as _,
    page::Chapter,
    screen::glossary::{Glossary, message::Message},
    text::remove_think_tags,
};
use gpui_kit::{AppContext as _, Context, Window};
use std::path::PathBuf;

impl Glossary {
    pub(super) fn select_epub(&self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        cx.spawn(async move |this, cx| {
            if let Some(handle) = select_epub().await {
                let buffer = handle.read().await;
                let chapters = cx.background_spawn(async { get_chapters(buffer) }).await?;
                this.send(cx, Message::EpubLoaded(handle.path().into(), chapters))?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to open epub", window, cx);
    }

    pub(super) fn set_epub(&mut self, path: PathBuf, chapters: Vec<Chapter>) {
        self.stop();
        self.file_path = Some(path);
        self.chapters = chapters;
    }

    pub(super) fn save(&self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(name) = self.file_stem() else {
            return;
        };
        let Some(chapter) = self.chapters.iter().rfind(|c| !c.content.is_empty()) else {
            return;
        };
        let file_name = format!("{name}.md");
        let content = remove_think_tags(&chapter.content);
        cx.background_spawn(async move {
            if let Some(handle) = save_file(file_name).await {
                handle.write(content.as_bytes()).await?;
            }
            Ok::<_, Error>(())
        })
        .detach_and_prompt_err("Failed to save glossary", window, cx);
    }

    /// Saves chapter `chapter`'s glossary as markdown, wherever the user picks.
    pub(super) fn save_chapter(&self, chapter: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(chapter) = self.chapters.get(chapter).cloned() else {
            return;
        };
        cx.background_spawn(async move {
            if let Some(name) = chapter.file_stem()
                && let Some(handle) = save_file(format!("{name}.md")).await
            {
                handle.write(chapter.content.as_ref()).await?;
            }
            Ok::<_, Error>(())
        })
        .detach_and_prompt_err("Failed to save chapter", window, cx);
    }

    pub(super) fn show_complete(&self, cx: &mut Context<Self>) {
        let file_name = self.epub_name().unwrap_or_default().into_owned();
        cx.spawn(async move |_, _| glossary_complete(file_name).await)
            .detach();
    }
}
