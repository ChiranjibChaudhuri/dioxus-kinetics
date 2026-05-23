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
    let Some(canvas) = canvas_from_mounted(&evt) else { return; };
    let physical_size = resize_canvas_to_css_size(&canvas);

    spawn(async move {
        if let Some(state) = SurfaceState::from_canvas(canvas, physical_size).await {
            surface_state.set(Some(state));
            // Task 5 wires the frame loop here.
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
    _surface_state: Signal<Option<SurfaceState>>,
    _motion_state: Signal<MotionState>,
    _material: LiquidMaterial,
    _background: Option<BackgroundSource>,
    _rect: Option<[f32; 4]>,
    _width: u32,
    _height: u32,
) {
    // Wired in Task 5.
}
