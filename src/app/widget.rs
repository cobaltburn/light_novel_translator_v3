use crate::{
    app::{Translator, message::Message},
    message::send,
    widget::{AddButton, CloseButton},
};
use gpui_kit::{
    Context, Hsla, IntoElement, ParentElement, Styled as _,
    component::{
        Icon, IconName,
        tab::{Tab, TabBar},
    },
    div, px,
};

impl Translator {
    pub fn tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        TabBar::new("translation-tab-bar")
            .on_click(send(cx, |i: &usize| Message::SelectTab(*i)))
            .selected_index(self.active_tab)
            .children(self.tabs(cx))
            .suffix(AddButton::new(
                "tab-bar-suffix",
                send(cx, |_| Message::NewTab),
            ))
    }

    pub fn tabs(&self, cx: &mut Context<Self>) -> impl Iterator<Item = Tab> {
        self.translations
            .iter()
            .map(|t| t.read(cx))
            .map(|t| (t.epub_stem().unwrap_or_default(), t.is_running()))
            .map(|(t, active)| {
                Tab::new().min_w(px(200.0)).label(t).prefix(div().children(
                    active.then_some(Icon::new(IconName::Play).text_color(Hsla::green())),
                ))
            })
            .enumerate()
            .map(|(i, tab)| {
                let close = send(cx, move |_| Message::CloseTab(i));
                // Without this, the click would also select the tab being closed.
                tab.suffix(CloseButton::new(i, move |event, window, cx| {
                    cx.stop_propagation();
                    close(event, window, cx);
                }))
            })
    }
}
