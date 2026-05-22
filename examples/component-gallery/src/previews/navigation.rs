use dioxus::prelude::*;
use kinetics::prelude::*;

pub fn sidebar_preview() -> Element {
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
