use unified_ui::prelude::*;

#[test]
fn prelude_exposes_semantic_components_and_tokens() {
    let theme = Theme::default();
    let recipe = resolve_glass(
        &theme,
        GlassRequest::new(
            GlassLevel::Floating,
            GlassTone::Neutral,
            GlassDensity::Comfortable,
        ),
    );

    assert_eq!(
        ButtonVariant::Primary.class_name(),
        "ui-button ui-button--primary"
    );
    assert_eq!(recipe.backdrop_blur_px, 18.0);
}

#[test]
fn default_features_do_not_expose_gsap_or_hyperframes_names() {
    let public_names = unified_ui::public_api_names();

    assert!(!public_names.iter().any(|name| name.contains("Gsap")));
    assert!(!public_names.iter().any(|name| name.contains("HyperFrames")));
}

#[test]
fn prelude_exposes_advanced_components_and_styles() {
    let css = library_css();

    assert!(css.contains(".ui-command-menu"));
    assert_eq!(
        MetricTone::Success.class_name(),
        "ui-metric-card ui-metric-card--success"
    );
    assert_eq!(ToastTone::Warning.role(), "alert");

    let tabs = [TabItem::new("one", "One")];
    let panels = [TabPanel::new("one", "Panel")];
    assert_eq!(tabs[0].label, "One");
    assert_eq!(panels[0].content, "Panel");

    let commands = [CommandGroup::new(
        "Navigation",
        vec![CommandItem::new("home", "Home", "Open dashboard")],
    )];
    assert_eq!(commands[0].items[0].id, "home");
}

#[test]
fn public_api_names_include_advanced_wave_names() {
    let names = unified_ui::public_api_names();

    for expected in [
        "TextField",
        "Checkbox",
        "Switch",
        "Tabs",
        "Dialog",
        "Toast",
        "CommandMenu",
        "Tooltip",
        "Toolbar",
        "Sidebar",
        "MetricCard",
        "EmptyState",
    ] {
        assert!(
            names.contains(&expected),
            "missing public API name {expected}"
        );
    }
}

#[test]
fn public_api_names_use_native_system_boundaries() {
    let names = unified_ui::public_api_names();

    for expected in ["Timeline", "Composition", "CaptureStage"] {
        assert!(
            names.contains(&expected),
            "missing native system name {expected}"
        );
    }

    for rejected in ["Gsap", "GSAP", "HyperFrames", "Remotion"] {
        assert!(
            !names.iter().any(|name| name.contains(rejected)),
            "public names must not expose bridge term {rejected}"
        );
    }
}

#[test]
fn prelude_exposes_functional_component_names() {
    let names = unified_ui::public_api_names();

    for expected in [
        "ActionControl",
        "TextEntry",
        "ChoiceMark",
        "StateSwitch",
        "ViewSwitcher",
        "ActionBar",
        "NavigationRail",
        "MetricReadout",
        "BlankState",
        "ModalLayer",
        "NoticeStack",
        "CommandFinder",
        "ContextHint",
        "ContentPlane",
        "GlassLayer",
    ] {
        assert!(
            names.contains(&expected),
            "missing functional name {expected}"
        );
    }
}
