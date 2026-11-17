use crate::{
    page::{Activity, Page},
    screen::{DisplayType, SavePage, Translate},
};
use gpui_kit::{
    App, ClickEvent, Div, FocusHandle, Hsla, IntoElement, ParentElement as _, Styled, Window,
    assets::IconName,
    base::h_flex,
    component::{Icon, sidebar::SidebarMenuItem},
    div,
};

impl Page {
    pub fn sidebar_menu_item(
        &self,
        page: usize,
        active: bool,
        on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
        focus_handle: FocusHandle,
    ) -> SidebarMenuItem {
        let file_stem = self.file_stem().unwrap_or_default();
        let activity = self.activity;
        let sections = self.sections.len();
        SidebarMenuItem::new(format!("{}. {}", page, file_stem))
            .active(active)
            .on_click(on_click)
            .context_menu(move |menu, _, _| {
                let menu = menu
                    .scrollable(true)
                    .max_h(300.0)
                    .min_w(200.0)
                    .action_context(focus_handle.clone())
                    .menu("save", Box::new(SavePage { page }))
                    .separator()
                    .menu("translate", Box::new(Translate::All(page)))
                    .menu("translate page", Box::new(Translate::Page(page)))
                    .separator();
                (0..sections).fold(menu, |menu, part| {
                    menu.menu(
                        format!("translate part {}", part + 1),
                        Box::new(Translate::Part { page, part }),
                    )
                })
            })
            .suffix(move |_, _| sidebar_icon(activity))
    }

    pub fn text(&self, display: DisplayType) -> impl Iterator<Item = impl IntoElement> {
        self.sections
            .iter()
            .enumerate()
            .map(move |(i, s)| match display {
                DisplayType::Full => s.content_div(i),
                DisplayType::End => s.end_lines_div(i),
                DisplayType::Japanese => s.japanese_div(i),
            })
            .flatten()
    }

    /* pub fn error_cards(&self) -> impl IntoElement {
        todo!()
    } */
}

fn sidebar_icon(activity: Activity) -> Div {
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
