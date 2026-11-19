use crate::screen::translation::Translation;
use gpui_kit::{
    Context, InteractiveElement as _, IntoElement, ParentElement as _, Render, Styled, Window,
    base::{StyledExt, h_flex, v_flex},
    px,
};

impl Render for Translation {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .min_h_0()
            .size_full()
            .paddings(px(10.0))
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::save_page))
            .on_action(cx.listener(Self::scroll_to))
            .on_action(cx.listener(Self::set_display_action))
            .on_action(cx.listener(Self::translate_action))
            .child(self.menu_bar(cx))
            .child(
                h_flex()
                    .gap_3()
                    .min_h_0()
                    .size_full()
                    .child(self.page_side_bar(cx))
                    .child(self.text_box(cx)),
            )
    }
}
