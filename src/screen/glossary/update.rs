//! The one place the glossary screen's state changes: [`Update::update`].

use crate::{
    message::Update,
    screen::glossary::{Glossary, message::Message},
};
use gpui_kit::{Context, Window};

impl Update for Glossary {
    type Message = Message;

    fn update(&mut self, message: Message, window: &mut Window, cx: &mut Context<Self>) {
        match message {
            Message::SelectEpub => self.select_epub(window, cx),
            Message::Save => self.save(window, cx),
            Message::SaveChapter(chapter) => self.save_chapter(chapter, window, cx),
            Message::SelectChapter(chapter) => self.current_chapter = chapter,
            Message::RunFromCurrent => {
                let pending = (self.current_chapter..self.chapters.len()).collect();
                self.start(pending, window, cx);
            }
            Message::Run(run) => self.run(run, window, cx),
            Message::Stop => self.stop(),
            Message::Server(message) => self.server.update(message, window, cx),

            // An epub loaded while a run started is dropped, as the run is using the chapters.
            Message::EpubLoaded(path, chapters) => {
                if !self.is_running() {
                    self.set_epub(path, chapters);
                }
            }

            Message::AppendContent { chapter, text } => self.append_content(chapter, &text),
            Message::ChapterDone { chapter, result } => {
                self.chapter_done(chapter, result, window, cx)
            }
        }
    }
}
