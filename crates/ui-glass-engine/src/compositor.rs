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
    noise_view: wgpu::TextureView,
    noise_sampler: wgpu::Sampler,
    mipmap_pipeline: wgpu::RenderPipeline,
}

impl Compositor {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        let (noise_view, noise_sampler) = create_noise_resources(&device, &queue);
        let mipmap_pipeline = crate::pipeline::build_mipmap_pipeline(&device);
        Self {
            device,
            queue,
            compose_cache: HashMap::new(),
            blur_cache: HashMap::new(),
            noise_view,
            noise_sampler,
            mipmap_pipeline,
        }
    }

    pub fn pipeline_cache_len(&self) -> usize {
        self.compose_cache.len() + self.blur_cache.len()
    }

    pub fn noise_view(&self) -> &wgpu::TextureView { &self.noise_view }
    pub fn noise_sampler(&self) -> &wgpu::Sampler { &self.noise_sampler }

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

    /// Test-only entry point: render a single region while overriding the
    /// uniform values. Used by golden tests for POINTER / SCROLL / time-driven
    /// features before Plan 3's ui-motion bridge replaces this path.
    #[cfg(any(test, feature = "headless"))]
    pub fn render_with_uniforms(
        &mut self,
        bg_view: &wgpu::TextureView,
        output_view: &wgpu::TextureView,
        canvas_size: [f32; 2],
        rect_px: [f32; 4],
        material: &LiquidMaterial,
        uniforms_override: GlassUniforms,
    ) {
        let compose_key = ComposeKey { features: material.features };
        let blur_h_key = BlurKey { direction: BlurDirection::Horizontal, taps: 13 };
        let blur_v_key = BlurKey { direction: BlurDirection::Vertical, taps: 13 };
        let _ = self.ensure_compose(compose_key);
        let _ = self.ensure_blur(blur_h_key);
        let _ = self.ensure_blur(blur_v_key);
        let mipped = if material.features.contains(ui_glass::GlassFeatures::TINT_ADAPT) {
            Some(self.materialize_mipped_bg(bg_view, canvas_size))
        } else { None };
        let effective_bg_view: &wgpu::TextureView = mipped
            .as_ref()
            .map(|(_, view)| view)
            .unwrap_or(bg_view);

        let compose = self.compose_cache.get(&compose_key).unwrap();
        let blur_h = self.blur_cache.get(&blur_h_key).unwrap();
        let blur_v = self.blur_cache.get(&blur_v_key).unwrap();
        let mut uniforms = uniforms_override;
        uniforms.rect = rect_px;
        uniforms.canvas_size = canvas_size;
        render_glass_to_texture(
            &self.device, &self.queue, effective_bg_view, output_view, &uniforms,
            blur_h, blur_v, compose,
            &self.noise_view, &self.noise_sampler,
        );
    }

    /// Build a mipmapped copy of `src_view` and return owners + a view of all
    /// levels. Caller must keep the returned `Texture` alive for the duration
    /// of any pass that samples the view.
    fn materialize_mipped_bg(
        &self,
        src_view: &wgpu::TextureView,
        canvas_size: [f32; 2],
    ) -> (wgpu::Texture, wgpu::TextureView) {
        let w = canvas_size[0] as u32;
        let h = canvas_size[1] as u32;
        let levels = ((w.max(h) as f32).log2().floor() as u32 + 1).max(1);
        let scratch = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("mipped-bg-scratch"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: levels,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("mip-sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let mipmap_bgl = crate::pipeline::mipmap_bind_group_layout(&self.device);
        let mut encoder = self.device.create_command_encoder(&Default::default());

        // Pass 0: blit src_view → scratch level 0.
        let level0_view = scratch.create_view(&wgpu::TextureViewDescriptor {
            base_mip_level: 0, mip_level_count: Some(1),
            ..Default::default()
        });
        let blit_bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("mip-blit-bg"),
            layout: &mipmap_bgl,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(src_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
            ],
        });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("mip-blit"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &level0_view, resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None, occlusion_query_set: None,
            });
            pass.set_pipeline(&self.mipmap_pipeline);
            pass.set_bind_group(0, &blit_bg, &[]);
            pass.draw(0..3, 0..1);
        }

        // Passes 1..levels: each samples mip n-1 and writes mip n.
        for level in 1..levels {
            let src_level_view = scratch.create_view(&wgpu::TextureViewDescriptor {
                base_mip_level: level - 1, mip_level_count: Some(1),
                ..Default::default()
            });
            let dst_level_view = scratch.create_view(&wgpu::TextureViewDescriptor {
                base_mip_level: level, mip_level_count: Some(1),
                ..Default::default()
            });
            let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("mip-bg"),
                layout: &mipmap_bgl,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&src_level_view) },
                    wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
                ],
            });
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("mip-gen"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &dst_level_view, resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None, occlusion_query_set: None,
            });
            pass.set_pipeline(&self.mipmap_pipeline);
            pass.set_bind_group(0, &bg, &[]);
            pass.draw(0..3, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));

        let full_view = scratch.create_view(&Default::default());
        (scratch, full_view)
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

            let mipped = if region.material.features.contains(ui_glass::GlassFeatures::TINT_ADAPT) {
                Some(self.materialize_mipped_bg(bg_view, canvas_size))
            } else { None };
            let effective_bg_view: &wgpu::TextureView = mipped
                .as_ref()
                .map(|(_, view)| view)
                .unwrap_or(bg_view);

            let compose = self.compose_cache.get(&compose_key).unwrap();
            let blur_h = self.blur_cache.get(&blur_h_key).unwrap();
            let blur_v = self.blur_cache.get(&blur_v_key).unwrap();

            render_glass_to_texture(
                &self.device,
                &self.queue,
                effective_bg_view,
                output_view,
                &uniforms,
                blur_h,
                blur_v,
                compose,
                &self.noise_view,
                &self.noise_sampler,
            );
        }
    }
}

fn create_noise_resources(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> (wgpu::TextureView, wgpu::Sampler) {
    let (w, h) = (256u32, 256u32);
    let pixels = crate::noise::generate_noise_rgba(w, h, 0xDEADBEEF);

    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("liquid-glass-noise"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(w * 4),
            rows_per_image: Some(h),
        },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );

    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("liquid-glass-noise-sampler"),
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    (view, sampler)
}
