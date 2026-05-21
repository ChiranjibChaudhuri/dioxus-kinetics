use dioxus::prelude::*;
use ui_timeline::{KineticId, TimelineId};

#[component]
pub fn TimelineScope(id: String, #[props(default)] autoplay: bool, children: Element) -> Element {
    let timeline_id = TimelineId::new(id);

    rsx! {
        section {
            class: "ui-timeline-scope",
            "data-timeline-id": "{timeline_id.0}",
            "data-autoplay": if autoplay { "true" } else { "false" },
            {children}
        }
    }
}

#[component]
pub fn KineticBox(
    id: String,
    #[props(default = "fade-in".to_string())] cue: String,
    children: Element,
) -> Element {
    let kinetic_id = KineticId::new(id);

    rsx! {
        div {
            class: "ui-kinetic-box",
            "data-kinetic-id": "{kinetic_id.0}",
            "data-motion-cue": "{cue}",
            {children}
        }
    }
}

#[component]
pub fn KineticText(
    id: String,
    text: String,
    #[props(default = "text-flow".to_string())] cue: String,
) -> Element {
    let kinetic_id = KineticId::new(id);

    rsx! {
        span {
            class: "ui-kinetic-text",
            "data-kinetic-id": "{kinetic_id.0}",
            "data-motion-cue": "{cue}",
            aria_label: "{text}",
            "{text}"
        }
    }
}

#[component]
pub fn PresenceGate(#[props(default = true)] present: bool, children: Element) -> Element {
    if !present {
        return rsx! {};
    }

    rsx! {
        div {
            class: "ui-presence-gate",
            "data-presence": "present",
            {children}
        }
    }
}
