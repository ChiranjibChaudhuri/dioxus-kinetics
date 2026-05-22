use dioxus::prelude::*;
use kinetics::prelude::*;

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
