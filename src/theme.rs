//! The app's themes, built into the binary from the `themes` folder so they
//! load wherever the app is run from.

use gpui_kit::{
    App,
    component::{Theme, ThemeRegistry},
};

const DEFAULT_THEME: &str = "Tokyo Night";

/// A theme file's name and its contents, read from the `themes` folder at compile time.
macro_rules! theme_file {
    ($name:literal) => {
        (
            $name,
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/themes/", $name)),
        )
    };
}

/// Each file in the `themes` folder. Add a file here to make its themes available.
const THEME_FILES: [(&str, &str); 21] = [
    theme_file!("adventure.json"),
    theme_file!("alduin.json"),
    theme_file!("asciinema.json"),
    theme_file!("aurora.json"),
    theme_file!("ayu.json"),
    theme_file!("catppuccin.json"),
    theme_file!("everforest.json"),
    theme_file!("fahrenheit.json"),
    theme_file!("flexoki.json"),
    theme_file!("gruvbox.json"),
    theme_file!("harper.json"),
    theme_file!("hybrid.json"),
    theme_file!("jellybeans.json"),
    theme_file!("kibble.json"),
    theme_file!("macos-classic.json"),
    theme_file!("mellifluous.json"),
    theme_file!("molokai.json"),
    theme_file!("solarized.json"),
    theme_file!("spaceduck.json"),
    theme_file!("tokyonight.json"),
    theme_file!("twilight.json"),
];

/// Registers the built-in themes and applies the default one.
pub fn init_themes(cx: &mut App) {
    let registry = ThemeRegistry::global_mut(cx);
    for (name, content) in THEME_FILES {
        if let Err(error) = registry.load_themes_from_str(content) {
            tracing::error!("Failed to load theme file {name}: {error}");
        }
    }

    if let Some(theme) = ThemeRegistry::global(cx)
        .themes()
        .get(DEFAULT_THEME)
        .cloned()
    {
        Theme::update(cx, |current| current.apply_config(&theme));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::component::ThemeSet;
    use std::{collections::BTreeSet, fs};

    #[test]
    fn every_theme_file_parses_and_the_default_theme_is_among_them() {
        let names: Vec<_> = THEME_FILES
            .iter()
            .flat_map(|(file, content)| {
                serde_json::from_str::<ThemeSet>(content)
                    .unwrap_or_else(|error| panic!("{file}: {error}"))
                    .themes
            })
            .map(|theme| theme.name)
            .collect();
        assert!(names.iter().any(|name| name == DEFAULT_THEME));
    }

    #[test]
    fn every_file_in_the_themes_folder_is_built_in() {
        let folder = concat!(env!("CARGO_MANIFEST_DIR"), "/themes");
        let on_disk: BTreeSet<_> = fs::read_dir(folder)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".json"))
            .collect();
        let built_in: BTreeSet<_> = THEME_FILES
            .iter()
            .map(|(file, _)| file.to_string())
            .collect();
        assert_eq!(on_disk, built_in);
    }
}
