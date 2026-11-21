use crate::{app::Translator, error::Result};
use gpui_kit::{
    AppContext, WindowOptions, application,
    assets::Assets,
    component::{Theme, ThemeRegistry},
    init, open_window,
};
use std::sync::Mutex;
use std::{fs::File, path::PathBuf};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _, util::SubscriberInitExt as _};

mod app;
mod client;
mod dialogs;
mod epub;
mod error;
mod page;
mod screen;
mod server;
mod text;
mod widget;

const LOG_FILE: &str = "light_novel_translator.log";

fn main() -> Result<()> {
    let log_file = File::options().create(true).append(true).open(LOG_FILE)?;

    tracing_subscriber::registry()
        .with(EnvFilter::new(
            "light_novel_translator=debug,reqwest_retry=warn,error",
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(Mutex::new(log_file)),
        )
        .init();

    application().with_assets(Assets).run(|cx| {
        init(cx);
        let _ = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, |cx| {
            let themes = ThemeRegistry::global(cx).themes();
            if let Some(theme) = themes.get("Tokyo Night").cloned() {
                Theme::update(cx, |current| current.apply_config(&theme))
            }
        });

        open_window(WindowOptions::default(), cx, |window, cx| {
            cx.new(|cx| Translator::new(window, cx))
        })
        .expect("Failed to open window");
    });
    Ok(())
}
