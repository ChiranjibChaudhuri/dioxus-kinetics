use dioxus::prelude::*;
use ui_capture::CaptureStageDescriptor;

#[component]
pub fn CaptureStage(id: String, viewport: String, frame: u32, children: Element) -> Element {
    let stage = CaptureStageDescriptor::new(id, "");

    rsx! {
        section {
            class: "ui-capture-stage",
            "data-capture-id": "{stage.id}",
            "data-viewport": "{viewport}",
            "data-frame": "{frame}",
            {children}
        }
    }
}
