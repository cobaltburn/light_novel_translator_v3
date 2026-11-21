use crate::{
    screen::Glossary,
    widget::{LabelButton, PrimaryBorder as _, entry_sidebar, primary_button},
};
use gpui_kit::{
    Anchor, Context, IntoElement, ParentElement as _, Styled as _,
    base::{Disableable as _, StyledExt as _, h_flex},
    component::{popover::Popover, scroll::ScrollableElement as _},
    div, px, text,
};

impl Glossary {
    pub fn menu_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(self.epub_popover(cx))
            .child(self.server.server_popover(cx))
            .child(self.glossary_button(cx))
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
                h_flex().justify_center().items_center().child(
                    primary_button("save-button", "save")
                        .disabled(running)
                        .on_click(cx.listener(Self::save)),
                ),
            )
    }

    pub fn glossary_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.is_running() {
            primary_button("cancel-button", "cancel").on_click(cx.listener(|this, _, _, cx| {
                this.stop();
                cx.notify();
            }))
        } else {
            primary_button("glossary-button", "glossary")
                .disabled(!self.can_run())
                .on_click(cx.listener(Self::run))
        }
    }

    pub fn chapter_side_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let items = self
            .chapters
            .iter()
            .enumerate()
            .map(|(i, c)| c.sidebar_menu_item(i, self.current_chapter == i, self, cx));
        entry_sidebar("chapter-sidebar", items, cx)
    }

    pub fn text_box(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex_auto()
            .min_size_0()
            .rounded_md()
            .primary_border(cx)
            .children(self.current_chapter().map(|chapter| {
                div()
                    .size_full()
                    .paddings(px(5.0))
                    .child(text!(chapter.content.as_str()))
                    .overflow_y_scrollbar()
            }))
    }
}
