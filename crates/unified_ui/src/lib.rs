#![forbid(unsafe_code)]

pub mod prelude {
    pub use motion_core::{Ease, PresenceState, Spring, SpringStep, Transition};
    pub use ui_core::{
        A11yContract, ComponentContract, ComponentId, ComponentRole, FocusPolicy, TargetSize,
    };
    pub use ui_dioxus::{
        Button, ButtonVariant, Checkbox, CommandGroup, CommandItem, CommandMenu, Dialog,
        EmptyState, GlassSurface, MetricCard, MetricTone, Sidebar, SidebarItem, SidebarSection,
        Stack, Surface, Switch, TabItem, TabPanel, Tabs, TextField, Toast, ToastTone, Toolbar,
        Tooltip,
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
        "IconButton",
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
        "Surface",
        "GlassSurface",
        "Presence",
        "Transition",
        "Sequence",
        "SharedLayout",
        "SharedElement",
        "Timeline",
        "TimelineScope",
        "Composition",
        "FrameStage",
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
