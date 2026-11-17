use std::path::Path;

use rfd::FileHandle;

pub async fn select_epub_dialog() -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .set_title("select epub")
        .add_filter("epub", &["epub"])
        .pick_file()
        .await
}

pub async fn select_glossary_dialog() -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .set_title("select glossary")
        .pick_file()
        .await
}

pub async fn save_translation_dialog(name: impl Into<String>) -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .set_title("save translation")
        .set_file_name(name)
        .save_file()
        .await
}

pub async fn select_recovery_dialog() -> Option<FileHandle> {
    rfd::AsyncFileDialog::new()
        .add_filter("json", &["json"])
        .set_title("recovery")
        .pick_file()
        .await
}

pub async fn complete_dialog(file_name: impl AsRef<str>) {
    let file_name = Path::new(file_name.as_ref())
        .file_prefix()
        .unwrap_or_default()
        .to_string_lossy();

    rfd::AsyncMessageDialog::new()
        .set_title("Translation Complete")
        .set_description(file_name)
        .set_buttons(rfd::MessageButtons::Ok)
        .show()
        .await;
}
