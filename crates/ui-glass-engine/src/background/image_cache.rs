//! Texture cache for `Image::Static(path)` and `Image::Dynamic(handle)` background
//! sources. Static images are loaded once and cached by URL/path; dynamic images
//! are uploaded by the host and tracked via integer handles.
//!
//! ## Color space
//!
//! `upload_rgba` interprets the input buffer as **sRGB-encoded RGBA** — the
//! same format produced by the `image` crate's `to_rgba8()` on a typical PNG.
//! The wgpu texture is allocated as `Rgba8UnormSrgb`, which means the GPU
//! applies sRGB→linear conversion automatically when the shader samples it.
//! If you have linear RGBA, encode to sRGB first (or use a future
//! `upload_rgba_linear` overload — not implemented in Plan 3).

use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageHandle(pub u64);

pub struct ImageCache {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    by_handle: HashMap<ImageHandle, Arc<wgpu::Texture>>,
    by_path: HashMap<String, Arc<wgpu::Texture>>,
    next_id: u64,
}

impl ImageCache {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        Self {
            device, queue,
            by_handle: HashMap::new(),
            by_path: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn upload_rgba(&mut self, pixels: &[u8], w: u32, h: u32) -> ImageHandle {
        let tex = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("user-image"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &tex, mip_level: 0, origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            pixels,
            wgpu::TexelCopyBufferLayout {
                offset: 0, bytes_per_row: Some(w * 4), rows_per_image: Some(h),
            },
            wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        );
        let handle = ImageHandle(self.next_id);
        self.next_id += 1;
        self.by_handle.insert(handle, Arc::new(tex));
        handle
    }

    /// Static image upload — Plan 3 accepts pre-loaded RGBA buffers (no IO).
    /// Plan 4 will add asset/URL plumbing.
    pub fn upload_static(&mut self, key: &str, pixels: &[u8], w: u32, hgt: u32) {
        let h = self.upload_rgba(pixels, w, hgt);
        let tex = self.by_handle.remove(&h).unwrap();
        self.by_path.insert(key.to_string(), tex);
    }

    pub fn get(&self, handle: &ImageHandle) -> Option<Arc<wgpu::Texture>> {
        self.by_handle.get(handle).cloned()
    }

    pub fn get_static(&self, key: &str) -> Option<Arc<wgpu::Texture>> {
        self.by_path.get(key).cloned()
    }
}
