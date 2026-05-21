use dioxus::prelude::*;

#[component]
pub fn TimelineScope(id: String, #[props(default)] autoplay: bool, children: Element) -> Element {
    rsx! {
        section {
            class: "ui-timeline-scope",
            "data-timeline-id": "{id}",
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
    rsx! {
        div {
            class: "ui-kinetic-box",
            "data-kinetic-id": "{id}",
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
    rsx! {
        span {
            class: "ui-kinetic-text",
            "data-kinetic-id": "{id}",
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
