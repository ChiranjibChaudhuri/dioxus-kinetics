//! Golden-image comparison. The first run writes the golden file (when env var
//! `UPDATE_GOLDEN=1` is set); subsequent runs compare against it within a
//! per-pixel tolerance.

use std::path::PathBuf;

use ui_glass::LiquidMaterial;
use ui_glass_engine::headless::TestHarness;
use ui_glass_engine::{Compositor, GlassRegion};

const GOLDEN: &str = "tests/assets/floating_neutral_128.png";
const TOLERANCE: u8 = 4;

#[test]
fn floating_neutral_matches_golden() {
    let pixels = render_test_scene();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(GOLDEN);
    if std::env::var("UPDATE_GOLDEN").is_ok() || !path.exists() {
        write_png(&path, &pixels, 128, 128);
        if !std::env::var("UPDATE_GOLDEN").is_ok() {
            panic!(
                "golden missing at {}; wrote new reference. Re-run to verify.",
                path.display()
            );
        }
        return;
    }

    let expected = read_png(&path);
    assert_eq!(expected.len(), pixels.len(), "size mismatch");

    let mut diffs = 0usize;
    let mut worst = 0u8;
    for (a, b) in expected.iter().zip(pixels.iter()) {
        let d = a.abs_diff(*b);
        if d > TOLERANCE { diffs += 1; }
        if d > worst { worst = d; }
    }

    // Allow up to 0.5% of subpixels to exceed tolerance (compiler/driver jitter).
    let max_allowed = pixels.len() / 200;
    assert!(
        diffs <= max_allowed,
        "{diffs} subpixels exceeded tolerance {TOLERANCE} (max worst diff: {worst}); \
         set UPDATE_GOLDEN=1 to refresh",
    );
}

fn render_test_scene() -> Vec<u8> {
    let h = pollster::block_on(TestHarness::new()).unwrap();
    let (w, hgt) = (128u32, 128u32);
    let bg = create_gradient(h.device(), h.queue(), w, hgt);
    let out = create_output(h.device(), w, hgt);

    let mut comp = Compositor::new(h.device().clone(), h.queue().clone());
    comp.render(
        &bg.create_view(&Default::default()),
        &out.create_view(&Default::default()),
        [w as f32, hgt as f32],
        &[GlassRegion {
            rect_px: [24.0, 24.0, 80.0, 80.0],
            material: LiquidMaterial::floating().tint(
                ui_tokens::Color::rgba(255, 255, 255, 1.0),
                0.35,
            ),
        }],
    );

    read_back(h.device(), h.queue(), &out, w, hgt)
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
            texture: &t, mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
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
