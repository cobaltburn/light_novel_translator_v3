use std::path::PathBuf;

/// A translated page, as markdown, to build into the epub.
#[non_exhaustive]
#[derive(Default, Debug)]
pub struct FormatPage {
    pub path: PathBuf,
    pub content: String,
}

impl From<(PathBuf, String)> for FormatPage {
    fn from((path, content): (PathBuf, String)) -> Self {
        FormatPage { path, content }
    }
}

#[non_exhaustive]
#[derive(Default, Debug)]
pub struct EpubMetadata {
    pub title: String,
    pub authors: String,
}
