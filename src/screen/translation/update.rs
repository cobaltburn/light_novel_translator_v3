//! Changes to the tab's pages as their translation streams in.

use crate::{
    page::{Activity, Section},
    screen::Translation,
};
use gpui_kit::Context;

impl Translation {
    pub(super) fn set_activity(&mut self, page: usize, activity: Activity, cx: &mut Context<Self>) {
        if let Some(page) = self.pages.get_mut(page) {
            page.activity = activity;
            cx.notify();
        }
    }

    pub(super) fn append_content(
        &mut self,
        page: usize,
        part: usize,
        text: &str,
        cx: &mut Context<Self>,
    ) {
        if let Some(section) = self.section_mut(page, part) {
            section.content.push_str(text);
            cx.notify();
        }
    }

    pub(super) fn clean_section(&mut self, page: usize, part: usize, cx: &mut Context<Self>) {
        if let Some(section) = self.section_mut(page, part) {
            section.clean();
            cx.notify();
        }
    }

    fn section_mut(&mut self, page: usize, part: usize) -> Option<&mut Section> {
        self.pages.get_mut(page)?.sections.get_mut(part)
    }
}
