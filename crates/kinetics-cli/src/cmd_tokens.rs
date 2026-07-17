//! `kinetics tokens` — export the design-token system as CSS custom
//! properties so a downstream app can re-skin every kinetics surface by
//! injecting one stylesheet near its root.

use kinetics::prelude::{export_tokens_css, Theme, ThemeMode};

pub fn run(mode: &str) -> Result<(), String> {
    let theme = match mode.to_ascii_lowercase().as_str() {
        "light" => Theme::default(),
        "dark" => Theme::dark(),
        other => {
            return Err(format!(
                "unknown mode {other:?} (expected \"light\" or \"dark\")"
            ));
        }
    };
    let selector = match theme.mode {
        ThemeMode::Light => "data-ui-theme=\"light\"",
        ThemeMode::Dark => "data-ui-theme=\"dark\"",
    };
    eprintln!("kinetics: exporting token CSS ({selector})");
    print!("{}", export_tokens_css(&theme));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kinetics::prelude::Color;

    #[test]
    fn light_mode_export_contains_root_selector() {
        let css = export_tokens_css(&Theme::default());
        assert!(css.contains(":root, [data-ui-theme=\"light\"]"));
    }

    #[test]
    fn run_rejects_unknown_mode() {
        assert!(run("neon").is_err());
    }

    #[test]
    fn run_accepts_dark_case_insensitive() {
        // Dark export must target the dark selector.
        let css = export_tokens_css(&Theme::dark());
        assert!(css.contains("[data-ui-theme=\"dark\"]"));
        assert!(run("DARK").is_ok());
    }

    #[test]
    fn custom_theme_round_trips_into_export() {
        let mut theme = Theme::default();
        theme.semantic.primary = Color::rgba(10, 20, 30, 1.0);
        let css = export_tokens_css(&theme);
        assert!(css.contains("rgba(10, 20, 30, 1.000)"));
    }
}
