//! The tab's connection to the translation server.

use crate::{
    client::{Client, Settings},
    error::TaskExt as _,
    screen::Translation,
    server::ServerUpdate,
};
use gpui_kit::{Context, Window};

impl Translation {
    /// Refreshes the model list from the server the tab is connected to, if any.
    pub fn load_models(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.server.connected() {
            return;
        }
        let client = self.server.client.clone();
        cx.spawn_in(window, async move |this, cx| {
            let models = client.get_models().await?;
            this.update_in(cx, |this, window, cx| {
                this.server.set_models(models, window, cx);
                cx.notify();
            })?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to load models", window, cx);
    }
}

impl ServerUpdate for Translation {
    fn set_think(&mut self, index: &usize) {
        self.server.set_think(index);
    }

    fn set_method(&mut self, index: &usize) {
        self.server.set_method(index);
    }

    fn settings_mut(&mut self) -> &mut Settings {
        &mut self.server.settings
    }

    fn connect(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) {
        cx.spawn_in(window, async move |this, cx| {
            let client = Client::ollama();
            let models = client.get_models().await?;

            this.update_in(cx, |this, window, cx| {
                this.server.client = client;
                this.server.set_models(models, window, cx);
                cx.notify();
            })?;

            Ok(())
        })
        .detach_and_prompt_err("Failed to connect to server", window, cx);
    }
}
