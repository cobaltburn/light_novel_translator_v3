use crate::{
    client::{Client, Settings},
    error::{Error, Result},
    server::{Method, ServerBuilder, ServerUpdate},
};
use gpui_kit::{Context, Entity, Task, Window, component::select::SelectState};

#[derive(Debug)]
pub struct Server {
    pub client: Client,
    pub models: Entity<SelectState<Vec<String>>>,
    pub handles: Vec<Task<Result<()>>>,
    pub settings: Settings,
    pub method: Method,
}

impl Server {
    pub fn new<T: ServerUpdate + 'static>(window: &mut Window, cx: &mut Context<T>) -> Self {
        ServerBuilder::default().build(window, cx)
    }

    pub fn connected(&self) -> bool {
        self.client.connected()
    }

    pub fn builder(&self) -> ServerBuilder {
        ServerBuilder {
            client: self.client.clone(),
            think: self.settings.think,
            context_window: self.settings.context_window.0,
            temperature: self.settings.temperature.0,
            top_p: self.settings.top_p.0,
            repeat_penalty: self.settings.repeat_penalty.0,
            method: self.method,
        }
    }

    pub fn check_ready<T>(&self, cx: &mut Context<T>) -> Result<String> {
        if !self.connected() {
            return Err(Error::ServerError("Not connected to a server"));
        }
        let current_model = self.models.read(cx).selected_value();
        current_model
            .cloned()
            .ok_or(Error::ServerError("No model selected"))
    }
}
