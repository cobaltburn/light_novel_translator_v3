use crate::{
    client::THINK_ORDER,
    server::{METHOD_ORDER, Server, ServerUpdate},
};
use gpui_kit::{
    Context, Entity, IntoElement, ParentElement as _, Styled as _,
    base::{h_flex, input::InputState},
    component::{
        ActiveTheme,
        button::{Button, ButtonVariants as _},
        input::NumberInput,
        label::Label,
        popover::Popover,
        radio::RadioGroup,
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
        let think_radio: Vec<_> = THINK_ORDER.into_iter().copied().collect();
        let think_index = think_radio.iter().position(|t| t == &self.settings.think);

        let method_radio: Vec<_> = METHOD_ORDER.into_iter().copied().collect();
        let method_index = method_radio.iter().position(|m| m == &self.method);

        Popover::new("server-popover")
            .gap_3()
            .trigger(
                Button::new("server-trigger")
                    .label("Server")
                    .primary()
                    .outline(),
            )
            .child(
                h_flex().gap_2().child(Label::new("Server:")).child(
                    Button::new("server-button")
                        .label("connect")
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.connect(window, cx);
                        }))
                        .primary()
                        .outline(),
                ),
            )
            .child(
                h_flex().gap_2().child(Label::new("Think:")).child(
                    RadioGroup::horizontal("think-group")
                        .children(think_radio)
                        .selected_index(think_index)
                        .on_click(cx.listener(|this, index, _, _| this.set_think(index))),
                ),
            )
            .child(
                h_flex().gap_2().child(Label::new("Excution:")).child(
                    RadioGroup::horizontal("method-group")
                        .children(method_radio)
                        .selected_index(method_index)
                        .on_click(cx.listener(|this, index, _, _| this.set_method(index))),
                ),
            )
            .child(number_row(
                "Context window:",
                &self.settings.context_window.1,
            ))
            .child(number_row("Temperature:", &self.settings.temperature.1))
            .child(number_row("Top P:", &self.settings.top_p.1))
            .child(number_row(
                "Repeat penalty:",
                &self.settings.repeat_penalty.1,
            ))
    }
}

fn number_row(label: &'static str, state: &Entity<InputState>) -> impl IntoElement {
    h_flex()
        .gap_2()
        .child(Label::new(label))
        .child(NumberInput::new(state).accessibility_label(label))
}
