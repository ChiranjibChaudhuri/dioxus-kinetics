//! Pure-function presence lifecycle. No async, no Dioxus, fully testable.

const SETTLE_EPSILON: f32 = 0.001;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceState {
    Entering,
    Visible,
    Exiting,
    Unmounted,
}

impl PresenceState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entering => "entering",
            Self::Visible => "visible",
            Self::Exiting => "exiting",
            Self::Unmounted => "unmounted",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PresenceInputs {
    pub present: bool,
    pub value: f32,
    pub prev_state: Option<PresenceState>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PresenceTransition {
    pub state: PresenceState,
    pub target: f32,
}

pub fn advance_presence(inputs: PresenceInputs) -> PresenceTransition {
    let PresenceInputs {
        present,
        value,
        prev_state,
    } = inputs;

    match (present, prev_state) {
        (true, None) => PresenceTransition {
            state: PresenceState::Entering,
            target: 1.0,
        },
        (false, None) => PresenceTransition {
            state: PresenceState::Unmounted,
            target: 0.0,
        },
        (true, Some(PresenceState::Entering)) => {
            if (1.0 - value).abs() <= SETTLE_EPSILON {
                PresenceTransition {
                    state: PresenceState::Visible,
                    target: 1.0,
                }
            } else {
                PresenceTransition {
                    state: PresenceState::Entering,
                    target: 1.0,
                }
            }
        }
        (true, Some(PresenceState::Visible)) => PresenceTransition {
            state: PresenceState::Visible,
            target: 1.0,
        },
        (true, Some(PresenceState::Exiting | PresenceState::Unmounted)) => PresenceTransition {
            state: PresenceState::Entering,
            target: 1.0,
        },
        (false, Some(PresenceState::Visible | PresenceState::Entering)) => PresenceTransition {
            state: PresenceState::Exiting,
            target: 0.0,
        },
        (false, Some(PresenceState::Exiting)) => {
            if value.abs() <= SETTLE_EPSILON {
                PresenceTransition {
                    state: PresenceState::Unmounted,
                    target: 0.0,
                }
            } else {
                PresenceTransition {
                    state: PresenceState::Exiting,
                    target: 0.0,
                }
            }
        }
        (false, Some(PresenceState::Unmounted)) => PresenceTransition {
            state: PresenceState::Unmounted,
            target: 0.0,
        },
    }
}
