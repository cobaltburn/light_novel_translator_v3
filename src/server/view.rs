use crate::{
    client::{THINK_ORDER, Think},
    server::{METHOD_ORDER, Server, ServerUpdate},
    widget::primary_button,
};
use gpui_kit::{
    Context, Entity, IntoElement, ParentElement as _, Styled as _,
    base::{h_flex, input::InputState},
    component::{
        ActiveTheme,
        input::NumberInput,
        label::Label,
        popover::Popover,
        radio::{Radio, RadioGroup},
        select::Select,
    },
    px,
};

impl Server {
    pub fn model_select<T>(&self, cx: &Context<T>) -> impl IntoElement {
        let width = px(300.0);
        Select::new(&self.models)
            .border_color(cx.theme().primary)
            .max_w(width)
            .menu_width(width)
    }

    pub fn server_popover<T: ServerUpdate + 'static>(
        &self,
        cx: &mut Context<'_, T>,
    ) -> impl IntoElement {
        let think_index = THINK_ORDER.iter().position(|t| t == &self.settings.think);
        let method_index = METHOD_ORDER.iter().position(|m| m == &self.method);

        Popover::new("server-popover")
            .gap_3()
            .trigger(primary_button("server-trigger", "Server"))
            .child(labeled_row(
                "Server:",
                primary_button("server-button", "connect").on_click(cx.listener(
                    |this, _, window, cx| {
                        this.connect(window, cx);
                    },
                )),
            ))
            .child(labeled_row(
                "Think:",
                RadioGroup::horizontal("think-group")
                    .children(THINK_ORDER.iter().copied())
                    .selected_index(think_index)
                    .on_click(cx.listener(|this, index, _, _| this.set_think(index))),
            ))
            .child(labeled_row(
                "Excution:",
                RadioGroup::horizontal("method-group")
                    .children(METHOD_ORDER.iter().copied())
                    .selected_index(method_index)
                    .on_click(cx.listener(|this, index, _, _| this.set_method(index))),
            ))
            .child(number_row("Context window:", &self.inputs.context_window))
            .child(number_row("Temperature:", &self.inputs.temperature))
            .child(number_row("Top P:", &self.inputs.top_p))
            .child(number_row("Repeat penalty:", &self.inputs.repeat_penalty))
    }
}

/// A control with a label in front of it.
fn labeled_row(label: &'static str, control: impl IntoElement) -> impl IntoElement {
    h_flex().gap_2().child(Label::new(label)).child(control)
}

fn number_row(label: &'static str, state: &Entity<InputState>) -> impl IntoElement {
    labeled_row(label, NumberInput::new(state).accessibility_label(label))
}

impl From<Think> for Radio {
    fn from(label: Think) -> Self {
        Self::new(label.to_string()).label(label.to_string())
    }
}
