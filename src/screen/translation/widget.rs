use crate::{
    page::Page,
    screen::{ScrollTo, SetDisplay, Translation},
    widget::{
        LabelButton, PrimaryBorder as _, action_menu, entry_sidebar, numbered_items,
        page::DisplayType, primary_button,
    },
};
use gpui_kit::{
    Anchor, Context, InteractiveElement as _, IntoElement, ParentElement as _,
    StatefulInteractiveElement as _, Styled as _, Window,
    base::{Disableable, StyledExt as _, h_flex},
    component::{
        menu::{ContextMenuExt, PopupMenu},
        popover::Popover,
        scroll::ScrollableElement,
    },
    div, px,
};

impl Translation {
    pub fn menu_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(self.epub_popover(cx))
            .child(self.server.server_popover(cx))
            .child(self.translate_button(cx))
            .child(self.server.model_select(cx))
    }

    pub fn epub_popover(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let running = self.is_running();
        let epub = self.epub_name().unwrap_or_default();
        Popover::new("epub-popover")
            .gap_3()
            .min_w(px(450.0))
            .anchor(Anchor::TopLeft)
            .trigger(primary_button("epub-trigger", "epub"))
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
                .disable(running || self.file_path.is_none()),
            )
            .child(
                h_flex().justify_center().items_center().gap_2().children([
                    primary_button("save-button", "save")
                        .disabled(running)
                        .on_click(cx.listener(Self::save)),
                    primary_button("recover-button", "recover")
                        .disabled(running)
                        .on_click(cx.listener(Self::recover)),
                ]),
            )
    }

    pub fn translate_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_running() {
            primary_button("cancel-button", "cancel").on_click(cx.listener(|this, _, _, cx| {
                this.stop();
                cx.notify();
            }))
        } else {
            primary_button("translate-button", "translate")
                .disabled(!self.can_run())
                .on_click(cx.listener(Self::translate))
        }
    }

    pub fn page_side_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let items = self
            .pages
            .iter()
            .enumerate()
            .map(|(i, p)| p.sidebar_menu_item(i, self.current_page == i, self, cx));
        entry_sidebar("page-sidebar", items, cx)
    }

    pub fn text_box(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex_auto()
            .min_size_0()
            .rounded_md()
            .primary_border(cx)
            .children(self.current_page().map(|page| self.page_text(page, cx)))
            .context_menu(self.text_menu())
    }

    /// The scrolling text of `page`, with its error cards floating over it.
    fn page_text(&self, page: &Page, cx: &Context<Self>) -> impl IntoElement {
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
                    .children(page.text(self.display, self.current_page, cx)),
            )
            .vertical_scrollbar(&self.scroll_handle)
            .child(page.error_cards(self.current_page, cx))
    }

    /// The text box's context menu: how to display the text, and where to scroll.
    fn text_menu(
        &self,
    ) -> impl Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static {
        let focus_handle = self.focus_handle.clone();
        let display = self.display;
        let sections = self.current_page().map_or_default(|p| p.sections.len());
        move |menu, _, _| {
            let menu = DisplayType::ALL.into_iter().fold(
                action_menu(menu, focus_handle.clone()).separator(),
                |menu, item| {
                    menu.menu_with_check(item.label(), display == item, Box::new(SetDisplay(item)))
                },
            );
            numbered_items(menu, sections, "section", |part| {
                Box::new(ScrollTo { part })
            })
        }
    }
}
