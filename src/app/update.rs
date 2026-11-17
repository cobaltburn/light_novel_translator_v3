use crate::{
    app::{Screen, Translator},
    screen::Translation,
};
use gpui_kit::{AppContext, Context, Window};

impl Translator {
    pub fn close_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.translations.len() > 1 {
            self.translations.remove(index);
            if self.active_tab > index {
                self.active_tab -= 1;
            }
            self.active_tab = self.active_tab.min(self.translations.len() - 1);
            cx.notify();
        }
    }

    pub fn new_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let server_builder = { self.active_tab().read(cx).server.builder() };
        let tab = cx.new(|cx| {
            let mut tab = Translation::from_server(server_builder.build(window, cx), cx);
            tab.load_models(window, cx);
            tab
        });
        self.translations.push(tab);
        cx.notify();
    }

    pub fn set_screen(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.screen = screen;
        cx.notify();
    }
}
