use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Translation,
    Glossary,
    Build,
    Doc,
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let view = match self {
            Screen::Doc => "Document",
            Screen::Build => "Build",
            Screen::Translation => "Translation",
            Screen::Glossary => "Glossary",
        };
        write!(f, "{}", view)
    }
}
