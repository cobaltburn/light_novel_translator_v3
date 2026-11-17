use crate::{
    client::SharedHistory,
    page::{Job, Page},
    screen::DisplayType,
    server::Server,
};
use gpui_kit::{Context, ElementId, FocusHandle, Window, accesskit::Uuid};
use std::{borrow::Cow, collections::VecDeque, path::PathBuf};

#[derive(Debug)]
pub struct Translation {
    pub id: ElementId,
    pub focus_handle: FocusHandle,
    pub server: Server,
    pub file_path: Option<PathBuf>,
    pub glossary: Option<(PathBuf, String)>,
    pub current_page: usize,
    pub pages: Vec<Page>,
    pub display: DisplayType,
    pub pending: VecDeque<Job>,
    pub history: Option<(usize, SharedHistory)>,
}

impl Translation {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            id: Uuid::new_v4().into(),
            focus_handle: cx.focus_handle(),
            server: Server::new(window, cx),
            file_path: None,
            glossary: None,
            current_page: 0,
            pages: Vec::new(),
            pending: VecDeque::new(),
            display: DisplayType::default(),
            history: None,
        }
    }

    pub fn from_server(server: Server, cx: &mut Context<Self>) -> Self {
        Self {
            id: Uuid::new_v4().into(),
            focus_handle: cx.focus_handle(),
            server: server,
            file_path: None,
            glossary: None,
            current_page: 0,
            pages: Vec::new(),
            pending: VecDeque::new(),
            display: DisplayType::default(),
            history: None,
        }
    }

    pub fn current_page(&self) -> Option<&Page> {
        self.pages.get(self.current_page)
    }

    pub fn epub_name(&self) -> Option<Cow<'_, str>> {
        Some(self.file_path.as_ref()?.file_name()?.to_string_lossy())
    }

    pub fn glossary_name(&self) -> Option<Cow<'_, str>> {
        Some(self.glossary.as_ref()?.0.file_name()?.to_string_lossy())
    }

    pub fn is_running(&self) -> bool {
        !self.server.handles.is_empty()
    }

    pub fn can_translate(&self) -> bool {
        self.server.connected() && self.file_path.is_some()
    }
}
