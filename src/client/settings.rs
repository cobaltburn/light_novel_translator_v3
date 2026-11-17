use crate::server::ServerUpdate;
use gpui_kit::{
    AppContext, Context, Entity, Window,
    base::input::{InputEvent, InputState},
    component::radio::Radio,
};
use serde::{Serialize, Serializer};
use serde_json::Value;
use std::{fmt, str::FromStr};

pub const TEMPERATURE: f64 = 0.8;
pub const TOP_P: f64 = 0.8;
pub const REPEAT_PENALTY: f64 = 1.0;
pub const DEFAULT_CONTEXT_WINDOW: usize = 10;
pub const THINK_ORDER: &[Think] = &[Think::None, Think::Low, Think::Medium, Think::High];
pub const USIZE_STEP: usize = 1;
pub const F64_STEP: f64 = 0.05;

#[derive(Debug, Clone)]
pub struct Settings {
    pub think: Think,
    pub context_window: (usize, Entity<InputState>),
    pub temperature: (f64, Entity<InputState>),
    pub top_p: (f64, Entity<InputState>),
    pub repeat_penalty: (f64, Entity<InputState>),
}

impl Settings {
    pub fn new<T: ServerUpdate + 'static>(window: &mut Window, cx: &mut Context<T>) -> Self {
        let context_window = DEFAULT_CONTEXT_WINDOW.to_string();
        let context_window =
            number_input_entity(context_window, 1.0, 100.0, USIZE_STEP as f64, window, cx);

        let temperature =
            number_input_entity(TEMPERATURE.to_string(), 0.1, 2.0, F64_STEP, window, cx);

        let top_p = number_input_entity(TOP_P.to_string(), 0.1, 2.0, F64_STEP, window, cx);

        let repeat_penalty =
            number_input_entity(REPEAT_PENALTY.to_string(), 0.0, 2.0, F64_STEP, window, cx);

        bind_input(&context_window, |s| &mut s.context_window.0, window, cx);
        bind_input(&temperature, |s| &mut s.temperature.0, window, cx);
        bind_input(&top_p, |s| &mut s.top_p.0, window, cx);
        bind_input(&repeat_penalty, |s| &mut s.repeat_penalty.0, window, cx);

        Settings {
            think: Think::default(),
            context_window: (DEFAULT_CONTEXT_WINDOW, context_window),
            temperature: (TEMPERATURE, temperature),
            top_p: (TOP_P, top_p),
            repeat_penalty: (REPEAT_PENALTY, repeat_penalty),
        }
    }

    pub fn from_values<T: ServerUpdate + 'static>(
        think: Think,
        context_window: usize,
        temperature: f64,
        top_p: f64,
        repeat_penalty: f64,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Self {
        let cw = context_window.to_string();
        let cw_state = number_input_entity(cw, 1.0, 100.0, USIZE_STEP as f64, window, cx);

        let temp_state =
            number_input_entity(temperature.to_string(), 0.1, 2.0, F64_STEP, window, cx);

        let top_p_state = number_input_entity(top_p.to_string(), 0.1, 2.0, F64_STEP, window, cx);

        let rp_state =
            number_input_entity(repeat_penalty.to_string(), 0.0, 2.0, F64_STEP, window, cx);

        bind_input(&cw_state, |s| &mut s.context_window.0, window, cx);
        bind_input(&temp_state, |s| &mut s.temperature.0, window, cx);
        bind_input(&top_p_state, |s| &mut s.top_p.0, window, cx);
        bind_input(&rp_state, |s| &mut s.repeat_penalty.0, window, cx);

        Settings {
            think,
            context_window: (context_window, cw_state),
            temperature: (temperature, temp_state),
            top_p: (top_p, top_p_state),
            repeat_penalty: (repeat_penalty, rp_state),
        }
    }

    pub fn agent_params(&self) -> Value {
        serde_json::json!({
            "top_p": &self.top_p.0,
            "repeat_penalty": &self.repeat_penalty.0,
            "think": &self.think,
        })
    }
}

pub fn number_input_entity<T>(
    value: impl Into<String>,
    min: f64,
    max: f64,
    step: f64,
    window: &mut Window,
    cx: &mut Context<T>,
) -> Entity<InputState> {
    cx.new(|cx| {
        InputState::new(window, cx)
            .default_value(value.into())
            .min(min)
            .max(max)
            .step(step)
    })
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
    T: ServerUpdate + 'static,
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

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Think {
    #[default]
    High,
    Medium,
    Low,
    None,
}

impl Serialize for Think {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Think::High => serializer.serialize_str("high"),
            Think::Medium => serializer.serialize_str("medium"),
            Think::Low => serializer.serialize_str("low"),
            Think::None => serializer.serialize_bool(false),
        }
    }
}

impl fmt::Display for Think {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<Think> for Radio {
    fn from(label: Think) -> Self {
        Self::new(label.to_string()).label(label.to_string())
    }
}
