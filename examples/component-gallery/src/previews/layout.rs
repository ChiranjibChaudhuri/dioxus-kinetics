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
