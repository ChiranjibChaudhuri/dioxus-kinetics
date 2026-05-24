//! Behavioral assertions: each Tier 1 feature should observably change the
//! pixels it touches. These tests don't compare to a golden — they compare
//! "feature on" to "feature off" and assert that *some* fraction of pixels
//! differ. This guards against a feature regressing into a no-op.
//!
//! The background is a 16x16 checkerboard — even a 1-pixel sample shift
//! produces large diffs across cell boundaries, so the threshold can stay at
//! 2% without false negatives for features that produce small geometric
//! displacements.

mod common;

use ui_glass::{AmbientMesh, LiquidMaterial};
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};

fn diff_count(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b.iter())
        .filter(|(x, y)| x.abs_diff(**y) > 1)
        .count()
}

const W: u32 = 96;
const H: u32 = 96;
const MIN_AFFECTED_FRACTION: f64 = 0.02;

fn base() -> LiquidMaterial {
    LiquidMaterial::new()
        .blur(2.0) // small blur so refractive shifts still register
        .radius(20.0)
        .tint(ui_tokens::Color::rgba(255, 255, 255, 1.0), 0.05)
}

fn render_with_checkerboard(w: u32, h: u32, material: LiquidMaterial) -> Vec<u8> {
    let harness = pollster::block_on(TestHarness::new()).unwrap();
    let bg = create_checkerboard(harness.device(), harness.queue(), w, h);
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

fn create_checkerboard(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    w: u32,
    h: u32,
) -> wgpu::Texture {
    let t = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("checker-bg"),
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
    let cell = 8u32;
    let mut px = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let on = ((x / cell) + (y / cell)).is_multiple_of(2);
            let v = if on { 250 } else { 30 };
            px.extend_from_slice(&[v, v, v, 255]);
        }
    }
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &t,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &px,
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

fn create_output(device: &std::sync::Arc<wgpu::Device>, w: u32, h: u32) -> wgpu::Texture {
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

fn read_back(
    device: &std::sync::Arc<wgpu::Device>,
    queue: &std::sync::Arc<wgpu::Queue>,
    tex: &wgpu::Texture,
    w: u32,
    h: u32,
) -> Vec<u8> {
    let bpr = (w * 4).div_ceil(256) * 256;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("readback"),
        size: (bpr * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bpr),
                rows_per_image: Some(h),
            },
        },
        wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(enc.finish()));
    let slice = buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| {
        tx.send(r).unwrap();
    });
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

#[test]
fn refract_changes_output() {
    let off = render_with_checkerboard(W, H, base());
    let on = render_with_checkerboard(
        W,
        H,
        base()
            .refract(1.0)
            .noise(4.0, 0.0)
            .surface_curvature(0.8)
            .thickness(3.0),
    );
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(
        frac > MIN_AFFECTED_FRACTION,
        "REFRACT changed only {:.2}% of pixels",
        frac * 100.0
    );
}

#[test]
fn disperse_changes_output() {
    let off = render_with_checkerboard(W, H, base());
    let on = render_with_checkerboard(W, H, base().disperse(6.0));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(
        frac > MIN_AFFECTED_FRACTION,
        "DISPERSE changed only {:.2}% of pixels",
        frac * 100.0
    );
}

#[test]
fn specular_changes_output() {
    let off = render_with_checkerboard(W, H, base());
    let on = render_with_checkerboard(
        W,
        H,
        base()
            .specular(45.0_f32.to_radians(), 0.8)
            .edge_falloff(20.0),
    );
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(
        frac > MIN_AFFECTED_FRACTION,
        "SPECULAR changed only {:.2}% of pixels",
        frac * 100.0
    );
}

#[test]
fn inner_shadow_changes_output() {
    let off = render_with_checkerboard(W, H, base());
    let on = render_with_checkerboard(W, H, base().inner_shadow(8.0, 0.5));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(
        frac > MIN_AFFECTED_FRACTION,
        "INNER_SHADOW changed only {:.2}% of pixels",
        frac * 100.0
    );
}

#[test]
fn ambient_mesh_changes_output() {
    let off = render_with_checkerboard(W, H, base());
    let on = render_with_checkerboard(W, H, base().ambient_mesh(AmbientMesh::Aurora));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(
        frac > MIN_AFFECTED_FRACTION,
        "AMBIENT_MESH changed only {:.2}% of pixels",
        frac * 100.0
    );
}

#[test]
fn tint_adapt_changes_output() {
    let off = render_with_checkerboard(W, H, base());
    let on = render_with_checkerboard(W, H, base().adapt_to_background(0.5));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(
        frac > MIN_AFFECTED_FRACTION,
        "TINT_ADAPT changed only {:.2}% of pixels",
        frac * 100.0
    );
}
