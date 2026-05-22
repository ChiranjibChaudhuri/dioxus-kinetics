//! Animation value hook.

use dioxus::prelude::*;
use ui_motion::Transition;

use crate::reduced_motion::use_reduced_motion;

/// Animates a Rust value toward `target` over `transition` time.
///
/// MVP: returns the target synchronously. SSR and reduced-motion paths
/// render the settled state. Live per-frame ticking is reserved for a
/// follow-up enhancement and does not change this hook's signature.
pub fn use_animation_value(target: f32, transition: Transition) -> ReadSignal<f32> {
    let _ = transition;
    let reduced = use_reduced_motion();
    let _ = reduced; // reserved for future use; currently snap to target either way
    let value = use_signal(|| target);
    ReadSignal::from(value)
}
