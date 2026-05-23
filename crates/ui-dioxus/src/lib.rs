#![forbid(unsafe_code)]

mod buttons;
mod capture;
mod composition;
mod display;
mod forms;
mod kinetics;
mod layout;
mod navigation;
mod overlays;

use dioxus::prelude::*;
use ui_glass::{GlassDensity, GlassLevel, GlassTone};

pub use buttons::{IconButton, IconButtonSize, IconButtonTone};
pub use capture::CaptureStage;
pub use composition::{FrameClip, FrameLayer, FrameStage};
pub use display::{EmptyState as BlankState, MetricCard as MetricReadout};
pub use display::{EmptyState, MetricCard, MetricTone};
pub use forms::{Checkbox as ChoiceMark, Switch as StateSwitch, TextField as TextEntry};
pub use forms::{Checkbox, Switch, TextField, TextFieldType};
pub use kinetics::{
    Cue, KineticBox, KineticText, Presence, PresenceCue, PresenceGate, Sequence, SequenceContext,
    TimelineScope,
};
pub use layout::{SharedElement, SharedLayout};
pub use navigation::{Sidebar, SidebarItem, SidebarSection, TabItem, TabPanel, Tabs, Toolbar};
pub use navigation::{Sidebar as NavigationRail, Tabs as ViewSwitcher, Toolbar as ActionBar};
pub use overlays::{CommandGroup, CommandItem, CommandMenu, Dialog, Toast, ToastTone, Tooltip};
pub use overlays::{
    CommandMenu as CommandFinder, Dialog as ModalLayer, Toast as NoticeStack,
    Tooltip as ContextHint,
};
#[cfg(feature = "liquid-glass")]
pub use ui_glass_dioxus::{LiquidSurface, LiquidSurfaceProps};

pub use Button as ActionControl;
pub use GlassSurface as GlassLayer;
pub use Surface as ContentPlane;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

impl ButtonVariant {
    pub const fn class_name(self) -> &'static str {
        match self {
            Self::Primary => "ui-button ui-button--primary",
            Self::Secondary => "ui-button ui-button--secondary",
            Self::Ghost => "ui-button ui-button--ghost",
            Self::Danger => "ui-button ui-button--danger",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] disabled: bool,
    #[props(default)] loading: bool,
    #[props(default = "button".to_string())] button_type: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let class = if loading {
        format!("{} ui-button--loading", variant.class_name())
    } else {
        variant.class_name().to_string()
    };

    rsx! {
        button {
            class: "{class}",
            disabled: disabled || loading,
            r#type: "{button_type}",
            "aria-busy": if loading { "true" } else { "false" },
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn Surface(children: Element) -> Element {
    rsx! {
        section {
            class: "ui-surface",
            {children}
        }
    }
}

#[component]
pub fn GlassSurface(
    #[props(default)] level: GlassLevel,
    #[props(default)] tone: GlassTone,
    #[props(default)] density: GlassDensity,
    children: Element,
) -> Element {
    #[cfg(feature = "liquid-glass")]
    {
        use ui_glass::{
            GlassDepth, MaterialDensity, MaterialEdge, MaterialRequest, MaterialTone,
            MaterialVibrancy, LiquidMaterial,
        };
        use ui_glass_engine::capabilities::{detect, Tier};

        let tier = detect().best_tier();
        let material = LiquidMaterial::from(
            MaterialRequest::new(
                match level {
                    GlassLevel::Subtle => GlassDepth::Raised,
                    GlassLevel::Floating => GlassDepth::Floating,
                    GlassLevel::Overlay => GlassDepth::Overlay,
                    GlassLevel::Chrome => GlassDepth::Chrome,
                },
                match tone {
                    GlassTone::Neutral => MaterialTone::Neutral,
                    GlassTone::Primary => MaterialTone::Primary,
                    GlassTone::Success => MaterialTone::Success,
                    GlassTone::Warning => MaterialTone::Warning,
                    GlassTone::Danger => MaterialTone::Danger,
                    GlassTone::Info => MaterialTone::Info,
                },
            )
            .with_density(match density {
                GlassDensity::Compact => MaterialDensity::Compact,
                GlassDensity::Comfortable => MaterialDensity::Comfortable,
                GlassDensity::Spacious => MaterialDensity::Spacious,
            })
            .with_edge(MaterialEdge::Hairline)
            .with_vibrancy(MaterialVibrancy::Standard),
        );

        return match tier {
            Tier::WgpuWebGpu | Tier::WgpuWebGl2 => rsx! {
                ui_glass_dioxus::LiquidSurface {
                    material,
                    {children}
                }
            },
            Tier::SvgFilter | Tier::SolidCss | Tier::Off => {
                glass_surface_css(level, tone, density, children)
            }
        };
    }

    #[cfg(not(feature = "liquid-glass"))]
    {
        glass_surface_css(level, tone, density, children)
    }
}

fn glass_surface_css(
    level: GlassLevel,
    tone: GlassTone,
    density: GlassDensity,
    children: Element,
) -> Element {
    rsx! {
        section {
            class: "ui-glass-surface",
            "data-glass-level": glass_level_name(level),
            "data-glass-tone": glass_tone_name(tone),
            "data-glass-density": glass_density_name(density),
            {children}
        }
    }
}

#[component]
pub fn Stack(#[props(default = "md".to_string())] gap: String, children: Element) -> Element {
    rsx! {
        div {
            class: "ui-stack ui-stack--gap-{gap}",
            {children}
        }
    }
}

pub const fn glass_level_name(level: GlassLevel) -> &'static str {
    match level {
        GlassLevel::Subtle => "subtle",
        GlassLevel::Floating => "floating",
        GlassLevel::Overlay => "overlay",
        GlassLevel::Chrome => "chrome",
    }
}

pub const fn glass_tone_name(tone: GlassTone) -> &'static str {
    match tone {
        GlassTone::Neutral => "neutral",
        GlassTone::Primary => "primary",
        GlassTone::Success => "success",
        GlassTone::Warning => "warning",
        GlassTone::Danger => "danger",
        GlassTone::Info => "info",
    }
}

pub const fn glass_density_name(density: GlassDensity) -> &'static str {
    match density {
        GlassDensity::Compact => "compact",
        GlassDensity::Comfortable => "comfortable",
        GlassDensity::Spacious => "spacious",
    }
}
