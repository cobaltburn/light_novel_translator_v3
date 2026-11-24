use crate::{
    message::{Update, send},
    page::{ErrorType, Page},
    text::japanese_ranges,
    widget::page::{DisplayType, section_div},
};
use gpui_kit::{ClickEvent, Context, IntoElement};

pub trait PageAction: Update {
    fn start_section(page: usize, part: usize) -> Self::Message;

    fn start_errors(page: usize) -> Self::Message;
}

impl Page {
    /// Each section of the page at index `page`, as `display` shows it. When the
    /// translation is shown, sections that failed the Japanese check have their
    /// Japanese characters highlighted. The section labels can't be clicked while
    /// `running`, since starting a section cancels the current run.
    pub fn text<T: PageAction>(
        &self,
        display: DisplayType,
        page: usize,
        running: bool,
        cx: &Context<T>,
    ) -> impl Iterator<Item = impl IntoElement> {
        let name = self
            .path
            .file_name()
            .map_or_default(|e| e.to_string_lossy());

        let japanese_error = |part| {
            self.errors
                .iter()
                .any(|e| e.index == part && matches!(e.error, ErrorType::Japanese))
        };

        self.sections.iter().enumerate().map(move |(i, s)| {
            let text = display.section_text(s);
            let shows_translation = matches!(display, DisplayType::Full | DisplayType::End);
            let highlights = if shows_translation && japanese_error(i) {
                japanese_ranges(&text)
            } else {
                Vec::new()
            };
            let on_click =
                (!running).then(|| send(cx, move |_: &ClickEvent| T::start_section(page, i)));
            section_div(&name, i, text, highlights, on_click)
        })
    }
}
