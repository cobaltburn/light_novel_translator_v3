use crate::screen::translation::Translation;
use gpui_kit::{
    Context, InteractiveElement as _, IntoElement, ParentElement as _, Render, Styled, Window,
    base::{StyledExt, h_flex, v_flex},
    px,
};

impl Render for Translation {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .min_h_0()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::save_page))
            .on_action(cx.listener(Self::set_display_action))
            .on_action(cx.listener(Self::translate_action))
            .paddings(px(10.0))
            .size_full()
            .gap_2()
            .child(self.menu_bar(cx))
            .child(
                h_flex()
                    .size_full()
                    .min_h_0()
                    .gap_3()
                    .child(self.page_side_bar(cx))
                    .child(self.text_box()),
            )
    }
}
