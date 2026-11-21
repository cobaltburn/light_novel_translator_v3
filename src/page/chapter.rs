use crate::page::Activity;
use std::{borrow::Cow, path::PathBuf};

#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct Chapter {
    pub path: PathBuf,
    pub text: String,
    pub content: String,
    pub activity: Activity,
}

impl Chapter {
    pub fn new(path: PathBuf, text: String) -> Self {
        Chapter {
            path,
            text,
            ..Default::default()
        }
    }

    pub fn file_stem(&self) -> Option<Cow<'_, str>> {
        Some(self.path.file_stem()?.to_string_lossy())
    }
}
