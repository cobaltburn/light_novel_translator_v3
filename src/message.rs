//! Iced-style state changes: a view's state changes only in [`Update::update`],
//! from the messages its listeners, inputs and tasks send it.

use crate::error::Result;
use gpui_kit::{App, AppContext, Context, WeakEntity, Window};

/// A view whose state changes only in [`Self::update`], from the messages sent to it.
pub trait Update: Sized + 'static {
    type Message: 'static;

    /// Applies `message` to the view.
    fn update(&mut self, message: Self::Message, window: &mut Window, cx: &mut Context<Self>);

    /// Applies `message`, then re-renders the view. Every sender goes through this.
    fn dispatch(&mut self, message: Self::Message, window: &mut Window, cx: &mut Context<Self>) {
        self.update(message, window, cx);
        cx.notify();
    }
}

/// A listener that sends `cx`'s view the message `message` builds from each
/// event: a click, a drop, a tab or radio index, or a GPUI action.
///
/// This does what `cx.listener` does, but builds the closure itself: the type
/// `cx.listener` returns keeps `cx` borrowed, and this one doesn't.
pub fn send<T, E, F>(
    cx: &Context<T>,
    message: F,
) -> impl Fn(&E, &mut Window, &mut App) + 'static + use<T, E, F>
where
    T: Update,
    E: ?Sized,
    F: Fn(&E) -> T::Message + 'static,
{
    let view = cx.entity().downgrade();
    move |event: &E, window: &mut Window, cx: &mut App| {
        view.update(cx, |this, cx| this.dispatch(message(event), window, cx))
            .ok();
    }
}

/// Sending messages to a view from a task.
pub trait SendAsync<T: Update> {
    /// Sends `message` to the view. A message that ends the task must be sent
    /// as its last step, since `update` may drop the task that sent it.
    fn send<C: AppContext>(&self, cx: &mut C, message: T::Message) -> Result<()>;
}

impl<T: Update> SendAsync<T> for WeakEntity<T> {
    fn send<C: AppContext>(&self, cx: &mut C, message: T::Message) -> Result<()> {
        self.update_in(cx, |this, window, cx| this.dispatch(message, window, cx))?;
        Ok(())
    }
}
