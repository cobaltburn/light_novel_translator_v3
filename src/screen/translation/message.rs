use crate::{
    error::Result,
    page::{Activity, Page},
    screen::translation::Translate,
    server,
    widget::page::DisplayType,
};
use gpui_kit::ExternalPaths;
use std::path::PathBuf;

/// What a translation tab can be told to do.
#[derive(Debug)]
pub enum Message {
    SelectEpub,
    DropEpub(ExternalPaths),
    SelectGlossary,
    Save,
    Recover,
    SavePage(usize),
    SelectPage(usize),
    SetDisplay(DisplayType),
    ScrollTo(usize),
    TranslateFromCurrent,
    Translate(Translate),
    Stop,
    Server(server::message::Message),

    // Results of the tasks above.
    EpubLoaded(PathBuf, Vec<Page>),
    GlossaryLoaded(PathBuf, String),
    Recovered(Vec<Page>),

    // The progress of a run, sent by the task of the job in progress.
    SetActivity {
        page: usize,
        activity: Activity,
    },
    AppendContent {
        page: usize,
        part: usize,
        text: String,
    },
    ClearSection {
        page: usize,
        part: usize,
    },
    CleanSection {
        page: usize,
        part: usize,
    },
    /// The job over `page` ended, the last message its task sends.
    JobDone {
        page: usize,
        result: Result<()>,
    },
}

impl From<server::message::Message> for Message {
    fn from(message: server::message::Message) -> Self {
        Message::Server(message)
    }
}
