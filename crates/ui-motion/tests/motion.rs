use motion_core::{Ease, PresenceState, Spring, Transition};
use ui_motion as motion_core;

#[test]
fn reduced_motion_collapses_transition_duration() {
    let transition = Transition::tween(180).reduced();

    assert_eq!(transition.duration_ms(), 0);
}

#[test]
fn spring_step_moves_toward_target() {
    let spring = Spring::snappy();
    let value = spring.step(0.0, 10.0, 0.0, 1.0 / 60.0).value;

    assert!(value > 0.0);
    assert!(value < 10.0);
}

#[test]
fn presence_state_keeps_exit_lifecycle_explicit() {
    assert_eq!(
        PresenceState::Present.request_exit(),
        PresenceState::Exiting
    );
    assert_eq!(PresenceState::Exiting.finish_exit(), PresenceState::Removed);
}

#[test]
fn invalid_spring_values_do_not_produce_non_finite_output() {
    let spring = Spring {
        stiffness: f32::NAN,
        damping: f32::NEG_INFINITY,
        mass: 0.0,
    };

    let step = spring.step(f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::INFINITY);

    assert!(step.value.is_finite());
    assert!(step.velocity.is_finite());
}

#[test]
fn reduced_spring_becomes_zero_duration_linear_tween() {
    let transition = Transition::spring(Spring::snappy()).reduced();

    assert_eq!(
        transition,
        Transition::Tween {
            duration_ms: 0,
            ease: Ease::Linear,
        }
    );
}

#[test]
fn presence_state_no_op_transitions_are_idempotent() {
    assert_eq!(
        PresenceState::Removed.request_exit(),
        PresenceState::Removed
    );
    assert_eq!(PresenceState::Present.finish_exit(), PresenceState::Present);
    assert_eq!(PresenceState::Removed.finish_exit(), PresenceState::Removed);
}

#[test]
fn fixed_duration_ms_distinguishes_tween_from_spring() {
    assert_eq!(Transition::tween(180).fixed_duration_ms(), Some(180));
    assert_eq!(
        Transition::spring(Spring::snappy()).fixed_duration_ms(),
        None
    );
}
