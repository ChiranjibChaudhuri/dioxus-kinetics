//! Shared helpers for golden-image tests. Renders a glass region over a
//! gradient background and compares against a checked-in PNG within tolerance.

#![allow(dead_code)]

use std::path::PathBuf;

use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};

const TOLERANCE: u8 = 4;

pub fn render_with_material(w: u32, h: u32, material: LiquidMaterial) -> Vec<u8> {
    let harness = pollster::block_on(TestHarness::new()).unwrap();
    let bg = create_gradient(harness.device(), harness.queue(), w, h);
    let out = create_output(harness.device(), w, h);

    let mut comp = Compositor::new(harness.device().clone(), harness.queue().clone());
    let inset = (w.min(h) / 5) as f32;
    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [w as f32, h as f32],
        &[GlassRegion::new(
            [inset, inset, w as f32 - inset * 2.0, h as f32 - inset * 2.0],
            material,
        )],
    );
    read_back(harness.device(), harness.queue(), &out, w, h)
}

pub fn prepare_pointer_scene(
    w: u32, h: u32,
    material: LiquidMaterial,
    pointer_norm: [f32; 2],
) -> Vec<u8> {
    let harness = pollster::block_on(TestHarness::new()).unwrap();
    let bg = create_gradient(harness.device(), harness.queue(), w, h);
    let out = create_output(harness.device(), w, h);
    let inset = (w.min(h) / 5) as f32;
    let rect = [inset, inset, w as f32 - inset * 2.0, h as f32 - inset * 2.0];

    let mut comp = Compositor::new(harness.device().clone(), harness.queue().clone());
    let uniforms = ui_glass_engine::GlassUniforms::from_material(&material, rect, [w as f32, h as f32])
        .with_pointer(pointer_norm);
    comp.render_with_uniforms(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [w as f32, h as f32],
        rect,
        &material,
        uniforms,
    );
    read_back(harness.device(), harness.queue(), &out, w, h)
}

pub fn prepare_scroll_scene(
    w: u32, h: u32,
    material: LiquidMaterial,
    scroll_vel: [f32; 2],
) -> Vec<u8> {
    let harness = pollster::block_on(TestHarness::new()).unwrap();
    let bg = create_gradient(harness.device(), harness.queue(), w, h);
    let out = create_output(harness.device(), w, h);
    let inset = (w.min(h) / 5) as f32;
    let rect = [inset, inset, w as f32 - inset * 2.0, h as f32 - inset * 2.0];

    let mut comp = Compositor::new(harness.device().clone(), harness.queue().clone());
    let uniforms = ui_glass_engine::GlassUniforms::from_material(&material, rect, [w as f32, h as f32])
        .with_scroll_velocity(scroll_vel);
    comp.render_with_uniforms(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [w as f32, h as f32],
        rect,
        &material,
        uniforms,
    );
    read_back(harness.device(), harness.queue(), &out, w, h)
}

pub fn golden_check(golden_rel_path: &str, pixels: &[u8], w: u32, h: u32) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(golden_rel_path);
    if std::env::var("UPDATE_GOLDEN").is_ok() || !path.exists() {
        write_png(&path, pixels, w, h);
        if std::env::var("UPDATE_GOLDEN").is_err() {
            panic!(
                "golden missing at {}; wrote new reference. Re-run to verify.",
                path.display()
            );
        }
        return;
    }
    let expected = read_png(&path);
    assert_eq!(expected.len(), pixels.len(), "size mismatch for {golden_rel_path}");
    let mut diffs = 0usize;
    let mut worst = 0u8;
    for (a, b) in expected.iter().zip(pixels.iter()) {
        let d = a.abs_diff(*b);
        if d > TOLERANCE { diffs += 1; }
        if d > worst { worst = d; }
    }
    let max_allowed = pixels.len() / 200;
    assert!(
        diffs <= max_allowed,
        "{golden_rel_path}: {diffs} subpixels exceeded tolerance {TOLERANCE} (worst {worst}); \
         set UPDATE_GOLDEN=1 to refresh",
    );
}

fn create_gradient(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    w: u32, h: u32,
) -> wgpu::Texture {
    let t = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("gradient-bg"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let mut px = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let r = (x * 255 / w.max(1)) as u8;
            let g = ((x + y) * 255 / (w + h).max(1)) as u8;
            let b = (y * 255 / h.max(1)) as u8;
            px.extend_from_slice(&[r, g, b, 255]);
        }
    }
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &t, mip_level: 0, origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &px,
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
            texture: tex, mip_level: 0, origin: wgpu::Origin3d::ZERO,
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

fn write_png(path: &std::path::Path, pixels: &[u8], w: u32, h: u32) {
    let img = image::RgbaImage::from_raw(w, h, pixels.to_vec())
        .expect("pixel buffer size mismatch");
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    img.save(path).expect("png write");
}

fn read_png(path: &std::path::Path) -> Vec<u8> {
    let img = image::open(path).expect("png open").to_rgba8();
    img.into_raw()
}
