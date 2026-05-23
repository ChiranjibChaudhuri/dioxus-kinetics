//! Public render entry point. Filled in across Tasks 9–12.

use ui_glass::LiquidMaterial;

#[derive(Clone, Copy, Debug)]
pub struct GlassRegion {
    pub rect_px: [f32; 4], // x, y, w, h
    pub material: LiquidMaterial,
}

pub struct Compositor;
