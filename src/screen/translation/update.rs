//! The one place the tab's state changes: [`Update::update`], and the changes
//! to its pages as their translation streams in.

use crate::{
    message::Update,
    page::{Activity, Job, Section},
    screen::translation::{Translation, message::Message},
};
use gpui_kit::{Context, Window};

impl Update for Translation {
    type Message = Message;

    fn update(&mut self, message: Message, window: &mut Window, cx: &mut Context<Self>) {
        match message {
            Message::SelectEpub => self.select_epub(window, cx),
            Message::DropEpub(paths) => self.drop_epub(&paths, window, cx),
            Message::SelectGlossary => self.select_glossary(window, cx),
            Message::Save => self.save(window, cx),
            Message::Recover => self.recover(window, cx),
            Message::SavePage(page) => self.save_page(page, window, cx),
            Message::SelectPage(page) => self.current_page = page,
            Message::SetDisplay(display) => self.display = display,
            Message::ScrollTo(part) => self.scroll_handle.scroll_to_top_of_item(part),
            Message::TranslateFromCurrent => {
                let jobs = Job::run_from(self.current_page, self.pages.len());
                self.start(jobs, window, cx);
            }
            Message::Translate(translate) => self.translate(translate, window, cx),
            Message::Stop => self.stop(),
            Message::Server(message) => self.server.update(message, window, cx),

            // A file loaded while a run started is dropped, as the run is using the pages.
            Message::EpubLoaded(path, pages) => {
                if !self.is_running() {
                    self.set_pages(path, pages);
                }
            }
            Message::GlossaryLoaded(path, glossary) => {
                if !self.is_running() {
                    self.glossary = Some((path, glossary));
                }
            }
            Message::Recovered(pages) => {
                if !self.is_running() {
                    self.recover_pages(pages);
                }
            }

            Message::SetActivity { page, activity } => self.set_activity(page, activity),
            Message::AppendContent { page, part, text } => self.append_content(page, part, &text),
            Message::CleanSection { page, part } => self.clean_section(page, part),
            Message::Backup => self.backup(window, cx),
            Message::JobDone { page, result } => self.job_done(page, result, window, cx),
        }
    }
}

impl Translation {
    fn set_activity(&mut self, page: usize, activity: Activity) {
        if let Some(page) = self.pages.get_mut(page) {
            page.activity = activity;
        }
    }

    fn append_content(&mut self, page: usize, part: usize, text: &str) {
        if let Some(section) = self.section_mut(page, part) {
            section.content.push_str(text);
        }
    }

    fn clean_section(&mut self, page: usize, part: usize) {
        if let Some(section) = self.section_mut(page, part) {
            section.clean();
        }
    }

    fn section_mut(&mut self, page: usize, part: usize) -> Option<&mut Section> {
        self.pages.get_mut(page)?.sections.get_mut(part)
    }
}
