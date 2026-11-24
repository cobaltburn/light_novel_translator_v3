use crate::app::Screen;

/// What the root view can be told to do.
#[derive(Debug)]
pub enum Message {
    SetScreen(Screen),
    SelectTab(usize),
    NewTab,
    CloseTab(usize),
}
