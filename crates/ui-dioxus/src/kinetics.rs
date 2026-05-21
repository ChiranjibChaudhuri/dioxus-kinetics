use dioxus::prelude::*;
use ui_motion::{Ease, Transition};
use ui_runtime::{use_animation_value, use_presence_state, PresenceState};
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PresenceCue {
    #[default]
    Fade,
    Rise,
    Slide,
    Scale,
}

impl PresenceCue {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fade => "fade",
            Self::Rise => "rise",
            Self::Slide => "slide",
            Self::Scale => "scale",
        }
    }
}

const DEFAULT_ENTER: Transition = Transition::Tween {
    duration_ms: 220,
    ease: Ease::Standard,
};

const DEFAULT_EXIT: Transition = Transition::Tween {
    duration_ms: 180,
    ease: Ease::Standard,
};

#[component]
pub fn Presence(
    present: bool,
    #[props(default = DEFAULT_ENTER)] enter: Transition,
    #[props(default = DEFAULT_EXIT)] exit: Transition,
    #[props(default)] cue: PresenceCue,
    children: Element,
) -> Element {
    let state = use_presence_state(present, enter, exit);
    let value = use_animation_value(
        if present { 1.0 } else { 0.0 },
        if present { enter } else { exit },
    );

    if state() == PresenceState::Unmounted {
        return rsx! {};
    }

    let state_str = state().as_str();
    let cue_str = cue.as_str();
    let v = value();

    rsx! {
        div {
            class: "ui-presence",
            "data-presence-cue": "{cue_str}",
            "data-presence-state": "{state_str}",
            style: "--ui-presence-t: {v};",
            {children}
        }
    }
}
