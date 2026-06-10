#![allow(deprecated)]

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
fn prelude_reexports_cheatsheet_named_types() {
    let _ = TextFieldType::Email;

    #[cfg(feature = "composition")]
    let _ = ClipFill::None;

    #[cfg(feature = "liquid-glass")]
    let _ = LiquidSurface;

    let names = kinetics::public_api_names();
    assert!(
        names.contains(&"TextFieldType"),
        "missing public API name TextFieldType"
    );

    #[cfg(feature = "composition")]
    assert!(
        names.contains(&"ClipFill"),
        "missing public API name ClipFill"
    );

    #[cfg(feature = "liquid-glass")]
    assert!(
        names.contains(&"LiquidSurface"),
        "missing public API name LiquidSurface"
    );
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
        "ThemeProvider",
        "use_theme_mode",
        "use_density",
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

#[test]
fn prelude_and_public_names_cover_chart_kit() {
    let _ = Sparkline;
    let _ = TrendLine;
    let _ = LineChart;
    let _ = TrendChart;
    let _ = BarChart;
    let _ = ComparisonChart;
    let _ = DonutGauge;
    let _ = ProgressDial;
    let series = ChartSeries::new("Revenue", vec![1.0, 2.0]);
    assert_eq!(series.name, "Revenue");
    assert_eq!(ChartTone::default(), ChartTone::Primary);

    let names = kinetics::public_api_names();
    for expected in [
        "Sparkline",
        "TrendLine",
        "LineChart",
        "TrendChart",
        "BarChart",
        "ComparisonChart",
        "DonutGauge",
        "ProgressDial",
        "ChartSeries",
        "ChartTone",
    ] {
        assert!(names.contains(&expected), "missing chart name {expected}");
    }
}

#[test]
fn prelude_and_public_names_cover_sortable_surfaces() {
    let _ = SortableList;
    let _ = ReorderList;
    let _ = KanbanBoard;
    let _ = WorkflowBoard;

    let columns = vec![KanbanColumn::new(
        "todo",
        "To do",
        vec![SortableItem::new("a", "Task A")],
    )];
    let mv = KanbanMove {
        item_id: "a".into(),
        from_column: "todo".into(),
        to_column: "todo".into(),
        to_index: 0,
    };
    let next = apply_kanban_move(&columns, &mv);
    assert_eq!(next[0].items.len(), 1);

    let names = kinetics::public_api_names();
    for expected in [
        "SortableList",
        "ReorderList",
        "SortableItem",
        "KanbanBoard",
        "WorkflowBoard",
        "KanbanColumn",
        "KanbanMove",
        "apply_kanban_move",
    ] {
        assert!(
            names.contains(&expected),
            "missing sortable name {expected}"
        );
    }
}

#[test]
fn prelude_and_public_names_cover_tour_and_voice() {
    let _ = Tour;
    let _ = GuidedTour;
    let _ = Spotlight;
    let _ = Waveform;
    let _ = AudioLevels;
    let _ = VoiceInput;

    let step = TourStep::new("s1", "Welcome", "Body")
        .with_target("hero")
        .with_placement(TourPlacement::Top);
    assert_eq!(step.target_id, "hero");
    assert_eq!(VoiceInputState::Error.live_role(), "alert");

    let names = kinetics::public_api_names();
    for expected in [
        "Tour",
        "GuidedTour",
        "TourStep",
        "TourPlacement",
        "Spotlight",
        "Waveform",
        "AudioLevels",
        "VoiceInput",
        "VoiceInputState",
    ] {
        assert!(names.contains(&expected), "missing name {expected}");
    }
}
