use crate::{
    page::Page,
    screen::{ScrollTo, SetDisplay, Translate, Translation},
    widget::{
        LabelButton, PrimaryBorder as _, action_menu, entry_sidebar, numbered_items,
        page::DisplayType, primary_button,
    },
};
use gpui_kit::{
    Anchor, AnyElement, Context, ExternalPaths, InteractiveElement as _, IntoElement,
    ParentElement as _, StatefulInteractiveElement as _, Styled as _, Window,
    base::{Disableable, StyledExt as _, h_flex},
    component::{
        ActiveTheme,
        button::{ButtonVariants as _, DropdownButton},
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
        let epub = self.epub_stem().unwrap_or_default();
        Popover::new("epub-popover")
            .gap_3()
            .w(px(500.0))
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

    /// Translates from the current page, with a menu for retrying every failed
    /// section of the book. While running, it cancels the run instead.
    pub fn translate_button(&self, cx: &mut Context<Self>) -> AnyElement {
        if self.is_running() {
            return primary_button("cancel-button", "cancel")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.stop();
                    cx.notify();
                }))
                .into_any_element();
        }
        let focus_handle = self.focus_handle.clone();
        let errors = self.error_jobs().len();
        DropdownButton::new("translate-dropdown")
            .primary()
            .outline()
            .disabled(!self.can_run())
            .button(
                primary_button("translate-button", "translate")
                    .on_click(cx.listener(Self::translate)),
            )
            .dropdown_menu_with_anchor(Anchor::TopCenter, move |menu, _, _| {
                action_menu(menu, focus_handle.clone()).menu_with_disabled(
                    format!("retry all errors ({errors})"),
                    Box::new(Translate::Errors),
                    errors == 0,
                )
            })
            .into_any_element()
    }

    pub fn page_side_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let running = self.is_running();
        let items = self
            .pages
            .iter()
            .enumerate()
            .map(|(i, p)| p.sidebar_menu_item(i, self.current_page == i, running, self, cx));
        entry_sidebar("page-sidebar", items, cx)
    }

    pub fn text_box(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex_auto()
            .min_size_0()
            .rounded_md()
            .primary_border(cx)
            .drag_over::<ExternalPaths>(|style, _, _, cx| {
                let theme = cx.theme();
                style.border_color(theme.accent).bg(theme.drop_target)
            })
            .on_drop(cx.listener(Self::on_drop_epub))
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
                    .children(page.text(self.display, self.current_page, self.is_running(), cx)),
            )
            .vertical_scrollbar(&self.scroll_handle)
            .children(page.error_cards(self.current_page, self.is_running(), cx))
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
            numbered_items(menu.separator(), sections, "section", false, |part| {
                Box::new(ScrollTo { part })
            })
        }
    }
}
