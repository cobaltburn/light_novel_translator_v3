use super::actions::Run;
use crate::{error::Result, page::Chapter, server};
use std::path::PathBuf;

/// What the glossary screen can be told to do.
#[derive(Debug)]
pub enum Message {
    SelectEpub,
    Save,
    SaveChapter(usize),
    SelectChapter(usize),
    /// Runs from the current chapter through to the end of the book.
    RunFromCurrent,
    Run(Run),
    Stop,
    Server(server::message::Message),

    // Results of the tasks above.
    EpubLoaded(PathBuf, Vec<Chapter>),

    // The progress of a run, sent by the task of the pass in progress.
    AppendContent {
        chapter: usize,
        text: String,
    },
    /// The pass over `chapter` ended, the last message its task sends.
    ChapterDone {
        chapter: usize,
        result: Result<()>,
    },
}

impl From<server::message::Message> for Message {
    fn from(message: server::message::Message) -> Self {
        Message::Server(message)
    }
}
