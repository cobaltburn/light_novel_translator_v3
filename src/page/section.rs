use crate::text::clean_invisible_chars;
use gpui_kit::SharedString;
use rig::message::Message;
use serde::{Deserialize, Serialize};

const SECTION_CAPACITY: usize = 8 * 1024;
const END_LINES: usize = 10;

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
            Message::user(self.japanese.to_string()),
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

    pub fn end_lines(&self) -> String {
        let lines: Vec<_> = self.content.lines().collect();
        lines
            .get(lines.len().saturating_sub(END_LINES)..)
            .unwrap_or_default()
            .join("\n")
    }
}
