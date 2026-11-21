use crate::{page::Section, text::japanese_ranges};
use gpui_kit::SharedString;
use std::ops::Range;

/// Which part of each section a page view shows.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayType {
    #[default]
    Full,
    End,
    Japanese,
}

impl DisplayType {
    pub const ALL: [DisplayType; 3] = [DisplayType::Full, DisplayType::End, DisplayType::Japanese];

    pub fn label(self) -> &'static str {
        match self {
            DisplayType::Full => "full",
            DisplayType::End => "end",
            DisplayType::Japanese => "japanese",
        }
    }

    /// The text of `section` this display shows.
    pub fn section_text(self, section: &Section) -> SharedString {
        match self {
            DisplayType::Full => section.content.clone().into(),
            DisplayType::End => section.end_lines().into(),
            DisplayType::Japanese => section.japanese.clone(),
        }
    }

    /// Byte ranges of `text` to highlight: its Japanese characters, when the
    /// translation is shown and the section failed the Japanese check.
    pub fn section_highlights(self, text: &str, japanese_error: bool) -> Vec<Range<usize>> {
        match self {
            DisplayType::Full | DisplayType::End if japanese_error => japanese_ranges(text),
            _ => Vec::new(),
        }
    }
}
