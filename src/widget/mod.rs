mod add_button;
mod border;
mod button;
mod close_button;
mod label_button;
mod menu;
mod number_input;
pub mod page;

pub use add_button::*;
pub use border::*;
pub use button::*;
pub use close_button::*;
pub use label_button::*;
pub use menu::*;
pub use number_input::*;

use gpui_kit::{App, ClickEvent, Window};

pub type ListnerFn = dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static;
