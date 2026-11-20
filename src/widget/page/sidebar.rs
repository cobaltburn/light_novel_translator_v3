use crate::{
    page::{Activity, Page},
    widget::{action_menu, numbered_items},
};
use gpui_kit::{
    Action, Context, Div, Focusable, Hsla, ParentElement as _, Styled as _,
    assets::IconName,
    base::h_flex,
    component::{Icon, sidebar::SidebarMenuItem},
    div,
};

/// The actions a page's sidebar item offers in its context menu.
pub trait SidebarAction: Focusable + Sized + 'static {
    fn save_action(page: usize) -> Box<dyn Action>;
    fn all_action(page: usize) -> Box<dyn Action>;
    fn page_action(page: usize) -> Box<dyn Action>;
    fn section_action(page: usize, part: usize) -> Box<dyn Action>;
    fn set_page(&mut self, page: usize, cx: &mut Context<Self>);
}

impl Page {
    /// The page's item in `view`'s sidebar. Its context menu dispatches to
    /// `view`'s focus handle, where the view handles the actions.
    pub fn sidebar_menu_item<T: SidebarAction>(
        &self,
        page: usize,
        active: bool,
        view: &T,
        cx: &Context<T>,
    ) -> SidebarMenuItem {
        let focus_handle = view.focus_handle(cx);
        let file_stem = self.file_stem().unwrap_or_default();
        let activity = self.activity;
        let sections = self.sections.len();
        SidebarMenuItem::new(format!("{}. {}", page, file_stem))
            .active(active)
            .on_click(cx.listener(move |this, _, _, cx| this.set_page(page, cx)))
            .context_menu(move |menu, _, _| {
                let menu = action_menu(menu, focus_handle.clone())
                    .menu("save", T::save_action(page))
                    .separator()
                    .menu("translate", T::all_action(page))
                    .menu("translate page", T::page_action(page))
                    .separator();
                numbered_items(menu, sections, "translate part", |part| {
                    T::section_action(page, part)
                })
            })
            .suffix(move |_, _| activity_icon(activity))
    }
}

/// How far a page's translation got, shown beside its name.
pub fn activity_icon(activity: Activity) -> Div {
    match activity {
        Activity::Incomplete => div(),
        Activity::Complete => div().child(Icon::new(IconName::Check).text_color(Hsla::green())),
        Activity::Active(i) => h_flex()
            .gap_1()
            .child(i.to_string())
            .child(Icon::new(IconName::Play).text_color(Hsla::green())),
        Activity::Error(i) => h_flex()
            .gap_1()
            .child(i.to_string())
            .child(Icon::new(IconName::Close).text_color(Hsla::red())),
    }
}
