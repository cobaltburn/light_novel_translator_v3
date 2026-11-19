use crate::{
    screen::{DisplayType, ScrollTo, SetDisplay, Translation},
    widget::LabelButton,
};
use gpui_kit::{
    Anchor, Context, InteractiveElement as _, IntoElement, ParentElement as _,
    StatefulInteractiveElement as _, Styled as _,
    base::{Disableable, StyledExt as _, h_flex},
    component::{
        ActiveTheme,
        button::{Button, ButtonVariants},
        menu::ContextMenuExt,
        popover::Popover,
        scroll::ScrollableElement,
        sidebar::Sidebar,
    },
    div, px,
};

impl Translation {
    pub fn epub_popover(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let running = self.is_running();
        let epub = self.epub_name().unwrap_or_default();
        Popover::new("epub-popover")
            .gap_3()
            .min_w(px(450.0))
            .anchor(Anchor::TopLeft)
            .trigger(
                Button::new("epub-trigger")
                    .label("epub")
                    .primary()
                    .outline(),
            )
            .child(
                LabelButton::new("epub-button", "epub", epub, cx.listener(Self::select_epub))
                    .disable(running),
            )
            .child(
                LabelButton::new(
                    "glossary-button",
                    "glossary",
                    self.glossary_name().unwrap_or_default(),
                    cx.listener(Self::select_glossary),
                )
                .disable(running),
            )
            .child(
                h_flex().justify_center().items_center().gap_2().children([
                    Button::new("save-button")
                        .label("save")
                        .primary()
                        .outline()
                        .disabled(running)
                        .on_click(cx.listener(Self::save)),
                    Button::new("recover-button")
                        .label("recover")
                        .primary()
                        .outline()
                        .disabled(running)
                        .on_click(cx.listener(Self::recover)),
                ]),
            )
    }

    pub fn menu_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .paddings(px(5.0))
            .child(self.epub_popover(cx))
            .child(self.server.server_popover(cx))
            .child(self.translate_button(cx))
            .child(self.server.model_select(cx))
    }

    pub fn translate_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_running() {
            Button::new("cancel-button")
                .label("cancel")
                .primary()
                .outline()
                .on_click(cx.listener(|this, _, _, cx| {
                    this.stop();
                    cx.notify();
                }))
        } else {
            Button::new("translate-button")
                .label("translate")
                .primary()
                .outline()
                .disabled(!self.can_translate())
                .on_click(cx.listener(Self::translate))
        }
    }

    pub fn page_side_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new("page-sidebar")
            .h_full()
            .border_1()
            .rounded_md()
            .paddings(px(5.0))
            .border_color(cx.theme().primary)
            .children(self.pages.iter().enumerate().map(|(i, p)| {
                p.sidebar_menu_item(
                    i,
                    self.current_page == i,
                    cx.listener(move |this, _, _, cx| this.set_page(i, cx)),
                    self.focus_handle.clone(),
                )
            }))
    }

    pub fn text_box(&self, cx: &Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let display = self.display;
        let section_len = self.current_page().map_or_default(|p| p.sections.len());
        div()
            .size_full()
            .flex_auto()
            .border_1()
            .min_size_0()
            .rounded_md()
            .paddings(px(5.0))
            .border_color(cx.theme().primary)
            .children(self.current_page().map(|page| {
                div()
                    .relative()
                    .size_full()
                    .child(
                        div()
                            .id("text-scroll")
                            .size_full()
                            .paddings(px(5.0))
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll_handle)
                            .children(page.text(display, self.current_page, cx)),
                    )
                    .vertical_scrollbar(&self.scroll_handle)
                    .child(page.error_cards(cx))
            }))
            .context_menu(move |menu, _window, _cx| {
                let items = [
                    ("full", DisplayType::Full, SetDisplay::Full),
                    ("end", DisplayType::End, SetDisplay::End),
                    ("japanese", DisplayType::Japanese, SetDisplay::Japanese),
                ];
                let menu = items.into_iter().fold(
                    menu.action_context(focus_handle.clone()),
                    |menu, (label, t, action)| {
                        menu.menu_with_check(label, display == t, Box::new(action))
                    },
                );
                (0..section_len).fold(menu, |menu, part| {
                    menu.menu(format!("section {}", part + 1), Box::new(ScrollTo { part }))
                })
            })
    }
}
