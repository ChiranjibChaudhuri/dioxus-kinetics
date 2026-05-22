use ui_runtime::reduced_motion::ReducedMotion;
use ui_runtime::state::{advance_presence, PresenceInputs, PresenceState};

#[test]
fn initial_present_true_starts_entering() {
    let s = advance_presence(PresenceInputs {
        present: true,
        value: 0.0,
        prev_state: None,
    });
    assert_eq!(s.state, PresenceState::Entering);
    assert_eq!(s.target, 1.0);
}

#[test]
fn initial_present_false_is_unmounted() {
    let s = advance_presence(PresenceInputs {
        present: false,
        value: 0.0,
        prev_state: None,
    });
    assert_eq!(s.state, PresenceState::Unmounted);
    assert_eq!(s.target, 0.0);
}

#[test]
fn entering_settles_to_visible_when_value_near_one() {
    let s = advance_presence(PresenceInputs {
        present: true,
        value: 0.9995,
        prev_state: Some(PresenceState::Entering),
    });
    assert_eq!(s.state, PresenceState::Visible);
}

#[test]
fn visible_with_present_false_starts_exiting() {
    let s = advance_presence(PresenceInputs {
        present: false,
        value: 1.0,
        prev_state: Some(PresenceState::Visible),
    });
    assert_eq!(s.state, PresenceState::Exiting);
    assert_eq!(s.target, 0.0);
}

#[test]
fn exiting_settles_to_unmounted_when_value_near_zero() {
    let s = advance_presence(PresenceInputs {
        present: false,
        value: 0.0005,
        prev_state: Some(PresenceState::Exiting),
    });
    assert_eq!(s.state, PresenceState::Unmounted);
}

#[test]
fn exiting_interrupted_by_present_true_starts_entering() {
    let s = advance_presence(PresenceInputs {
        present: true,
        value: 0.4,
        prev_state: Some(PresenceState::Exiting),
    });
    assert_eq!(s.state, PresenceState::Entering);
    assert_eq!(s.target, 1.0);
}

#[test]
fn reduced_motion_struct_carries_flag() {
    let on = ReducedMotion(true);
    let off = ReducedMotion(false);
    assert!(on.0);
    assert!(!off.0);
}
