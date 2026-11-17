use crate::app::{Screen, Translator};
use gpui_kit::{
    Context, IntoElement, ParentElement, Render, Styled, Window,
    base::{h_flex, v_flex},
    component::sidebar::{Sidebar, SidebarMenuItem},
    px,
};

impl Render for Translator {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let screens = [
            Screen::Translation,
            Screen::Glossary,
            Screen::Build,
            Screen::Doc,
        ];
        h_flex()
            .min_size_0()
            .size_full()
            .child(Sidebar::new("navigation-sidebar").w(px(150.0)).children(
                screens.into_iter().map(|screen| {
                    SidebarMenuItem::new(screen.to_string()).on_click(cx.listener(
                        move |this, _, _, cx| {
                            this.set_screen(screen, cx);
                        },
                    ))
                }),
            ))
            .child(match self.screen {
                Screen::Translation => self.translation_view(window, cx),
                Screen::Glossary => todo!(),
                Screen::Build => todo!(),
                Screen::Doc => todo!(),
            })
    }
}

impl Translator {
    pub fn translation_view(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .min_size_0()
            .child(self.tab_bar(cx))
            .child(self.active_tab().clone())
    }
}
