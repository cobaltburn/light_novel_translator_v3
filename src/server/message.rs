use crate::client::Client;

/// The number settings, each edited by one of the server's inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsField {
    ContextWindow,
    Temperature,
    TopP,
    RepeatPenalty,
}

/// What a [`super::Server`] can be told to do. The view that owns the server
/// wraps these in its own messages, and passes them on to [`super::Server::update`].
#[derive(Debug)]
pub enum Message {
    Connect,
    /// Refreshes the model list from the server the view is connected to, if any.
    LoadModels,
    SetThink(usize),
    SetMethod(usize),
    /// The text of a setting's input changed.
    SettingChanged(SettingsField, String),

    Connected(Client, Vec<String>),
    ModelsLoaded(Vec<String>),
}
