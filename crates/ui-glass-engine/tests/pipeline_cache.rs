use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};

#[test]
fn compositor_reuses_pipeline_for_same_features() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());

    let bg = make_solid(h.device(), h.queue(), 64, 64, [0, 0, 200, 255]);
    let out = make_output(h.device(), 64, 64);
    let bg_view = bg.create_view(&Default::default());
    let out_view = out.create_view(&Default::default());

    let region = GlassRegion {
        rect_px: [8.0, 8.0, 48.0, 48.0],
        material: LiquidMaterial::new().blur(6.0).radius(8.0),
    };

    comp.render(&bg_view, &out_view, [64.0, 64.0], &[region]);
    let after_first = comp.pipeline_cache_len();
    comp.render(&bg_view, &out_view, [64.0, 64.0], &[region]);
    let after_second = comp.pipeline_cache_len();

    assert_eq!(after_first, after_second, "second render must reuse pipelines");
    assert!(after_first >= 3, "expected at least blur-h + blur-v + compose cached, got {after_first}");
}

#[test]
fn compositor_caches_distinct_pipelines_per_feature_mask() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());

    let bg = make_solid(h.device(), h.queue(), 64, 64, [0, 0, 200, 255]);
    let out = make_output(h.device(), 64, 64);
    let bg_view = bg.create_view(&Default::default());
    let out_view = out.create_view(&Default::default());

    comp.render(&bg_view, &out_view, [64.0, 64.0], &[GlassRegion {
        rect_px: [8.0, 8.0, 48.0, 48.0],
        material: LiquidMaterial::new().blur(6.0),
    }]);
    let after_blur_only = comp.pipeline_cache_len();

    comp.render(&bg_view, &out_view, [64.0, 64.0], &[GlassRegion {
        rect_px: [8.0, 8.0, 48.0, 48.0],
        material: LiquidMaterial::new().blur(6.0).specular(0.78, 0.6),
    }]);
    let after_blur_plus_specular = comp.pipeline_cache_len();

    assert!(
        after_blur_plus_specular > after_blur_only,
        "new feature mask should add a compose pipeline (was {after_blur_only}, now {after_blur_plus_specular})",
    );
}

fn make_solid(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    w: u32, h: u32, rgba: [u8; 4],
) -> wgpu::Texture {
    let t = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("bg"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let pixels: Vec<u8> = (0..(w * h)).flat_map(|_| rgba).collect();
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &t, mip_level: 0, origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h),
        },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    t
}

fn make_output(device: &std::sync::Arc<wgpu::Device>, w: u32, h: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("out"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}
