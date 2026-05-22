use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn sidebar_preview() -> Element {
    rsx! { SidebarPreviewBody {} }
}

#[component]
fn SidebarPreviewBody() -> Element {
    let mut selected = use_signal(|| "settings".to_string());
    rsx! {
        Sidebar {
            selected: selected.read().clone(),
            sections: vec![SidebarSection::new(
                "Workspace",
                vec![
                    SidebarItem::new("home", "Home", "#home"),
                    SidebarItem::new("settings", "Settings", "#settings"),
                    SidebarItem::new("billing", "Billing", "#billing"),
                ],
            )],
            onnavigate: move |id: String| selected.set(id),
        }
    }
}
