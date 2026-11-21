use crate::{
    client::Settings,
    server::ServerUpdate,
    widget::{NumberRange, number_input_entity},
};
use gpui_kit::{
    Context, Entity, Window,
    base::input::{InputEvent, InputState},
};
use std::str::FromStr;

const USIZE_STEP: f64 = 1.0;
const F64_STEP: f64 = 0.05;

const CONTEXT_WINDOW_RANGE: NumberRange = NumberRange {
    min: 1.0,
    max: 100.0,
    step: USIZE_STEP,
};
const TEMPERATURE_RANGE: NumberRange = NumberRange {
    min: 0.1,
    max: 2.0,
    step: F64_STEP,
};
const TOP_P_RANGE: NumberRange = NumberRange {
    min: 0.1,
    max: 2.0,
    step: F64_STEP,
};
const REPEAT_PENALTY_RANGE: NumberRange = NumberRange {
    min: 0.0,
    max: 2.0,
    step: F64_STEP,
};

/// The number inputs that edit a server's [`Settings`].
#[derive(Debug, Clone)]
pub struct SettingsInputs {
    pub context_window: Entity<InputState>,
    pub temperature: Entity<InputState>,
    pub top_p: Entity<InputState>,
    pub repeat_penalty: Entity<InputState>,
}

impl SettingsInputs {
    /// Inputs showing `settings`, each writing back into the view's settings as it changes.
    pub fn new<T: ServerUpdate>(
        settings: &Settings,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Self {
        Self {
            context_window: bound_input(
                settings.context_window,
                CONTEXT_WINDOW_RANGE,
                |s| &mut s.context_window,
                window,
                cx,
            ),
            temperature: bound_input(
                settings.temperature,
                TEMPERATURE_RANGE,
                |s| &mut s.temperature,
                window,
                cx,
            ),
            top_p: bound_input(settings.top_p, TOP_P_RANGE, |s| &mut s.top_p, window, cx),
            repeat_penalty: bound_input(
                settings.repeat_penalty,
                REPEAT_PENALTY_RANGE,
                |s| &mut s.repeat_penalty,
                window,
                cx,
            ),
        }
    }
}

/// A number input starting at `value`, bound to `field` of the settings.
fn bound_input<T, V>(
    value: V,
    range: NumberRange,
    field: fn(&mut Settings) -> &mut V,
    window: &mut Window,
    cx: &mut Context<T>,
) -> Entity<InputState>
where
    T: ServerUpdate,
    V: FromStr + ToString + 'static,
{
    let input = number_input_entity(value.to_string(), range, window, cx);
    bind_input(&input, field, window, cx);
    input
}

/// Keeps a cached settings value in sync with its input.
///
/// Typing and the +/- buttons (stepping is internal because `step`/`min`/`max`
/// are set) both emit `InputEvent::Change`, so this one handler covers both.
/// Text that doesn't parse yet (e.g. "" or "0.") leaves the last value in place.
pub fn bind_input<T, V>(
    input: &Entity<InputState>,
    field: fn(&mut Settings) -> &mut V,
    window: &mut Window,
    cx: &mut Context<'_, T>,
) where
    T: ServerUpdate,
    V: FromStr + 'static,
{
    cx.subscribe_in(input, window, move |this, state, event, _, cx| {
        if !matches!(event, InputEvent::Change) {
            return;
        }
        if let Ok(value) = state.read(cx).value().trim().parse::<V>() {
            *field(this.settings_mut()) = value;
            cx.notify();
        }
    })
    .detach();
}
