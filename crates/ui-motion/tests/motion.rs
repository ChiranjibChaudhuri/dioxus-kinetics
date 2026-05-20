use ui_motion::{PresenceState, Spring, Transition};

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
    assert_eq!(PresenceState::Present.request_exit(), PresenceState::Exiting);
    assert_eq!(PresenceState::Exiting.finish_exit(), PresenceState::Removed);
}
