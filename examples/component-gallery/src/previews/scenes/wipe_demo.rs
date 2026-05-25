use dioxus::prelude::*;
use kinetics::prelude::*;
use ui_blocks::WipeTransition;

#[component]
pub fn WipeDemoScene() -> Element {
    rsx! {
        Scene {
            id: "wipe-demo",
            width: 1280,
            height: 720,
            duration_ms: 2_500.0,
            autoplay: Some(true),
            controls: Some(true),
            WipeTransition { duration_ms: 2_500.0, angle_deg: Some(120.0),
                div { class: "scene-wipe-fill",
                    style: "background: linear-gradient(120deg, #a04bfa, #4bbafa);",
                    h2 { style: "padding: 80px;", "Cinematic wipes ship in ui-blocks." }
                }
            }
        }
    }
}
