use gpui_kit::{
    ElementId, SharedString,
    component::button::{Button, ButtonVariants as _},
};

/// The outlined primary button used for every command in the app.
pub fn primary_button(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Button {
    Button::new(id).label(label).primary().outline()
}
