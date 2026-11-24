//! The tab's GPUI actions, which its menus dispatch. Each one is sent on to
//! [`crate::message::Update::update`] as a [`Message`].

use crate::{
    screen::translation::{Translation, message::Message},
    widget::{
        SidebarAction,
        page::{DisplayType, PageAction, PageSidebarAction},
    },
};
use gpui_kit::Action;

#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = translation, no_json)]
pub struct SetDisplay(pub DisplayType);

#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = translation, no_json)]
pub struct SavePage {
    pub page: usize,
}

#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = translation, no_json)]
pub struct ScrollTo {
    pub part: usize,
}

#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = translation, no_json)]
pub enum Translate {
    All(usize),
    Page(usize),
    Part { page: usize, part: usize },
    Errors,
    PageErrors(usize),
}

impl PageAction for Translation {
    fn start_section(page: usize, part: usize) -> Message {
        Message::Translate(Translate::Part { page, part })
    }

    fn start_errors(page: usize) -> Message {
        Message::Translate(Translate::PageErrors(page))
    }
}

impl SidebarAction for Translation {
    const RUN_LABELS: [&'static str; 2] = ["translate", "translate page"];

    fn save_action(page: usize) -> Box<dyn Action> {
        Box::new(SavePage { page })
    }

    fn run_all_action(page: usize) -> Box<dyn Action> {
        Box::new(Translate::All(page))
    }

    fn run_one_action(page: usize) -> Box<dyn Action> {
        Box::new(Translate::Page(page))
    }

    fn select(page: usize) -> Message {
        Message::SelectPage(page)
    }
}

impl PageSidebarAction for Translation {
    fn section_action(page: usize, part: usize) -> Box<dyn Action> {
        Box::new(Translate::Part { page, part })
    }
}
