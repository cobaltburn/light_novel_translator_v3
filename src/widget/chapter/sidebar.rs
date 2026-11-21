use crate::{
    page::{Activity, Chapter},
    widget::{SidebarAction, sidebar_item},
};
use gpui_kit::{
    Context, Div, Hsla, ParentElement as _, Styled as _,
    assets::IconName,
    component::{Icon, sidebar::SidebarMenuItem},
    div,
};

impl Chapter {
    /// The chapter's item in `view`'s sidebar. The save and run items are disabled while `running`.
    pub fn sidebar_menu_item<T: SidebarAction>(
        &self,
        chapter: usize,
        active: bool,
        running: bool,
        view: &T,
        cx: &Context<T>,
    ) -> SidebarMenuItem {
        let activity = self.activity;
        let name = self.file_stem().unwrap_or_default();
        sidebar_item(chapter, &name, active, running, view, cx, |menu| menu)
            .suffix(move |_, _| chapter_activity_icon(activity))
    }
}

/// Whether a chapter's glossary pass is running or done, shown beside its name.
fn chapter_activity_icon(activity: Activity) -> Div {
    match activity {
        Activity::Complete => div().child(Icon::new(IconName::Check).text_color(Hsla::green())),
        Activity::Active(_) => div().child(Icon::new(IconName::Play).text_color(Hsla::green())),
        Activity::Incomplete | Activity::Error(_) => div(),
    }
}
