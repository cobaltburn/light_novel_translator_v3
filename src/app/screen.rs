use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Translation,
    Glossary,
    Build,
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let view = match self {
            Screen::Build => "Build",
            Screen::Translation => "Translation",
            Screen::Glossary => "Glossary",
        };
        write!(f, "{}", view)
    }
}
