//! Public render entry point. Holds device/queue and exposes a single
//! `render()` call that does an end-to-end frame. Plan 1 creates pipelines
//! per render call; Plan 2 introduces the pipeline cache keyed by
//! `(GlassFeatures, BLUR_TAPS)`.

use std::sync::Arc;

use ui_glass::LiquidMaterial;

use crate::pipeline::ComposeKey;
use crate::render_graph::render_glass_to_texture;
use crate::uniforms::GlassUniforms;

#[derive(Clone, Copy, Debug)]
pub struct GlassRegion {
    pub rect_px: [f32; 4], // x, y, w, h
    pub material: LiquidMaterial,
}

pub struct Compositor {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl Compositor {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self { device, queue }
    }

    /// End-to-end render: bg → blur → compose → output.
    /// Plan 1 supports any number of regions; each is rendered in order with a
    /// fresh pipeline (no cache yet). Multi-region overlap with correct
    /// compositing lands in Plan 4 (background scene contract).
    pub fn render(
        &mut self,
        bg_view: &wgpu::TextureView,
        output_view: &wgpu::TextureView,
        canvas_size: [f32; 2],
        regions: &[GlassRegion],
    ) {
        debug_assert!(
            regions.len() <= 1,
            "Plan 1 multi-region renders overwrite each other; correct \
             overlap compositing lands in Plan 4",
        );
        for region in regions {
            let uniforms = GlassUniforms::from_material(
                &region.material,
                region.rect_px,
                canvas_size,
            );
            let key = ComposeKey { features: region.material.features };
            render_glass_to_texture(
                &self.device,
                &self.queue,
                bg_view,
                output_view,
                &uniforms,
                key,
            );
        }
    }
}
