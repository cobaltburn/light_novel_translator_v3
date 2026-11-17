use std::fmt;

use gpui_kit::component::radio::Radio;

pub const METHOD_ORDER: &[Method] = &[Method::Chain, Method::History];

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Method {
    #[default]
    History,
    Chain,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<Method> for Radio {
    fn from(label: Method) -> Self {
        Self::new(label.to_string()).label(label.to_string())
    }
}
