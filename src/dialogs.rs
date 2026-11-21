use rfd::FileHandle;
use std::path::{Path, PathBuf};

pub async fn select_epub() -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .set_title("select epub")
        .add_filter("epub", &["epub"])
        .pick_file()
        .await
}

pub async fn select_glossary() -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .set_title("select glossary")
        .pick_file()
        .await
}

pub async fn save_translation(name: impl Into<String>) -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .set_title("save translation")
        .set_file_name(name)
        .save_file()
        .await
}

pub async fn save_epub(file_name: impl Into<String>) -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .set_title("save epub")
        .set_file_name(file_name)
        .add_filter("epub", &["epub"])
        .save_file()
        .await
}

pub async fn save_file(file_name: impl Into<String>) -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .set_title("save")
        .set_file_name(file_name)
        .save_file()
        .await
}

pub async fn select_recovery() -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .add_filter("json", &["json"])
        .set_title("recovery")
        .pick_file()
        .await
}

pub async fn select_translated_folder(directory: Option<PathBuf>) -> Option<FileHandle> {
    let dialog = rfd::AsyncFileDialog::new().set_title("select translated folder");
    let dialog = match directory {
        Some(path) => dialog.set_directory(path),
        None => dialog,
    };
    dialog.pick_folder().await
}

pub async fn complete(file_name: impl AsRef<str>) {
    completed("Translation Complete", file_name.as_ref()).await;
}

pub async fn glossary_complete(file_name: impl AsRef<str>) {
    completed("Glossary Complete", file_name.as_ref()).await;
}

/// Tells the user a run over `file_name` finished.
async fn completed(title: &str, file_name: &str) {
    let file_name = Path::new(file_name)
        .file_prefix()
        .unwrap_or_default()
        .to_string_lossy();

    rfd::AsyncMessageDialog::new()
        .set_title(title)
        .set_description(file_name)
        .set_buttons(rfd::MessageButtons::Ok)
        .show()
        .await;
}
