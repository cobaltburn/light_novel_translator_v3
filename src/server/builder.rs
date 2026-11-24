use crate::{
    client::{Client, Settings},
    server::{Method, Server, ServerUpdate, SettingsInputs},
};
use gpui_kit::{AppContext as _, Context, Window, base::IndexPath, component::select::SelectState};

/// What a new [`Server`] starts from: the connection and settings of another
/// tab, or the defaults.
#[derive(Default)]
pub struct ServerBuilder {
    pub client: Client,
    pub settings: Settings,
    pub method: Method,
    pub model: Option<String>,
}

impl ServerBuilder {
    pub fn build<T: ServerUpdate>(self, window: &mut Window, cx: &mut Context<T>) -> Server {
        // Until the models load, the list is just the selected model.
        let (items, selected) = match self.model {
            Some(model) => (vec![model], Some(IndexPath::new(0))),
            None => (Vec::new(), None),
        };
        let models = cx.new(|cx| SelectState::new(items, selected, window, cx));
        let inputs = SettingsInputs::new(&self.settings, window, cx);

        Server {
            models,
            settings: self.settings,
            inputs,
            client: self.client,
            handles: Vec::new(),
            method: self.method,
        }
    }
}
