#![forbid(unsafe_code)]

//! Reusable cinematic block catalog for kinetics Scene compositions.

mod caption;
mod lower_third;

pub use caption::Caption;
pub use lower_third::{LowerThird, LowerThirdAccent};
