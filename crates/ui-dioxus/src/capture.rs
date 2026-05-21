use dioxus::prelude::*;

#[component]
pub fn CaptureStage(id: String, viewport: String, frame: u32, children: Element) -> Element {
    rsx! {
        section {
            class: "ui-capture-stage",
            "data-capture-id": "{id}",
            "data-viewport": "{viewport}",
            "data-frame": "{frame}",
            {children}
        }
    }
}
