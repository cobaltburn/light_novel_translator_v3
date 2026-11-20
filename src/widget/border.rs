use gpui_kit::{App, Styled, component::ActiveTheme as _};

pub trait PrimaryBorder: Styled + Sized {
    /// A thin border in the theme's primary color, used to frame panels.
    fn primary_border(self, cx: &App) -> Self {
        self.border_1().border_color(cx.theme().primary)
    }
}

impl<E: Styled> PrimaryBorder for E {}
