//! GPU-aligned uniform layout for the glass shader. The struct mirrors
//! `compose.wgsl`'s `GlassUniforms` block. Always 16-byte aligned; vec2 fields
//! pad to 8, vec4 to 16. See the WGSL file for the binding contract.

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GlassUniforms {
    pub rect: [f32; 4], // x, y, w, h in canvas px
    pub tint: [f32; 4], // rgba

    pub canvas_size: [f32; 2], // px
    pub pointer: [f32; 2],     // -1..1 normalized to surface

    pub scroll_velocity: [f32; 2],
    pub light_dir: [f32; 2], // unit vector from light_angle_rad

    pub radius: f32,
    pub thickness: f32,
    pub blur_radius: f32,
    pub saturation: f32,

    pub refract_strength: f32,
    pub surface_curvature: f32,
    pub noise_frequency: f32,
    pub noise_seed: f32,

    pub dispersion_px: f32,
    pub light_intensity: f32,
    pub edge_falloff: f32,
    pub inner_shadow_px: f32,

    pub inner_shadow_alpha: f32,
    pub adapt_strength: f32,
    pub time_seconds: f32,
    /// Overall surface opacity in `[0,1]`. The compose shader emits
    /// premultiplied-alpha `vec4(color * surface_alpha, surface_alpha)` and the
    /// compose pipeline blends with `PREMULTIPLIED_ALPHA_BLENDING`, so a value
    /// below `1.0` lets the glass composite translucently over the page behind
    /// the canvas. Defaults to `1.0` (fully opaque — identical to the historical
    /// behavior); opt in via [`GlassUniforms::with_surface_alpha`]. This field
    /// occupies the former `_pad0` tail slot, so the 16-byte-aligned layout that
    /// `compose.wgsl`'s `GlassUniforms` block mirrors is unchanged.
    pub surface_alpha: f32,
}

impl Default for GlassUniforms {
    fn default() -> Self {
        Self {
            rect: [0.0; 4],
            tint: [1.0; 4],
            canvas_size: [1.0, 1.0],
            pointer: [0.0; 2],
            scroll_velocity: [0.0; 2],
            light_dir: [1.0, 0.0],
            radius: 0.0,
            thickness: 1.0,
            blur_radius: 0.0,
            saturation: 1.0,
            refract_strength: 0.0,
            surface_curvature: 0.0,
            noise_frequency: 1.0,
            noise_seed: 0.0,
            dispersion_px: 0.0,
            light_intensity: 0.0,
            edge_falloff: 0.0,
            inner_shadow_px: 0.0,
            inner_shadow_alpha: 0.0,
            adapt_strength: 0.0,
            time_seconds: 0.0,
            surface_alpha: 1.0,
        }
    }
}

impl GlassUniforms {
    pub fn with_pointer(mut self, pointer_norm: [f32; 2]) -> Self {
        self.pointer = pointer_norm;
        self
    }

    /// Sets the overall surface opacity (premultiplied alpha) used by the
    /// compose shader. `1.0` is fully opaque (default); lower values let the
    /// page behind the canvas show through the glass.
    pub fn with_surface_alpha(mut self, alpha: f32) -> Self {
        self.surface_alpha = if alpha.is_finite() {
            alpha.clamp(0.0, 1.0)
        } else {
            1.0
        };
        self
    }

    pub fn with_scroll_velocity(mut self, vel_norm: [f32; 2]) -> Self {
        self.scroll_velocity = vel_norm;
        self
    }

    pub fn with_time(mut self, seconds: f32) -> Self {
        self.time_seconds = seconds;
        self
    }

    pub fn from_material(
        material: &ui_glass::LiquidMaterial,
        rect_px: [f32; 4],
        canvas_size: [f32; 2],
    ) -> Self {
        Self {
            rect: rect_px,
            tint: [
                material.tint.r as f32 / 255.0,
                material.tint.g as f32 / 255.0,
                material.tint.b as f32 / 255.0,
                material.tint_alpha,
            ],
            canvas_size,
            pointer: [0.0; 2],
            scroll_velocity: [0.0; 2],
            light_dir: [
                material.light_angle_rad.cos(),
                material.light_angle_rad.sin(),
            ],
            radius: material.radius_px,
            thickness: material.thickness_px,
            blur_radius: material.blur_radius_px,
            saturation: material.saturation,
            refract_strength: material.refraction_strength,
            surface_curvature: material.surface_curvature,
            noise_frequency: material.noise_frequency,
            noise_seed: material.noise_seed,
            dispersion_px: material.dispersion_px,
            light_intensity: material.light_intensity,
            edge_falloff: material.edge_falloff_px,
            inner_shadow_px: material.inner_shadow_px,
            inner_shadow_alpha: material.inner_shadow_alpha,
            adapt_strength: material.adapt_to_background,
            time_seconds: 0.0,
            // Opaque by default to preserve the historical look; translucency is
            // opt-in via `with_surface_alpha`.
            surface_alpha: 1.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct BlurUniforms {
    pub canvas_size: [f32; 2],
    pub blur_radius_px: f32,
    pub _pad: f32,
}

impl BlurUniforms {
    pub fn new(canvas_size: [f32; 2], blur_radius_px: f32) -> Self {
        Self {
            canvas_size,
            blur_radius_px,
            _pad: 0.0,
        }
    }
}
