//! Public render entry point. Owns the wgpu device/queue and two pipeline
//! caches: one keyed by `ComposeKey` (compose pipelines), one keyed by
//! `BlurKey` (blur pipelines per direction × tap count).

use std::collections::HashMap;
use std::sync::Arc;

use ui_glass::LiquidMaterial;

use crate::pipeline::{
    build_blur_pipeline, build_compose_pipeline, BlurDirection, BlurKey, ComposeKey,
};
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
    compose_cache: HashMap<ComposeKey, wgpu::RenderPipeline>,
    blur_cache: HashMap<BlurKey, wgpu::RenderPipeline>,
}

impl Compositor {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            device,
            queue,
            compose_cache: HashMap::new(),
            blur_cache: HashMap::new(),
        }
    }

    pub fn pipeline_cache_len(&self) -> usize {
        self.compose_cache.len() + self.blur_cache.len()
    }

    fn ensure_compose(&mut self, key: ComposeKey) -> &wgpu::RenderPipeline {
        self.compose_cache
            .entry(key)
            .or_insert_with(|| build_compose_pipeline(&self.device, key))
    }

    fn ensure_blur(&mut self, key: BlurKey) -> &wgpu::RenderPipeline {
        self.blur_cache
            .entry(key)
            .or_insert_with(|| build_blur_pipeline(&self.device, key.direction, key.taps))
    }

    pub fn render(
        &mut self,
        bg_view: &wgpu::TextureView,
        output_view: &wgpu::TextureView,
        canvas_size: [f32; 2],
        regions: &[GlassRegion],
    ) {
        debug_assert!(
            regions.len() <= 1,
            "Plan 1/2 multi-region renders overwrite each other; correct \
             overlap compositing lands in Plan 3",
        );
        for region in regions {
            let uniforms = GlassUniforms::from_material(
                &region.material,
                region.rect_px,
                canvas_size,
            );
            let compose_key = ComposeKey { features: region.material.features };
            let blur_h_key = BlurKey { direction: BlurDirection::Horizontal, taps: 13 };
            let blur_v_key = BlurKey { direction: BlurDirection::Vertical, taps: 13 };

            // Materialize pipelines up front so the immutable borrow in
            // render_glass_to_texture doesn't conflict with the mutable cache.
            let _ = self.ensure_compose(compose_key);
            let _ = self.ensure_blur(blur_h_key);
            let _ = self.ensure_blur(blur_v_key);

            let compose = self.compose_cache.get(&compose_key).unwrap();
            let blur_h = self.blur_cache.get(&blur_h_key).unwrap();
            let blur_v = self.blur_cache.get(&blur_v_key).unwrap();

            render_glass_to_texture(
                &self.device,
                &self.queue,
                bg_view,
                output_view,
                &uniforms,
                blur_h,
                blur_v,
                compose,
            );
        }
    }
}
