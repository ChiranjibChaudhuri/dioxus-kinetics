#![forbid(unsafe_code)]

//! Dioxus integration for the Liquid Glass engine.
//!
//! Provides `<LiquidSurface>` — a component that mounts a wgpu-rendered
//! glass surface backed by `ui-glass-engine::Compositor`. On web/desktop/mobile
//! (WebView) targets, the component initializes wgpu against an HTML canvas
//! and drives per-frame rendering via the `ui-runtime` scheduler. On native
//! (non-wasm32) targets the component compiles but renders a placeholder
//! — Blitz/native integration is deferred.

pub mod component;
pub mod motion_bridge;
pub mod surface_state;

#[cfg(target_arch = "wasm32")]
pub mod web;

#[cfg(not(target_arch = "wasm32"))]
pub mod stub;

pub use component::{LiquidSurface, LiquidSurfaceProps};
pub use surface_state::GlassPower;
