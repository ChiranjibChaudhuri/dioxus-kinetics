#![forbid(unsafe_code)]

//! Dioxus Kinetics — the single intended downstream facade for the workspace.
//!
//! Downstream apps should depend on this crate alone and pull everything in
//! through the prelude:
//!
//! ```ignore
//! use kinetics::prelude::*;
//! ```
//!
//! The prelude re-exports the stable, semantically named public surface of the
//! underlying crates (components, tokens, motion primitives, and feature-gated
//! runtimes) so callers never reach into the individual `ui-*` crates directly.
//!
//! See `docs/ai-cheatsheet.md` for ready-to-paste prelude examples and
//! `docs/component-naming.md` for the naming conventions behind these exports.

pub mod prelude {
    pub use motion_core::{Ease, PresenceState, Spring, SpringStep, Transition};
    pub use ui_core::{
        A11yContract, ComponentContract, ComponentId, ComponentRole, FocusPolicy, TargetSize,
    };
    #[allow(deprecated)]
    pub use ui_dioxus::{
        apply_kanban_move, Accordion, AccordionSection, ActionBar, ActionControl, ActionMenu,
        AgentStep, AgentStepState, AgentTimeline, AiStatus, AiStatusState, Alert, AlertTone,
        AssistantPanel, AssistantSide, AudioLevels, Avatar, AvatarSize, Badge, BadgeTone, BarChart,
        BlankState, Breadcrumb, BreadcrumbItem, Button, ButtonVariant, CaptureStage, ChartSeries,
        ChartTone, Checkbox, ChoiceMark, CitationChip, Clip, Combobox, ComboboxOption,
        CommandFinder, CommandGroup, CommandItem, CommandMenu, ComparisonChart, ContentPlane,
        ContextHint, Cue, DataTable, DataTableColumn, DataTableRow, DatePicker, Dialog,
        DialogAction, DialogActionTone, DonutGauge, DropdownMenu, DropdownMenuItem, EmptyState,
        FrameClip, FrameLayer, FrameStage, GlassLayer, GlassSurface, GuidedTour, Heading,
        IconButton, IconButtonSize, IconButtonTone, KanbanBoard, KanbanColumn, KanbanMove,
        KineticBox, KineticText, LineChart, MetricCard, MetricReadout, MetricTone, ModalLayer,
        MotionPath, NavigationRail, NoticeStack, OptionGroup, Pagination, Popover, PopoverSide,
        Presence, PresenceCue, PresenceGate, Progress, ProgressDial, PromptInput, RadioGroup,
        RadioOption, ReorderList, Scene, SceneContext, SegmentItem, SegmentedControl, Select,
        SelectOption, Sequence, SequenceContext, SharedElement, SharedLayout, Sheet, SheetSide,
        Sidebar, SidebarItem, SidebarSection, Skeleton, Slider, SortDirection, SortableItem,
        SortableList, SourceCard, SourceRail, Sparkline, Spinner, SplitMode, SplitText, Spotlight,
        Stack, StateSwitch, Stepper, StepperStep, StreamingText, Surface, Switch, TabItem,
        TabPanel, Tabs, Text, TextEntry, TextField, TextFieldType, TextVariant, TimelineScope,
        Toast, ToastEntry, ToastTone, Toaster, Toolbar, Tooltip, Tour, TourPlacement, TourStep,
        TrendChart, TrendLine, ViewSwitcher, VoiceInput, VoiceInputState, Waveform, WorkflowBoard,
    };
    pub use ui_glass::{
        resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRecipe, GlassRequest, GlassTone,
    };
    pub use ui_layout::{compute_flip, FlipDelta, Rect};
    pub use ui_styles::{base_css, library_css, COMPONENT_CSS};
    #[cfg(feature = "timeline")]
    pub use ui_timeline::{
        Axis, MotionCue, PathPoint, ResolvedMotionState, Timeline, TimelineClock,
    };
    pub use ui_tokens::{
        Color, Density, MotionPreference, MotionScale, RadiusScale, SemanticColors, SpacingScale,
        Theme, ThemeMode, TransparencyPreference,
    };

    #[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
    pub use ui_dom::{glass_style, CssStyleWriter};

    #[cfg(feature = "native")]
    pub use ui_native::{plan_native_glass, NativeCapabilities, NativeGlassPlan};

    #[cfg(feature = "timeline")]
    pub use ui_timeline::{TimelineCapability, TimelineRuntime};

    #[cfg(feature = "composition")]
    pub use ui_composition::{ClipFill, Composition};

    #[cfg(feature = "capture")]
    pub use ui_capture::CaptureStageDescriptor;

    #[cfg(feature = "liquid-glass")]
    pub use ui_dioxus::{GlassPower, LiquidSurface, LiquidSurfaceProps};

    #[cfg(feature = "runtime")]
    pub use ui_runtime::{
        use_animation_value, use_density, use_element_computed_style, use_element_rect,
        use_presence_animation, use_presence_state, use_reduced_motion,
        use_shared_element_registry, use_theme_mode, use_timeline_sample, CssKeyframesAdapter,
        ElementSnapshot, FrameAdapter, FrameAdapterHandle, FrameAdapterRegistry,
        MountedRectCallback, ReducedMotion, SceneClock, SceneDriver, SceneState,
        ScrollObserverConfig, SequenceAdapter, SharedElementRegistry, SharedTransition,
        ThemeProvider, WaapiAdapter,
    };

    #[cfg(feature = "icons")]
    pub use ui_icons::*;

    #[cfg(feature = "blocks")]
    pub use ui_blocks::{
        Caption, LowerThird, LowerThirdAccent, MetricCounter, SocialOverlay, SocialPlatform,
        WipeTransition,
    };
}

/// The introspectable list of stable public names exposed through
/// [`prelude`]. This list is pinned by `tests/prelude.rs` so that renames or
/// accidental removals of the facade's public surface are caught at test time.
pub fn public_api_names() -> Vec<&'static str> {
    let mut names = vec![
        "Button",
        "ActionControl",
        "IconButton",
        "IconButtonTone",
        "IconButtonSize",
        "TextField",
        "TextEntry",
        "TextFieldType",
        "Checkbox",
        "ChoiceMark",
        "Switch",
        "StateSwitch",
        "Tabs",
        "ViewSwitcher",
        "Dialog",
        "ModalLayer",
        "Toast",
        "NoticeStack",
        "CommandMenu",
        "CommandFinder",
        "Tooltip",
        "ContextHint",
        "Toolbar",
        "ActionBar",
        "Sidebar",
        "NavigationRail",
        "MetricCard",
        "MetricReadout",
        "EmptyState",
        "BlankState",
        "Surface",
        "ContentPlane",
        "GlassSurface",
        "GlassLayer",
        "Presence",
        "PresenceCue",
        "Transition",
        "Sequence",
        "Cue",
        "SequenceContext",
        "Axis",
        "ResolvedMotionState",
        "SharedLayout",
        "SharedElement",
        "Timeline",
        "TimelineScope",
        "KineticBox",
        "KineticText",
        "PresenceGate",
        "Composition",
        "FrameStage",
        "FrameClip",
        "FrameLayer",
        "CaptureStage",
        "Combobox",
        "RadioGroup",
        "DropdownMenu",
        "OptionGroup",
        "ActionMenu",
        "Scene",
        "Clip",
        "SceneContext",
        "MotionPath",
        "PathPoint",
        "SplitText",
        "SplitMode",
        "StreamingText",
        "AiStatus",
        "CitationChip",
        "SourceCard",
        "SourceRail",
        "PromptInput",
        "AssistantPanel",
        "AgentTimeline",
        "Heading",
        "Text",
        "Badge",
        "Avatar",
        "Spinner",
        "Sheet",
        "Toaster",
        "ToastEntry",
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
        "SortableList",
        "ReorderList",
        "SortableItem",
        "KanbanBoard",
        "WorkflowBoard",
        "KanbanColumn",
        "KanbanMove",
        "apply_kanban_move",
        "Tour",
        "GuidedTour",
        "TourStep",
        "TourPlacement",
        "Spotlight",
        "Waveform",
        "AudioLevels",
        "VoiceInput",
        "VoiceInputState",
    ];

    #[cfg(feature = "icons")]
    names.extend_from_slice(&[
        "Close",
        "Check",
        "ChevronDown",
        "ChevronRight",
        "Plus",
        "Minus",
        "Trash",
        "Search",
        "Sparkle",
        "Stop",
        "Send",
        "Quote",
        "Globe",
        "Copy",
        "Link",
    ]);

    #[cfg(feature = "runtime")]
    names.extend_from_slice(&[
        "use_timeline_sample",
        "SharedTransition",
        "SharedElementRegistry",
        "ElementSnapshot",
        "use_shared_element_registry",
        "use_element_rect",
        "use_element_computed_style",
        "SceneState",
        "SceneClock",
        "SceneDriver",
        "ScrollObserverConfig",
        "FrameAdapter",
        "FrameAdapterRegistry",
        "SequenceAdapter",
        "WaapiAdapter",
        "CssKeyframesAdapter",
        "ThemeProvider",
        "use_theme_mode",
        "use_density",
    ]);

    #[cfg(feature = "blocks")]
    names.extend_from_slice(&[
        "LowerThird",
        "LowerThirdAccent",
        "Caption",
        "WipeTransition",
        "MetricCounter",
        "SocialOverlay",
        "SocialPlatform",
    ]);

    #[cfg(feature = "composition")]
    names.push("ClipFill");

    #[cfg(feature = "liquid-glass")]
    names.push("LiquidSurface");

    names
}

#[cfg(feature = "timeline")]
pub mod timeline {
    pub use ui_timeline::{TimelineCapability, TimelineRuntime};
}

#[cfg(feature = "composition")]
pub mod composition {
    pub use ui_composition::Composition;
}

#[cfg(feature = "capture")]
pub mod capture {
    pub use ui_capture::CaptureStageDescriptor;
}
