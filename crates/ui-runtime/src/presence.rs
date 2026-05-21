//! Presence state hook.

use dioxus::prelude::*;
use ui_motion::Transition;

use crate::animation::use_animation_value;
use crate::state::{advance_presence, PresenceInputs, PresenceState};

pub fn use_presence_state(
    present: bool,
    enter: Transition,
    exit: Transition,
) -> ReadSignal<PresenceState> {
    let mut state = use_signal(|| {
        if present {
            PresenceState::Entering
        } else {
            PresenceState::Unmounted
        }
    });

    let active_transition = if present { enter } else { exit };
    let value = use_animation_value(if present { 1.0 } else { 0.0 }, active_transition);

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

    // Synchronous SSR/reduced-motion resolution: if Entering and value
    // is already at target (1.0), settle to Visible immediately.
    let snapshot = state();
    if snapshot == PresenceState::Entering && (value() - 1.0).abs() <= 0.001 {
        state.set(PresenceState::Visible);
    }

    ReadSignal::from(state)
}
