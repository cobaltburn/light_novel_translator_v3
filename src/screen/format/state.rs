use crate::{
    epub::{EpubMetadata, FormatPage},
    screen::format::EpubMetadataInput,
};
use gpui_kit::{Context, Image, Window};
use rbook::Epub;
use std::{borrow::Cow, path::PathBuf, sync::Arc};

#[non_exhaustive]
#[derive(Debug)]
pub struct Format {
    pub pages: Vec<FormatPage>,
    pub source_folder: Option<String>,
    pub epub_path: Option<PathBuf>,
    pub epub: Option<Epub>,
    pub cover: Option<Arc<Image>>,
    pub metadata: EpubMetadata,
    pub inputs: EpubMetadataInput,
}

impl Format {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let inputs = EpubMetadataInput::new(window, cx);
        Self {
            pages: Vec::new(),
            source_folder: None,
            epub_path: None,
            epub: None,
            cover: None,
            metadata: EpubMetadata::default(),
            inputs,
        }
    }

    pub fn epub_name(&self) -> Option<Cow<'_, str>> {
        Some(self.epub_path.as_ref()?.file_name()?.to_string_lossy())
    }
}
