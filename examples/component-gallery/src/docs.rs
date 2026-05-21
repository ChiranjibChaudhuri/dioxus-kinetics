use dioxus::prelude::*;
use unified_ui::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentCategory {
    Actions,
    Inputs,
    Layout,
    Surfaces,
    Feedback,
    Motion,
}

impl ComponentCategory {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Actions => "Actions",
            Self::Inputs => "Inputs",
            Self::Layout => "Layout",
            Self::Surfaces => "Surfaces",
            Self::Feedback => "Feedback",
            Self::Motion => "Motion",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::Actions => "Command controls that trigger a product action.",
            Self::Inputs => "Controls that collect user-entered data.",
            Self::Layout => "Structure primitives for arranging interface regions.",
            Self::Surfaces => "Containers that define visual layers and material treatment.",
            Self::Feedback => "Overlays and messages that respond to user or system state.",
            Self::Motion => "Lifecycle and layout motion primitives for continuity.",
        }
    }

    pub const fn slug(self) -> &'static str {
        match self {
            Self::Actions => "actions",
            Self::Inputs => "inputs",
            Self::Layout => "layout",
            Self::Surfaces => "surfaces",
            Self::Feedback => "feedback",
            Self::Motion => "motion",
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
        ComponentCategory::Actions,
        ComponentCategory::Inputs,
        ComponentCategory::Layout,
        ComponentCategory::Surfaces,
        ComponentCategory::Feedback,
        ComponentCategory::Motion,
    ]
}

pub fn component_docs() -> &'static [ComponentDoc] {
    &COMPONENT_DOCS
}

const BASIC_ACCESSIBILITY: &str = "Renders native semantic elements and stable focusable controls.";

const COMPONENT_DOCS: [ComponentDoc; 21] = [
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
        status: ComponentStatus::ComingSoon,
        summary: "A compact icon-only command control with an accessible label.",
        snippet: ICON_BUTTON_SNIPPET,
        accessibility: "Reserved for a later icon system while keeping the gallery contract stable.",
        render: None,
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
        status: ComponentStatus::ComingSoon,
        summary: "Coordinates mounted, exiting, and removed states for animated lifecycles.",
        snippet: PRESENCE_SNIPPET,
        accessibility: "Coming soon; motion helpers will respect reduced-motion preferences.",
        render: None,
    },
    ComponentDoc {
        name: "Sequence",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Orders small motion steps into predictable product transitions.",
        snippet: SEQUENCE_SNIPPET,
        accessibility: "Coming soon; sequence helpers will preserve reduced-motion fallbacks.",
        render: None,
    },
    ComponentDoc {
        name: "SharedLayout",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Coordinates layout continuity across related regions.",
        snippet: SHARED_LAYOUT_SNIPPET,
        accessibility: "Coming soon; layout motion will keep DOM semantics stable.",
        render: None,
    },
    ComponentDoc {
        name: "SharedElement",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Marks an element that can visually continue between layout states.",
        snippet: SHARED_ELEMENT_SNIPPET,
        accessibility: "Coming soon; shared elements will avoid replacing meaningful content.",
        render: None,
    },
];

const BUTTON_SNIPPET: &str = r#"Button {
    variant: ButtonVariant::Primary,
    "Save changes"
}"#;

const ICON_BUTTON_SNIPPET: &str = r#"IconButton {
    label: "Archive",
    icon: ArchiveIcon,
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
    visible: is_open,
    Dialog { title: "Invite member" }
}"#;

const SEQUENCE_SNIPPET: &str = r#"Sequence {
    steps: onboarding_steps,
}"#;

const SHARED_LAYOUT_SNIPPET: &str = r#"SharedLayout {
    layout_id: "billing-plan",
    children
}"#;

const SHARED_ELEMENT_SNIPPET: &str = r#"SharedElement {
    element_id: "customer-row-42",
    children
}"#;

fn button_preview() -> Element {
    rsx! {
        div { class: "gallery-inline",
            Button { variant: ButtonVariant::Primary, "Save changes" }
            Button { variant: ButtonVariant::Secondary, "Review" }
            Button { variant: ButtonVariant::Ghost, "Dismiss" }
            Button { variant: ButtonVariant::Danger, "Delete" }
        }
    }
}

fn command_menu_preview() -> Element {
    rsx! {
        CommandMenu {
            open: true,
            query: "rep",
            selected_id: "reports",
            groups: vec![CommandGroup::new(
                "Navigation",
                vec![
                    CommandItem::new("dashboard", "Open dashboard", "Go to overview"),
                    CommandItem::new("reports", "Open reports", "Review exports"),
                ],
            )],
        }
    }
}

fn toolbar_preview() -> Element {
    rsx! {
        Toolbar {
            primary: vec!["New".to_string(), "Filter".to_string(), "Export".to_string()],
            secondary: "Updated now",
        }
    }
}

fn text_field_preview() -> Element {
    rsx! {
        TextField {
            id: "workspace-name",
            label: "Workspace name",
            value: "Acme Ops",
            help_text: "Visible to teammates",
            leading_text: "Org",
        }
    }
}

fn checkbox_preview() -> Element {
    rsx! {
        Checkbox {
            id: "weekly-summary",
            label: "Send weekly summary",
            checked: true,
            description: "Every Monday morning",
        }
    }
}

fn switch_preview() -> Element {
    rsx! {
        Switch {
            id: "auto-renew",
            label: "Auto renew",
            checked: true,
            description: "Keep billing active",
        }
    }
}

fn stack_preview() -> Element {
    rsx! {
        Stack { gap: "sm".to_string(),
            Button { "Create workspace" }
            Button { variant: ButtonVariant::Secondary, "Import data" }
        }
    }
}

fn tabs_preview() -> Element {
    rsx! {
        Tabs {
            selected: "billing",
            items: vec![
                TabItem::new("overview", "Overview"),
                TabItem::new("billing", "Billing"),
            ],
            panels: vec![
                TabPanel::new("overview", "Account summary"),
                TabPanel::new("billing", "Payment method active"),
            ],
        }
    }
}

fn sidebar_preview() -> Element {
    rsx! {
        Sidebar {
            selected: "settings",
            sections: vec![SidebarSection::new(
                "Workspace",
                vec![
                    SidebarItem::new("home", "Home", "#home"),
                    SidebarItem::new("settings", "Settings", "#settings"),
                ],
            )],
        }
    }
}

fn surface_preview() -> Element {
    rsx! {
        Surface {
            h4 { "Pipeline health" }
            p { "12 workflows running" }
        }
    }
}

fn glass_surface_preview() -> Element {
    rsx! {
        GlassSurface {
            level: GlassLevel::Floating,
            tone: GlassTone::Neutral,
            density: GlassDensity::Comfortable,
            h4 { "Revenue operations" }
            p { "Forecast updated 4 minutes ago" }
        }
    }
}

fn metric_card_preview() -> Element {
    rsx! {
        MetricCard {
            label: "Net revenue",
            value: "$128.4k",
            delta: "+12.5%",
            tone: MetricTone::Success,
        }
    }
}

fn dialog_preview() -> Element {
    rsx! {
        Dialog {
            open: true,
            title: "Archive workspace",
            description: "Move this workspace out of active navigation.",
            body: "Team members can still request access later.",
            actions: vec!["Cancel".to_string(), "Archive".to_string()],
        }
    }
}

fn toast_preview() -> Element {
    rsx! {
        Toast {
            tone: ToastTone::Success,
            title: "Report exported",
            description: "The PDF is ready.",
            action_label: "Open",
            dismiss_label: "Dismiss",
        }
    }
}

fn tooltip_preview() -> Element {
    rsx! {
        Tooltip {
            id: "net-revenue-tip",
            visible: true,
            trigger_label: "Net revenue",
            content: "Revenue after refunds and credits.",
        }
    }
}

fn empty_state_preview() -> Element {
    rsx! {
        EmptyState {
            title: "No reports yet",
            description: "Create a report to share performance with your team.",
            action_label: "Create report",
        }
    }
}
