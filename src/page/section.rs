use crate::text::clean_invisible_chars;
use gpui_kit::{
    App, ClickEvent, Div, Hsla, InteractiveElement, IntoElement, ParentElement as _, SharedString,
    StatefulInteractiveElement, Styled as _, Window, div, px, text,
};
use rig::message::Message;
use serde::{Deserialize, Serialize};

const SECTION_CAPACITY: usize = 8 * 1024;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Section {
    pub japanese: SharedString,
    pub content: String,
}

impl Section {
    pub fn new(japanese: String) -> Self {
        Self {
            japanese: japanese.into(),
            content: String::with_capacity(SECTION_CAPACITY),
        }
    }

    pub fn history_message(&self) -> [Message; 2] {
        [
            Message::user(&self.japanese.to_string()),
            Message::assistant(&self.content),
        ]
    }

    pub fn clean(&mut self) {
        self.content = clean_invisible_chars(self.content.as_str());
        self.content = self.content.replace(['“', '”'], "\"");
        self.split_lines();
    }

    fn split_lines(&mut self) {
        let lines: Vec<_> = self.content.lines().collect();
        let mut text = Vec::new();
        for [l1, l2] in lines.array_windows() {
            text.push(*l1);
            if !l1.is_empty() && !l2.is_empty() {
                text.push("");
            }
        }

        if let Some(last) = lines.last() {
            text.push(last);
        };

        self.content = text.join("\n")
    }

    // TODO highlight japanese characters incase of error
    pub fn content_div(
        &self,
        part: usize,
        name: &str,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Div {
        div()
            .child(part_div(name, part, self.content.len(), on_click))
            .child(text!(self.content.clone()))
    }

    pub fn end_lines_div(
        &self,
        part: usize,
        name: &str,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Div {
        let end_lines = self.end_lines();
        div()
            .child(part_div(name, part, end_lines.len(), on_click))
            .child(text!(end_lines))
    }

    pub fn japanese_div(
        &self,
        part: usize,
        name: &str,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Div {
        div()
            .child(part_div(name, part, self.japanese.len(), on_click))
            .child(text!(self.japanese.clone()))
    }

    fn end_lines(&self) -> String {
        let lines: Vec<_> = self.content.lines().collect();
        lines
            .get(lines.len().saturating_sub(10)..)
            .unwrap_or_default()
            .join("\n")
    }
}

pub fn part_div(
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
