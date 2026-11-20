use crate::{
    page::Page,
    widget::page::{DisplayType, section_div},
};
use gpui_kit::{Context, IntoElement, Window};

/// What clicking a section's label does.
pub trait PageAction: Sized + 'static {
    fn start_section(
        &mut self,
        page: usize,
        part: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    );
}

impl Page {
    /// Each section of the page at index `page`, as `display` shows it.
    pub fn text<T: PageAction>(
        &self,
        display: DisplayType,
        page: usize,
        cx: &Context<T>,
    ) -> impl Iterator<Item = impl IntoElement> {
        let name = self
            .path
            .file_name()
            .map_or_default(|e| e.to_string_lossy());

        let on_click = move |part| {
            cx.listener(move |this, _, window, cx| this.start_section(page, part, window, cx))
        };

        self.sections
            .iter()
            .enumerate()
            .map(move |(i, s)| section_div(&name, i, display.section_text(s), on_click(i)))
    }
}
