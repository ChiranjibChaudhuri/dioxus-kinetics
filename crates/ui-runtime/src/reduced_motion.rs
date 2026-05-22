//! Reduced-motion context. The application root provides a `ReducedMotion`
//! context value; hooks consume it to decide whether to skip animation.

use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReducedMotion(pub bool);

pub fn use_reduced_motion() -> bool {
    try_consume_context::<ReducedMotion>()
        .map(|rm| rm.0)
        .unwrap_or(false)
}
