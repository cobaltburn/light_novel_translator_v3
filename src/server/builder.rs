use crate::{
    client::{Client, DEFAULT_CONTEXT_WINDOW, REPEAT_PENALTY, Settings, TEMPERATURE, TOP_P, Think},
    server::{Method, Server, ServerUpdate},
};
use gpui_kit::{AppContext as _, Context, Window, component::select::SelectState};

pub struct ServerBuilder {
    pub client: Client,
    pub think: Think,
    pub context_window: usize,
    pub temperature: f64,
    pub top_p: f64,
    pub repeat_penalty: f64,
    pub method: Method,
}

impl ServerBuilder {
    pub fn build<T: ServerUpdate + 'static>(
        self,
        window: &mut Window,
        cx: &mut Context<T>,
    ) -> Server {
        let models = cx.new(|cx| SelectState::new(Vec::new(), None, window, cx));

        let settings = Settings::from_values(
            self.think,
            self.context_window,
            self.temperature,
            self.top_p,
            self.repeat_penalty,
            window,
            cx,
        );

        Server {
            models,
            settings,
            client: self.client,
            handles: Vec::new(),
            method: self.method,
        }
    }
}

impl Default for ServerBuilder {
    fn default() -> Self {
        Self {
            client: Client::default(),
            think: Think::default(),
            context_window: DEFAULT_CONTEXT_WINDOW,
            temperature: TEMPERATURE,
            top_p: TOP_P,
            repeat_penalty: REPEAT_PENALTY,
            method: Method::default(),
        }
    }
}
