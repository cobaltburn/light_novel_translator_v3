use crate::{
    client::{Settings, THINK_ORDER},
    server::{METHOD_ORDER, Server},
};
use gpui_kit::{Context, Window, base::IndexPath};

pub trait ServerUpdate {
    fn set_think(&mut self, index: &usize);

    fn set_method(&mut self, index: &usize);

    /// Access to the live settings so input subscriptions can write back into them.
    fn settings_mut(&mut self) -> &mut Settings;

    fn connect(&mut self, window: &mut Window, cx: &mut Context<'_, Self>)
    where
        Self: Sized;
}

impl Server {
    pub fn set_models<T>(&mut self, models: Vec<String>, window: &mut Window, cx: &mut Context<T>) {
        self.models.update(cx, |this, cx| {
            this.set_items(models.clone(), window, cx);
            this.set_selected_index(Some(IndexPath::new(0)), window, cx);
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
