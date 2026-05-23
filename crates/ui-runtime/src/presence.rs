//! Presence state hook.

use dioxus::prelude::*;
use ui_motion::Transition;

use crate::animation::use_animation_value_from;
use crate::state::{advance_presence, PresenceInputs, PresenceState};

/// Returns the lifecycle state for a `present` flag plus the underlying
/// animated `t` value (`0.0` hidden, `1.0` visible). Callers that need both
/// — for example to drive a `--ui-presence-t` CSS variable — should use this
/// hook and reuse the value rather than running a second [`use_animation_value`]
/// in parallel.
pub fn use_presence_animation(
    present: bool,
    enter: Transition,
    exit: Transition,
) -> (ReadSignal<PresenceState>, ReadSignal<f32>) {
    let reduced = crate::reduced_motion::use_reduced_motion();

    let mut state = use_signal(|| {
        if present {
            if reduced { PresenceState::Visible } else { PresenceState::Entering }
        } else {
            PresenceState::Unmounted
        }
    });

    let active_transition = if present { enter } else { exit };
    let (initial, target) = if present { (0.0, 1.0) } else { (1.0, 0.0) };
    let value = use_animation_value_from(initial, target, active_transition);

    use_effect(move || {
        let snapshot = state();
        let next = advance_presence(PresenceInputs {
            present,
            value: value(),
            prev_state: Some(snapshot),
        });
        if next.state != snapshot {
            state.set(next.state);
        }
    });

    let snapshot = state();
    if snapshot == PresenceState::Entering && (value() - 1.0).abs() <= 0.001 {
        state.set(PresenceState::Visible);
    }
    if snapshot == PresenceState::Exiting && value().abs() <= 0.001 {
        state.set(PresenceState::Unmounted);
    }

    (ReadSignal::from(state), value)
}

pub fn use_presence_state(
    present: bool,
    enter: Transition,
    exit: Transition,
) -> ReadSignal<PresenceState> {
    use_presence_animation(present, enter, exit).0
}
