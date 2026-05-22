use std::collections::HashMap;

use dioxus::prelude::*;
use ui_motion::{Ease, Transition};
use ui_runtime::{use_animation_value, use_presence_state, use_timeline_sample, PresenceState};
use ui_timeline::{
    FillMode, KineticId, MotionCue, MotionSegment, MotionTarget, ResolvedMotionState, Timeline,
    TimelineClock, TimelineId, TimelineTrack,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Cue {
    pub target_id: String,
    pub start_ms: f32,
    pub motion: MotionCue,
}

impl Cue {
    pub fn new(target_id: impl Into<String>, start_ms: f32, motion: MotionCue) -> Self {
        Self {
            target_id: target_id.into(),
            start_ms,
            motion,
        }
    }
}

#[derive(Clone, Default)]
pub struct SequenceContext {
    pub states: HashMap<String, ResolvedMotionState>,
}

fn cue_duration_ms(motion: &MotionCue) -> f32 {
    let transition = match motion {
        MotionCue::Opacity { transition, .. } => transition,
        MotionCue::Translate { transition, .. } => transition,
        MotionCue::Scale { transition, .. } => transition,
        MotionCue::Rotate { transition, .. } => transition,
    };
    transition.estimated_duration_ms()
}

fn cues_to_timeline(id: &str, cues: Vec<Cue>) -> Timeline {
    let mut max_end = 0.0_f32;
    let mut timeline = Timeline::new(id, 0.0);
    for cue in cues {
        let duration_ms = cue_duration_ms(&cue.motion);
        let end = cue.start_ms + duration_ms;
        if end > max_end {
            max_end = end;
        }
        let track = TimelineTrack::new(
            MotionTarget::node(cue.target_id.clone()),
            vec![MotionSegment::new(cue.start_ms, duration_ms, cue.motion)],
        );
        timeline = timeline.with_track(track);
    }
    Timeline {
        duration_ms: max_end,
        fill: FillMode::Forwards,
        ..timeline
    }
}

#[component]
pub fn Sequence(
    #[props(default)] timeline: Option<Timeline>,
    #[props(default)] cues: Option<Vec<Cue>>,
    #[props(default = "sequence".to_string())] id: String,
    #[props(default = TimelineClock::Playback { elapsed_ms: 0.0 })] clock: TimelineClock,
    children: Element,
) -> Element {
    let resolved_timeline = timeline
        .clone()
        .or_else(|| cues.clone().map(|cues| cues_to_timeline(&id, cues)));

    let Some(timeline_value) = resolved_timeline else {
        return rsx! {
            section {
                class: "ui-sequence",
                "data-timeline-id": "{id}",
                {children}
            }
        };
    };

    let sample = use_timeline_sample(timeline_value, clock);

    // Seed the context exactly once with the initial sample so SSR renders the
    // settled inline styles for each kinetic id. `use_hook` runs only on the
    // first render, avoiding a per-render race with the effect below.
    let mut ctx_signal = use_hook(|| {
        let snapshot = sample();
        let mut states = HashMap::new();
        for state in snapshot.states {
            if let MotionTarget::Node(kinetic_id) = &state.target {
                states.insert(kinetic_id.0.clone(), state.clone());
            }
        }
        Signal::new(SequenceContext { states })
    });
    use_context_provider(|| ctx_signal);

    use_effect(move || {
        let snapshot = sample();
        let mut states = HashMap::new();
        for state in snapshot.states {
            if let MotionTarget::Node(kinetic_id) = &state.target {
                states.insert(kinetic_id.0.clone(), state.clone());
            }
        }
        ctx_signal.set(SequenceContext { states });
    });

    rsx! {
        section {
            class: "ui-sequence",
            "data-timeline-id": "{id}",
            {children}
        }
    }
}

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
    let kinetic_id = KineticId::new(id.clone());
    let style = try_consume_context::<Signal<SequenceContext>>()
        .and_then(|sig| sig.read().states.get(&kinetic_id.0).cloned())
        .map(|state| state.inline_style())
        .unwrap_or_default();

    rsx! {
        div {
            class: "ui-kinetic-box",
            "data-kinetic-id": "{kinetic_id.0}",
            "data-motion-cue": "{cue}",
            style: "{style}",
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
