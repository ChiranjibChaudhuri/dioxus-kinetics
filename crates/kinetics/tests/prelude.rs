use kinetics::prelude::*;

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
    let public_names = kinetics::public_api_names();

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
    let names = kinetics::public_api_names();

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
    let names = kinetics::public_api_names();

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
    let names = kinetics::public_api_names();

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

#[test]
fn prelude_reexports_functional_component_aliases() {
    let _ = ActionControl;
    let _ = TextEntry;
    let _ = ChoiceMark;
    let _ = StateSwitch;
    let _ = ViewSwitcher;
    let _ = ActionBar;
    let _ = NavigationRail;
    let _ = MetricReadout;
    let _ = BlankState;
    let _ = ModalLayer;
    let _ = NoticeStack;
    let _ = CommandFinder;
    let _ = ContextHint;
    let _ = ContentPlane;
    let _ = GlassLayer;
}

#[test]
fn prelude_and_public_names_cover_kinetic_system_components() {
    let _ = TimelineScope;
    let _ = KineticBox;
    let _ = KineticText;
    let _ = PresenceGate;
    let _ = FrameStage;
    let _ = FrameClip;
    let _ = FrameLayer;
    let _ = CaptureStage;

    let names = kinetics::public_api_names();
    for expected in [
        "TimelineScope",
        "KineticBox",
        "KineticText",
        "PresenceGate",
        "FrameStage",
        "FrameClip",
        "FrameLayer",
        "CaptureStage",
    ] {
        assert!(
            names.contains(&expected),
            "missing kinetic system name {expected}"
        );
    }
}

#[test]
fn public_api_includes_runtime_and_icons() {
    let names = kinetics::public_api_names();
    for expected in [
        "IconButton",
        "IconButtonTone",
        "IconButtonSize",
        "Presence",
        "PresenceCue",
        "Close",
        "Check",
        "ChevronDown",
        "ChevronRight",
        "Plus",
        "Minus",
        "Trash",
        "Search",
    ] {
        assert!(
            names.contains(&expected),
            "missing public API name {expected}",
        );
    }
}

#[test]
fn public_api_includes_sequence_runtime_names() {
    let names = kinetics::public_api_names();
    for expected in [
        "Sequence",
        "Cue",
        "SequenceContext",
        "Axis",
        "use_timeline_sample",
        "ResolvedMotionState",
    ] {
        assert!(names.contains(&expected), "missing {expected}");
    }
}

#[test]
fn public_api_includes_shared_layout_and_shared_element() {
    let names = kinetics::public_api_names();
    for expected in [
        "SharedLayout",
        "SharedElement",
        "SharedTransition",
        "SharedElementRegistry",
        "ElementSnapshot",
        "use_shared_element_registry",
        "use_element_rect",
        "use_element_computed_style",
    ] {
        assert!(names.contains(&expected), "missing {expected}");
    }
}
