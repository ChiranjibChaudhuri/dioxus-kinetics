//! `<LiquidSurface>` Dioxus component.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;

use ui_glass::LiquidMaterial;
use ui_glass_engine::background::BackgroundSource;

use crate::motion_bridge::MotionState;
use crate::surface_state::SurfaceState;

/// Combined lifetime guard for everything spawned on canvas mount. Dropping
/// this stops the frame loop and removes all event listeners.
#[cfg(target_arch = "wasm32")]
struct LiveResources {
    _frame: ui_runtime::scheduler::FrameHandle,
    _listeners: crate::motion_bridge::MotionListenersGuard,
}

#[cfg(not(target_arch = "wasm32"))]
struct LiveResources;

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
    let motion_state: Signal<MotionState> = use_signal(MotionState::default);

    // LiveResources stored in a stable Rc<RefCell<...>> via use_hook so that
    // non-Clone, non-PartialEq values (FrameHandle, MotionListenersGuard) can
    // be held across async boundaries without Signal's PartialEq constraint.
    // The Rc is dropped when the component unmounts, which drops LiveResources,
    // which drops FrameHandle (sets cancelled=true, stops rAF) and
    // MotionListenersGuard (removes gloo-events listeners).
    let live: Rc<RefCell<Option<LiveResources>>> = use_hook(|| Rc::new(RefCell::new(None)));

    // Step 1: Signal holding the current GlassRegion, built from props.
    // GlassRegion derives Clone + PartialEq so Signal<GlassRegion> is valid.
    let region: Signal<ui_glass_engine::GlassRegion> = use_signal(|| {
        let r = props.rect.unwrap_or([0.0, 0.0, props.width as f32, props.height as f32]);
        let mut gr = ui_glass_engine::GlassRegion::new(r, props.material);
        if let Some(bg) = props.background.clone() {
            gr = gr.with_background(bg);
        }
        gr
    });

    // Step 2: Re-build and update the region signal whenever props change.
    let material = props.material;
    let background = props.background.clone();
    let rect = props.rect;
    let width = props.width;
    let height = props.height;

    {
        let mut region = region;
        let background = background.clone();
        use_effect(move || {
            let r = rect.unwrap_or([0.0, 0.0, width as f32, height as f32]);
            let mut gr = ui_glass_engine::GlassRegion::new(r, material);
            if let Some(bg) = background.clone() {
                gr = gr.with_background(bg);
            }
            region.set(gr);
        });
    }

    let inline_style = format!(
        "position: relative; display: inline-block; width: {}px; height: {}px;",
        props.width, props.height,
    );
    let canvas_style = "position: absolute; inset: 0; width: 100%; height: 100%; \
                        z-index: 0; pointer-events: none; display: block;";
    let foreground_style = "position: absolute; inset: 0; z-index: 1; pointer-events: auto;";

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
                        live.clone(),
                        region,
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
    live: Rc<RefCell<Option<LiveResources>>>,
    region: Signal<ui_glass_engine::GlassRegion>,
    width: u32,
    height: u32,
) {
    use crate::web::{canvas_from_mounted, resize_canvas_to_css_size};

    // Idempotency guard: if LiveResources is already set for this component,
    // don't re-spawn wgpu init + listeners + frame loop. Dioxus may re-fire
    // onmounted on the same element in some rehydration/route-change paths.
    // Using `live` as the idempotency signal (rather than surface_state) is
    // cleaner — it's only set after both surface_state and listeners are wired.
    if live.borrow().is_some() {
        return;
    }

    let Some(canvas) = canvas_from_mounted(&evt) else { return; };
    let physical_size = resize_canvas_to_css_size(&canvas);
    let canvas_for_listeners = canvas.clone();

    spawn(async move {
        if let Some(state) = SurfaceState::from_canvas(canvas, physical_size).await {
            surface_state.set(Some(state));
            let listeners = crate::motion_bridge::attach_listeners(&canvas_for_listeners, motion_state);
            let frame = start_frame_loop(surface_state, motion_state, region, width, height);
            live.borrow_mut().replace(LiveResources { _frame: frame, _listeners: listeners });
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn handle_canvas_mounted(
    _evt: MountedEvent,
    _surface_state: Signal<Option<SurfaceState>>,
    _motion_state: Signal<MotionState>,
    _live: Rc<RefCell<Option<LiveResources>>>,
    _region: Signal<ui_glass_engine::GlassRegion>,
    _width: u32,
    _height: u32,
) {
    // No-op on non-web (Blitz/native canvas integration deferred).
}

// Step 3: start_frame_loop now accepts Signal<GlassRegion> instead of individual
// material/background/rect props. Each frame it reads the current region from the
// signal, so prop changes are reflected immediately without restarting the loop.
#[cfg(target_arch = "wasm32")]
fn start_frame_loop(
    surface_state: Signal<Option<SurfaceState>>,
    mut motion_state: Signal<MotionState>,
    region: Signal<ui_glass_engine::GlassRegion>,
    width: u32,
    height: u32,
) -> ui_runtime::scheduler::FrameHandle {
    use ui_runtime::scheduler::{spawn_frame_loop, ControlFlow};

    let start_time = web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0);

    let mut surface_state = surface_state;

    let handle = spawn_frame_loop(move |_dt_ms| {
        // Clone region each frame (shallow: LiquidMaterial is Copy,
        // Option<BackgroundSource> has a small Vec for gradient stops).
        let region_owned: ui_glass_engine::GlassRegion = region.read().clone();

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

        // 2. Cached transparent fallback bg.
        state.compositor.ensure_transparent_bg([state.physical_size.0, state.physical_size.1]);
        // Re-create the view each frame — it's free (just a wrapper around the
        // texture handle); allocates no GPU memory. The texture itself is the
        // cached one from ensure_transparent_bg.
        let bg_view = state.compositor.transparent_bg_texture()
            .expect("ensure_transparent_bg was called above")
            .create_view(&Default::default());

        // 3. Update motion + render
        let inputs = motion_state.read().to_motion_inputs(start_time);

        // Decay scroll velocity each frame so the surface settles after the
        // user stops scrolling. Applied AFTER reading the inputs so this
        // frame's render uses the un-decayed value (smoother trail); next
        // frame uses the decayed value.
        motion_state.with_mut(|s| {
            s.scroll_velocity_px[0] *= 0.85;
            s.scroll_velocity_px[1] *= 0.85;
            if s.scroll_velocity_px[0].abs() < 0.01 { s.scroll_velocity_px[0] = 0.0; }
            if s.scroll_velocity_px[1].abs() < 0.01 { s.scroll_velocity_px[1] = 0.0; }
        });

        state.compositor.update_inputs(inputs);
        state.compositor.render(
            &bg_view,
            &output_view,
            [width as f32, height as f32],
            std::slice::from_ref(&region_owned),
        );

        // 4. Present
        frame.present();

        ControlFlow::Continue
    });

    handle
}
