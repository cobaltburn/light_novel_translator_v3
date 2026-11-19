use crate::{
    page::{Activity, ErrorType, Page, PageError},
    screen::{DisplayType, SavePage, Translate, Translation},
};
use gpui_kit::{
    App, ClickEvent, Context, Div, FocusHandle, Hsla, InteractiveElement as _, IntoElement,
    ParentElement as _, Styled, Window,
    assets::IconName,
    base::{h_flex, v_flex},
    component::{
        ActiveTheme, Icon,
        button::{Button, ButtonVariants as _},
        sidebar::SidebarMenuItem,
    },
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

    pub fn text(
        &self,
        display: DisplayType,
        page: usize,
        cx: &Context<Translation>,
    ) -> impl Iterator<Item = impl IntoElement> {
        let name = self
            .path
            .file_name()
            .map_or_default(|e| e.to_string_lossy());

        let on_click = move |part| {
            cx.listener(move |this, _, window, cx| {
                this.translate_action(&Translate::Part { page, part }, window, cx)
            })
        };

        self.sections
            .iter()
            .enumerate()
            .map(move |(i, s)| match display {
                DisplayType::Full => s.content_div(i, &name, on_click(i)),
                DisplayType::End => s.end_lines_div(i, &name, on_click(i)),
                DisplayType::Japanese => s.japanese_div(i, &name, on_click(i)),
            })
            .flatten()
    }

    pub fn error_cards(&self, cx: &Context<Translation>) -> impl IntoElement {
        let make_empty_button = |id: usize| {
            Button::new(format!("empty-button-{}", id))
                .primary()
                .label(format!("Empty Section: {:2}", id + 1))
        };
        let error_buttons = self
            .errors
            .iter()
            .enumerate()
            .map(|(id, e)| e.error_button(id));

        let empty_buttons = self
            .sections
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.content.is_empty().then_some(i))
            .map(make_empty_button);

        v_flex()
            .absolute()
            .top_2()
            .right_4()
            .gap_1()
            .p_1()
            .rounded_md()
            .bg(cx.theme().popover)
            .occlude()
            .children(empty_buttons.chain(error_buttons))
    }
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

impl PageError {
    pub fn error_button(&self, id: usize) -> Button {
        let position = self.index + 1;
        let label = match self.error {
            ErrorType::Japanese => format!("Japanese error: {:2}", position),
            ErrorType::Size(c) => format!("Size error: {:2}, count:{}", position, c),
            ErrorType::Repeat => format!("Repeat error: {:2}", position),
            ErrorType::Copy => format!("Copy error: {:2}", position),
            ErrorType::Image => format!("Image Tag error: {:2}", position),
            ErrorType::Header => format!("Header Tag error: {:2}", position),
            ErrorType::Link => format!("Link Tag error: {:2}", position),
            ErrorType::End => format!("End Punctuation error: {:2}", position),
        };

        Button::new(format!("error-button-{}", id))
            .primary()
            .label(label)
    }
}
