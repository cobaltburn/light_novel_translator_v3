use crate::widget::ListnerFn;
use gpui_kit::{
    App, ClickEvent, ElementId, Hsla, IntoElement, ParentElement as _, RenderOnce, SharedString,
    Styled, Window,
    base::{Disableable, h_flex},
    component::button::{Button, ButtonVariants as _},
    div, text,
};

#[derive(IntoElement)]
pub struct LabelButton {
    id: ElementId,
    label: SharedString,
    text: SharedString,
    on_click: Box<ListnerFn>,
    disable: bool,
}

impl LabelButton {
    pub fn new<T>(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        text: impl Into<SharedString>,
        on_click: T,
    ) -> Self
    where
        T: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    {
        Self {
            id: id.into(),
            label: label.into(),
            text: text.into(),
            on_click: Box::new(on_click),
            disable: false,
        }
    }

    pub fn disable(mut self, disable: bool) -> Self {
        self.disable = disable;
        self
    }
}

impl RenderOnce for LabelButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        h_flex()
            .gap_2()
            .w_full()
            .child(
                Button::new(self.id)
                    .label(self.label)
                    .primary()
                    .outline()
                    .disabled(self.disable)
                    .on_click(self.on_click),
            )
            .child(
                div()
                    .p_1()
                    .w_full()
                    .border_1()
                    .rounded_sm()
                    .border_color(Hsla::white())
                    .child(text!(self.text)),
            )
    }
}
