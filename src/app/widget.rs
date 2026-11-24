use crate::{
    app::Translator,
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
            .on_click(cx.listener(|this, i, _, cx| {
                this.active_tab = *i;
                cx.notify();
            }))
            .selected_index(self.active_tab)
            .children(self.tabs(cx))
            .suffix(AddButton::new(
                "tab-bar-suffix",
                cx.listener(|this, _, window, cx| {
                    this.new_tab(window, cx);
                    cx.notify();
                }),
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
                tab.suffix(CloseButton::new(
                    i,
                    cx.listener(move |this, _, _, cx| {
                        cx.stop_propagation();
                        this.close_tab(i, cx);
                    }),
                ))
            })
    }
}
