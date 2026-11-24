use crate::{
    client::Settings,
    message::Update,
    server::message::{Message, SettingsField},
    widget::{NumberRange, number_input_entity},
};
use gpui_kit::{
    Context, Entity, Window,
    base::input::{InputEvent, InputState},
};

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
    /// Inputs showing `settings`, each sending its text to the view's server as it changes.
    pub fn new<T>(settings: &Settings, window: &mut Window, cx: &mut Context<T>) -> Self
    where
        T: Update<Message: From<Message>>,
    {
        Self {
            context_window: bound_input(
                settings.context_window,
                CONTEXT_WINDOW_RANGE,
                SettingsField::ContextWindow,
                window,
                cx,
            ),
            temperature: bound_input(
                settings.temperature,
                TEMPERATURE_RANGE,
                SettingsField::Temperature,
                window,
                cx,
            ),
            top_p: bound_input(settings.top_p, TOP_P_RANGE, SettingsField::TopP, window, cx),
            repeat_penalty: bound_input(
                settings.repeat_penalty,
                REPEAT_PENALTY_RANGE,
                SettingsField::RepeatPenalty,
                window,
                cx,
            ),
        }
    }
}

/// A number input starting at `value`, bound to `field` of the settings.
fn bound_input<T>(
    value: impl ToString,
    range: NumberRange,
    field: SettingsField,
    window: &mut Window,
    cx: &mut Context<T>,
) -> Entity<InputState>
where
    T: Update<Message: From<Message>>,
{
    let input = number_input_entity(value.to_string(), range, window, cx);
    bind_input(&input, field, window, cx);
    input
}

/// Keeps a settings value in sync with its input, by sending the input's
/// text each time it changes. The server ignores text that doesn't parse.
///
/// Typing and the +/- buttons (stepping is internal because `step`/`min`/`max`
/// are set) both emit `InputEvent::Change`, so this one handler covers both.
fn bind_input<T>(
    input: &Entity<InputState>,
    field: SettingsField,
    window: &mut Window,
    cx: &mut Context<'_, T>,
) where
    T: Update<Message: From<Message>>,
{
    cx.subscribe_in(input, window, move |this, state, event, window, cx| {
        if !matches!(event, InputEvent::Change) {
            return;
        }
        let text = state.read(cx).value().to_string();
        this.dispatch(Message::SettingChanged(field, text).into(), window, cx);
    })
    .detach();
}
