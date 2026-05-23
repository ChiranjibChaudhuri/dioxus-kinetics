//! Holds the wgpu pipeline for a single LiquidSurface instance. Initialized
//! asynchronously when the canvas mounts; lives in a `Signal<Option<...>>`
//! on the component.

#[cfg(target_arch = "wasm32")]
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use ui_glass_engine::Compositor;

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
    /// surface creation fails.
    pub async fn from_canvas(
        canvas: web_sys::HtmlCanvasElement,
        physical_size: (u32, u32),
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
                power_preference: wgpu::PowerPreference::HighPerformance,
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

        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps.formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let alpha_mode = caps.alpha_modes
            .iter()
            .copied()
            .find(|m| *m == wgpu::CompositeAlphaMode::PreMultiplied)
            .unwrap_or(caps.alpha_modes[0]);

        surface.configure(&device, &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: physical_size.0,
            height: physical_size.1,
            present_mode: caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode,
            view_formats: vec![],
        });

        let compositor = Compositor::new(device.clone(), queue.clone());

        Some(Self {
            device, queue, surface, surface_format, compositor, physical_size,
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
