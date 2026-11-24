use crate::{
    app::{Translator, message::Message},
    message::Update,
    screen::Translation,
    server,
};
use gpui_kit::{AppContext, Context, Window};

impl Update for Translator {
    type Message = Message;

    fn update(&mut self, message: Message, window: &mut Window, cx: &mut Context<Self>) {
        match message {
            Message::SetScreen(screen) => self.screen = screen,
            Message::SelectTab(index) => self.active_tab = index,
            Message::NewTab => self.new_tab(window, cx),
            Message::CloseTab(index) => self.close_tab(index),
        }
    }
}

impl Translator {
    fn close_tab(&mut self, index: usize) {
        if self.translations.len() > 1 {
            self.translations.remove(index);
            if self.active_tab > index {
                self.active_tab -= 1;
            }
            self.active_tab = self.active_tab.min(self.translations.len() - 1);
        }
    }

    fn new_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let server_builder = { self.active_tab().read(cx).server.builder() };
        let tab = cx.new(|cx| {
            let mut tab = Translation::from_server(server_builder.build(window, cx), cx);
            tab.update(server::message::Message::LoadModels.into(), window, cx);
            tab
        });
        self.translations.push(tab);
    }
}
