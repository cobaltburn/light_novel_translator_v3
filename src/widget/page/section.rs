use gpui_kit::{
    App, ClickEvent, Div, HighlightStyle, Hsla, InteractiveElement as _, IntoElement,
    ParentElement as _, SharedString, StatefulInteractiveElement as _, Styled as _, StyledText,
    Window, div, px, text,
};
use std::ops::Range;

/// A section's text under its [`section_label`], with the `highlights` byte
/// ranges of the text shown in red.
pub fn section_div(
    name: &str,
    part: usize,
    text: SharedString,
    highlights: Vec<Range<usize>>,
    on_click: Option<impl Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
) -> Div {
    let label = section_label(name, part, text.len(), on_click);
    let body = if highlights.is_empty() {
        text!(text).into_any_element()
    } else {
        let red = HighlightStyle {
            color: Some(Hsla::red()),
            ..Default::default()
        };
        StyledText::new(text)
            .with_highlights(highlights.into_iter().map(|range| (range, red)))
            .into_any_element()
    };
    div().child(label).child(body)
}

/// A label naming a section and the length of its shown text. It is
/// clickable only when given an `on_click`.
pub fn section_label(
    name: &str,
    part: usize,
    count: usize,
    on_click: Option<impl Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
) -> impl IntoElement {
    let label = div()
        .id(format!("{name}-{part}"))
        .min_size_0()
        .max_w(px(120.0))
        .py_3()
        .text_color(Hsla::red())
        .child(format!("Section: {}\nCount: {}", part + 1, count));
    match on_click {
        Some(on_click) => label.hover(|s| s.underline()).on_click(on_click),
        None => label,
    }
}
