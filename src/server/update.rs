use crate::{
    client::{Client, THINK_ORDER},
    error::TaskExt as _,
    message::{SendAsync as _, Update},
    server::{
        METHOD_ORDER, Server,
        message::{Message, SettingsField},
    },
};
use gpui_kit::{Context, Window, base::IndexPath};

impl Server {
    /// Applies `message` to the server of the view `cx` belongs to. The view
    /// passes on the server's messages to this, and gets the results of the
    /// server's tasks back as its own messages.
    pub fn update<T>(&mut self, message: Message, window: &mut Window, cx: &mut Context<T>)
    where
        T: Update<Message: From<Message>>,
    {
        match message {
            Message::Connect => connect(window, cx),
            Message::LoadModels => self.load_models(window, cx),
            Message::SetThink(index) => self.set_think(index),
            Message::SetMethod(index) => self.set_method(index),
            Message::SettingChanged(field, text) => self.set_setting(field, &text),
            Message::Connected(client, models) => {
                self.client = client;
                self.set_models(models, window, cx);
            }
            Message::ModelsLoaded(models) => self.set_models(models, window, cx),
        }
    }

    fn load_models<T>(&self, window: &mut Window, cx: &mut Context<T>)
    where
        T: Update<Message: From<Message>>,
    {
        if !self.connected() {
            return;
        }
        let client = self.client.clone();
        cx.spawn_in(window, async move |this, cx| {
            let models = client.get_models().await?;
            this.send(cx, Message::ModelsLoaded(models).into())?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to load models", window, cx);
    }

    fn set_models<T>(&mut self, models: Vec<String>, window: &mut Window, cx: &mut Context<T>) {
        self.models.update(cx, |this, cx| {
            this.set_items(models.clone(), window, cx);
            this.set_selected_index(Some(IndexPath::new(0)), window, cx);
        });
    }

    fn set_think(&mut self, index: usize) {
        if let Some(think) = THINK_ORDER.get(index).cloned() {
            self.settings.think = think;
        }
    }

    fn set_method(&mut self, index: usize) {
        if let Some(method) = METHOD_ORDER.get(index).cloned() {
            self.method = method;
        }
    }

    /// Text that doesn't parse yet (e.g. "" or "0.") leaves the last value in place.
    fn set_setting(&mut self, field: SettingsField, text: &str) {
        let text = text.trim();
        let settings = &mut self.settings;
        match field {
            SettingsField::ContextWindow => set_parsed(&mut settings.context_window, text),
            SettingsField::Temperature => set_parsed(&mut settings.temperature, text),
            SettingsField::TopP => set_parsed(&mut settings.top_p, text),
            SettingsField::RepeatPenalty => set_parsed(&mut settings.repeat_penalty, text),
        }
    }

    pub fn abort(&mut self) {
        self.handles.clear();
    }
}

/// Connects to the server and loads its models.
fn connect<T>(window: &mut Window, cx: &mut Context<T>)
where
    T: Update<Message: From<Message>>,
{
    cx.spawn_in(window, async move |this, cx| {
        let client = Client::ollama();
        let models = client.get_models().await?;
        this.send(cx, Message::Connected(client, models).into())?;
        Ok(())
    })
    .detach_and_prompt_err("Failed to connect to server", window, cx);
}

fn set_parsed<V: std::str::FromStr>(value: &mut V, text: &str) {
    if let Ok(parsed) = text.parse() {
        *value = parsed;
    }
}
