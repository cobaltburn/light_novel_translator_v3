use crate::{page::Chapter, server::Server};
use gpui_kit::{App, Context, FocusHandle, Focusable, Window};
use std::{borrow::Cow, collections::VecDeque, path::PathBuf};

#[derive(Debug)]
pub struct Glossary {
    pub focus_handle: FocusHandle,
    pub server: Server,
    pub file_path: Option<PathBuf>,
    pub current_chapter: usize,
    pub chapters: Vec<Chapter>,
    /// Chapters the current run has not sent yet.
    pub pending: VecDeque<usize>,
}

impl Glossary {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            server: Server::new(window, cx),
            file_path: None,
            current_chapter: 0,
            chapters: Vec::new(),
            pending: VecDeque::new(),
        }
    }

    pub fn epub_name(&self) -> Option<Cow<'_, str>> {
        Some(self.file_path.as_ref()?.file_name()?.to_string_lossy())
    }

    pub fn file_stem(&self) -> Option<Cow<'_, str>> {
        Some(self.file_path.as_ref()?.file_stem()?.to_string_lossy())
    }

    pub fn is_running(&self) -> bool {
        !self.server.handles.is_empty()
    }

    pub fn current_chapter(&self) -> Option<&Chapter> {
        self.chapters.get(self.current_chapter)
    }

    pub fn can_run(&self) -> bool {
        self.server.connected() && self.file_path.is_some()
    }
}

impl Focusable for Glossary {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
