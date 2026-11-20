//! Views of a [`crate::page::Page`], shared by any screen that lists or shows pages.

mod display;
mod errors;
mod section;
mod sidebar;
mod text;

pub use display::*;
pub use section::*;
pub use sidebar::*;
pub use text::*;
