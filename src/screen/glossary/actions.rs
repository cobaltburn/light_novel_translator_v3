use crate::{
    dialogs::save_file,
    error::{Error, TaskExt as _},
    screen::Glossary,
    widget::SidebarAction,
};
use gpui_kit::{Action, AppContext as _, Context, Window};
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = glossary, no_json)]
pub struct SaveChapter {
    pub chapter: usize,
}

#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = glossary, no_json)]
pub enum Run {
    /// From the chapter through to the end of the book.
    All(usize),
    /// The chapter alone.
    Chapter(usize),
}

impl Glossary {
    pub fn save_chapter(
        &mut self,
        action: &SaveChapter,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(chapter) = self.chapters.get(action.chapter).cloned() else {
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

    pub fn run_action(&mut self, action: &Run, window: &mut Window, cx: &mut Context<Self>) {
        let pending = match *action {
            Run::All(i) => (i..self.chapters.len()).collect(),
            Run::Chapter(i) => VecDeque::from([i]),
        };
        self.start(pending, window, cx);
    }
}

impl SidebarAction for Glossary {
    const RUN_LABELS: [&'static str; 2] = ["run", "run chapter"];

    fn save_action(chapter: usize) -> Box<dyn Action> {
        Box::new(SaveChapter { chapter })
    }

    fn run_all_action(chapter: usize) -> Box<dyn Action> {
        Box::new(Run::All(chapter))
    }

    fn run_one_action(chapter: usize) -> Box<dyn Action> {
        Box::new(Run::Chapter(chapter))
    }

    fn select(&mut self, chapter: usize, cx: &mut Context<Self>) {
        self.current_chapter = chapter;
        cx.notify();
    }
}
