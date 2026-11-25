use crate::{
    app::Translator,
    widget::{AddButton, CloseButton},
};
use gpui_kit::{
    App, Context, Hsla, IntoElement, ParentElement, Styled as _,
    component::{
        Icon, IconName,
        tab::{Tab, TabBar},
    },
    div, px,
};

impl Translator {
    pub fn tab_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        TabBar::new("translation-tab-bar")
            .menu(true)
            .selected_index(self.active_tab)
            .track_scroll(&self.tab_scroll_handle)
            .on_click(cx.listener(Self::set_tab))
            .children(self.tabs(cx))
            .suffix(AddButton::new("tab-bar-suffix", cx.listener(Self::new_tab)))
    }

    fn tab_labels(&self, cx: &App) -> Vec<String> {
        let stems: Vec<_> = self
            .translations
            .iter()
            .map(|t| t.read(cx).epub_stem())
            .collect();
        stems
            .iter()
            .enumerate()
            .map(|(i, stem)| match stem {
                None => format!("Untitled {}", i + 1),
                Some(s) if stems.iter().filter(|o| o.as_ref() == Some(s)).count() > 1 => {
                    format!("{s} ({})", i + 1)
                }

                Some(s) => s.to_string(),
            })
            .collect()
    }

    pub fn tabs(&self, cx: &mut Context<Self>) -> impl Iterator<Item = Tab> {
        self.tab_labels(cx)
            .into_iter()
            .zip(self.translations.iter().map(|t| t.read(cx)))
            .enumerate()
            .map(|(i, (label, tab))| {
                let active = tab.is_running();
                Tab::new()
                    .min_w(px(200.0))
                    .label(label)
                    .prefix(div().children(
                        active.then(|| Icon::new(IconName::Play).text_color(Hsla::green())),
                    ))
                    .suffix(CloseButton::new(
                        i,
                        cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.close_tab(i, cx);
                        }),
                    ))
            })
    }
}
