use ui_tokens::{Color, Density, Theme, ThemeMode, TransparencyPreference};

#[test]
fn default_theme_has_apple_like_surface_bias() {
    let theme = Theme::default();

    assert_eq!(theme.mode, ThemeMode::Light);
    assert_eq!(theme.density, Density::Comfortable);
    assert_eq!(theme.radius.medium_px, 10.0);
    assert_eq!(theme.transparency, TransparencyPreference::Allow);
    assert_eq!(
        theme.semantic.background.css_rgba(),
        "rgba(246, 247, 249, 1.000)"
    );
}

#[test]
fn colors_are_css_serializable() {
    let color = Color::rgba(12, 34, 56, 0.375);

    assert_eq!(color.css_rgba(), "rgba(12, 34, 56, 0.375)");
}
