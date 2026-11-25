use crate::{
    app::Screen,
    screen::{Format, Glossary, Translation},
};
use gpui_kit::{AppContext, Context, Entity, ScrollHandle, Window};

#[derive(Debug)]
pub struct Translator {
    pub screen: Screen,
    pub active_tab: usize,
    pub tab_scroll_handle: ScrollHandle,
    pub translations: Vec<Entity<Translation>>,
    pub glossary: Entity<Glossary>,
    pub format: Entity<Format>,
}

impl Translator {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let translation_entity = cx.new(|cx| Translation::new(window, cx));
        let format = cx.new(|cx| Format::new(window, cx));
        let glossary = cx.new(|cx| Glossary::new(window, cx));
        Translator {
            format,
            glossary,
            active_tab: 0,
            screen: Screen::default(),
            translations: vec![translation_entity],
            tab_scroll_handle: ScrollHandle::new(),
        }
    }

    pub fn active_tab(&self) -> &Entity<Translation> {
        self.translations.get(self.active_tab).unwrap()
    }
}
