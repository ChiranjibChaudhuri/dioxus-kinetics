use ui_styles::{base_css, library_css, COMPONENT_CSS};

#[test]
fn component_css_covers_sequence_wrapper() {
    assert!(COMPONENT_CSS.contains(".ui-sequence"));
}

#[test]
fn base_css_exposes_theme_density_and_preference_hooks() {
    let css = base_css();

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
fn component_css_covers_icon_button_and_presence() {
    let css = COMPONENT_CSS;
    for selector in [
        ".ui-icon-button",
        ".ui-icon-button--neutral",
        ".ui-icon-button--primary",
        ".ui-icon-button--danger",
        ".ui-icon-button--compact",
        ".ui-icon-button--default",
        ".ui-icon-button--spacious",
        ".ui-icon-button-glyph",
        ".ui-presence",
        "[data-presence-cue=\"fade\"]",
        "[data-presence-cue=\"rise\"]",
        "[data-presence-cue=\"slide\"]",
        "[data-presence-cue=\"scale\"]",
    ] {
        assert!(css.contains(selector), "missing selector {selector}");
    }
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

#[test]
fn component_css_covers_shared_layout_and_shared_element() {
    assert!(COMPONENT_CSS.contains(".ui-shared-layout"));
    assert!(COMPONENT_CSS.contains(".ui-shared-element"));
}

#[test]
fn light_root_declares_four_elevation_variables() {
    let css = library_css();
    assert!(css.contains("--ui-elevation-0:"));
    assert!(css.contains("--ui-elevation-1:"));
    assert!(css.contains("--ui-elevation-2:"));
    assert!(css.contains("--ui-elevation-3:"));
}

#[test]
fn dark_theme_re_declares_elevation_variables() {
    let css = library_css();
    let dark_idx = css
        .find("[data-ui-theme=\"dark\"]")
        .expect("dark theme block exists");
    let dark_block = &css[dark_idx..];
    assert!(dark_block.contains("--ui-elevation-0:"));
    assert!(dark_block.contains("--ui-elevation-3:"));
}

#[test]
fn surface_uses_elevation_0() {
    let css = library_css();
    let block = component_block(&css, ".ui-surface,");
    assert!(
        block.contains("box-shadow: var(--ui-elevation-0)"),
        "surface block missing elevation-0: {block}"
    );
}

#[test]
fn metric_card_uses_elevation_1() {
    let css = library_css();
    let block = component_block(&css, ".ui-metric-card,");
    assert!(
        block.contains("box-shadow: var(--ui-elevation-1)")
            || css.contains(".ui-metric-card {")
                && css[css.find(".ui-metric-card {").unwrap()..]
                    .contains("box-shadow: var(--ui-elevation-1)")
    );
}

#[test]
fn tooltip_uses_elevation_1() {
    let css = library_css();
    assert!(
        css.contains(".ui-tooltip-content")
            && css[css.find(".ui-tooltip-content").unwrap()..]
                .contains("box-shadow: var(--ui-elevation-1)")
    );
}

#[test]
fn toast_uses_elevation_2() {
    let css = library_css();
    let idx = css.find(".ui-toast {").expect(".ui-toast rule exists");
    assert!(css[idx..]
        .split('}')
        .next()
        .unwrap()
        .contains("box-shadow: var(--ui-elevation-2)"));
}

#[test]
fn command_menu_panel_uses_elevation_2() {
    let css = library_css();
    let idx = css
        .find(".ui-command-menu-panel")
        .expect(".ui-command-menu-panel rule exists");
    assert!(css[idx..].contains("box-shadow: var(--ui-elevation-2)"));
}

#[test]
fn dialog_panel_uses_elevation_3() {
    let css = library_css();
    let idx = css
        .find(".ui-dialog-panel")
        .expect(".ui-dialog-panel rule exists");
    assert!(css[idx..].contains("box-shadow: var(--ui-elevation-3)"));
}

fn component_block<'a>(css: &'a str, selector_prefix: &str) -> &'a str {
    let idx = css.find(selector_prefix).expect("selector exists");
    let rest = &css[idx..];
    let end = rest.find('}').unwrap_or(rest.len());
    &rest[..end]
}

#[test]
fn reduced_motion_ancestor_scope_disables_transitions_globally() {
    let css = library_css();
    assert!(
        css.contains(r#"[data-ui-motion="reduced"]"#),
        "expected motion-policy ancestor scope"
    );
    // The scope must neutralize transitions on at least the kinetic + button + switch + menu classes.
    let block_start = css.find(r#"[data-ui-motion="reduced"]"#).unwrap();
    let block = &css[block_start..];
    for selector in [
        ".ui-button",
        ".ui-kinetic-box",
        ".ui-switch-thumb",
        ".ui-icon-button",
    ] {
        assert!(
            block.contains(selector),
            "motion-reduced scope should target {selector}"
        );
    }
}

#[test]
fn solid_glass_ancestor_scope_targets_every_backdrop_filter_class() {
    let css = library_css();
    assert!(css.contains(r#"[data-ui-glass-policy="solid"]"#));

    // Enumerate every class that introduces backdrop-filter and ensure the ancestor scope covers it.
    for class in [
        ".ui-glass-surface",
        ".ui-glass-layer",
        ".ui-dialog-panel",
        ".ui-command-menu-panel",
    ] {
        let pattern = format!(r#"[data-ui-glass-policy="solid"] {class}"#);
        assert!(
            css.contains(&pattern),
            "missing solid-glass override for {class}: pattern {pattern}",
        );
    }
}

#[test]
fn solid_glass_ancestor_scope_neutralizes_backdrop_filter() {
    let css = library_css();
    let idx = css.find(r#"[data-ui-glass-policy="solid"]"#).unwrap();
    let block = &css[idx..];
    assert!(block.contains("backdrop-filter: none"));
    assert!(block.contains("background: var(--ui-glass-solid)"));
}
