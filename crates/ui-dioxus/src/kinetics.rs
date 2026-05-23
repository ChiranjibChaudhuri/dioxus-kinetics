use std::collections::HashMap;

use dioxus::prelude::*;
use ui_motion::{Ease, Transition};
use ui_runtime::{use_presence_animation, use_timeline_sample, PresenceState};
use ui_timeline::{
    FillMode, KineticId, MotionCue, MotionSegment, MotionTarget, ResolvedMotionState, Timeline,
    TimelineClock, TimelineId, TimelineTrack,
};

// ---------------------------------------------------------------------------
// WAAPI integration — wasm32 only.
//
// `ResolvedMotionState` is sample-only (it stores the sampled scalar values,
// not the original `MotionCue` with `from / to / transition`). To keep the
// original cue data accessible at mount time, `SequenceContext` is extended
// with a `cues` map (kinetic-id → `MotionCue`) that the `Sequence` parent
// populates when it is given a `cues` list.  `KineticBox` reads that map on
// mount and drives a WAAPI animation from the cue's own `from/to/transition`.
//
// Non-wasm builds compile a no-op stub so every call-site in `KineticBox`
// below is unconditional.
// ---------------------------------------------------------------------------
#[cfg(target_arch = "wasm32")]
mod kinetics_waapi {
    use ui_motion::keyframes_for_transition;
    use ui_runtime::waapi::{
        is_supported, keyframes_to_js, options_object, AnimatedProperty, WaapiAnimation,
    };
    use ui_timeline::{Axis, MotionCue};
    use web_sys::Element;

    /// Called from the `onmounted` handler of a `KineticBox`.
    /// Plays the WAAPI animation that matches the cue originally registered for
    /// this kinetic id. Does nothing if WAAPI is unsupported or no cue is found.
    pub(super) fn play_cue_on_mount(element: &Element, cue: &MotionCue) {
        if !is_supported() {
            return;
        }
        let Some((property, from, to, transition)) = axis_for_cue(cue) else {
            return;
        };
        let keyframes = keyframes_for_transition(from, to, transition);
        let js_keyframes = keyframes_to_js(property, &keyframes);
        let js_options = options_object(keyframes.duration_ms, 0.0);
        // keyframes_to_js returns JsValue directly (T6 note) — pass as &js_keyframes.
        let _ = WaapiAnimation::play(element, &js_keyframes, &js_options);
    }

    fn axis_for_cue(
        cue: &MotionCue,
    ) -> Option<(AnimatedProperty, f32, f32, ui_motion::Transition)> {
        match *cue {
            MotionCue::Opacity {
                from,
                to,
                transition,
            } => Some((AnimatedProperty::Opacity, from, to, transition)),
            MotionCue::Translate {
                axis: Axis::X,
                from,
                to,
                transition,
            } => Some((AnimatedProperty::TranslateX, from, to, transition)),
            MotionCue::Translate {
                axis: Axis::Y,
                from,
                to,
                transition,
            } => Some((AnimatedProperty::TranslateY, from, to, transition)),
            MotionCue::Scale {
                from,
                to,
                transition,
            } => Some((AnimatedProperty::Scale, from, to, transition)),
            MotionCue::Rotate {
                from_deg,
                to_deg,
                transition,
            } => Some((AnimatedProperty::Rotate, from_deg, to_deg, transition)),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod kinetics_waapi {
    use ui_timeline::MotionCue;

    /// No-op on non-wasm targets.
    pub(super) fn play_cue_on_mount(_element: &(), _cue: &MotionCue) {}
}

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
    /// Original `MotionCue` per kinetic-id, keyed the same way as `states`.
    /// Populated by `Sequence` when it is given a `cues` list so that
    /// `KineticBox` can reconstruct `(from, to, transition)` for WAAPI.
    pub cues: HashMap<String, MotionCue>,
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
    // `Both` so that segments with a `start_ms > 0` emit their `from` value
    // before the segment starts (otherwise body/cta in a staggered sequence
    // render in their final state at t=0 and only the first cue is visible).
    // Forwards-fill still applies past the end, so the timeline settles to
    // its `to` value as before.
    Timeline {
        duration_ms: max_end,
        fill: FillMode::Both,
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
    // Build a flat map of kinetic-id → original MotionCue from the `cues` list.
    // This is stored in the context so that KineticBox can reconstruct the
    // (from, to, transition) triplet needed to drive a WAAPI animation on mount.
    let cue_map: HashMap<String, MotionCue> = cues
        .as_deref()
        .map(|list| {
            list.iter()
                .map(|c| (c.target_id.clone(), c.motion))
                .collect()
        })
        .unwrap_or_default();

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
        Signal::new(SequenceContext {
            states,
            cues: cue_map.clone(),
        })
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
        ctx_signal.set(SequenceContext {
            states,
            cues: cue_map.clone(),
        });
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

    // Snapshot from context: the sampled inline style (for SSR/hydration
    // pre-paint) and the original MotionCue (for WAAPI playback on mount).
    let ctx_snapshot = try_consume_context::<Signal<SequenceContext>>()
        .map(|sig| sig.read().clone());

    let style = ctx_snapshot
        .as_ref()
        .and_then(|ctx| ctx.states.get(&kinetic_id.0).cloned())
        .map(|state| state.inline_style())
        .unwrap_or_default();

    // Capture the original MotionCue so the onmounted handler can drive WAAPI.
    let motion_cue = ctx_snapshot
        .as_ref()
        .and_then(|ctx| ctx.cues.get(&kinetic_id.0).copied());

    // On wasm32: when the element mounts, kick off a WAAPI animation for the
    // cue registered in the parent Sequence. The inline style above provides
    // the correct settled-state for SSR/hydration; WAAPI takes over from the
    // `from` value and animates to the `to` value in the browser.
    //
    // On non-wasm: the handler is a no-op.
    let onmounted = EventHandler::new(move |evt: MountedEvent| {
        let Some(cue_data) = motion_cue else {
            return;
        };
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(element) = evt.downcast::<web_sys::Element>() {
                kinetics_waapi::play_cue_on_mount(element, &cue_data);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = evt;
            kinetics_waapi::play_cue_on_mount(&(), &cue_data);
        }
    });

    rsx! {
        div {
            class: "ui-kinetic-box",
            "data-kinetic-id": "{kinetic_id.0}",
            "data-motion-cue": "{cue}",
            style: "{style}",
            onmounted: onmounted,
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

/// A binary gate that synchronously removes its children when `present` is
/// false. Unlike [`Presence`], it does **not** keep the children mounted during
/// an exit animation — there is no exit transition. Use [`Presence`] when you
/// need an animated unmount; reach for `PresenceGate` only when the goal is a
/// keyed conditional render that should never animate out.
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
    let (state, value) = use_presence_animation(present, enter, exit);

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
