use crate::{
    message::Update as _,
    screen::format::{Format, message::Message},
};
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
        bind_input(&title, Message::TitleChanged, window, cx);
        let authors = cx.new(|cx| InputState::new(window, cx));
        bind_input(&authors, Message::AuthorsChanged, window, cx);

        Self { title, authors }
    }

    pub fn clear(&self, window: &mut Window, cx: &mut Context<Format>) {
        self.title.update(cx, |e, cx| e.clean(window, cx));
        self.authors.update(cx, |e, cx| e.clean(window, cx));
    }
}

/// Sends the message `changed` builds from the input's text each time it changes.
fn bind_input(
    input: &Entity<InputState>,
    changed: fn(String) -> Message,
    window: &mut Window,
    cx: &mut Context<'_, Format>,
) {
    cx.subscribe_in(input, window, move |this, state, event, window, cx| {
        if matches!(event, InputEvent::Change) {
            let value = state.read(cx).value().to_string();
            this.dispatch(changed(value), window, cx);
        }
    })
    .detach();
}
