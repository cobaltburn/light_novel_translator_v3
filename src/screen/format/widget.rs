use crate::{
    screen::format::Format,
    widget::{LabelButton, PrimaryBorder as _, primary_button},
};
use gpui_kit::{
    Context, Div, IntoElement, ObjectFit, ParentElement, Styled, StyledImage,
    base::{h_flex, v_flex},
    div, img, text,
};

impl Format {
    /// The source epub, and its cover.
    pub fn epub_column(&self, cx: &mut Context<Self>) -> impl IntoElement {
        source_column(
            LabelButton::new(
                "epub-label-button",
                "epub",
                self.epub_name().unwrap_or_default(),
                cx.listener(|this, _, window, cx| this.select_epub(window, cx)),
            ),
            self.cover_image(cx),
        )
    }

    /// The folder of translated pages, and the pages in it.
    pub fn content_column(&self, cx: &mut Context<Self>) -> impl IntoElement {
        source_column(
            LabelButton::new(
                "content-label-button",
                "content",
                self.source_folder.as_ref().map_or_default(|s| s.as_str()),
                cx.listener(|this, _, window, cx| {
                    this.select_folder(window, cx);
                }),
            ),
            self.content_files(cx),
        )
    }

    pub fn cover_image(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .relative()
            .size_full()
            .items_center()
            .justify_center()
            .rounded_sm()
            .primary_border(cx)
            .children(self.cover.as_ref().map(|image| {
                img(image.clone())
                    .absolute()
                    .inset_0()
                    .size_full()
                    .object_fit(ObjectFit::ScaleDown)
            }))
    }

    pub fn content_files(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .rounded_sm()
            .primary_border(cx)
            .children(
                self.pages
                    .iter()
                    .filter_map(|p| p.path.file_prefix())
                    .map(|p| div().child(text!(p.to_string_lossy()))),
            )
    }

    pub fn build_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().w_full().justify_end().child(
            primary_button("build-button", "build")
                .flex()
                .on_click(cx.listener(|this, _, window, cx| this.build_epub(window, cx))),
        )
    }
}

/// Half of the screen: a picker for one source over a panel showing what was picked.
fn source_column(picker: LabelButton, panel: impl IntoElement) -> Div {
    v_flex()
        .gap_3()
        .h_full()
        .w_1_2()
        .min_size_0()
        .child(picker)
        .child(panel)
}
