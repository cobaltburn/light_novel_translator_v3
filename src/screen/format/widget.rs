use crate::screen::format::Format;
use gpui_kit::{
    Context, IntoElement, ObjectFit, ParentElement, Styled, StyledImage,
    base::{h_flex, v_flex},
    component::{
        ActiveTheme,
        button::{Button, ButtonVariants},
    },
    div, img, text,
};

impl Format {
    pub fn cover_image(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .relative()
            .size_full()
            .items_center()
            .justify_center()
            .rounded_sm()
            .border_1()
            .border_color(cx.theme().primary)
            .children(self.cover.as_ref().map(|image| {
                img(image.clone())
                    .absolute()
                    .inset_0()
                    .size_full()
                    .object_fit(ObjectFit::ScaleDown)
            }))
    }

    pub fn content_files(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .rounded_sm()
            .border_1()
            .border_color(cx.theme().primary)
            .children(
                self.pages
                    .iter()
                    .filter_map(|p| p.path.file_prefix())
                    .map(|p| div().child(text!(p.to_string_lossy()))),
            )
    }

    pub fn build_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().w_full().justify_end().child(
            Button::new("build-button")
                .primary()
                .outline()
                .flex()
                .label("build")
                .on_click(cx.listener(|this, _, window, cx| this.build_epub(window, cx))),
        )
    }
}
