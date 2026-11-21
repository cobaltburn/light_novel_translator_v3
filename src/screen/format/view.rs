use crate::screen::format::Format;
use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    base::{h_flex, v_flex},
};

impl Render for Format {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .px_5()
            .gap_3()
            .h_5_6()
            .w_full()
            .min_w_0()
            .child(
                h_flex()
                    .gap_2()
                    .size_full()
                    .child(self.epub_column(cx))
                    .child(self.content_column(cx)),
            )
            .child(self.build_button(cx))
    }
}
