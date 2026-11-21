use crate::{
    dialogs::{glossary_complete, save_file, select_epub},
    epub::get_chapters,
    error::{Error, TaskExt as _},
    page::Chapter,
    screen::Glossary,
    text::remove_think_tags,
};
use gpui_kit::{AppContext as _, ClickEvent, Context, Window};
use std::path::PathBuf;

impl Glossary {
    pub fn select_epub(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        cx.spawn(async move |this, cx| {
            if let Some(handle) = select_epub().await {
                let buffer = handle.read().await;
                let chapters = cx.background_spawn(async { get_chapters(buffer) }).await?;

                this.update(cx, |this, cx| {
                    if !this.is_running() {
                        this.set_epub(handle.path().to_path_buf(), chapters);
                        cx.notify();
                    }
                })?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to open epub", window, cx);
    }

    pub fn set_epub(&mut self, path: PathBuf, chapters: Vec<Chapter>) {
        self.stop();
        self.file_path = Some(path);
        self.chapters = chapters;
    }

    pub fn save(&mut self, _: &ClickEvent, window: &mut Window, cx: &mut Context<Self>) {
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

    pub(super) fn show_complete(&self, cx: &mut Context<Self>) {
        let file_name = self.epub_name().unwrap_or_default().into_owned();
        cx.spawn(async move |_, _| glossary_complete(file_name).await)
            .detach();
    }
}
