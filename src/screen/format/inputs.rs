use crate::{epub::EpubMetadata, screen::format::Format};
use gpui_kit::{
    AppContext as _, Context, Entity, Window,
    base::input::{InputEvent, InputState},
};

#[non_exhaustive]
#[derive(Debug)]
pub struct EpubMetadataInput {
    pub title: Entity<InputState>,
    pub authors: Entity<InputState>,
}

impl EpubMetadataInput {
    pub fn new(window: &mut Window, cx: &mut Context<Format>) -> Self {
        let title = cx.new(|cx| InputState::new(window, cx));
        bind_input(&title, |e| &mut e.title, window, cx);
        let authors = cx.new(|cx| InputState::new(window, cx));
        bind_input(&authors, |e| &mut e.authors, window, cx);

        Self { title, authors }
    }

    pub fn clear(&self, window: &mut Window, cx: &mut Context<Format>) {
        self.title.update(cx, |e, cx| e.clean(window, cx));
        self.authors.update(cx, |e, cx| e.clean(window, cx));
    }
}

fn bind_input(
    input: &Entity<InputState>,
    field: fn(&mut EpubMetadata) -> &mut String,
    window: &mut Window,
    cx: &mut Context<'_, Format>,
) {
    cx.subscribe_in(input, window, move |this, state, event, _, cx| {
        if !matches!(event, InputEvent::Change) {
            return;
        }
        let value = state.read(cx).value();
        *field(&mut this.metadata) = value.to_string();
        cx.notify();
    })
    .detach();
}
