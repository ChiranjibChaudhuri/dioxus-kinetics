#![forbid(unsafe_code)]

pub mod prelude {
    pub use motion_core::{Ease, PresenceState, Spring, SpringStep, Transition};
    pub use ui_core::{
        A11yContract, ComponentContract, ComponentId, ComponentRole, FocusPolicy, TargetSize,
    };
    #[allow(deprecated)]
    pub use ui_dioxus::{
        Accordion, AccordionSection, ActionBar, ActionControl, ActionMenu, Alert, AlertTone,
        BlankState, Breadcrumb, BreadcrumbItem, Button, ButtonVariant, CaptureStage, Checkbox,
        ChoiceMark, Clip, Combobox, ComboboxOption, CommandFinder, CommandGroup, CommandItem,
        CommandMenu, ContentPlane, ContextHint, Cue, DataTable, DataTableColumn, DataTableRow,
        DatePicker, Dialog, DialogAction, DialogActionTone, DropdownMenu, DropdownMenuItem,
        EmptyState, FrameClip, FrameLayer, FrameStage, GlassLayer, GlassSurface, IconButton,
        IconButtonSize, IconButtonTone, KineticBox, KineticText, MetricCard, MetricReadout,
        MetricTone, ModalLayer, NavigationRail, NoticeStack, OptionGroup, Pagination, Popover,
        PopoverSide, Presence, PresenceCue, PresenceGate, Progress, RadioGroup, RadioOption, Scene,
        SceneContext, SegmentItem, SegmentedControl, Select, SelectOption, Sequence,
        SequenceContext, SharedElement, SharedLayout, Sidebar, SidebarItem, SidebarSection,
        Skeleton, Slider, SortDirection, Stack, StateSwitch, Stepper, StepperStep, Surface, Switch,
        TabItem, TabPanel, Tabs, TextEntry, TextField, TimelineScope, Toast, ToastTone, Toolbar,
        Tooltip, ViewSwitcher,
    };
    pub use ui_glass::{
        resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRecipe, GlassRequest, GlassTone,
    };
    pub use ui_layout::{compute_flip, FlipDelta, Rect};
    pub use ui_styles::{base_css, library_css, COMPONENT_CSS};
    #[cfg(feature = "timeline")]
    pub use ui_timeline::{Axis, MotionCue, ResolvedMotionState, TimelineClock};
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
    pub use ui_composition::Composition;

    #[cfg(feature = "capture")]
    pub use ui_capture::CaptureStageDescriptor;

    #[cfg(feature = "runtime")]
    pub use ui_runtime::{
        use_animation_value, use_element_computed_style, use_element_rect, use_presence_animation,
        use_presence_state, use_reduced_motion, use_shared_element_registry, use_timeline_sample,
        CssKeyframesAdapter, ElementSnapshot, FrameAdapter, FrameAdapterHandle,
        FrameAdapterRegistry, MountedRectCallback, ReducedMotion, SceneClock, SceneState,
        SequenceAdapter, SharedElementRegistry, SharedTransition, WaapiAdapter,
    };

    #[cfg(feature = "icons")]
    pub use ui_icons::*;
}

pub fn public_api_names() -> Vec<&'static str> {
    let mut names = vec![
        "Button",
        "ActionControl",
        "IconButton",
        "IconButtonTone",
        "IconButtonSize",
        "TextField",
        "TextEntry",
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
        "FrameAdapter",
        "FrameAdapterRegistry",
        "SequenceAdapter",
        "WaapiAdapter",
        "CssKeyframesAdapter",
    ]);

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
