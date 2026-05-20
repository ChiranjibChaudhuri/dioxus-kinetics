#![forbid(unsafe_code)]

pub mod prelude {
    pub use ui_core::{
        A11yContract, ComponentContract, ComponentId, ComponentRole, FocusPolicy, TargetSize,
    };
    pub use ui_dioxus::{Button, ButtonVariant, GlassSurface, Stack, Surface};
    pub use ui_glass::{
        resolve_glass, GlassDensity, GlassLevel, GlassPolicy, GlassRecipe, GlassRequest, GlassTone,
    };
    pub use ui_layout::{compute_flip, FlipDelta, Rect};
    pub use ui_motion::{Ease, PresenceState, Spring, SpringStep, Transition};
    pub use ui_tokens::{
        Color, Density, MotionPreference, MotionScale, RadiusScale, SemanticColors, SpacingScale,
        Theme, ThemeMode, TransparencyPreference,
    };

    #[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
    pub use ui_dom::{glass_style, CssStyleWriter};

    #[cfg(feature = "native")]
    pub use ui_native::{plan_native_glass, NativeCapabilities, NativeGlassPlan};
}

pub fn public_api_names() -> &'static [&'static str] {
    &[
        "Button",
        "IconButton",
        "TextField",
        "Checkbox",
        "Tabs",
        "Dialog",
        "Toast",
        "Surface",
        "GlassSurface",
        "Presence",
        "Transition",
        "Sequence",
        "SharedLayout",
        "SharedElement",
    ]
}

#[cfg(feature = "gsap")]
pub mod gsap {
    pub use ui_gsap::{GsapBackend, GsapCapability};
}

#[cfg(feature = "hyperframes-export")]
pub mod hyperframes {
    pub use ui_hyperframes::{Composition, RenderTrack};
}
