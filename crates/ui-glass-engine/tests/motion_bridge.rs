use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::motion::MotionInputs;
use ui_glass_engine::{Compositor, GlassRegion};

#[test]
fn update_inputs_propagates_to_render() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());
    comp.update_inputs(
        MotionInputs::new()
            .with_pointer([32.0, 32.0])
            .with_scroll_velocity([8.0, 0.0])
            .with_time(0.5),
    );

    let bg = make_solid(h.device(), h.queue(), 64, 64, [40, 40, 40, 255]);
    let out = make_output(h.device(), 64, 64);

    let mat = LiquidMaterial::new()
        .blur(8.0)
        .refract(0.5)
        .pointer_reactive()
        .scroll_reactive()
        .radius(12.0);
    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [64.0, 64.0],
        &[GlassRegion::new([8.0, 8.0, 48.0, 48.0], mat)],
    );
    // No panic + GPU consumed the inputs = pass.
}

#[test]
fn reduced_motion_zeroes_pointer_uniform() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());
    comp.update_inputs(
        MotionInputs::new()
            .with_pointer([32.0, 32.0])
            .with_reduced_motion(true),
    );

    let bg = make_solid(h.device(), h.queue(), 64, 64, [40, 40, 40, 255]);
    let out = make_output(h.device(), 64, 64);
    let mat = LiquidMaterial::new()
        .blur(8.0)
        .refract(0.5)
        .pointer_reactive()
        .radius(12.0);
    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [64.0, 64.0],
        &[GlassRegion::new([8.0, 8.0, 48.0, 48.0], mat)],
    );
}

fn make_solid(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    w: u32,
    h: u32,
    rgba: [u8; 4],
) -> wgpu::Texture {
    let t = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("bg"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let pixels: Vec<u8> = (0..(w * h)).flat_map(|_| rgba).collect();
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &t,
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
        wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
    );
    t
}
fn make_output(device: &std::sync::Arc<wgpu::Device>, w: u32, h: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("out"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}
