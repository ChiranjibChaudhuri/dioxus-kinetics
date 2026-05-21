use ui_styles::{library_css, BASE_CSS, COMPONENT_CSS};

#[test]
fn base_css_exposes_theme_density_and_preference_hooks() {
    let css = BASE_CSS;

    assert!(css.contains(":root"));
    assert!(css.contains("[data-ui-theme=\"dark\"]"));
    assert!(css.contains("[data-ui-density=\"compact\"]"));
    assert!(css.contains("[data-ui-density=\"spacious\"]"));
    assert!(css.contains("[data-ui-transparency=\"reduced\"]"));
    assert!(css.contains("@media (prefers-reduced-motion: reduce)"));
}

#[test]
fn component_css_covers_advanced_component_classes() {
    let css = COMPONENT_CSS;

    for selector in [
        ".ui-text-field",
        ".ui-checkbox",
        ".ui-switch",
        ".ui-tabs",
        ".ui-dialog",
        ".ui-toast",
        ".ui-command-menu",
        ".ui-tooltip",
        ".ui-toolbar",
        ".ui-sidebar",
        ".ui-metric-card",
        ".ui-empty-state",
        ".ui-glass-surface",
        ".ui-button:disabled",
        ".ui-field--invalid",
    ] {
        assert!(css.contains(selector), "missing selector {selector}");
    }
}

#[test]
fn library_css_concatenates_base_and_component_css() {
    let css = library_css();

    assert!(css.contains(":root"));
    assert!(css.contains(".ui-button"));
    assert!(css.contains(".ui-dialog"));
}

#[test]
fn component_css_covers_native_kinetics_systems() {
    let css = COMPONENT_CSS;

    for selector in [
        ".ui-glass-layer",
        ".ui-timeline-scope",
        ".ui-kinetic-box",
        ".ui-kinetic-text",
        ".ui-presence-gate",
        ".ui-frame-stage",
        ".ui-frame-clip",
        ".ui-frame-layer",
        ".ui-capture-stage",
    ] {
        assert!(css.contains(selector), "missing selector {selector}");
    }
}
