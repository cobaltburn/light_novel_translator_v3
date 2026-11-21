use crate::page::Section;
use gpui_kit::SharedString;

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
}
