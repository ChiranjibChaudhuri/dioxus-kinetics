use dioxus::prelude::*;

use crate::previews::{
    actions::{
        button_preview, command_menu_preview, dropdown_menu_preview, icon_button_preview,
        toolbar_preview,
    },
    capture::capture_stage_preview,
    composition::frame_stage_preview,
    feedback::{
        alert_preview, dialog_preview, empty_state_preview, popover_preview, progress_preview,
        skeleton_preview, toast_preview, tooltip_preview,
    },
    foundations::glass_layer_preview,
    inputs::{
        checkbox_preview, combobox_preview, data_table_preview, date_picker_preview,
        radio_group_preview, select_preview, slider_preview, switch_preview, text_field_preview,
    },
    layout::{accordion_preview, stack_preview, tabs_preview},
    liquid_glass::liquid_surface_preview,
    motion::{
        kinetic_box_preview, kinetic_text_preview, presence_gate_preview, presence_preview,
        sequence_preview, timeline_scope_preview,
    },
    navigation::{
        breadcrumb_preview, pagination_preview, segmented_control_preview, sidebar_preview,
        stepper_preview,
    },
    shared::{shared_element_preview, shared_layout_preview},
    surfaces::{glass_surface_preview, metric_card_preview, surface_preview},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentCategory {
    Foundations,
    Actions,
    Inputs,
    Navigation,
    Layout,
    Surfaces,
    Feedback,
    DataWorkflows,
    Motion,
    Composition,
    Capture,
    Scene,
}

impl ComponentCategory {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Foundations => "Foundations",
            Self::Actions => "Actions",
            Self::Inputs => "Inputs",
            Self::Navigation => "Navigation",
            Self::Layout => "Layout",
            Self::Surfaces => "Surfaces",
            Self::Feedback => "Feedback",
            Self::DataWorkflows => "Data workflows",
            Self::Motion => "Motion",
            Self::Composition => "Composition",
            Self::Capture => "Capture",
            Self::Scene => "Scene",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Foundations => "Material and surface primitives that anchor the visual system.",
            Self::Actions => "Command controls that trigger a product action.",
            Self::Inputs => "Controls that collect user-entered data.",
            Self::Navigation => "Wayfinding controls that move between product regions.",
            Self::Layout => "Structure primitives for arranging interface regions.",
            Self::Surfaces => "Containers that define visual layers and material treatment.",
            Self::Feedback => "Overlays and messages that respond to user or system state.",
            Self::DataWorkflows => "Readouts and surfaces that summarize product data.",
            Self::Motion => "Lifecycle and layout motion primitives for continuity.",
            Self::Composition => {
                "Frame-addressable scenes for previews and export-safe compositions."
            }
            Self::Capture => "Viewport and frame targets for documentation and capture runners.",
            Self::Scene => {
                "Seekable cinematic compositions: one paused clock drives every animation runtime."
            }
        }
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Foundations => "foundations",
            Self::Actions => "actions",
            Self::Inputs => "inputs",
            Self::Navigation => "navigation",
            Self::Layout => "layout",
            Self::Surfaces => "surfaces",
            Self::Feedback => "feedback",
            Self::DataWorkflows => "data-workflows",
            Self::Motion => "motion",
            Self::Composition => "composition",
            Self::Capture => "capture",
            Self::Scene => "scene",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentStatus {
    Ready,
    ComingSoon,
}

impl ComponentStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::ComingSoon => "Coming soon",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ComponentDoc {
    pub name: &'static str,
    pub category: ComponentCategory,
    pub status: ComponentStatus,
    pub summary: &'static str,
    pub snippet: &'static str,
    pub accessibility: &'static str,
    pub render: Option<fn() -> Element>,
}

pub fn categories() -> &'static [ComponentCategory] {
    &[
        ComponentCategory::Foundations,
        ComponentCategory::Actions,
        ComponentCategory::Inputs,
        ComponentCategory::Navigation,
        ComponentCategory::Layout,
        ComponentCategory::Surfaces,
        ComponentCategory::Feedback,
        ComponentCategory::DataWorkflows,
        ComponentCategory::Motion,
        ComponentCategory::Composition,
        ComponentCategory::Capture,
        ComponentCategory::Scene,
    ]
}

pub fn component_docs() -> &'static [ComponentDoc] {
    &COMPONENT_DOCS
}

const BASIC_ACCESSIBILITY: &str = "Renders native semantic elements and stable focusable controls.";

const COMPONENT_DOCS: [ComponentDoc; 55] = [
    ComponentDoc {
        name: "Button",
        category: ComponentCategory::Actions,
        status: ComponentStatus::Ready,
        summary: "Triggers a user action with semantic variants for primary, secondary, quiet, and destructive commands.",
        snippet: BUTTON_SNIPPET,
        accessibility: BASIC_ACCESSIBILITY,
        render: Some(button_preview),
    },
    ComponentDoc {
        name: "IconButton",
        category: ComponentCategory::Actions,
        status: ComponentStatus::Ready,
        summary: "A compact icon-only command control with an accessible label, three tones, and three sizes.",
        snippet: ICON_BUTTON_SNIPPET,
        accessibility: "Accessible name comes from the `label` prop, exposed on `aria-label`. The icon child uses `aria-hidden`.",
        render: Some(icon_button_preview),
    },
    ComponentDoc {
        name: "CommandMenu",
        category: ComponentCategory::Actions,
        status: ComponentStatus::Ready,
        summary: "A controlled command-search surface with grouped actions and empty state.",
        snippet: COMMAND_MENU_SNIPPET,
        accessibility: "Uses dialog and listbox-oriented semantics for command discovery.",
        render: Some(command_menu_preview),
    },
    ComponentDoc {
        name: "Toolbar",
        category: ComponentCategory::Actions,
        status: ComponentStatus::Ready,
        summary: "A command grouping surface for page and workflow actions.",
        snippet: TOOLBAR_SNIPPET,
        accessibility: "Uses role toolbar and grouped command regions.",
        render: Some(toolbar_preview),
    },
    ComponentDoc {
        name: "DropdownMenu",
        category: ComponentCategory::Actions,
        status: ComponentStatus::Ready,
        summary: "Anchored `role=\"menu\"` overlay for action lists — kebab menus, \"More actions\" buttons, overflow menus. Different from `CommandMenu` (no search, menu/menuitem semantics rather than listbox/option). Separators are first-class items.",
        snippet: DROPDOWN_MENU_SNIPPET,
        accessibility: "Panel is `role=\"menu\"`; rows are `role=\"menuitem\"` rendered as native `<button>` so disabled, focus, and click semantics come for free. Dividers carry `role=\"separator\"`.",
        render: Some(dropdown_menu_preview),
    },
    ComponentDoc {
        name: "TextField",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::Ready,
        summary: "A labeled text input with help, error, disabled, and adornment states.",
        snippet: TEXT_FIELD_SNIPPET,
        accessibility: "Associates label, input, help text, and error text with stable ids.",
        render: Some(text_field_preview),
    },
    ComponentDoc {
        name: "Checkbox",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::Ready,
        summary: "A labeled binary or mixed selection control for settings and lists.",
        snippet: CHECKBOX_SNIPPET,
        accessibility: "Uses native checkbox behavior and aria-checked for mixed state.",
        render: Some(checkbox_preview),
    },
    ComponentDoc {
        name: "Switch",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::Ready,
        summary: "A labeled on/off control for immediate settings.",
        snippet: SWITCH_SNIPPET,
        accessibility: "Uses role switch and aria-checked so assistive tech reads state.",
        render: Some(switch_preview),
    },
    ComponentDoc {
        name: "Stack",
        category: ComponentCategory::Layout,
        status: ComponentStatus::Ready,
        summary: "Arranges children in a vertical rhythm with semantic spacing tokens.",
        snippet: STACK_SNIPPET,
        accessibility: BASIC_ACCESSIBILITY,
        render: Some(stack_preview),
    },
    ComponentDoc {
        name: "Tabs",
        category: ComponentCategory::Layout,
        status: ComponentStatus::Ready,
        summary: "A controlled tab interface for switching between related panels.",
        snippet: TABS_SNIPPET,
        accessibility: "Uses tablist, tab, and tabpanel roles with selected state.",
        render: Some(tabs_preview),
    },
    ComponentDoc {
        name: "Sidebar",
        category: ComponentCategory::Layout,
        status: ComponentStatus::Ready,
        summary: "A compact app navigation rail with sections and selected item state.",
        snippet: SIDEBAR_SNIPPET,
        accessibility: "Uses nav semantics and aria-current on the selected item.",
        render: Some(sidebar_preview),
    },
    ComponentDoc {
        name: "Surface",
        category: ComponentCategory::Surfaces,
        status: ComponentStatus::Ready,
        summary: "Creates a solid content layer for panels, sections, and grouped SaaS workflows.",
        snippet: SURFACE_SNIPPET,
        accessibility: BASIC_ACCESSIBILITY,
        render: Some(surface_preview),
    },
    ComponentDoc {
        name: "GlassSurface",
        category: ComponentCategory::Surfaces,
        status: ComponentStatus::Ready,
        summary: "Creates a translucent material layer with semantic level, tone, and density attributes.",
        snippet: GLASS_SURFACE_SNIPPET,
        accessibility: BASIC_ACCESSIBILITY,
        render: Some(glass_surface_preview),
    },
    ComponentDoc {
        name: "MetricCard",
        category: ComponentCategory::Surfaces,
        status: ComponentStatus::Ready,
        summary: "A dashboard metric surface with label, value, delta, tone, and sparkline region.",
        snippet: METRIC_CARD_SNIPPET,
        accessibility: "Keeps metric text readable and marks decorative sparkline region hidden.",
        render: Some(metric_card_preview),
    },
    ComponentDoc {
        name: "Dialog",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::Ready,
        summary: "A controlled modal surface for focused decisions and workflows.",
        snippet: DIALOG_SNIPPET,
        accessibility: "Uses role dialog and aria-modal; focus trapping is a later helper layer.",
        render: Some(dialog_preview),
    },
    ComponentDoc {
        name: "Toast",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::Ready,
        summary: "A status notification with tone, description, action, and dismiss affordance.",
        snippet: TOAST_SNIPPET,
        accessibility: "Uses status or alert live-region roles based on tone.",
        render: Some(toast_preview),
    },
    ComponentDoc {
        name: "Tooltip",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::Ready,
        summary: "A controlled explanatory layer connected to trigger text.",
        snippet: TOOLTIP_SNIPPET,
        accessibility: "Connects trigger and tooltip content with aria-describedby.",
        render: Some(tooltip_preview),
    },
    ComponentDoc {
        name: "EmptyState",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::Ready,
        summary: "A polished empty state for missing reports, records, or workflows.",
        snippet: EMPTY_STATE_SNIPPET,
        accessibility: "Uses semantic section content and a clear action button when present.",
        render: Some(empty_state_preview),
    },
    ComponentDoc {
        name: "Presence",
        category: ComponentCategory::Motion,
        status: ComponentStatus::Ready,
        summary: "Renders children with an enter/exit animation lifecycle; settles into the rendered state on SSR and reduced-motion paths.",
        snippet: PRESENCE_SNIPPET,
        accessibility: "Hidden state renders no children; the entering and visible states keep the DOM stable for assistive tech.",
        render: Some(presence_preview),
    },
    ComponentDoc {
        name: "Sequence",
        category: ComponentCategory::Motion,
        status: ComponentStatus::Ready,
        summary: "Orchestrates multiple kinetic boxes through a coordinated timeline of property cues.",
        snippet: SEQUENCE_SNIPPET,
        accessibility: "The sample is deterministic per clock; reduced-motion policies render the settled state.",
        render: Some(sequence_preview),
    },
    ComponentDoc {
        name: "SharedLayout",
        category: ComponentCategory::Motion,
        status: ComponentStatus::Ready,
        summary: "Provides a scoped shared-element registry for descendant SharedElement components.",
        snippet: SHARED_LAYOUT_SNIPPET,
        accessibility: "Pure wrapper; renders children unchanged.",
        render: Some(shared_layout_preview),
    },
    ComponentDoc {
        name: "SharedElement",
        category: ComponentCategory::Motion,
        status: ComponentStatus::Ready,
        summary: "Marks an element with a shared identity for cross-tree FLIP transitions; SSR-safe.",
        snippet: SHARED_ELEMENT_SNIPPET,
        accessibility: "data-shared-id attribute carries the identity; reduced-motion renders at the settled state.",
        render: Some(shared_element_preview),
    },
    ComponentDoc {
        name: "TimelineScope",
        category: ComponentCategory::Motion,
        status: ComponentStatus::Ready,
        summary: "Coordinates native Rust timeline cues for Dioxus UI motion.",
        snippet: TIMELINE_SCOPE_SNIPPET,
        accessibility: "Reduced motion policies collapse timeline cues to stable states.",
        render: Some(timeline_scope_preview),
    },
    ComponentDoc {
        name: "KineticBox",
        category: ComponentCategory::Motion,
        status: ComponentStatus::Ready,
        summary: "Tags a region with a motion cue and stable kinetic id so timeline cues can target it.",
        snippet: KINETIC_BOX_SNIPPET,
        accessibility: "Motion cue is exposed via data attributes; reduced-motion policies replace cues with stable presentation.",
        render: Some(kinetic_box_preview),
    },
    ComponentDoc {
        name: "PresenceGate",
        category: ComponentCategory::Motion,
        status: ComponentStatus::Ready,
        summary: "Renders children only when the presence flag is set; gallery preview compares present and hidden states.",
        snippet: PRESENCE_GATE_SNIPPET,
        accessibility: "Hidden state renders no children; assistive tech does not encounter stale content.",
        render: Some(presence_gate_preview),
    },
    ComponentDoc {
        name: "FrameStage",
        category: ComponentCategory::Composition,
        status: ComponentStatus::Ready,
        summary: "Renders a deterministic frame-addressable scene for previews and export-safe compositions.",
        snippet: FRAME_STAGE_SNIPPET,
        accessibility: "Frame content remains readable at the selected frame and does not depend on wall-clock animation.",
        render: Some(frame_stage_preview),
    },
    ComponentDoc {
        name: "CaptureStage",
        category: ComponentCategory::Capture,
        status: ComponentStatus::Ready,
        summary: "Declares a viewport and frame target for documentation, tests, and future capture runners.",
        snippet: CAPTURE_STAGE_SNIPPET,
        accessibility: "Capture previews preserve semantic text and expose stable frame metadata.",
        render: Some(capture_stage_preview),
    },
    ComponentDoc {
        name: "GlassLayer",
        category: ComponentCategory::Foundations,
        status: ComponentStatus::Ready,
        summary: "Functional material name for translucent glass surfaces with solid fallback behavior.",
        snippet: GLASS_LAYER_SNIPPET,
        accessibility: "Text contrast is validated against solid fallback surfaces.",
        render: Some(glass_layer_preview),
    },
    ComponentDoc {
        name: "LiquidSurface",
        category: ComponentCategory::Surfaces,
        status: ComponentStatus::Ready,
        summary: "Pointer-reactive frosted surface with refraction, dispersion, specular, ambient mesh, and tint adaptation. Runs on a wgpu-backed canvas via the ui-glass-dioxus engine.",
        snippet: LIQUID_SURFACE_SNIPPET,
        accessibility: "Canvas is decorative; foreground children are DOM elements with full pointer-events and accessible text.",
        render: Some(liquid_surface_preview),
    },
    ComponentDoc {
        name: "KineticText",
        category: ComponentCategory::Motion,
        status: ComponentStatus::Ready,
        summary: "Span-level text node tagged with a `data-motion-cue` for the kinetics runtime. The wrapping `Sequence` or `TimelineScope` drives the actual transition.",
        snippet: KINETIC_TEXT_SNIPPET,
        accessibility: "Carries an `aria-label` that mirrors the text content so motion never strips the readable string.",
        render: Some(kinetic_text_preview),
    },
    // -------- Coming-soon backlog (Spec 5+ roadmap) --------
    ComponentDoc {
        name: "Select",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::Ready,
        summary: "Single-select dropdown built on `Popover`. Trigger shows the selected option's label (or placeholder); the popover renders a `role=\"listbox\"` of options with selection state, disabled rows, and a chevron icon.",
        snippet: SELECT_SNIPPET,
        accessibility: "Trigger is `role=\"combobox\"` with `aria-haspopup=\"listbox\"`; options are `role=\"option\"` with `aria-selected` + `aria-disabled`. Typeahead filter + Combobox mode is a future spec.",
        render: Some(select_preview),
    },
    ComponentDoc {
        name: "DatePicker",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::Ready,
        summary: "Calendar-grid temporal input built on `Popover`. Renders a trigger that shows the selected ISO date, opens a month-navigable grid, emits `on_select(YYYY-MM-DD)` on click. Range mode + locale-aware month names are a future spec.",
        snippet: DATE_PICKER_SNIPPET,
        accessibility: "Trigger is `aria-haspopup=\"dialog\"` + `aria-expanded`; the grid is `role=\"grid\"` with `role=\"columnheader\"` weekday cells and `role=\"gridcell\"` day buttons exposing `aria-selected` + ISO `aria-label`.",
        render: Some(date_picker_preview),
    },
    ComponentDoc {
        name: "Combobox",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::Ready,
        summary: "Typeahead-filtered single-select built on `Popover`. Trigger is a text input; the listbox is narrowed by `query` via the pure `filter_options` helper (case-insensitive substring match). Use over `Select` when the option list is long enough that scanning is faster than scrolling.",
        snippet: COMBOBOX_SNIPPET,
        accessibility: "Input is `role=\"combobox\"` with `aria-autocomplete=\"list\"`, `aria-haspopup=\"listbox\"`, and `aria-controls` pointing at the listbox; options carry `role=\"option\"` + `aria-selected`. Empty state is a polite live region (`role=\"status\"`).",
        render: Some(combobox_preview),
    },
    ComponentDoc {
        name: "RadioGroup",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::Ready,
        summary: "Mutually-exclusive choice picker rendered as native `<input type=\"radio\">` inputs sharing a `name`. Each option carries label + optional description copy. Different from `SegmentedControl`: that's a button group for short, equally-weighted choices; `RadioGroup` is for descriptive, form-submittable options.",
        snippet: RADIO_GROUP_SNIPPET,
        accessibility: "Native `<fieldset>` + `<legend>`; the option list carries `role=\"radiogroup\"`. Browsers handle arrow-key roving focus and form submission automatically.",
        render: Some(radio_group_preview),
    },
    ComponentDoc {
        name: "Slider",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::Ready,
        summary: "Continuous numeric input rendered as a native `<input type=\"range\">`. Keyboard support (Arrows, Page Up/Down, Home/End) and touch/pointer drag both work out of the box.",
        snippet: SLIDER_SNIPPET,
        accessibility: "Native `<input type=\"range\">` semantics; `aria-valuetext` for human-readable announcement (e.g. \"60%\" instead of \"60\").",
        render: Some(slider_preview),
    },
    ComponentDoc {
        name: "SegmentedControl",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::Ready,
        summary: "Mutually-exclusive choice picker rendered as a button group, complementing radio inputs for short option sets like view-mode switchers.",
        snippet: SEGMENTED_CONTROL_SNIPPET,
        accessibility: "`role=\"radiogroup\"` with `aria-label`; each option carries `role=\"radio\"` + `aria-checked`.",
        render: Some(segmented_control_preview),
    },
    ComponentDoc {
        name: "Popover",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::Ready,
        summary: "Anchored overlay panel — minimal viable version with controlled open state, four anchor sides, and Escape-to-dismiss. Foundation for the upcoming Select / DatePicker / Menu overlays; a future spec layers viewport-flip + collision detection.",
        snippet: POPOVER_SNIPPET,
        accessibility: "Trigger carries `aria-haspopup=\"dialog\"` + `aria-expanded` + `aria-controls`; the panel is `role=\"dialog\"`.",
        render: Some(popover_preview),
    },
    ComponentDoc {
        name: "Alert",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::Ready,
        summary: "Page-level message banner with severity tones (Neutral/Success/Warning/Danger/Info). Persists in layout; complements Toast for non-ephemeral context.",
        snippet: ALERT_SNIPPET,
        accessibility: "`role=\"alert\"` for high-severity tones (Danger/Warning); `role=\"status\"` (polite live region) otherwise.",
        render: Some(alert_preview),
    },
    ComponentDoc {
        name: "Progress",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::Ready,
        summary: "Determinate progress bar (value 0.0–1.0) and indeterminate spinner-style variant. Pair with `Skeleton` for content-shape loading placeholders.",
        snippet: PROGRESS_SNIPPET,
        accessibility: "WAI-ARIA progressbar pattern with `aria-valuenow`/`aria-valuetext`; indeterminate animation respects `prefers-reduced-motion`.",
        render: Some(progress_preview),
    },
    ComponentDoc {
        name: "Skeleton",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::Ready,
        summary: "Neutral pulsing block that preserves content shape while data loads. Composable to build headline/paragraph/card placeholders.",
        snippet: SKELETON_SNIPPET,
        accessibility: "`aria-hidden=\"true\"` (the skeleton is decorative; the surrounding live region or label announces loading state).",
        render: Some(skeleton_preview),
    },
    ComponentDoc {
        name: "DataTable",
        category: ComponentCategory::DataWorkflows,
        status: ComponentStatus::Ready,
        summary: "Native `<table>` primitive with optional caption + sortable column headers. Sorting itself is the caller's responsibility — on click, the component emits `on_sort(column_key)` and the consumer re-sorts the row slice. Column resize, sticky headers, and virtualization are deferred.",
        snippet: DATA_TABLE_SNIPPET,
        accessibility: "Native `<table>` / `<thead>` / `<tbody>` semantics; sortable headers carry `aria-sort` (`none` / `ascending` / `descending`) and contain a `<button>` with an `aria-label` so screen readers announce the sort intent.",
        render: Some(data_table_preview),
    },
    ComponentDoc {
        name: "Pagination",
        category: ComponentCategory::DataWorkflows,
        status: ComponentStatus::Ready,
        summary: "Offset-style page-jump control for data-heavy lists. Renders first/current±1/last with ellipsis fills; prev/next buttons disabled at boundaries.",
        snippet: PAGINATION_SNIPPET,
        accessibility: "`<nav aria-label>` landmark; current page emits `aria-current=\"page\"`; per-button `aria-label` reads each page number.",
        render: Some(pagination_preview),
    },
    ComponentDoc {
        name: "Breadcrumb",
        category: ComponentCategory::Navigation,
        status: ComponentStatus::Ready,
        summary: "Hierarchical wayfinding trail. The last item renders as the current location (no link, `aria-current=\"page\"`); earlier items are anchor links separated by a visual divider.",
        snippet: BREADCRUMB_SNIPPET,
        accessibility: "`<nav aria-label>` landmark with ordered list; the divider character is `aria-hidden`.",
        render: Some(breadcrumb_preview),
    },
    ComponentDoc {
        name: "Stepper",
        category: ComponentCategory::Navigation,
        status: ComponentStatus::Ready,
        summary: "Multi-step workflow tracker with completed / active / upcoming states. Horizontal and vertical orientations; each step is clickable.",
        snippet: STEPPER_SNIPPET,
        accessibility: "Ordered list with per-step status announced via `aria-current=\"step\"` on the active step and visually-hidden state text on every step.",
        render: Some(stepper_preview),
    },
    ComponentDoc {
        name: "Accordion",
        category: ComponentCategory::Layout,
        status: ComponentStatus::Ready,
        summary: "Collapsible content sections with single- or multi-expand behaviour (controlled by the consumer). Disabled-section support; renders a `+`/`−` marker per section.",
        snippet: ACCORDION_SNIPPET,
        accessibility: "WAI-ARIA disclosure pattern: each header is a `<button>` with `aria-expanded` + `aria-controls`; the region carries `role=\"region\"` + `aria-labelledby`.",
        render: Some(accordion_preview),
    },
    ComponentDoc {
        name: "Scene · Product Intro 10s",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "Seekable 10-second cinematic composition: title, FLIP card deck, metric counter, CTA pulse — one paused clock for every runtime.",
        snippet: SCENE_PRODUCT_INTRO_SNIPPET,
        accessibility: "Scrubber is keyboard-operable; reduced-motion renders the settled state and disables the scrubber with an explicit tag.",
        render: Some(crate::previews::scene::product_intro_preview),
    },
    ComponentDoc {
        name: "Scene · Scroll-pinned Story",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ScrollTrigger-style: a 10-second narrative pinned to a 200vh region. Scroll drives elapsed_ms via IntersectionObserver + window scroll.",
        snippet: SCENE_SCROLL_STORY_SNIPPET,
        accessibility: "Reduced motion settles immediately and ignores scroll. Each beat's text is independently labeled.",
        render: Some(crate::previews::scene::scroll_pinned_story_preview),
    },
    ComponentDoc {
        name: "Scene · Split Headline",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "SplitText: per-character spans with sequential data-stagger-index. Screen readers read the parent aria-label; the per-glyph spans are aria-hidden.",
        snippet: SCENE_SPLIT_HEADLINE_SNIPPET,
        accessibility: "Parent carries the full text via aria-label; glyph spans are aria-hidden so screen readers do not enumerate.",
        render: Some(crate::previews::scene::split_headline_preview),
    },
    ComponentDoc {
        name: "Scene · Curved Trajectory",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "MotionPath: a KineticBox traces a parametric S-curve sampled by arc length. Optional rotate-along-path tangent.",
        snippet: SCENE_CURVED_TRAJECTORY_SNIPPET,
        accessibility: "Visual-only decoration; the icon glyph remains in the DOM and is not announced.",
        render: Some(crate::previews::scene::curved_trajectory_preview),
    },
    ComponentDoc {
        name: "Scene · Lower Third Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: LowerThird chyron with name + role inside a 4s Scene + HoldEnd clip.",
        snippet: SCENE_LOWER_THIRD_SNIPPET,
        accessibility: "Parent aria-label carries \"<name>, <role>\".",
        render: Some(crate::previews::scene::lower_third_demo_preview),
    },
    ComponentDoc {
        name: "Scene · Caption Reading-Pace Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: Caption block driving SplitText { Word } at 320ms/word reading pace.",
        snippet: SCENE_CAPTION_SNIPPET,
        accessibility: "SplitText parent carries the full text via aria-label; word spans are aria-hidden.",
        render: Some(crate::previews::scene::caption_demo_preview),
    },
    ComponentDoc {
        name: "Scene · Wipe Transition Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: WipeTransition CSS mask sweep at 120deg across a gradient backdrop.",
        snippet: SCENE_WIPE_SNIPPET,
        accessibility: "Decorative; underlying heading is in normal reading order.",
        render: Some(crate::previews::scene::wipe_demo_preview),
    },
    ComponentDoc {
        name: "Scene · Metric Counter Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: MetricCounter with label + value + delta. Three sequential KineticText lines staggered via a parent TimelineScope.",
        snippet: SCENE_METRIC_COUNTER_SNIPPET,
        accessibility: "Each line is independently readable. The delta line is optional and omitted entirely when delta_text is None.",
        render: Some(crate::previews::scene::metric_counter_demo_preview),
    },
    ComponentDoc {
        name: "Scene · Social Overlay Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "ui-blocks: SocialOverlay notification card with platform brand accent. SocialPlatform enum covers Instagram, Twitter, YouTube, TikTok.",
        snippet: SCENE_SOCIAL_OVERLAY_SNIPPET,
        accessibility: "Handle + message are independently labeled; the platform accent is decorative.",
        render: Some(crate::previews::scene::social_overlay_demo_preview),
    },
    ComponentDoc {
        name: "Scene · Manual Driver Demo",
        category: ComponentCategory::Scene,
        status: ComponentStatus::Ready,
        summary: "SceneDriver::Manual disables autoplay. The transport scrubber is the only way to advance elapsed_ms — useful for export pipelines, frame-stepping, and scrubbable demos.",
        snippet: SCENE_MANUAL_DRIVER_SNIPPET,
        accessibility: "Scrubber is keyboard-operable; the underlying text remains in the natural reading order.",
        render: Some(crate::previews::scene::manual_driver_demo_preview),
    },
];

const BUTTON_SNIPPET: &str = r#"Button {
    variant: ButtonVariant::Primary,
    "Save changes"
}"#;

const ICON_BUTTON_SNIPPET: &str = r#"IconButton {
    label: "Archive".to_string(),
    tone: IconButtonTone::Neutral,
    Close { size: 16 }
}"#;

const COMMAND_MENU_SNIPPET: &str = r#"CommandMenu {
    open: true,
    query: "rep",
    selected_id: "reports",
    groups: command_groups,
}"#;

const TOOLBAR_SNIPPET: &str = r#"Toolbar {
    primary: vec!["New".to_string(), "Filter".to_string()],
    secondary: "Updated now",
}"#;

const TEXT_FIELD_SNIPPET: &str = r#"TextField {
    id: "workspace-name",
    label: "Workspace name",
    value: "Acme Ops",
    help_text: "Visible to teammates",
}"#;

const CHECKBOX_SNIPPET: &str = r#"Checkbox {
    id: "weekly-summary",
    label: "Send weekly summary",
    checked: true,
    description: "Every Monday morning",
}"#;

const SWITCH_SNIPPET: &str = r#"Switch {
    id: "auto-renew",
    label: "Auto renew",
    checked: true,
    description: "Keep billing active",
}"#;

const STACK_SNIPPET: &str = r#"Stack {
    gap: "sm".to_string(),
    Button { "Create" }
    Button {
        variant: ButtonVariant::Secondary,
        "Cancel"
    }
}"#;

const TABS_SNIPPET: &str = r#"Tabs {
    selected: "billing",
    items: tabs,
    panels: panels,
}"#;

const SIDEBAR_SNIPPET: &str = r#"Sidebar {
    selected: "settings",
    sections: navigation_sections,
}"#;

const SURFACE_SNIPPET: &str = r#"Surface {
    h3 { "Pipeline health" }
    p { "12 workflows running" }
}"#;

const GLASS_SURFACE_SNIPPET: &str = r#"GlassSurface {
    level: GlassLevel::Floating,
    tone: GlassTone::Neutral,
    density: GlassDensity::Comfortable,
    "Revenue operations"
}"#;

const METRIC_CARD_SNIPPET: &str = r#"MetricCard {
    label: "Net revenue",
    value: "$128.4k",
    delta: "+12.5%",
    tone: MetricTone::Success,
}"#;

const DIALOG_SNIPPET: &str = r#"Dialog {
    open: true,
    title: "Delete workspace",
    description: "This action cannot be undone.",
}"#;

const TOAST_SNIPPET: &str = r#"Toast {
    tone: ToastTone::Success,
    title: "Report exported",
    description: "The PDF is ready.",
}"#;

const TOOLTIP_SNIPPET: &str = r#"Tooltip {
    id: "net-revenue-tip",
    visible: true,
    trigger_label: "Net revenue",
    content: "Revenue after refunds and credits.",
}"#;

const EMPTY_STATE_SNIPPET: &str = r#"EmptyState {
    title: "No reports yet",
    description: "Create a report to share performance.",
    action_label: "Create report",
}"#;

const PRESENCE_SNIPPET: &str = r#"Presence {
    present: is_visible,
    cue: PresenceCue::Rise,
    p { "Hello" }
}"#;

const SEQUENCE_SNIPPET: &str = r#"Sequence {
    cues: Some(vec![
        Cue::new("title", 0.0, MotionCue::Opacity { from: 0.0, to: 1.0, transition: Transition::tween(220) }),
        Cue::new("body", 120.0, MotionCue::Translate { axis: Axis::Y, from: 12.0, to: 0.0, transition: Transition::tween(200) }),
        Cue::new("cta", 320.0, MotionCue::Scale { from: 0.94, to: 1.0, transition: Transition::tween(240) }),
    ]),
    KineticBox { id: "title", h3 { "Welcome" } }
    KineticBox { id: "body", p { "Subtle entry" } }
    KineticBox { id: "cta", Button { "Get started" } }
}"#;

const SHARED_LAYOUT_SNIPPET: &str = r#"SharedLayout {
    SharedElement { id: "hero",
        p { "Cross-tree" }
    }
}"#;

const SHARED_ELEMENT_SNIPPET: &str = r#"SharedElement {
    id: "hero",
    p { "Identity persists across renders" }
}"#;

const TIMELINE_SCOPE_SNIPPET: &str = r#"TimelineScope {
    id: "dashboard-enter",
    autoplay: true,
    KineticBox {
        id: "metric-card",
        cue: "rise-in",
        "Revenue"
    }
}"#;

const KINETIC_BOX_SNIPPET: &str = r#"KineticBox {
    id: "metric-card",
    cue: "rise-in",
    "Tile body"
}"#;

const PRESENCE_GATE_SNIPPET: &str = r#"PresenceGate {
    present: is_visible,
    p { "Visible state" }
}"#;

const FRAME_STAGE_SNIPPET: &str = r#"FrameStage {
    composition: Composition::new("launch-demo", 1920, 1080, 30, 180),
    frame: 42,
    FrameClip {
        start: 0,
        duration: 60,
        FrameLayer {
            id: "title",
            depth: 10,
            "Dioxus Kinetics"
        }
    }
}"#;

const CAPTURE_STAGE_SNIPPET: &str = r#"CaptureStage {
    id: "component-showcase",
    viewport: "desktop",
    frame: 72,
    "Preview"
}"#;

const GLASS_LAYER_SNIPPET: &str = r#"GlassLayer {
    level: GlassLevel::Floating,
    tone: GlassTone::Neutral,
    density: GlassDensity::Comfortable,
    "Revenue operations"
}"#;

const LIQUID_SURFACE_SNIPPET: &str = r#"LiquidSurface {
    material: LiquidMaterial::floating()
        .ambient_mesh(AmbientMesh::Aurora)
        .pointer_reactive()
        .radius(24.0)
        .tint(Color::rgba(255, 255, 255, 1.0), 0.18),
    background: Some(BackgroundSource::Gradient(Gradient::conic(
        [0.5, 0.5], 0.0,
        vec![
            GradientStop { offset: 0.0, color: Color::rgba( 80, 100, 220, 1.0) },
            GradientStop { offset: 0.5, color: Color::rgba(180,  80, 180, 1.0) },
            GradientStop { offset: 1.0, color: Color::rgba( 80, 100, 220, 1.0) },
        ],
    ))),
    width: 400,
    height: 240,
    div {
        style: "padding: 16px; color: white; font-weight: 600;",
        "Hover me"
    }
}"#;

const KINETIC_TEXT_SNIPPET: &str = r#"KineticText {
    id: "headline",
    text: "Welcome aboard",
    cue: "text-flow",
}"#;

const ALERT_SNIPPET: &str = r#"Alert {
    tone: AlertTone::Warning,
    title: "Quota at 92%",
    description: "Plan auto-upgrades on Friday at midnight.",
}"#;

const PROGRESS_SNIPPET: &str = r#"Progress {
    label: "Importing rows",
    value: 0.65,
    description: "8 060 / 12 400",
}"#;

const SKELETON_SNIPPET: &str = r#"Skeleton {
    height: "20px",
    width: "60%",
    radius: "6px",
}"#;

const SELECT_SNIPPET: &str = r#"Select {
    id: "billing-cadence",
    label: "Billing cadence",
    selected: "monthly",
    options: vec![
        SelectOption::new("monthly", "Monthly"),
        SelectOption::new("annual", "Annual"),
    ],
    on_select: move |v: String| /* update */ {},
}"#;

const DATE_PICKER_SNIPPET: &str = r#"DatePicker {
    id: "report-cutoff",
    label: "Report cutoff",
    value: "2026-05-23",
    on_select: move |iso: String| /* update */ {},
}"#;

const DATA_TABLE_SNIPPET: &str = r#"DataTable {
    columns: vec![
        DataTableColumn::new("workspace", "Workspace"),
        DataTableColumn::new("revenue", "Revenue").sortable(),
        DataTableColumn::new("seats", "Seats").sortable(),
    ],
    rows,
    sort_key: "revenue",
    sort_direction: SortDirection::Descending,
    on_sort: move |key: String| /* re-sort rows */ {},
}"#;

const POPOVER_SNIPPET: &str = r#"Popover {
    id: "filters-popover",
    open: is_open,
    side: PopoverSide::Bottom,
    trigger: rsx! { Button { "Filters" } },
    on_open_change: move |next| set_open(next),
    div { /* body content */ }
}"#;

const SLIDER_SNIPPET: &str = r#"Slider {
    id: "media-volume",
    label: "Volume",
    value: 60.0,
    min: 0.0,
    max: 100.0,
    step: 1.0,
    value_text: "60%",
}"#;

const SEGMENTED_CONTROL_SNIPPET: &str = r#"SegmentedControl {
    options: vec![
        SegmentItem::new("grid", "Grid"),
        SegmentItem::new("list", "List"),
    ],
    selected: "grid",
    group_label: "View mode",
}"#;

const PAGINATION_SNIPPET: &str = r#"Pagination {
    page: 3,
    total_pages: 12,
    on_select: move |p: u32| /* navigate */ {},
}"#;

const BREADCRUMB_SNIPPET: &str = r##"Breadcrumb {
    items: vec![
        BreadcrumbItem::link("Workspaces", "#"),
        BreadcrumbItem::link("Acme Ops", "#"),
        BreadcrumbItem::current("Reports"),
    ],
}"##;

const STEPPER_SNIPPET: &str = r#"Stepper {
    steps: vec![
        StepperStep::new("plan", "Plan"),
        StepperStep::new("checkout", "Checkout"),
        StepperStep::new("review", "Review"),
    ],
    current: "checkout",
}"#;

const ACCORDION_SNIPPET: &str = r#"Accordion {
    sections: vec![
        AccordionSection::new("billing", "Billing", "Payment + invoices"),
        AccordionSection::new("members", "Team", "Invite teammates"),
    ],
    expanded: vec!["billing"],
    on_toggle: move |id: String| { /* update */ },
}"#;

const SCENE_PRODUCT_INTRO_SNIPPET: &str = r##"Scene {
    id: "product-intro",
    width: 1920,
    height: 1080,
    duration_ms: 10_000.0,
    autoplay: true,
    controls: true,
    Clip { start_ms: 0.0,    duration_ms: 2_400.0, fill: ClipFill::HoldEnd, /* title */ }
    Clip { start_ms: 800.0,  duration_ms: 2_400.0, fill: ClipFill::HoldEnd, /* body  */ }
    Clip { start_ms: 3_000.0,duration_ms: 4_000.0,                          /* deck  */ }
    Clip { start_ms: 4_800.0,duration_ms: 2_200.0,                          /* count */ }
    Clip { start_ms: 6_800.0,duration_ms: 3_200.0, fill: ClipFill::HoldEnd, /* CTA   */ }
}"##;

const SCENE_SCROLL_STORY_SNIPPET: &str = r##"Scene {
    id: "scroll-story",
    duration_ms: 10_000.0,
    driver: Some(SceneDriver::Scroll(
        ScrollObserverConfig::new("#scroll-story-trigger"),
    )),
    Clip { start_ms: 0.0, duration_ms: 2_500.0, /* headline */ }
    Clip { start_ms: 2_500.0, duration_ms: 2_500.0, /* body */ }
    Clip { start_ms: 5_000.0, duration_ms: 2_500.0, /* feature */ }
    Clip { start_ms: 7_500.0, duration_ms: 2_500.0, /* CTA */ }
}"##;

const SCENE_SPLIT_HEADLINE_SNIPPET: &str = r##"Scene {
    id: "split-headline",
    duration_ms: 2_500.0,
    TimelineScope { id: "split-headline-timeline", autoplay: true,
        SplitText {
            text: "Kinetics typography, glyph by glyph.".to_string(),
            split_by: Some(SplitMode::Character),
        }
    }
}"##;

const SCENE_CURVED_TRAJECTORY_SNIPPET: &str = r##"Scene {
    id: "curved-trajectory",
    duration_ms: 4_000.0,
    Sequence {
        timeline: Some(/* MotionCue::Path with PathPoint::Bezier ... */),
        MotionPath {
            id: "trajectory-icon",
            path: vec![PathPoint::Line { end: (0.0, 0.0) }, PathPoint::Bezier { /* ... */ }],
            duration_ms: 4_000.0,
            KineticBox { id: "trajectory-icon", "•" }
        }
    }
}"##;

const COMBOBOX_SNIPPET: &str = r#"Combobox {
    id: "ticket-finder",
    label: "Find a ticket",
    value: "ord-2024-12-04",
    query: "ord-2024",
    options: vec![
        ComboboxOption::new("ord-2024-12-04", "ORD-2024-12-04 — Globex"),
        ComboboxOption::new("ord-2024-11-30", "ORD-2024-11-30 — Initech"),
    ],
    on_query: move |q: String| /* update */ {},
    on_select: move |v: String| /* update */ {},
}"#;

const RADIO_GROUP_SNIPPET: &str = r#"RadioGroup {
    id: "billing-plan",
    label: "Billing plan",
    name: "billing-plan",
    value: "monthly",
    options: vec![
        RadioOption::new("monthly", "Monthly")
            .with_description("Billed on the first of every month"),
        RadioOption::new("annual", "Annual")
            .with_description("Save 18% versus monthly"),
    ],
    on_change: move |v: String| /* update */ {},
}"#;

const DROPDOWN_MENU_SNIPPET: &str = r#"DropdownMenu {
    id: "workspace-actions",
    trigger: rsx! { Button { "More actions" } },
    items: vec![
        DropdownMenuItem::new("rename", "Rename"),
        DropdownMenuItem::new("duplicate", "Duplicate"),
        DropdownMenuItem::separator("div-1"),
        DropdownMenuItem::new("delete", "Delete"),
    ],
    on_select: move |id: String| /* dispatch */ {},
}"#;

const SCENE_LOWER_THIRD_SNIPPET: &str = r##"Scene {
    id: "lower-third-demo",
    duration_ms: 4_000.0,
    Clip { start_ms: 500.0, duration_ms: 3_000.0, fill: ClipFill::HoldEnd,
        LowerThird {
            name: "Ada Lovelace".to_string(),
            role: "Mathematician".to_string(),
            accent: Some(LowerThirdAccent::Primary),
        }
    }
}"##;

const SCENE_CAPTION_SNIPPET: &str = r##"Scene {
    id: "caption-demo",
    duration_ms: 3_500.0,
    TimelineScope { id: "caption-timeline", autoplay: true,
        Caption {
            text: "Built with kinetics ui-blocks.".to_string(),
            reading_pace_ms_per_word: Some(320.0),
        }
    }
}"##;

const SCENE_WIPE_SNIPPET: &str = r##"Scene {
    id: "wipe-demo",
    duration_ms: 2_500.0,
    WipeTransition { duration_ms: 2_500.0, angle_deg: Some(120.0),
        /* gradient-filled backdrop */
    }
}"##;

const SCENE_METRIC_COUNTER_SNIPPET: &str = r##"Scene {
    id: "metric-counter-demo",
    duration_ms: 4_000.0,
    TimelineScope { id: "metric-counter-timeline", autoplay: true,
        Clip { start_ms: 200.0, duration_ms: 3_500.0, fill: ClipFill::HoldEnd,
            MetricCounter {
                label: "Active users".to_string(),
                value: "1,287".to_string(),
                delta_text: Some("+24% week over week".to_string()),
            }
        }
    }
}"##;

const SCENE_SOCIAL_OVERLAY_SNIPPET: &str = r##"Scene {
    id: "social-overlay-demo",
    duration_ms: 3_000.0,
    Clip { start_ms: 200.0, duration_ms: 2_500.0, fill: ClipFill::HoldEnd,
        SocialOverlay {
            platform: SocialPlatform::Instagram,
            handle: "@kineticsui".to_string(),
            message: "Just followed you!".to_string(),
        }
    }
}"##;

const SCENE_MANUAL_DRIVER_SNIPPET: &str = r##"Scene {
    id: "manual-driver-demo",
    duration_ms: 5_000.0,
    autoplay: Some(false),
    controls: Some(true),
    driver: Some(SceneDriver::Manual),
    Clip { start_ms: 0.0, duration_ms: 5_000.0, fill: ClipFill::HoldBoth,
        KineticText { id: "manual-driver-headline", text: "Drag the scrubber. No autoplay.".into(), cue: "fade-in" }
    }
    Clip { start_ms: 1_500.0, duration_ms: 3_500.0, fill: ClipFill::HoldEnd,
        KineticText { id: "manual-driver-body", text: "SceneDriver::Manual disables the rAF loop entirely.".into(), cue: "rise-in" }
    }
}"##;
