use crate::epub::{EpubMetadata, FormatPage};
use gpui_kit::Image;
use rbook::Epub;
use std::{borrow::Cow, path::PathBuf, sync::Arc};

#[non_exhaustive]
#[derive(Default, Debug)]
pub struct Format {
    pub pages: Vec<FormatPage>,
    pub source_folder: Option<String>,
    pub epub_path: Option<PathBuf>,
    pub epub: Option<Epub>,
    pub cover: Option<Arc<Image>>,
    pub metadata: EpubMetadata,
}

impl Format {
    pub fn epub_name(&self) -> Option<Cow<'_, str>> {
        Some(self.epub_path.as_ref()?.file_name()?.to_string_lossy())
    }
}
