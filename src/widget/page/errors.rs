use crate::{
    page::{ErrorType, Page, PageError},
    widget::page::{PageAction, start_section_listener},
};
use gpui_kit::{
    Context, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, SharedString,
    StatefulInteractiveElement as _, Styled as _,
    base::{Disableable as _, v_flex},
    component::{
        ActiveTheme as _,
        button::{Button, ButtonVariants as _},
    },
    px,
};
use std::ops::Not;

impl Page {
    /// A floating list of the page's empty sections and failed checks, or
    /// `None` when it has neither. Clicking a card re-runs the translation of
    /// its section. The cards are disabled while `running`, since starting a
    /// section cancels the current run.
    pub fn error_cards<T: PageAction>(
        &self,
        page: usize,
        running: bool,
        cx: &Context<T>,
    ) -> Option<impl IntoElement> {
        // No cards to show, so no box to float over the text.
        if self.errors.is_empty() && !self.check_incomplete() {
            return None;
        }
        let on_click = |part| start_section_listener(cx, page, part);
        let empty_buttons = self
            .sections
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.content.is_empty().then_some(i))
            .map(|i| {
                card_button(
                    format!("empty-button-{i}"),
                    format!("Empty Section: {:2}", i + 1),
                    running,
                )
                .on_click(on_click(i))
            });

        let error_buttons = self.errors.iter().enumerate().map(|(id, e)| {
            card_button(format!("error-button-{id}"), error_label(e), running)
                .on_click(on_click(e.index))
        });

        Some(
            v_flex()
                .id("error-cards")
                .p_1()
                .gap_1()
                .top_2()
                .right_4()
                .occlude()
                .absolute()
                .min_h_0()
                .rounded_md()
                .max_h(px(300.0))
                .overflow_y_scroll()
                .bg(cx.theme().popover)
                .children(self.errors.is_empty().not().then(|| {
                    card_button(format!("error-button-all"), format!("All errors"), running)
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.start_errors(page, window, cx)
                        }))
                }))
                .children(empty_buttons.chain(error_buttons)),
        )
    }
}

fn card_button(id: impl Into<ElementId>, label: impl Into<SharedString>, disabled: bool) -> Button {
    Button::new(id).primary().label(label).disabled(disabled)
}

fn error_label(error: &PageError) -> String {
    let position = error.index + 1;
    match error.error {
        ErrorType::Japanese => format!("Japanese error: {:2}", position),
        ErrorType::Size(c) => format!("Size error: {:2}, count:{}", position, c),
        ErrorType::Repeat => format!("Repeat error: {:2}", position),
        ErrorType::Copy => format!("Copy error: {:2}", position),
        ErrorType::Image => format!("Image Tag error: {:2}", position),
        ErrorType::Header => format!("Header Tag error: {:2}", position),
        ErrorType::Link => format!("Link Tag error: {:2}", position),
        ErrorType::End => format!("End Punctuation error: {:2}", position),
    }
}
