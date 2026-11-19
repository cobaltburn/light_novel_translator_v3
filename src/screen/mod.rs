pub mod translation;

use gpui_kit::FocusHandle;
pub use translation::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayType {
    #[default]
    Full,
    End,
    Japanese,
}

pub trait FocusHandleAccess {
    fn focus_handle(&self) -> &FocusHandle;
}
