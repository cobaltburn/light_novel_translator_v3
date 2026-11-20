use gpui_kit::{
    App, ClickEvent, Div, Hsla, InteractiveElement as _, IntoElement, ParentElement as _,
    SharedString, StatefulInteractiveElement as _, Styled as _, Window, div, px, text,
};

// TODO highlight japanese characters incase of error
/// A section's text under its [`section_label`].
pub fn section_div(
    name: &str,
    part: usize,
    text: SharedString,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Div {
    div()
        .child(section_label(name, part, text.len(), on_click))
        .child(text!(text))
}

/// A clickable label naming a section and the length of its shown text.
pub fn section_label(
    name: &str,
    part: usize,
    count: usize,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .id(format!("{name}-{part}"))
        .min_size_0()
        .max_w(px(120.0))
        .py_3()
        .text_color(Hsla::red())
        .hover(|s| s.underline())
        .child(format!("Section: {}\nCount: {}", part + 1, count))
        .on_click(on_click)
}
