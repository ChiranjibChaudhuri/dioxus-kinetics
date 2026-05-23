use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};
use ui_tokens::Color;

#[test]
fn two_regions_both_render_without_overwriting() {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());

    let bg = make_solid(h.device(), h.queue(), 128, 128, [0, 0, 200, 255]);
    let out = make_output(h.device(), 128, 128);

    let r1 = GlassRegion::new(
        [8.0, 8.0, 48.0, 48.0],
        LiquidMaterial::floating().tint(Color::rgba(255, 0, 0, 1.0), 0.6),
    );
    let r2 = GlassRegion::new(
        [72.0, 72.0, 48.0, 48.0],
        LiquidMaterial::floating().tint(Color::rgba(0, 255, 0, 1.0), 0.6),
    );

    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [128.0, 128.0],
        &[r1, r2],
    );

    let pixels = read_back(h.device(), h.queue(), &out, 128, 128);
    let p1 = ((32 * 128 + 32) * 4) as usize;
    assert!(pixels[p1] > 50, "expected red tint in region 1 (center 32,32), got R={}", pixels[p1]);
    let p2 = ((96 * 128 + 96) * 4) as usize;
    assert!(pixels[p2 + 1] > 50, "expected green tint in region 2 (center 96,96), got G={}", pixels[p2 + 1]);
    let corner_idx = 0;
    assert_eq!(pixels[corner_idx + 3], 0, "corner alpha should be 0");
}

fn make_solid(device: &std::sync::Arc<wgpu::Device>, queue: &std::sync::Arc<wgpu::Queue>, w: u32, h: u32, rgba: [u8; 4]) -> wgpu::Texture {
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
        wgpu::TexelCopyTextureInfo { texture: &t, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        &pixels,
        wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h) },
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
fn read_back(device: &std::sync::Arc<wgpu::Device>, queue: &std::sync::Arc<wgpu::Queue>, tex: &wgpu::Texture, w: u32, h: u32) -> Vec<u8> {
    let bpr = ((w * 4 + 255) / 256) * 256;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: (bpr * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo { texture: tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(h) },
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
