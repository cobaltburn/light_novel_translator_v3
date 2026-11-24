use crate::{
    message::send,
    screen::translation::{
        SavePage, ScrollTo, SetDisplay, Translate, Translation, message::Message,
    },
};
use gpui_kit::{
    Context, InteractiveElement as _, IntoElement, ParentElement as _, Render, Styled, Window,
    base::{h_flex, v_flex},
};

impl Render for Translation {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .px_5()
            .gap_3()
            .h_4_5()
            .my_auto()
            .track_focus(&self.focus_handle)
            .on_action(send(cx, |action: &SavePage| Message::SavePage(action.page)))
            .on_action(send(cx, |action: &ScrollTo| Message::ScrollTo(action.part)))
            .on_action(send(cx, |action: &SetDisplay| {
                Message::SetDisplay(action.0)
            }))
            .on_action(send(cx, |action: &Translate| {
                Message::Translate(action.clone())
            }))
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
