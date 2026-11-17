use gpui_kit::{
    App, ClickEvent, ElementId, IntoElement, RenderOnce, Window,
    assets::IconName,
    component::{
        Sizable as _,
        button::{Button, ButtonVariants as _},
    },
};

#[derive(IntoElement)]
pub struct AddButton {
    pub id: ElementId,
    pub on_click: Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>,
}

impl AddButton {
    pub fn new(
        id: impl Into<ElementId>,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            on_click: Box::new(on_click),
        }
    }
}

impl RenderOnce for AddButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        Button::new((self.id, "add-button"))
            .icon(IconName::Plus)
            .ghost()
            .xsmall()
            .on_click(self.on_click)
    }
}
