use ui_dom::{glass_style, CssStyleWriter};
use ui_glass::{resolve_glass, GlassDensity, GlassLevel, GlassRequest, GlassTone};
use ui_tokens::Theme;

#[test]
fn style_writer_serializes_declarations() {
    let style = CssStyleWriter::new()
        .set("color", "red")
        .set("min-height", "44px")
        .to_inline_style();

    assert_eq!(style, "color:red;min-height:44px;");
}

#[test]
fn glass_style_uses_backdrop_filter_when_supported() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable),
    );
    let style = glass_style(&recipe, true);

    assert!(style.contains("backdrop-filter:blur(18px) saturate(160%);"));
    assert!(style.contains("background:rgba(255, 255, 255, 0.720);"));
}

#[test]
fn glass_style_uses_solid_background_without_backdrop_support() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(GlassLevel::Floating, GlassTone::Neutral, GlassDensity::Comfortable),
    );
    let style = glass_style(&recipe, false);

    assert!(!style.contains("backdrop-filter"));
    assert!(style.contains("background:rgba(255, 255, 255, 1.000);"));
}
