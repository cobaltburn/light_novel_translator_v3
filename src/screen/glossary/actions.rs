//! The glossary screen's GPUI actions, which its menus dispatch. Each one is
//! sent on to [`crate::message::Update::update`] as a [`Message`].

use crate::{
    screen::glossary::{Glossary, message::Message},
    widget::SidebarAction,
};
use gpui_kit::Action;

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

    fn select(chapter: usize) -> Message {
        Message::SelectChapter(chapter)
    }
}
