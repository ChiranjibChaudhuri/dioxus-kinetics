//! Holds the wgpu pipeline for a single LiquidSurface instance. Initialized
//! asynchronously when the canvas mounts; lives in a `Signal<Option<...>>`
//! on the component.

// `wgpu::Device` / `wgpu::Queue` are `Send + Sync` on native targets but not
// on wasm32. The `Arc` wrapping below lives on a single-threaded Dioxus
// renderer task and is intentional — the wasm-only clippy warning is
// allowed for this file.
#![cfg_attr(target_arch = "wasm32", allow(clippy::arc_with_non_send_sync))]

#[cfg(target_arch = "wasm32")]
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use ui_glass_engine::{negotiate_surface_format, Compositor};

/// GPU power-preference hint for adapter selection. Most glass surfaces are
/// static chrome that does not justify spinning up the discrete GPU, so the
/// default is [`GlassPower::Low`]. Authors of full-screen, GPU-heavy hero
/// surfaces can opt into [`GlassPower::High`] via the `power` prop.
///
/// Kept as a UI-level enum (rather than re-exporting `wgpu::PowerPreference`)
/// so the public component API stays decoupled from the wgpu version.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlassPower {
    /// Prefer an integrated / low-power adapter. Default for static chrome.
    #[default]
    Low,
    /// Prefer the highest-performance (often discrete) adapter.
    High,
}

#[cfg(target_arch = "wasm32")]
impl GlassPower {
    fn to_wgpu(self) -> wgpu::PowerPreference {
        match self {
            GlassPower::Low => wgpu::PowerPreference::LowPower,
            GlassPower::High => wgpu::PowerPreference::HighPerformance,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub struct SurfaceState {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub surface: wgpu::Surface<'static>,
    pub surface_format: wgpu::TextureFormat,
    pub compositor: Compositor,
    pub physical_size: (u32, u32),
}

#[cfg(target_arch = "wasm32")]
impl SurfaceState {
    /// Initialize wgpu from a canvas element. Returns `None` if no adapter is
    /// available (e.g. WebGPU unavailable and no WebGL2 fallback) or if
    /// surface creation fails. The component treats `None` as a signal to
    /// render the `data-glass-*` CSS fallback markup.
    ///
    /// `power` controls the adapter power-preference hint; static chrome
    /// should pass [`GlassPower::Low`] (the default) so the integrated GPU is
    /// preferred over the discrete one.
    pub async fn from_canvas(
        canvas: web_sys::HtmlCanvasElement,
        physical_size: (u32, u32),
        power: GlassPower,
    ) -> Option<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
            .ok()?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: power.to_wgpu(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok()?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("liquid-surface-device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .ok()?;

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        // Negotiate a format the surface actually advertises. Without this
        // the compose pipeline's hardcoded `Rgba8UnormSrgb` mismatched the
        // surface's preferred format on Windows Chromium (`Bgra8UnormSrgb`),
        // producing per-frame "Invalid CommandBuffer" warnings in DevTools.
        let surface_format = negotiate_surface_format(&surface, &adapter);
        let caps = surface.get_capabilities(&adapter);

        let alpha_mode = caps
            .alpha_modes
            .iter()
            .copied()
            .find(|m| *m == wgpu::CompositeAlphaMode::PreMultiplied)
            .unwrap_or(caps.alpha_modes[0]);

        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: physical_size.0,
                height: physical_size.1,
                present_mode: caps.present_modes[0],
                desired_maximum_frame_latency: 2,
                alpha_mode,
                view_formats: vec![],
            },
        );

        let compositor =
            Compositor::with_output_format(device.clone(), queue.clone(), surface_format);

        Some(Self {
            device,
            queue,
            surface,
            surface_format,
            compositor,
            physical_size,
        })
    }

    pub fn resize(&mut self, physical_size: (u32, u32)) {
        if physical_size == self.physical_size || physical_size.0 == 0 || physical_size.1 == 0 {
            return;
        }
        self.physical_size = physical_size;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: physical_size.0,
            height: physical_size.1,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::PreMultiplied,
            view_formats: vec![],
        };
        self.surface.configure(&self.device, &config);
    }
}

// Native placeholder — full Blitz/native integration deferred to a future plan.
#[cfg(not(target_arch = "wasm32"))]
pub struct SurfaceState;

#[cfg(not(target_arch = "wasm32"))]
impl SurfaceState {
    pub fn resize(&mut self, _physical_size: (u32, u32)) {}
}

#[cfg(test)]
mod tests {
    use super::GlassPower;

    #[test]
    fn default_power_is_low() {
        // Static chrome must default to the low-power adapter so the discrete
        // GPU is not spun up for chrome surfaces.
        assert_eq!(GlassPower::default(), GlassPower::Low);
    }

    #[test]
    fn power_variants_distinct() {
        assert_ne!(GlassPower::Low, GlassPower::High);
    }
}
