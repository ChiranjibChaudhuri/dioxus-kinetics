#![cfg(not(target_arch = "wasm32"))]

//! Non-wasm stub for the WAAPI binding. Hooks that consume WAAPI fall
//! back to the legacy RAF path on these targets; this stub only needs
//! to provide types so the rest of the crate compiles.

use ui_motion::Keyframes;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimatedProperty {
    Opacity,
    TranslateX,
    TranslateY,
    Scale,
    Rotate,
    PresenceT,
}

pub fn is_supported() -> bool {
    false
}

pub struct WaapiAnimation;

impl WaapiAnimation {
    pub fn pause(&self) {}
    pub fn cancel(&self) {}
    pub fn set_current_time(&self, _ms: f32) {}
}

#[allow(dead_code)]
pub fn keyframes_to_js(_prop: AnimatedProperty, _keyframes: &Keyframes) {}

#[allow(dead_code)]
pub fn options_object(_duration_ms: f32) {}
