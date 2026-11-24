use gpui_kit::ExternalPaths;
use rbook::Epub;
use std::path::PathBuf;

/// What the format screen can be told to do.
#[derive(Debug)]
pub enum Message {
    SelectEpub,
    DropEpub(ExternalPaths),
    SelectFolder,
    DropFolder(ExternalPaths),
    TitleChanged(String),
    AuthorsChanged(String),
    Build,

    // Results of the tasks above. `EpubLoaded` is boxed, as an `Epub` is many
    // times larger than the other variants.
    EpubLoaded(PathBuf, Box<Epub>),
    /// The folder's name, and each markdown file in it with its content.
    FolderLoaded(String, Vec<(PathBuf, String)>),
}
