use crate::page::{ErrorType, Page, PageError};
use gpui_kit::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, SharedString,
    Styled as _,
    base::v_flex,
    component::{
        ActiveTheme as _,
        button::{Button, ButtonVariants as _},
    },
    px,
};

impl Page {
    /// A floating list of the page's empty sections and failed checks.
    pub fn error_cards(&self, cx: &App) -> impl IntoElement {
        let empty_buttons = self
            .sections
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.content.is_empty().then_some(i))
            .map(|i| {
                card_button(
                    format!("empty-button-{i}"),
                    format!("Empty Section: {:2}", i + 1),
                )
            });

        let error_buttons = self
            .errors
            .iter()
            .enumerate()
            .map(|(id, e)| card_button(format!("error-button-{id}"), error_label(e)));

        v_flex()
            .p_1()
            .gap_1()
            .top_2()
            .right_4()
            .occlude()
            .absolute()
            .rounded_md()
            .min_h_0()
            .max_h(px(300.0))
            .bg(cx.theme().popover)
            .children(empty_buttons.chain(error_buttons))
    }
}

fn card_button(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Button {
    Button::new(id).primary().label(label)
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
