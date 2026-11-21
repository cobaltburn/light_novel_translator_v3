use crate::{
    client::{Client, Settings},
    error::{Error, Result},
    server::{Method, ServerBuilder, ServerUpdate, SettingsInputs},
};
use gpui_kit::{Context, Entity, Task, Window, component::select::SelectState};

#[derive(Debug)]
pub struct Server {
    pub client: Client,
    pub models: Entity<SelectState<Vec<String>>>,
    pub handles: Vec<Task<Result<()>>>,
    pub settings: Settings,
    pub inputs: SettingsInputs,
    pub method: Method,
}

impl Server {
    pub fn new<T: ServerUpdate>(window: &mut Window, cx: &mut Context<T>) -> Self {
        ServerBuilder::default().build(window, cx)
    }

    pub fn connected(&self) -> bool {
        self.client.connected()
    }

    pub fn builder(&self) -> ServerBuilder {
        ServerBuilder {
            client: self.client.clone(),
            settings: self.settings,
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
