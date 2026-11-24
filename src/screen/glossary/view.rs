use crate::screen::Glossary;
use gpui_kit::{
    Context, InteractiveElement as _, IntoElement, ParentElement as _, Render, Styled as _, Window,
    base::{h_flex, v_flex},
};

impl Render for Glossary {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .px_5()
            .gap_3()
            .flex_1()
            .h_4_5()
            .my_auto()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::save_chapter))
            .on_action(cx.listener(Self::run_action))
            .child(self.menu_bar(cx))
            .child(
                h_flex()
                    .gap_3()
                    .min_h_0()
                    .size_full()
                    .child(self.chapter_side_bar(cx))
                    .child(self.text_box(cx)),
            )
    }
}
