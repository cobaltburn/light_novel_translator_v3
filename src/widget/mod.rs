mod add_button;
mod close_button;
mod label_button;

pub use add_button::*;
pub use close_button::*;
use gpui_kit::{App, ClickEvent, Window};
pub use label_button::*;

pub type ListnerFn = dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static;
