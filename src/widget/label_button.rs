use crate::widget::{ListnerFn, primary_button};
use gpui_kit::{
    App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, SharedString, StatefulInteractiveElement as _, Styled, Window,
    base::{Disableable, h_flex},
    component::{ActiveTheme, tooltip::Tooltip},
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
        let tooltip_text = self.text.clone();
        h_flex()
            .gap_2()
            .w_full()
            .min_w_0()
            .child(
                primary_button(self.id.clone(), self.label)
                    .flex_shrink_0()
                    .disabled(self.disable)
                    .on_click(self.on_click),
            )
            .child(
                div()
                    .id((self.id, "text"))
                    .p_1()
                    .flex_1()
                    .min_w_0()
                    .border_1()
                    .rounded_sm()
                    .text_ellipsis_middle()
                    .border_color(cx.theme().primary)
                    .tooltip(move |window, cx| Tooltip::new(tooltip_text.clone()).build(window, cx))
                    .child(text!(self.text)),
            )
    }
}
