#![forbid(unsafe_code)]

//! Animation runtime for the kinetics UI library.

pub mod state;

pub use state::{advance_presence, PresenceInputs, PresenceState, PresenceTransition};
