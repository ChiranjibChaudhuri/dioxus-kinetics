use std::collections::HashMap;

use dioxus::prelude::*;
use ui_motion::{Ease, Transition};
use ui_runtime::{
    use_animation_target, use_presence_animation, use_timeline_sample, AnimatedProperty,
    PresenceState,
};
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
// with a `cues` map (kinetic-id → `(MotionCue, start_ms)`) that the `Sequence`
// parent populates when it is given a `cues` list.  `KineticBox` reads that map
// and drives a WAAPI animation via `use_animation_target` with the cue's own
// `(from, to, transition)` and `start_ms` as the delay.
// ---------------------------------------------------------------------------
mod kinetic_animation {
    use ui_motion::Transition;
    use ui_runtime::waapi::AnimatedProperty;
    use ui_timeline::{Axis, MotionCue};

    /// Maps a `MotionCue` to the `(property, from, to, transition)` tuple
    /// required by `use_animation_target`. Returns `None` for cue variants
    /// that don't map to a single WAAPI property.
    ///
    /// `MotionCue::Path` fans out to translate_x + translate_y (and optionally
    /// rotate_deg) simultaneously, which doesn't fit the single-axis
    /// `AnimatedProperty` surface used by the WAAPI bridge. Path cues are
    /// driven through the `SequenceAdapter` inline-style path instead, so we
    /// return `None` here and let the sample-driven `style` attribute do the
    /// work.
    pub(super) fn pick_for_cue(cue: MotionCue) -> Option<(AnimatedProperty, f32, f32, Transition)> {
        match cue {
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
            // Path cues emit multi-axis samples (translate_x + translate_y,
            // optionally rotate_deg) and are handled via the inline-style
            // sample path rather than WAAPI.
            MotionCue::Path { .. } => None,
        }
    }
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
    /// Original `(MotionCue, start_ms)` per kinetic-id, keyed the same way as
    /// `states`. Populated by `Sequence` when it is given a `cues` list so that
    /// `KineticBox` can reconstruct `(from, to, transition)` and pass `start_ms`
    /// as the WAAPI delay via `use_animation_target::with_delay`.
    pub cues: HashMap<String, (MotionCue, f32)>,
}

fn cue_duration_ms(motion: &MotionCue) -> f32 {
    let transition = match motion {
        MotionCue::Opacity { transition, .. } => transition,
        MotionCue::Translate { transition, .. } => transition,
        MotionCue::Scale { transition, .. } => transition,
        MotionCue::Rotate { transition, .. } => transition,
        MotionCue::Path { transition, .. } => transition,
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
    #[props(default)] clock: Option<TimelineClock>,
    children: Element,
) -> Element {
    // Resolve effective clock with this precedence:
    //   1. Explicit `clock` prop wins (tests + advanced callers).
    //   2. Surrounding SceneContext.elapsed_ms when present.
    //   3. Default to TimelineClock::Playback { elapsed_ms: 0.0 }.
    let resolved_clock = match clock {
        Some(c) => c,
        None => match try_consume_context::<crate::scene_player::SceneContext>() {
            Some(scene_ctx) => TimelineClock::Manual {
                elapsed_ms: *scene_ctx.clock.elapsed_ms.read(),
            },
            None => TimelineClock::Playback { elapsed_ms: 0.0 },
        },
    };

    // Build a flat map of kinetic-id → (MotionCue, start_ms) from the `cues`
    // list.  This is stored in the context so that KineticBox can reconstruct
    // the (from, to, transition) triplet and use start_ms as the WAAPI delay.
    let cue_map: HashMap<String, (MotionCue, f32)> = cues
        .as_deref()
        .map(|list| {
            list.iter()
                .map(|c| (c.target_id.clone(), (c.motion.clone(), c.start_ms)))
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

    let sample = use_timeline_sample(timeline_value, resolved_clock);

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
pub fn TimelineScope(
    id: String,
    #[props(default)] autoplay: bool,
    #[props(default = 80.0)] stagger_step_ms: f32,
    children: Element,
) -> Element {
    let timeline_id = TimelineId::new(id);
    let step_ms = if stagger_step_ms.is_finite() && stagger_step_ms > 0.0 {
        stagger_step_ms
    } else {
        80.0
    };

    // Subscribe to the outer Scene's elapsed_ms Signal so this scope
    // re-renders whenever the Scene clock ticks. Without this, the
    // StaggerCursor consumers (KineticText, KineticBox, SplitText) stay
    // cached because Dioxus doesn't propagate the dirty bit through
    // intermediate context-provider parents that don't themselves read
    // the upstream Signal.
    if let Some(scene) = try_consume_context::<crate::scene_player::SceneContext>() {
        let _subscribe = *scene.clock.elapsed_ms.read();
    }

    // Provide a fresh StaggerCursor per render so kinetic leaves
    // inside the scope each grab a sequential index. SSR is single-
    // threaded so the Rc<Cell<u32>> counter is safe.
    //
    // `use_context_provider` only runs its initializer on the FIRST
    // render. To make the cursor restart at 0 on every render, we
    // call `.reset()` after retrieving the cached value — the
    // closure builds a new cursor on first render, and every
    // subsequent render reuses the same Rc<Cell<u32>> with its
    // counter reset to 0 before children render.
    let cursor = use_context_provider(|| crate::stagger::StaggerCursor::new(step_ms));
    cursor.reset();

    rsx! {
        section {
            class: "ui-timeline-scope",
            "data-timeline-id": "{timeline_id.0}",
            "data-autoplay": if autoplay { "true" } else { "false" },
            "data-stagger-step-ms": "{step_ms}",
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

    let ctx = try_consume_context::<Signal<SequenceContext>>();
    let state = ctx
        .as_ref()
        .and_then(|sig| sig.read().states.get(&kinetic_id.0).cloned());

    // Existing Sequence path: if there's a SequenceContext sample, use
    // its inline style (transform/opacity from the sample) AND drive
    // WAAPI as before (handled further down via use_animation_target).
    let sequence_inline = state.as_ref().map(|s| s.inline_style()).unwrap_or_default();

    // New cue-keyframe path: only when Sequence didn't provide a sample.
    // Priority: StaggerCursor + SceneContext → SceneContext only → none.
    let cue_inline = if sequence_inline.is_empty() {
        let stagger = try_consume_context::<crate::stagger::StaggerCursor>();
        let scene = try_consume_context::<crate::scene_player::SceneContext>();
        match (stagger, scene) {
            (Some(cursor), Some(ctx_scene)) => {
                let parent = *ctx_scene.clock.elapsed_ms.read();
                let index = cursor.next_index();
                let offset = index as f32 * cursor.step_ms;
                let local = (parent - offset).max(0.0);
                crate::cue_style::cue_inline_style(&cue, local)
            }
            (None, Some(ctx_scene)) => {
                let elapsed = *ctx_scene.clock.elapsed_ms.read();
                crate::cue_style::cue_inline_style(&cue, elapsed)
            }
            _ => String::new(),
        }
    } else {
        String::new()
    };

    let style = if !sequence_inline.is_empty() {
        sequence_inline
    } else {
        cue_inline
    };

    // Pull the original (MotionCue, start_ms) from the SequenceContext.
    // KineticBox children outside a Sequence have no cue → no WAAPI.
    // `MotionCue` is no longer `Copy` (since the `Path` variant holds a
    // `Vec<PathPoint>`), so we clone out of the context map and keep a
    // separate bool for the post-consume `has_cue` check below.
    let cue_data: Option<(MotionCue, f32)> = ctx
        .as_ref()
        .and_then(|sig| sig.read().cues.get(&kinetic_id.0).cloned());
    let has_cue = cue_data.is_some();

    // Resolve the cue to (property, from, to, transition).  When there is no
    // cue (KineticBox outside a Sequence) we fall back to a no-op opacity
    // 1.0→1.0 so the hook is always called unconditionally (Dioxus hook rule).
    let (property, from, to, transition, start_ms) = cue_data
        .and_then(|(mc, sms)| {
            kinetic_animation::pick_for_cue(mc).map(|(p, f, t, tr)| (p, f, t, tr, sms))
        })
        .unwrap_or((
            AnimatedProperty::Opacity,
            1.0,
            1.0,
            ui_motion::Transition::Tween {
                duration_ms: 0,
                ease: ui_motion::Ease::Standard,
            },
            0.0,
        ));

    // `use_animation_target` MUST be called unconditionally every render.
    // The returned handle carries the delay and is consumed in `onmounted`.
    let (target_handle, _value) = use_animation_target(property, from, to, transition);
    let target_handle = target_handle.with_delay(start_ms);

    // Only animate when there was real cue data; the no-op fallback produces a
    // zero-duration opacity 1→1 which is invisible — but we still skip the
    // play_on call to avoid unnecessary DOM work.
    let onmounted = {
        #[cfg(target_arch = "wasm32")]
        {
            EventHandler::new(move |evt: MountedEvent| {
                if !has_cue {
                    return;
                }
                if let Some(element) = evt.downcast::<web_sys::Element>() {
                    target_handle.play_on(element, from);
                }
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (target_handle, has_cue);
            EventHandler::new(|_: MountedEvent| {})
        }
    };

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

    // Effective elapsed_ms is the maximum-priority context available:
    //   1. StaggerCursor + SceneContext (positional stagger inside
    //      TimelineScope) → subtract `index * step_ms` from the
    //      surrounding Scene clock.
    //   2. SceneContext only → use the Scene's elapsed_ms directly.
    //   3. None → static markup (no animation style emitted).
    let stagger = try_consume_context::<crate::stagger::StaggerCursor>();
    let scene = try_consume_context::<crate::scene_player::SceneContext>();

    let inline_style: String = match (stagger, scene) {
        (Some(cursor), Some(ctx)) => {
            let parent = *ctx.clock.elapsed_ms.read();
            let index = cursor.next_index();
            let offset = index as f32 * cursor.step_ms;
            let local = (parent - offset).max(0.0);
            crate::cue_style::cue_inline_style(&cue, local)
        }
        (None, Some(ctx)) => {
            let elapsed = *ctx.clock.elapsed_ms.read();
            crate::cue_style::cue_inline_style(&cue, elapsed)
        }
        _ => String::new(),
    };

    rsx! {
        span {
            class: "ui-kinetic-text",
            "data-kinetic-id": "{kinetic_id.0}",
            "data-motion-cue": "{cue}",
            aria_label: "{text}",
            style: "{inline_style}",
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
