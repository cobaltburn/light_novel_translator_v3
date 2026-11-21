use crate::{
    page::{ErrorType, Page},
    text::japanese_ranges,
    widget::page::{DisplayType, section_div},
};
use gpui_kit::{App, ClickEvent, Context, IntoElement, Window};

/// What clicking a section's label or error card does.
pub trait PageAction: Sized + 'static {
    fn start_section(
        &mut self,
        page: usize,
        part: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    );
}

/// A click handler that calls [`PageAction::start_section`] for section `part` of page `page`.
///
/// This does what `cx.listener` does, but builds the closure itself: the type
/// `cx.listener` returns keeps `cx` borrowed, and this one doesn't.
pub fn start_section_listener<T: PageAction>(
    cx: &Context<T>,
    page: usize,
    part: usize,
) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static + use<T> {
    let view = cx.entity().downgrade();
    move |_, window, cx| {
        view.update(cx, |this, cx| this.start_section(page, part, window, cx))
            .ok();
    }
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
            let on_click = (!running).then(|| start_section_listener(cx, page, i));
            section_div(&name, i, text, highlights, on_click)
        })
    }
}
