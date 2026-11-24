use crate::{
    client::{Client, Settings, THINK_ORDER},
    error::TaskExt as _,
    server::{METHOD_ORDER, Server},
};
use gpui_kit::{Context, Window, base::IndexPath};

/// A view that owns a [`Server`]. The server's popover and inputs update it
/// through these methods, which all work on [`Self::server_mut`].
pub trait ServerUpdate: Sized + 'static {
    fn server_mut(&mut self) -> &mut Server;

    fn set_think(&mut self, index: &usize) {
        self.server_mut().set_think(index);
    }

    fn set_method(&mut self, index: &usize) {
        self.server_mut().set_method(index);
    }

    /// Access to the live settings so input subscriptions can write back into them.
    fn settings_mut(&mut self) -> &mut Settings {
        &mut self.server_mut().settings
    }

    /// Connects to the server and loads its models.
    fn connect(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        cx.spawn_in(window, async move |this, cx| {
            let client = Client::ollama();
            let models = client.get_models().await?;

            this.update_in(cx, |this, window, cx| {
                let server = this.server_mut();
                server.client = client;
                server.set_models(models, window, cx);
                cx.notify();
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to connect to server", window, cx);
    }

    /// Refreshes the model list from the server the view is connected to, if any.
    fn load_models(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let server = self.server_mut();
        if !server.connected() {
            return;
        }
        let client = server.client.clone();
        cx.spawn_in(window, async move |this, cx| {
            let models = client.get_models().await?;
            this.update_in(cx, |this, window, cx| {
                this.server_mut().set_models(models, window, cx);
                cx.notify();
            })?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to load models", window, cx);
    }
}

impl Server {
    pub fn set_models<T>(&mut self, models: Vec<String>, window: &mut Window, cx: &mut Context<T>) {
        self.models.update(cx, |this, cx| {
            // `set_items` keeps the selected model if the new list still has it.
            this.set_items(models.clone(), window, cx);
            if this.selected_value().is_none() {
                this.set_selected_index(Some(IndexPath::new(0)), window, cx);
            }
        });
    }

    pub fn set_think(&mut self, index: &usize) {
        if let Some(think) = THINK_ORDER.get(*index).cloned() {
            self.settings.think = think;
        }
    }

    pub fn set_method(&mut self, index: &usize) {
        if let Some(method) = METHOD_ORDER.get(*index).cloned() {
            self.method = method;
        }
    }

    pub fn abort(&mut self) {
        self.handles.clear();
    }
}
