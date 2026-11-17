use crate::{
    page::Job,
    screen::{DisplayType, Translation},
};
use gpui_kit::{Action, Context, Window};

#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = translation, no_json)]
pub enum SetDisplay {
    Full,
    End,
    Japanese,
}

#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = translation, no_json)]
pub struct SavePage {
    pub page: usize,
}

#[derive(Clone, PartialEq, Debug, Action)]
#[action(namespace = translation, no_json)]
pub enum Translate {
    All(usize),
    Page(usize),
    Part { page: usize, part: usize },
}

impl Translation {
    pub fn set_display_action(
        &mut self,
        action: &SetDisplay,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.display = match action {
            SetDisplay::Full => DisplayType::Full,
            SetDisplay::End => DisplayType::End,
            SetDisplay::Japanese => DisplayType::Japanese,
        };
        cx.notify();
    }

    pub fn save_page(&mut self, action: &SavePage, _: &mut Window, _cx: &mut Context<Self>) {
        let Some(_page) = self.pages.get(action.page) else {
            return;
        };
        // TODO: save `_page`.
    }

    pub fn translate_action(
        &mut self,
        action: &Translate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let jobs = match *action {
            Translate::All(page) => Job::run_from(page, self.pages.len()).collect(),
            Translate::Page(page) => vec![Job::Page(page)],
            Translate::Part { page, part } => vec![Job::Part { page, part }],
        };
        self.start(jobs, window, cx);
    }
}
