use crate::{app::Screen, screen::translation::Translation};
use gpui_kit::{AppContext, Context, Entity, Window};

#[derive(Debug)]
pub struct Translator {
    pub screen: Screen,
    pub active_tab: usize,
    pub translations: Vec<Entity<Translation>>,
}

impl Translator {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let entity = cx.new(|cx| Translation::new(window, cx));
        Translator {
            screen: Screen::default(),
            active_tab: 0,
            translations: vec![entity],
        }
    }

    pub fn active_tab(&self) -> &Entity<Translation> {
        self.translations.get(self.active_tab).unwrap()
    }
}
