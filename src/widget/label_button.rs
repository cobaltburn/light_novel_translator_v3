use crate::widget::{ListnerFn, primary_button};
use gpui_kit::{
    App, ClickEvent, ElementId, IntoElement, ParentElement as _, RenderOnce, SharedString, Styled,
    Window,
    base::{Disableable, h_flex},
    component::ActiveTheme,
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        h_flex()
            .gap_2()
            .w_full()
            .child(
                primary_button(self.id, self.label)
                    .disabled(self.disable)
                    .on_click(self.on_click),
            )
            .child(
                div()
                    .p_1()
                    .w_full()
                    .border_1()
                    .rounded_sm()
                    .border_color(cx.theme().primary)
                    .child(text!(self.text)),
            )
    }
}
