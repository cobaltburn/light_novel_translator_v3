use crate::{
    client::{Client, Settings},
    message::Update,
    server::{Method, Server, SettingsInputs, message::Message},
};
use gpui_kit::{AppContext as _, Context, Window, base::IndexPath, component::select::SelectState};

/// What a new [`Server`] starts from: the connection and settings of another
/// tab, or the defaults.
#[derive(Default)]
pub struct ServerBuilder {
    pub client: Client,
    pub settings: Settings,
    pub method: Method,
}

impl ServerBuilder {
    pub fn build<T>(self, window: &mut Window, cx: &mut Context<T>) -> Server
    where
        T: Update<Message: From<Message>>,
    {
        let models = cx.new(|cx| SelectState::new(Vec::new(), Some(IndexPath::new(0)), window, cx));
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
