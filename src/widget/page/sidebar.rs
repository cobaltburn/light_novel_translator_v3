use crate::{
    page::{Activity, Page},
    widget::{SidebarAction, numbered_items, sidebar_item},
};
use gpui_kit::{
    Action, Context, Div, Hsla, ParentElement as _, Styled as _,
    assets::IconName,
    base::h_flex,
    component::{Icon, sidebar::SidebarMenuItem},
    div,
};

/// A page's sidebar actions, which can also run each of its sections.
pub trait PageSidebarAction: SidebarAction {
    fn section_action(page: usize, part: usize) -> Box<dyn Action>;
}

impl Page {
    /// The page's item in `view`'s sidebar, whose menu can also run each section.
    /// The save and run items are disabled while `running`.
    pub fn sidebar_menu_item<T: PageSidebarAction>(
        &self,
        page: usize,
        active: bool,
        running: bool,
        view: &T,
        cx: &Context<T>,
    ) -> SidebarMenuItem {
        let activity = self.activity;
        let sections = self.sections.len();
        let name = self.file_stem().unwrap_or_default();
        sidebar_item(page, &name, active, running, view, cx, move |menu| {
            numbered_items(
                menu.separator(),
                sections,
                "translate part",
                running,
                |part| T::section_action(page, part),
            )
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
