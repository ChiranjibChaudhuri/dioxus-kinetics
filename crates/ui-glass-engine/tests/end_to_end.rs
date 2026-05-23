use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};

#[test]
fn floating_preset_over_blue_bg_writes_corner_pixels_outside_rect() {
    let mut h = pollster::block_on(TestHarness::new()).unwrap();
    let (w, hgt) = (128u32, 128u32);

    let bg = create_solid(h.device(), h.queue(), w, hgt, [0, 0, 200, 255]);
    let out = create_output(h.device(), w, hgt);

    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());
    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [w as f32, hgt as f32],
        &[GlassRegion {
            rect_px: [32.0, 32.0, 64.0, 64.0],
            material: LiquidMaterial::floating().tint(
                ui_tokens::Color::rgba(255, 255, 255, 1.0),
                0.4,
            ),
        }],
    );

    let pixels = read_back(h.device(), h.queue(), &out, w, hgt);

    // Pixel inside the rect (center of canvas) should be a tinted blue —
    // the blue background mixed with white tint at 40%.
    let center_idx = ((hgt / 2) * w + (w / 2)) as usize * 4;
    let r = pixels[center_idx];
    let g = pixels[center_idx + 1];
    let b = pixels[center_idx + 2];
    assert!(r > 90, "expected white-mixed red channel above 90, got {r}");
    assert!(g > 90, "expected white-mixed green channel above 90, got {g}");
    assert!(b < 255, "expected mixed blue, got pure white at center");

    // Pixel far outside the rect (top-left corner) should remain transparent
    // (compositor clears to TRANSPARENT before compose pass).
    let corner_idx = 0;
    assert_eq!(pixels[corner_idx + 3], 0, "corner alpha should be 0");
}

fn create_solid(
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
            texture: &t, mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
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

fn create_output(device: &std::sync::Arc<wgpu::Device>, w: u32, h: u32) -> wgpu::Texture {
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

fn read_back(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    tex: &wgpu::Texture, w: u32, h: u32,
) -> Vec<u8> {
    let bpr = ((w * 4 + 255) / 256) * 256;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: (bpr * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex, mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(h),
            },
        },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    queue.submit(Some(enc.finish()));
    let slice = buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| { tx.send(r).unwrap(); });
    let _ = device.poll(wgpu::PollType::Wait);
    rx.recv().unwrap().unwrap();
    let data = slice.get_mapped_range();
    let mut out = Vec::with_capacity((w * h * 4) as usize);
    for row in 0..h {
        let start = (row * bpr) as usize;
        out.extend_from_slice(&data[start..start + (w * 4) as usize]);
    }
    drop(data);
    buf.unmap();
    out
}
