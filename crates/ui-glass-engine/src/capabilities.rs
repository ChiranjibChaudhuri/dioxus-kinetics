//! Capability detection for the rendering tier ladder.
//!
//! Five tiers, top to bottom:
//!
//! - `Tier::WgpuWebGpu` — wgpu via WebGPU. All 9 features at full quality.
//! - `Tier::WgpuWebGl2` — wgpu via WebGL2. All 9 features, slightly reduced
//!   quality. Used as fallback when WebGPU is unavailable.
//! - `Tier::SvgFilter` — CSS `backdrop-filter: url(#kinetics-glass-...)` chain.
//!   No wgpu involved. Approximates the look with feGaussianBlur +
//!   feSpecularLighting + feColorMatrix + feDisplacementMap.
//! - `Tier::SolidCss` — solid surface via `--ui-glass-solid`. No filters.
//! - `Tier::Off` — engine disabled.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    WgpuWebGpu,
    WgpuWebGl2,
    SvgFilter,
    SolidCss,
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Capabilities {
    pub has_webgpu: bool,
    pub has_webgl2: bool,
    pub has_backdrop_filter: bool,
    pub reduced_motion: bool,
    pub reduced_transparency: bool,
    pub high_contrast: bool,
}

impl Capabilities {
    /// The most-capable tier this environment can support, modulo user
    /// preferences. High-contrast and reduced-transparency snap to SolidCss
    /// regardless of GPU availability.
    ///
    /// `reduced_motion` never selects a live GPU loop (`WgpuWebGpu` /
    /// `WgpuWebGl2`): the per-frame compositor render is an ongoing animation
    /// that the user has asked us to suppress. Instead it snaps down to the
    /// best *static* tier — `SvgFilter` (a non-animated CSS
    /// `backdrop-filter` chain) when backdrop-filter is available, otherwise
    /// `SolidCss`. Transparency is preserved where possible; only the live
    /// motion is dropped.
    pub fn best_tier(&self) -> Tier {
        if self.high_contrast || self.reduced_transparency {
            return Tier::SolidCss;
        }
        if self.reduced_motion {
            // Skip the wgpu tiers — they imply a continuous rAF render loop.
            // Fall straight to the static fallbacks.
            if self.has_backdrop_filter {
                return Tier::SvgFilter;
            }
            return Tier::SolidCss;
        }
        if self.has_webgpu {
            return Tier::WgpuWebGpu;
        }
        if self.has_webgl2 {
            return Tier::WgpuWebGl2;
        }
        if self.has_backdrop_filter {
            return Tier::SvgFilter;
        }
        Tier::SolidCss
    }
}

/// Detect runtime capabilities. On wasm32 this probes the browser; on native
/// it assumes WebGPU is available (wgpu always has a native backend).
#[cfg(target_arch = "wasm32")]
pub fn detect() -> Capabilities {
    use wasm_bindgen::JsCast;
    let window = web_sys::window();

    let has_webgpu = window
        .as_ref()
        .and_then(|w| js_sys::Reflect::get(w, &wasm_bindgen::JsValue::from_str("navigator")).ok())
        .and_then(|nav| js_sys::Reflect::get(&nav, &wasm_bindgen::JsValue::from_str("gpu")).ok())
        .map(|gpu| !gpu.is_undefined())
        .unwrap_or(false);

    // WebGL2 is essentially universal in 2026; check by trying to create a
    // throwaway context.
    let has_webgl2 = window
        .as_ref()
        .and_then(|w| w.document())
        .and_then(|d| d.create_element("canvas").ok())
        .and_then(|c| c.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        .and_then(|c| c.get_context("webgl2").ok().flatten())
        .is_some();

    // backdrop-filter — universal in modern browsers
    let has_backdrop_filter = true;

    let reduced_motion = matches_media_query(window.as_ref(), "(prefers-reduced-motion: reduce)");
    let reduced_transparency =
        matches_media_query(window.as_ref(), "(prefers-reduced-transparency: reduce)");
    let high_contrast = matches_media_query(window.as_ref(), "(prefers-contrast: more)");

    Capabilities {
        has_webgpu,
        has_webgl2,
        has_backdrop_filter,
        reduced_motion,
        reduced_transparency,
        high_contrast,
    }
}

#[cfg(target_arch = "wasm32")]
fn matches_media_query(window: Option<&web_sys::Window>, query: &str) -> bool {
    window
        .and_then(|w| w.match_media(query).ok().flatten())
        .map(|mql| mql.matches())
        .unwrap_or(false)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn detect() -> Capabilities {
    Capabilities {
        has_webgpu: true,
        has_webgl2: true,
        has_backdrop_filter: false,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    }
}

#[cfg(test)]
mod tests {
    use super::{Capabilities, Tier};

    #[test]
    fn reduced_motion_avoids_live_gpu_loop_uses_svg() {
        // Even with WebGPU available, reduced-motion must not select a
        // continuous render loop tier — it snaps to the static SvgFilter.
        let caps = Capabilities {
            has_webgpu: true,
            has_webgl2: true,
            has_backdrop_filter: true,
            reduced_motion: true,
            reduced_transparency: false,
            high_contrast: false,
        };
        assert_eq!(caps.best_tier(), Tier::SvgFilter);
    }

    #[test]
    fn reduced_motion_without_backdrop_filter_falls_to_solid() {
        let caps = Capabilities {
            has_webgpu: true,
            has_webgl2: true,
            has_backdrop_filter: false,
            reduced_motion: true,
            reduced_transparency: false,
            high_contrast: false,
        };
        assert_eq!(caps.best_tier(), Tier::SolidCss);
    }

    #[test]
    fn reduced_transparency_still_wins_over_reduced_motion() {
        // reduced_transparency forces SolidCss regardless of reduced_motion.
        let caps = Capabilities {
            has_webgpu: true,
            has_webgl2: true,
            has_backdrop_filter: true,
            reduced_motion: true,
            reduced_transparency: true,
            high_contrast: false,
        };
        assert_eq!(caps.best_tier(), Tier::SolidCss);
    }

    #[test]
    fn no_reduced_motion_still_prefers_webgpu() {
        let caps = Capabilities {
            has_webgpu: true,
            has_webgl2: true,
            has_backdrop_filter: true,
            reduced_motion: false,
            reduced_transparency: false,
            high_contrast: false,
        };
        assert_eq!(caps.best_tier(), Tier::WgpuWebGpu);
    }
}
