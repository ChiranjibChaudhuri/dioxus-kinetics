use dioxus::prelude::*;
use kinetics::prelude::*;

#[component]
pub fn CtaPulseScene() -> Element {
    rsx! {
        div { class: "scene-cta",
            Button { variant: ButtonVariant::Primary, "Start building" }
            span { class: "scene-cta-caption", "Free to try. No credit card." }
        }
    }
}
