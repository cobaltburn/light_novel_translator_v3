use gpui_kit::{App, AsyncWindowContext, PromptLevel, Task, Window};
use quick_xml::events::attributes::AttrError;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("ServerError: {0}")]
    ServerError(&'static str),

    #[error("BuildError: {0}")]
    BuildError(&'static str),

    #[error("Error: {0}")]
    Error(&'static str),

    #[error("Failed to save:\n{}", save_failures(.0))]
    SaveError(Vec<(PathBuf, std::io::Error)>),

    #[error("Page has no file name: {}", .0.display())]
    InvalidPagePath(PathBuf),

    #[error("Pages would overwrite each other:\n{}", page_collisions(.0))]
    PageNameCollision(Vec<Vec<PathBuf>>),

    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ArchiveError(#[from] rbook::ebook::errors::ArchiveError),

    #[error(transparent)]
    Ebookerror(#[from] rbook::ebook::errors::EbookError),

    #[error(transparent)]
    XmlError(#[from] quick_xml::Error),

    #[error(transparent)]
    EncodingError(#[from] quick_xml::encoding::EncodingError),

    #[error(transparent)]
    AttrError(#[from] AttrError),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error(transparent)]
    PromptError(#[from] rig::run::PromptError),

    #[error(transparent)]
    ProviderError(#[from] rig::ProviderError),

    #[error(transparent)]
    MemoryError(#[from] rig::memory::SessionError),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    GpuiError(#[from] gpui_kit::private::anyhow::Error),

    #[error(transparent)]
    FutureError(#[from] futures::channel::oneshot::Canceled),
}

fn page_collisions(collisions: &[Vec<PathBuf>]) -> String {
    collisions
        .iter()
        .map(|paths| {
            paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn save_failures(failures: &[(PathBuf, std::io::Error)]) -> String {
    failures
        .iter()
        .map(|(path, error)| format!("{}: {error}", path.display()))
        .collect::<Vec<_>>()
        .join("\n")
}

impl Error {
    pub async fn display_error(self, title: &'static str, cx: &mut AsyncWindowContext) {
        log::error!("{self:#}");
        let detail = format!("{self:#}");
        if let Ok(answer) = cx.update(|window, cx| {
            window.prompt(PromptLevel::Critical, title, Some(&detail), &["OK"], cx)
        }) {
            answer.await.ok();
        }
    }

    /// Shows the error in a dialog of its own, so it outlives the task reporting it.
    pub fn prompt(self, title: &'static str, window: &Window, cx: &App) {
        Task::ready(Err::<(), _>(self)).detach_and_prompt_err(title, window, cx);
    }
}

pub trait TaskExt {
    /// Run the task. If it returns an error, show it in a dialog.
    fn detach_and_prompt_err(self, title: &'static str, window: &Window, cx: &App);
}

impl<T: 'static> TaskExt for Task<Result<T>> {
    fn detach_and_prompt_err(self, title: &'static str, window: &Window, cx: &App) {
        window
            .spawn(cx, async move |cx| {
                if let Err(error) = self.await {
                    error.display_error(title, cx).await;
                }
            })
            .detach();
    }
}
