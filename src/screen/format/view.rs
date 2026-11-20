use crate::{screen::format::Format, widget::LabelButton};
use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    base::{h_flex, v_flex},
};

impl Render for Format {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let epub_name = self.epub_name().unwrap_or_default();
        let source_name = self.source_folder.as_ref().map_or_default(|s| s.as_str());
        v_flex()
            .p_3()
            .gap_3()
            .h_5_6()
            .w_full()
            .child(
                h_flex()
                    .gap_2()
                    .size_full()
                    .child(
                        v_flex()
                            .gap_3()
                            .h_full()
                            .w_1_2()
                            .min_size_0()
                            .child(LabelButton::new(
                                "epub-label-button",
                                "epub",
                                epub_name,
                                cx.listener(|this, _, window, cx| this.select_epub(window, cx)),
                            ))
                            .child(self.cover_image(cx)),
                    )
                    .child(
                        v_flex()
                            .gap_3()
                            .h_full()
                            .w_1_2()
                            .min_size_0()
                            .child(LabelButton::new(
                                "content-label-button",
                                "content",
                                source_name,
                                cx.listener(|this, _, window, cx| {
                                    this.select_folder(window, cx);
                                }),
                            ))
                            .child(self.content_files(cx)),
                    ),
            )
            .child(self.build_button(cx))
    }
}
