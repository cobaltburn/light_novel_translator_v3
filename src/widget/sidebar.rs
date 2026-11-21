//! A sidebar listing the entries of a run (pages, chapters), shared by the
//! screens that run over them.

use crate::widget::{PrimaryBorder as _, action_menu};
use gpui_kit::{
    Action, App, Context, ElementId, Focusable, Styled as _,
    component::{
        menu::PopupMenu,
        sidebar::{Sidebar, SidebarMenuItem},
    },
};

/// What an entry's sidebar item does: select the entry on click, and offer
/// these actions in its context menu.
pub trait SidebarAction: Focusable + Sized + 'static {
    /// The menu labels for running from the entry to the end, and the entry alone.
    const RUN_LABELS: [&'static str; 2];

    fn save_action(index: usize) -> Box<dyn Action>;
    fn run_all_action(index: usize) -> Box<dyn Action>;
    fn run_one_action(index: usize) -> Box<dyn Action>;
    fn select(&mut self, index: usize, cx: &mut Context<Self>);
}

/// A bordered sidebar holding `items`.
pub fn entry_sidebar(
    id: impl Into<ElementId>,
    items: impl IntoIterator<Item = SidebarMenuItem>,
    cx: &App,
) -> Sidebar<SidebarMenuItem> {
    Sidebar::new(id)
        .h_full()
        .rounded_md()
        .primary_border(cx)
        .children(items)
}

/// The sidebar item for entry `index` of `view`. Its context menu saves and
/// runs the entry, followed by whatever `more_items` adds, and dispatches to
/// `view`'s focus handle, where the view handles the actions. The save and run
/// items are disabled while `running`.
pub fn sidebar_item<T: SidebarAction>(
    index: usize,
    name: &str,
    active: bool,
    running: bool,
    view: &T,
    cx: &Context<T>,
    more_items: impl Fn(PopupMenu) -> PopupMenu + 'static,
) -> SidebarMenuItem {
    let focus_handle = view.focus_handle(cx);
    let [run_all, run_one] = T::RUN_LABELS;
    SidebarMenuItem::new(format!("{index}. {name}"))
        .active(active)
        .on_click(cx.listener(move |this, _, _, cx| this.select(index, cx)))
        .context_menu(move |menu, _, _| {
            more_items(
                action_menu(menu, focus_handle.clone())
                    .menu_with_disabled("save", T::save_action(index), running)
                    .separator()
                    .menu_with_disabled(run_all, T::run_all_action(index), running)
                    .menu_with_disabled(run_one, T::run_one_action(index), running),
            )
        })
}
