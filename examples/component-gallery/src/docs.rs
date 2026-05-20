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

const COMPONENT_DOCS: [ComponentDoc; 14] = [
    ComponentDoc {
        name: "Button",
        category: ComponentCategory::Actions,
        status: ComponentStatus::Ready,
        summary: "Triggers a user action with semantic variants for primary, secondary, quiet, and destructive commands.",
        snippet: BUTTON_SNIPPET,
        render: Some(button_preview),
    },
    ComponentDoc {
        name: "IconButton",
        category: ComponentCategory::Actions,
        status: ComponentStatus::ComingSoon,
        summary: "A compact icon-only command control with an accessible label.",
        snippet: ICON_BUTTON_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "TextField",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::ComingSoon,
        summary: "A labeled single-line text input for forms and filters.",
        snippet: TEXT_FIELD_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Checkbox",
        category: ComponentCategory::Inputs,
        status: ComponentStatus::ComingSoon,
        summary: "A binary choice control for settings, filters, and table selection.",
        snippet: CHECKBOX_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Stack",
        category: ComponentCategory::Layout,
        status: ComponentStatus::Ready,
        summary: "Arranges children in a vertical rhythm with semantic spacing tokens.",
        snippet: STACK_SNIPPET,
        render: Some(stack_preview),
    },
    ComponentDoc {
        name: "Surface",
        category: ComponentCategory::Surfaces,
        status: ComponentStatus::Ready,
        summary: "Creates a solid content layer for panels, sections, and grouped SaaS workflows.",
        snippet: SURFACE_SNIPPET,
        render: Some(surface_preview),
    },
    ComponentDoc {
        name: "GlassSurface",
        category: ComponentCategory::Surfaces,
        status: ComponentStatus::Ready,
        summary: "Creates a translucent material layer with semantic level, tone, and density attributes.",
        snippet: GLASS_SURFACE_SNIPPET,
        render: Some(glass_surface_preview),
    },
    ComponentDoc {
        name: "Tabs",
        category: ComponentCategory::Layout,
        status: ComponentStatus::ComingSoon,
        summary: "Switches between related panels without leaving the current workflow.",
        snippet: TABS_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Dialog",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::ComingSoon,
        summary: "Presents blocking decisions and focused workflows above the page.",
        snippet: DIALOG_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Toast",
        category: ComponentCategory::Feedback,
        status: ComponentStatus::ComingSoon,
        summary: "Shows short-lived status feedback after background or user-triggered events.",
        snippet: TOAST_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Presence",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Coordinates mounted, exiting, and removed states for animated lifecycles.",
        snippet: PRESENCE_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "Sequence",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Orders small motion steps into predictable product transitions.",
        snippet: SEQUENCE_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "SharedLayout",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Coordinates layout continuity across related regions.",
        snippet: SHARED_LAYOUT_SNIPPET,
        render: None,
    },
    ComponentDoc {
        name: "SharedElement",
        category: ComponentCategory::Motion,
        status: ComponentStatus::ComingSoon,
        summary: "Marks an element that can visually continue between layout states.",
        snippet: SHARED_ELEMENT_SNIPPET,
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

const TEXT_FIELD_SNIPPET: &str = r#"TextField {
    label: "Workspace name",
    value: workspace_name,
}"#;

const CHECKBOX_SNIPPET: &str = r#"Checkbox {
    label: "Send weekly summary",
    checked: true,
}"#;

const STACK_SNIPPET: &str = r#"Stack {
    gap: "sm".to_string(),
    Button { "Create" }
    Button {
        variant: ButtonVariant::Secondary,
        "Cancel"
    }
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

const TABS_SNIPPET: &str = r#"Tabs {
    value: "overview",
    items: tabs,
}"#;

const DIALOG_SNIPPET: &str = r#"Dialog {
    title: "Delete workspace",
    open: true,
}"#;

const TOAST_SNIPPET: &str = r#"Toast {
    tone: ToastTone::Success,
    "Report exported"
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

fn stack_preview() -> Element {
    rsx! {
        Stack { gap: "sm".to_string(),
            Button { "Create workspace" }
            Button { variant: ButtonVariant::Secondary, "Import data" }
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
