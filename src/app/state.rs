use crate::{
    app::Screen,
    screen::{format::Format, translation::Translation},
};
use gpui_kit::{AppContext, Context, Entity, Window};

#[derive(Debug)]
pub struct Translator {
    pub screen: Screen,
    pub active_tab: usize,
    pub translations: Vec<Entity<Translation>>,
    pub format: Entity<Format>,
}

impl Translator {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let translation_entity = cx.new(|cx| Translation::new(window, cx));
        let format = cx.new(|_| Format::default());
        Translator {
            format,
            active_tab: 0,
            screen: Screen::default(),
            translations: vec![translation_entity],
        }
    }

    pub fn active_tab(&self) -> &Entity<Translation> {
        self.translations.get(self.active_tab).unwrap()
    }
}
