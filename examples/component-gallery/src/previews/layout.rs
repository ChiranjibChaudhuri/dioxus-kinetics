use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn accordion_preview() -> Element {
    rsx! { AccordionPreviewBody {} }
}

#[component]
fn AccordionPreviewBody() -> Element {
    let mut expanded = use_signal(|| vec!["billing".to_string()]);
    let sections = vec![
        AccordionSection::new(
            "billing",
            "Billing details",
            "Your card ending in 4242 renews on the 1st of each month.",
        ),
        AccordionSection::new(
            "members",
            "Team members",
            "Invite collaborators by email or share a link.",
        ),
        AccordionSection::new(
            "danger",
            "Danger zone",
            "Archive the workspace or transfer ownership.",
        ),
    ];
    rsx! {
        Accordion {
            sections,
            expanded: expanded.read().clone(),
            on_toggle: move |id: String| {
                let mut next = expanded.read().clone();
                if let Some(pos) = next.iter().position(|x| x == &id) {
                    next.remove(pos);
                } else {
                    next.push(id);
                }
                expanded.set(next);
            },
        }
    }
}

pub fn stack_preview() -> Element {
    rsx! {
        Stack { gap: "sm".to_string(),
            Button { "Create workspace" }
            Button { variant: ButtonVariant::Secondary, "Import data" }
        }
    }
}

pub fn tabs_preview() -> Element {
    rsx! { TabsPreviewBody {} }
}

#[component]
fn TabsPreviewBody() -> Element {
    let mut selected = use_signal(|| "billing".to_string());
    rsx! {
        Tabs {
            selected: selected.read().clone(),
            items: vec![
                TabItem::new("overview", "Overview"),
                TabItem::new("billing", "Billing"),
                TabItem::new("usage", "Usage"),
            ],
            panels: vec![
                TabPanel::new("overview", "Account summary"),
                TabPanel::new("billing", "Payment method active"),
                TabPanel::new("usage", "92% of monthly quota used"),
            ],
            onselect: move |next: String| selected.set(next),
        }
    }
}
