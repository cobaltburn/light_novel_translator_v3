use crate::{
    dialogs::save_file,
    error::{Error, TaskExt as _},
    page::{Job, page_markdown},
    screen::{DisplayType, Translation},
};
use gpui_kit::{Action, AppContext, Context, Window};

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
pub struct ScrollTo {
    pub part: usize,
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

    pub fn save_page(&mut self, action: &SavePage, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(page) = self.pages.get(action.page) {
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

    pub fn scroll_to(&mut self, action: &ScrollTo, _: &mut Window, cx: &mut Context<Self>) {
        self.scroll_handle.scroll_to_top_of_item(action.part);
        cx.notify();
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
