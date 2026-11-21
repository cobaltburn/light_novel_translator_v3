use crate::{
    screen::format::Format,
    widget::{LabelButton, PrimaryBorder as _, primary_button},
};
use gpui_kit::{
    Context, Entity, ExternalPaths, InteractiveElement as _, IntoElement, ObjectFit, ParentElement,
    Styled, StyledImage,
    base::{Disableable, h_flex, input::InputState, v_flex},
    component::{ActiveTheme as _, input::Input, label::Label, scroll::ScrollableElement},
    div, img, text,
};

impl Format {
    pub fn epub_column(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .h_full()
            .w_1_2()
            .min_size_0()
            .child(LabelButton::new(
                "epub-label-button",
                "epub",
                self.epub_name().unwrap_or_default(),
                cx.listener(|this, _, window, cx| this.select_epub(window, cx)),
            ))
            .child(self.cover_image(cx))
    }

    /// The folder of translated pages, and the pages in it.
    pub fn content_column(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .h_full()
            .w_1_2()
            .min_size_0()
            .child(LabelButton::new(
                "content-label-button",
                "content",
                self.source_folder.as_ref().map_or_default(|s| s.as_str()),
                cx.listener(|this, _, window, cx| {
                    this.select_folder(window, cx);
                }),
            ))
            .child(labeled_input("Title:", &self.inputs.title))
            .child(labeled_input("Authors:", &self.inputs.authors))
            .child(self.content_files(cx))
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
            .drag_over::<ExternalPaths>(|style, _, _, cx| {
                let theme = cx.theme();
                style.border_color(theme.accent).bg(theme.drop_target)
            })
            .on_drop(cx.listener(Self::on_drop_epub))
            .children(self.cover.as_ref().map(|image| {
                img(image.clone())
                    .absolute()
                    .inset_0()
                    .size_full()
                    .object_fit(ObjectFit::ScaleDown)
            }))
    }

    pub fn content_files(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .min_h_0()
            .w_full()
            .rounded_sm()
            .primary_border(cx)
            .drag_over::<ExternalPaths>(|style, _, _, cx| {
                let theme = cx.theme();
                style.border_color(theme.accent).bg(theme.drop_target)
            })
            .on_drop(cx.listener(Self::on_drop_folder))
            .child(
                v_flex().p_2().size_full().overflow_y_scrollbar().children(
                    self.pages
                        .iter()
                        .filter_map(|p| p.path.file_prefix())
                        .map(|p| div().child(text!(p.to_string_lossy()))),
                ),
            )
    }

    pub fn build_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let can_build = self.epub_path.is_some() && self.source_folder.is_some();
        h_flex().w_full().justify_end().child(
            primary_button("build-button", "build")
                .flex()
                .disabled(!can_build)
                .on_click(cx.listener(|this, _, window, cx| this.build_epub(window, cx))),
        )
    }
}

/// A text input with a label in front of it.
fn labeled_input(label: &'static str, state: &Entity<InputState>) -> impl IntoElement {
    h_flex()
        .gap_1()
        .child(Label::new(label))
        .child(Input::new(state))
}
