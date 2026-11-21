use crate::{app::Translator, error::Result};
use gpui_kit::{AppContext, WindowOptions, application, assets::Assets, init, open_window};
use std::fs::File;
use std::sync::Mutex;
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
mod theme;
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
        theme::init_themes(cx);

        open_window(WindowOptions::default(), cx, |window, cx| {
            cx.new(|cx| Translator::new(window, cx))
        })
        .expect("Failed to open window");
    });
    Ok(())
}
