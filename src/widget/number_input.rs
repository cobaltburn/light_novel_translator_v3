use gpui_kit::{AppContext as _, Context, Entity, Window, base::input::InputState};

/// Bounds and step of a number input.
#[derive(Debug, Clone, Copy)]
pub struct NumberRange {
    pub min: f64,
    pub max: f64,
    pub step: f64,
}

/// An input that holds `value` and steps within `range`.
pub fn number_input_entity<T>(
    value: impl Into<String>,
    range: NumberRange,
    window: &mut Window,
    cx: &mut Context<T>,
) -> Entity<InputState> {
    cx.new(|cx| {
        InputState::new(window, cx)
            .default_value(value.into())
            .min(range.min)
            .max(range.max)
            .step(range.step)
    })
}
