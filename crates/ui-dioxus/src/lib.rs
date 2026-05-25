#![forbid(unsafe_code)]

mod accordion;
mod buttons;
mod capture;
mod combobox;
mod composition;
mod data_table;
mod datepicker;
mod display;
mod forms;
mod kinetics;
mod layout;
mod motion_path;
mod navigation;
mod overlays;
mod popover;
mod scene_player;
mod select;
mod split_text;

use dioxus::prelude::*;
use ui_glass::{GlassDensity, GlassLevel, GlassTone};

// ---------------------------------------------------------------------------
// Public component surface.
//
// Components are exported under TWO names per `docs/component-naming.md`:
//
//   1. Standard-library names (`Button`, `TextField`, `Checkbox`, `Dialog`, …)
//      — match the vocabulary every Dioxus/React developer already knows.
//   2. Functional / SaaS-role names (`ActionControl`, `TextEntry`,
//      `ChoiceMark`, `ModalLayer`, …) — describe the user-facing role
//      rather than the widget shape; preferred in product docs.
//
// Both surfaces are stable for the 0.1.x line. The `crates/kinetics`
// prelude re-exports the union; the gallery and SSR tests use the
// standard names. Do not collapse one set into the other without
// updating `docs/component-naming.md` and the `prelude.rs` integration
// test in `crates/kinetics/tests/prelude.rs`, which pins both surfaces.
// ---------------------------------------------------------------------------

pub use accordion::{Accordion, AccordionSection};
pub use buttons::{IconButton, IconButtonSize, IconButtonTone};
pub use capture::CaptureStage;
pub use combobox::{filter_options, Combobox, ComboboxOption};
#[allow(deprecated)]
pub use composition::{FrameClip, FrameLayer, FrameStage};
pub use data_table::{DataTable, DataTableColumn, DataTableRow, SortDirection};
pub use datepicker::{
    day_of_week, days_in_month, format_iso_date, parse_iso_date, DatePicker,
    DATEPICKER_DEFAULT_ANCHOR,
};
pub use display::{Alert, AlertTone, EmptyState, MetricCard, MetricTone, Progress, Skeleton};
pub use display::{EmptyState as BlankState, MetricCard as MetricReadout};
pub use forms::{Checkbox, RadioGroup, RadioOption, Slider, Switch, TextField, TextFieldType};
pub use forms::{
    Checkbox as ChoiceMark, RadioGroup as OptionGroup, Switch as StateSwitch,
    TextField as TextEntry,
};
pub use kinetics::{
    Cue, KineticBox, KineticText, Presence, PresenceCue, PresenceGate, Sequence, SequenceContext,
    TimelineScope,
};
pub use layout::{SharedElement, SharedLayout};
pub use motion_path::MotionPath;
pub use navigation::{
    Breadcrumb, BreadcrumbItem, Pagination, SegmentItem, SegmentedControl, Sidebar, SidebarItem,
    SidebarSection, Stepper, StepperStep, TabItem, TabPanel, Tabs, Toolbar,
};
pub use navigation::{Sidebar as NavigationRail, Tabs as ViewSwitcher, Toolbar as ActionBar};
pub use overlays::{
    CommandGroup, CommandItem, CommandMenu, Dialog, DialogAction, DialogActionTone, DropdownMenu,
    DropdownMenuItem, Toast, ToastTone, Tooltip,
};
pub use overlays::{
    CommandMenu as CommandFinder, Dialog as ModalLayer, DropdownMenu as ActionMenu,
    Toast as NoticeStack, Tooltip as ContextHint,
};
pub use popover::{Popover, PopoverSide};
pub use scene_player::{Clip, Scene, SceneContext};
pub use select::{Select, SelectOption};
pub use split_text::{SplitMode, SplitText};
#[cfg(feature = "liquid-glass")]
pub use ui_glass_dioxus::{LiquidSurface, LiquidSurfaceProps};
pub use ui_runtime::SceneState;

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
    /// Skip tier detection and always render the CSS fallback path. Use when
    /// the host page already contains many GlassSurface instances and the
    /// per-page WebGL-context cap (webkit: ~8) would force older canvases to
    /// be dropped — e.g. design-token showcase grids where the
    /// `data-glass-*` attributes are the contract being demonstrated.
    #[props(default)]
    force_css: bool,
    children: Element,
) -> Element {
    #[cfg(feature = "liquid-glass")]
    {
        if force_css {
            return glass_surface_css(level, tone, density, children);
        }

        use ui_glass::{
            GlassDepth, LiquidMaterial, MaterialDensity, MaterialEdge, MaterialRequest,
            MaterialTone, MaterialVibrancy,
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
