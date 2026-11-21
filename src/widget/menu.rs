use gpui_kit::{Action, FocusHandle, component::menu::PopupMenu};

const MENU_MAX_HEIGHT: f32 = 300.0;
const MENU_MIN_WIDTH: f32 = 200.0;

/// A scrollable context menu whose actions are dispatched to `focus`.
pub fn action_menu(menu: PopupMenu, focus: FocusHandle) -> PopupMenu {
    menu.scrollable(true)
        .max_h(MENU_MAX_HEIGHT)
        .min_w(MENU_MIN_WIDTH)
        .action_context(focus)
}

/// Appends one item per index, labelled `"{label} {index + 1}"`, all of them
/// disabled when `disabled`.
pub fn numbered_items(
    menu: PopupMenu,
    count: usize,
    label: &str,
    disabled: bool,
    action: impl Fn(usize) -> Box<dyn Action>,
) -> PopupMenu {
    (0..count).fold(menu, |menu, i| {
        menu.menu_with_disabled(format!("{label} {}", i + 1), action(i), disabled)
    })
}
