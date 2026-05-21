#![forbid(unsafe_code)]

pub mod prelude {
    pub use motion_core::{Ease, PresenceState, Spring, SpringStep, Transition};
    pub use ui_core::{
        A11yContract, ComponentContract, ComponentId, ComponentRole, FocusPolicy, TargetSize,
    };
    pub use ui_dioxus::{
        ActionBar, ActionControl, BlankState, Button, ButtonVariant, CaptureStage, Checkbox,
        ChoiceMark, CommandFinder, CommandGroup, CommandItem, CommandMenu, ContentPlane,
        ContextHint, Dialog, EmptyState, FrameClip, FrameLayer, FrameStage, GlassLayer,
        GlassSurface, KineticBox, KineticText, MetricCard, MetricReadout, MetricTone, ModalLayer,
        NavigationRail, NoticeStack, PresenceGate, Sidebar, SidebarItem, SidebarSection, Stack,
        StateSwitch, Surface, Switch, TabItem, TabPanel, Tabs, TextEntry, TextField, TimelineScope,
        Toast, ToastTone, Toolbar, Tooltip, ViewSwitcher,
    };
    pub use ui_glass::{
        resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRecipe, GlassRequest, GlassTone,
    };
    pub use ui_layout::{compute_flip, FlipDelta, Rect};
    pub use ui_styles::{library_css, BASE_CSS, COMPONENT_CSS};
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
}

pub fn public_api_names() -> &'static [&'static str] {
    &[
        "Button",
        "ActionControl",
        "IconButton",
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
        "Transition",
        "Sequence",
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
    ]
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
