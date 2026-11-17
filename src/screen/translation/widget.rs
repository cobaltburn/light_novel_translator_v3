use crate::{
    screen::{DisplayType, SetDisplay, Translation},
    widget::LabelButton,
};
use gpui_kit::{
    Anchor, Context, Hsla, IntoElement, ParentElement as _, Styled as _,
    base::{Disableable, StyledExt as _, h_flex},
    component::{
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
        Popover::new((self.id.clone(), "epub-popover"))
            .gap_3()
            .min_w(px(450.0))
            .anchor(Anchor::TopLeft)
            .trigger(
                Button::new((self.id.clone(), "epub-trigger"))
                    .label("epub")
                    .primary()
                    .outline(),
            )
            .child(LabelButton::new(
                (self.id.clone(), "epub-button"),
                "epub",
                self.epub_name().unwrap_or_default(),
                cx.listener(Self::select_epub),
            ))
            .child(LabelButton::new(
                (self.id.clone(), "glossary-button"),
                "glossary",
                self.glossary_name().unwrap_or_default(),
                cx.listener(Self::select_glossary),
            ))
            .child(
                h_flex().justify_center().items_center().gap_2().children([
                    Button::new((self.id.clone(), "save-button"))
                        .label("save")
                        .on_click(cx.listener(Self::save)),
                    Button::new((self.id.clone(), "recover-button"))
                        .label("recover")
                        .on_click(cx.listener(Self::recover)),
                ]),
            )
    }

    pub fn menu_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .paddings(px(5.0))
            .child(self.epub_popover(cx))
            .child(
                self.server
                    .server_popover((self.id.clone(), "server-popover"), cx),
            )
            .child(self.translate_button(cx))
            .child(self.server.model_select())
    }

    pub fn translate_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_running() {
            Button::new((self.id.clone(), "cancel-button"))
                .label("cancel")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.stop();
                    cx.notify();
                }))
        } else {
            Button::new((self.id.clone(), "translate-button"))
                .label("translate")
                .disabled(!self.can_translate())
                .on_click(cx.listener(Self::translate))
        }
    }

    pub fn page_side_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new((self.id.clone(), "page-sidebar"))
            .h_full()
            .border_1()
            .rounded_md()
            .paddings(px(5.0))
            .border_color(Hsla::white())
            .children(self.pages.iter().enumerate().map(|(i, p)| {
                p.sidebar_menu_item(
                    i,
                    self.current_page == i,
                    cx.listener(move |this, _, _, cx| this.set_page(i, cx)),
                    self.focus_handle.clone(),
                )
            }))
    }

    pub fn text_box(&self) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let display = self.display;
        div()
            .size_full()
            .flex_auto()
            .border_1()
            .min_size_0()
            .rounded_md()
            .paddings(px(5.0))
            .border_color(Hsla::white())
            .children(self.current_page().map(|page| {
                div()
                    .size_full()
                    .paddings(px(5.0))
                    .overflow_y_scrollbar()
                    .children(page.text(display))
            }))
            .context_menu(move |menu, _window, _cx| {
                let items = [
                    ("full", DisplayType::Full, SetDisplay::Full),
                    ("end", DisplayType::End, SetDisplay::End),
                    ("japanese", DisplayType::Japanese, SetDisplay::Japanese),
                ];
                items.into_iter().fold(
                    menu.action_context(focus_handle.clone()),
                    |menu, (label, t, action)| {
                        menu.menu_with_check(label, display == t, Box::new(action))
                    },
                )
            })
    }
}
