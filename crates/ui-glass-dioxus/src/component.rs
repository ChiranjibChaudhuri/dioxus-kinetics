//! `<LiquidSurface>` Dioxus component.

use dioxus::prelude::*;

use ui_glass::LiquidMaterial;
use ui_glass_engine::background::BackgroundSource;

use crate::motion_bridge::MotionState;
use crate::surface_state::SurfaceState;

#[derive(Props, Clone, PartialEq)]
pub struct LiquidSurfaceProps {
    /// Material descriptor for the glass surface.
    pub material: LiquidMaterial,

    /// Optional background source. When `None`, the surface samples a
    /// transparent fallback texture (the canvas's existing clear color shows
    /// through).
    #[props(default)]
    pub background: Option<BackgroundSource>,

    /// Surface rect within the canvas, in CSS pixels: `[x, y, w, h]`. Defaults
    /// to filling the canvas.
    #[props(default)]
    pub rect: Option<[f32; 4]>,

    /// Logical width in CSS pixels.
    #[props(default = 320)]
    pub width: u32,

    /// Logical height in CSS pixels.
    #[props(default = 200)]
    pub height: u32,

    /// Optional foreground children — rendered as DOM widgets on top of the
    /// canvas with `position: absolute; z-index: 1; pointer-events: auto`.
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn LiquidSurface(props: LiquidSurfaceProps) -> Element {
    // Signal carrying the wgpu state once the canvas mounts and async init
    // completes. None until ready.
    let surface_state: Signal<Option<SurfaceState>> = use_signal(|| None::<SurfaceState>);

    // MotionState holds the latest pointer/scroll/time + reduced-motion flag.
    // Task 6 will wire event listeners; for now it stays at default.
    let motion_state: Signal<MotionState> = use_signal(MotionState::default);

    let inline_style = format!(
        "position: relative; display: inline-block; width: {}px; height: {}px;",
        props.width, props.height,
    );
    let canvas_style = "position: absolute; inset: 0; width: 100%; height: 100%; \
                        z-index: 0; pointer-events: none; display: block;";
    let foreground_style = "position: absolute; inset: 0; z-index: 1; pointer-events: auto;";

    let material = props.material;
    let background = props.background.clone();
    let rect = props.rect;
    let width = props.width;
    let height = props.height;

    rsx! {
        div {
            class: "ui-liquid-surface",
            style: "{inline_style}",
            "data-glass-role": "liquid-surface",

            canvas {
                style: "{canvas_style}",
                width: "{width}",
                height: "{height}",
                onmounted: move |evt: MountedEvent| {
                    handle_canvas_mounted(
                        evt,
                        surface_state,
                        motion_state,
                        material,
                        background.clone(),
                        rect,
                        width,
                        height,
                    );
                },
            }
            div {
                style: "{foreground_style}",
                {props.children}
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn handle_canvas_mounted(
    evt: MountedEvent,
    mut surface_state: Signal<Option<SurfaceState>>,
    motion_state: Signal<MotionState>,
    material: LiquidMaterial,
    background: Option<BackgroundSource>,
    rect: Option<[f32; 4]>,
    width: u32,
    height: u32,
) {
    use crate::web::{canvas_from_mounted, resize_canvas_to_css_size};

    // Idempotency guard: if a SurfaceState already exists for this component,
    // don't re-spawn wgpu init + listeners + frame loop. Dioxus may re-fire
    // onmounted on the same element in some rehydration/route-change paths.
    if surface_state.read().is_some() {
        return;
    }

    let Some(canvas) = canvas_from_mounted(&evt) else { return; };
    let physical_size = resize_canvas_to_css_size(&canvas);
    let canvas_for_listeners = canvas.clone();

    spawn(async move {
        if let Some(state) = SurfaceState::from_canvas(canvas, physical_size).await {
            surface_state.set(Some(state));
            let guard = crate::motion_bridge::attach_listeners(&canvas_for_listeners, motion_state);
            std::mem::forget(guard);
            start_frame_loop(surface_state, motion_state, material, background, rect, width, height);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn handle_canvas_mounted(
    _evt: MountedEvent,
    _surface_state: Signal<Option<SurfaceState>>,
    _motion_state: Signal<MotionState>,
    _material: LiquidMaterial,
    _background: Option<BackgroundSource>,
    _rect: Option<[f32; 4]>,
    _width: u32,
    _height: u32,
) {
    // No-op on non-web (Blitz/native canvas integration deferred).
}

#[cfg(target_arch = "wasm32")]
fn start_frame_loop(
    surface_state: Signal<Option<SurfaceState>>,
    motion_state: Signal<MotionState>,
    material: LiquidMaterial,
    background: Option<BackgroundSource>,
    rect: Option<[f32; 4]>,
    width: u32,
    height: u32,
) {
    use ui_runtime::scheduler::{spawn_frame_loop, ControlFlow};
    use ui_glass_engine::GlassRegion;

    // Build the region once.
    let region_template = {
        let r = rect.unwrap_or([0.0, 0.0, width as f32, height as f32]);
        let mut gr = GlassRegion::new(r, material);
        if let Some(bg) = background.clone() {
            gr = gr.with_background(bg);
        }
        gr
    };

    let start_time = web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0);

    let mut surface_state = surface_state;

    let handle = spawn_frame_loop(move |_dt_ms| {
        let region = region_template.clone();
        let mut state_opt = surface_state.write();
        let Some(state) = state_opt.as_mut() else {
            return ControlFlow::Continue;
        };

        // 1. Acquire frame
        let frame = match state.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => {
                state.resize(state.physical_size);
                return ControlFlow::Continue;
            }
            Err(_) => return ControlFlow::Continue,
        };
        let output_view = frame.texture.create_view(&Default::default());

        // 2. Transparent fallback bg (allocated every frame — Plan 5 caches).
        let bg_tex = state.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("liquid-surface-bg-fallback"),
            size: wgpu::Extent3d {
                width: state.physical_size.0,
                height: state.physical_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let bg_view = bg_tex.create_view(&Default::default());

        // 3. Update motion + render
        let inputs = motion_state.read().to_motion_inputs(start_time);
        state.compositor.update_inputs(inputs);
        state.compositor.render(
            &bg_view,
            &output_view,
            [width as f32, height as f32],
            &[region],
        );

        // 4. Present
        frame.present();

        ControlFlow::Continue
    });

    // Plan 4 leaks the FrameHandle. Plan 5 will tie it to the component
    // unmount lifecycle via a use_drop / use_effect cleanup.
    std::mem::forget(handle);
}
