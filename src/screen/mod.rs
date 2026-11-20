pub mod format;
pub mod translation;

pub use translation::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayType {
    #[default]
    Full,
    End,
    Japanese,
}
