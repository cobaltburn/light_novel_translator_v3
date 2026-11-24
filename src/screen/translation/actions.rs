use crate::{
    dialogs::save_file,
    error::{Error, TaskExt as _},
    page::{Job, page_markdown},
    screen::Translation,
    widget::{
        SidebarAction,
        page::{DisplayType, PageAction, PageSidebarAction},
    },
};
use gpui_kit::{Action, AppContext, Context, Window};

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

impl Translation {
    pub fn set_display_action(
        &mut self,
        action: &SetDisplay,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.display = action.0;
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
        // Starting would cancel the run in progress.
        if self.is_running() {
            return;
        }
        let jobs = match *action {
            Translate::All(page) => Job::run_from(page, self.pages.len()).collect(),
            Translate::Page(page) => vec![Job::Page(page)],
            Translate::Part { page, part } => vec![Job::Part { page, part }],
            Translate::Errors => self.error_jobs(),
            Translate::PageErrors(page) => self.pages(page).map_or_default(|p| p.error_jobs(page)),
        };
        self.start(jobs, window, cx);
    }
}

impl PageAction for Translation {
    fn start_section(
        &mut self,
        page: usize,
        part: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.translate_action(&Translate::Part { page, part }, window, cx);
    }

    fn start_errors(&mut self, page: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.translate_action(&Translate::PageErrors(page), window, cx);
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

    fn select(&mut self, page: usize, cx: &mut Context<Self>) {
        self.current_page = page;
        cx.notify();
    }
}

impl PageSidebarAction for Translation {
    fn section_action(page: usize, part: usize) -> Box<dyn Action> {
        Box::new(Translate::Part { page, part })
    }
}
