use crate::{
    app::Translator,
    widget::{AddButton, CloseButton},
};
use gpui_kit::{
    Context, IntoElement, Styled as _,
    component::tab::{Tab, TabBar},
    px,
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
            .map(|t| t.read(cx).epub_name().unwrap_or_default())
            .map(|t| Tab::new().label(t).min_w(px(200.0)))
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
