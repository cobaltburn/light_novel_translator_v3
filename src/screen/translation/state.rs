use crate::{
    client::SharedHistory,
    error::{Error, Result},
    page::{Job, Page},
    server::Server,
    widget::page::DisplayType,
};
use gpui_kit::{App, Context, FocusHandle, Focusable, ScrollHandle, Window};
use std::{borrow::Cow, collections::VecDeque, path::PathBuf};

#[derive(Debug)]
pub struct Translation {
    pub focus_handle: FocusHandle,
    pub server: Server,
    pub file_path: Option<PathBuf>,
    pub glossary: Option<(PathBuf, String)>,
    pub current_page: usize,
    pub pages: Vec<Page>,
    pub display: DisplayType,
    pub pending: VecDeque<Job>,
    pub history: Option<(usize, SharedHistory)>,
    pub scroll_handle: ScrollHandle,
}

impl Translation {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self::from_server(Server::new(window, cx), cx)
    }

    pub fn from_server(server: Server, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            server,
            file_path: None,
            glossary: None,
            current_page: 0,
            pages: Vec::new(),
            pending: VecDeque::new(),
            display: DisplayType::default(),
            history: None,
            scroll_handle: ScrollHandle::new(),
        }
    }

    pub fn current_page(&self) -> Option<&Page> {
        self.pages(self.current_page)
    }

    pub fn pages(&self, index: usize) -> Option<&Page> {
        self.pages.get(index)
    }

    /// Every page up to and including `page`, the context its translation continues.
    pub fn pages_through(&self, page: usize) -> Result<&[Page]> {
        self.pages.get(..=page).ok_or(Error::Error("No such page"))
    }

    pub fn epub_name(&self) -> Option<Cow<'_, str>> {
        Some(self.file_path.as_ref()?.file_name()?.to_string_lossy())
    }

    pub fn epub_stem(&self) -> Option<Cow<'_, str>> {
        Some(self.file_path.as_ref()?.file_stem()?.to_string_lossy())
    }

    pub fn glossary_name(&self) -> Option<Cow<'_, str>> {
        Some(self.glossary.as_ref()?.0.file_name()?.to_string_lossy())
    }

    pub fn glossary_text(&self) -> Option<&str> {
        self.glossary
            .as_ref()
            .map(|(_, glossary)| glossary.as_str())
    }

    pub fn is_running(&self) -> bool {
        !self.server.handles.is_empty()
    }

    pub fn can_run(&self) -> bool {
        self.server.connected() && self.file_path.is_some()
    }

    pub fn error_jobs(&self) -> Vec<Job> {
        self.pages
            .iter()
            .enumerate()
            .flat_map(|(page, p)| p.error_jobs(page))
            .collect()
    }
}

impl Focusable for Translation {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
