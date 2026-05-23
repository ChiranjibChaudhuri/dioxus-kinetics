#![forbid(unsafe_code)]

//! Headless wgpu render engine for Liquid Glass surfaces.
//!
//! See `docs/superpowers/specs/2026-05-22-liquid-glass-engine-design.md` for
//! the design that drives this crate. Plan 1 covers the engine scaffold,
//! pipeline cache, and minimal shader (blur + SDF + tint).

pub mod compositor;
pub mod pipeline;
pub mod render_graph;
pub mod uniforms;

#[cfg(feature = "headless")]
pub mod headless;

pub use compositor::{Compositor, GlassRegion};
pub use uniforms::{BlurUniforms, GlassUniforms};
pub use render_graph::render_glass_to_texture;
