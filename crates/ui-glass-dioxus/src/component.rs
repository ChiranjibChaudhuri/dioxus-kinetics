//! `<LiquidSurface>` Dioxus component.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;

use ui_glass::LiquidMaterial;
use ui_glass_engine::background::BackgroundSource;

use crate::motion_bridge::MotionState;
use crate::surface_state::{GlassPower, SurfaceState};

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

    /// GPU power-preference hint for adapter selection. Defaults to
    /// [`GlassPower::Low`] since most glass surfaces are static chrome that
    /// does not justify the discrete GPU. Opt into [`GlassPower::High`] for
    /// large, animated hero surfaces.
    #[props(default)]
    pub power: GlassPower,

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

    // Set true when async wgpu init resolves to failure (no adapter / surface
    // creation failed). Drives the CSS fallback markup so the surface degrades
    // to a `data-glass-*` styled section instead of a transparent canvas.
    let wgpu_failed: Signal<bool> = use_signal(|| false);

    // MotionState holds the latest pointer/scroll/time + reduced-motion flag.
    let motion_state: Signal<MotionState> = use_signal(MotionState::default);

    // LiveResources stored in a stable Rc<RefCell<...>> via use_hook so that
    // non-Clone, non-PartialEq values (FrameHandle, MotionListenersGuard) can
    // be held across async boundaries without Signal's PartialEq constraint.
    // The Rc is dropped when the component unmounts, which drops LiveResources,
    // which drops FrameHandle (sets cancelled=true, stops rAF) and
    // MotionListenersGuard (removes gloo-events listeners).
    let live: Rc<RefCell<Option<LiveResources>>> = use_hook(|| Rc::new(RefCell::new(None)));

    // Shared dirty flag wiring prop changes + input listeners to the frame
    // loop. Created once here (stable across renders) so the region effect can
    // flip it on a prop change, and the same flag is threaded into
    // attach_listeners + start_frame_loop on mount. Starts true so the very
    // first painted frame is requested. See motion_bridge::DirtyFlag.
    let dirty: crate::motion_bridge::DirtyFlag =
        use_hook(|| std::rc::Rc::new(std::cell::Cell::new(true)));

    // Step 1: Signal holding the current GlassRegion, built from props.
    // GlassRegion derives Clone + PartialEq so Signal<GlassRegion> is valid.
    let region: Signal<ui_glass_engine::GlassRegion> = use_signal(|| {
        let r = props
            .rect
            .unwrap_or([0.0, 0.0, props.width as f32, props.height as f32]);
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
        let dirty = dirty.clone();
        // `rect`/`width`/`height`/`material`/`background` are plain props, not
        // signals, so a bare `use_effect` would run once on mount and never
        // re-run when they change — freezing the live surface on its initial
        // material/rect. `use_reactive` keyed on those props re-runs the effect
        // whenever any of them changes (mirrors popover.rs usage).
        use_effect(use_reactive(
            (&rect, &width, &height, &material, &background),
            move |(rect, width, height, material, background)| {
                let r = rect.unwrap_or([0.0, 0.0, width as f32, height as f32]);
                let mut gr = ui_glass_engine::GlassRegion::new(r, material);
                if let Some(bg) = background.clone() {
                    gr = gr.with_background(bg);
                }
                region.set(gr);
                // A region.set() alone does not repaint: the frame loop only
                // paints when the dirty flag is set or residual motion remains
                // (see motion_bridge). Mark dirty so the surface repaints once
                // to reflect the new props.
                dirty.set(true);
            },
        ));
    }

    let power = props.power;

    let inline_style = format!(
        "position: relative; display: inline-block; width: {}px; height: {}px;",
        props.width, props.height,
    );
    let canvas_style = "position: absolute; inset: 0; width: 100%; height: 100%; \
                        z-index: 0; pointer-events: none; display: block;";
    let foreground_style = "position: absolute; inset: 0; z-index: 1; pointer-events: auto;";

    // When wgpu init failed we render a CSS-only `data-glass-*` surface so the
    // chrome still reads as glass instead of leaving a transparent canvas. The
    // attribute strings are derived locally from the material — we deliberately
    // do NOT depend on ui-dioxus here.
    if *wgpu_failed.read() {
        let fallback_style = format!(
            "position: absolute; inset: 0; z-index: 0; width: 100%; height: 100%; \
             box-sizing: border-box;{}",
            material_radius_style(&material),
        );
        return rsx! {
            div {
                class: "ui-liquid-surface",
                style: "{inline_style}",
                "data-glass-role": "liquid-surface",
                "data-glass-fallback": "css",

                section {
                    class: "ui-glass-surface",
                    style: "{fallback_style}",
                    "data-glass-level": glass_level_attr(&material),
                    "data-glass-tone": glass_tone_attr(&material),
                    "data-glass-density": glass_density_attr(&material),
                }
                div {
                    style: "{foreground_style}",
                    {props.children}
                }
            }
        };
    }

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
                        wgpu_failed,
                        motion_state,
                        live.clone(),
                        region,
                        width,
                        height,
                        power,
                        dirty.clone(),
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

/// Derive the `data-glass-level` string from the material's blur/radius
/// profile, mirroring the constructor presets (`tooltip`/`button` → subtle,
/// `floating` → floating, `chrome` → chrome, `overlay` → overlay). Pure helper
/// so the fallback markup matches the live surface as closely as possible
/// without depending on ui-dioxus.
fn glass_level_attr(material: &LiquidMaterial) -> &'static str {
    let blur = material.blur_radius_px;
    // Chrome presets sit flush against the edge (radius 0) with a heavy blur.
    if material.radius_px == 0.0 && blur >= 24.0 {
        return "chrome";
    }
    if blur < 14.0 {
        "subtle"
    } else if blur < 22.0 {
        "floating"
    } else {
        "overlay"
    }
}

/// Derive the `data-glass-tone` string by matching the material tint against
/// the canonical tone tints used by `LiquidMaterial::from(MaterialRequest)`.
fn glass_tone_attr(material: &LiquidMaterial) -> &'static str {
    let c = material.tint;
    match (c.r, c.g, c.b) {
        (0, 102, 204) => "primary",
        (36, 138, 61) => "success",
        (176, 105, 0) => "warning",
        (196, 43, 43) => "danger",
        (20, 118, 191) => "info",
        _ => "neutral",
    }
}

/// `data-glass-density` for the fallback. `LiquidMaterial` does not retain the
/// originating density (it is folded into `radius_px`), so the fallback uses
/// the default comfortable density — it only affects radius, which the inline
/// style already carries from the material.
fn glass_density_attr(_material: &LiquidMaterial) -> &'static str {
    "comfortable"
}

/// Inline `border-radius` for the CSS fallback, taken straight from the
/// material so the fallback corner matches the GPU surface.
fn material_radius_style(material: &LiquidMaterial) -> String {
    if material.radius_px > 0.0 {
        format!(" border-radius: {}px;", material.radius_px)
    } else {
        String::new()
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(clippy::too_many_arguments)]
fn handle_canvas_mounted(
    evt: MountedEvent,
    mut surface_state: Signal<Option<SurfaceState>>,
    mut wgpu_failed: Signal<bool>,
    motion_state: Signal<MotionState>,
    live: Rc<RefCell<Option<LiveResources>>>,
    region: Signal<ui_glass_engine::GlassRegion>,
    width: u32,
    height: u32,
    power: GlassPower,
    dirty: crate::motion_bridge::DirtyFlag,
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

    let Some(canvas) = canvas_from_mounted(&evt) else {
        return;
    };
    let physical_size = resize_canvas_to_css_size(&canvas);
    let canvas_for_listeners = canvas.clone();

    spawn(async move {
        if let Some(state) = SurfaceState::from_canvas(canvas, physical_size, power).await {
            surface_state.set(Some(state));
            // Shared dirty flag (created in the component body and also flipped
            // by the region effect on prop change) wires the input listeners +
            // prop changes to the render loop so the surface only repaints when
            // something actually changes.
            let listeners = crate::motion_bridge::attach_listeners(
                &canvas_for_listeners,
                motion_state,
                dirty.clone(),
            );
            let frame = start_frame_loop(surface_state, motion_state, region, width, height, dirty);
            live.borrow_mut().replace(LiveResources {
                _frame: frame,
                _listeners: listeners,
            });
        } else {
            // Async wgpu init failed (no adapter / surface creation failed):
            // flip the fallback signal so the component re-renders the CSS
            // `data-glass-*` surface instead of a transparent canvas.
            wgpu_failed.set(true);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::too_many_arguments)]
fn handle_canvas_mounted(
    _evt: MountedEvent,
    _surface_state: Signal<Option<SurfaceState>>,
    _wgpu_failed: Signal<bool>,
    _motion_state: Signal<MotionState>,
    _live: Rc<RefCell<Option<LiveResources>>>,
    _region: Signal<ui_glass_engine::GlassRegion>,
    _width: u32,
    _height: u32,
    _power: GlassPower,
    _dirty: crate::motion_bridge::DirtyFlag,
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
    dirty: crate::motion_bridge::DirtyFlag,
) -> ui_runtime::scheduler::FrameHandle {
    use ui_runtime::scheduler::{spawn_frame_loop, ControlFlow};

    let start_time = web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0);

    // Document handle for the per-tick visibility check. Cached once.
    let document = web_sys::window().and_then(|w| w.document());

    let mut surface_state = surface_state;

    let handle = spawn_frame_loop(move |_dt_ms| {
        // Idle/visibility gating ----------------------------------------
        // a) While the document is hidden, skip ALL GPU work (no
        //    get_current_texture / render / present) but keep the rAF alive so
        //    we resume cleanly. The scheduler also guards this, but we guard
        //    here too so the GPU is never touched off-screen.
        let hidden = document.as_ref().map(|d| d.hidden()).unwrap_or(false);
        if hidden {
            return ControlFlow::Continue;
        }

        // b) Render only when something changed: an explicit dirty mark
        //    (pointer/scroll/visibility/prefs) OR residual scroll velocity that
        //    is still decaying. Under reduced-motion, residual velocity does
        //    NOT keep us live (see MotionState::should_render). When neither
        //    holds we idle this tick — the surface has settled.
        let is_dirty = dirty.get();
        if !motion_state.read().should_render(is_dirty) {
            return ControlFlow::Continue;
        }
        // Consume the dirty mark: we are about to paint the up-to-date state.
        // Residual scroll velocity (decayed below) keeps subsequent frames
        // rendering until it settles, at which point we idle again.
        dirty.set(false);

        // Clone region each frame (shallow: LiquidMaterial is Copy,
        // Option<BackgroundSource> has a small Vec for gradient stops).
        let region_owned: ui_glass_engine::GlassRegion = region.read().clone();

        let mut state_opt = surface_state.write();
        let Some(state) = state_opt.as_mut() else {
            // Not ready yet — keep dirty so we paint once it is.
            dirty.set(true);
            return ControlFlow::Continue;
        };

        // 1. Acquire frame
        let frame = match state.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => {
                state.resize(state.physical_size);
                // Re-arm: this frame did not paint; retry on the next tick.
                dirty.set(true);
                return ControlFlow::Continue;
            }
            Err(_) => {
                dirty.set(true);
                return ControlFlow::Continue;
            }
        };
        let output_view = frame.texture.create_view(&Default::default());

        // 2. Cached transparent fallback bg.
        state
            .compositor
            .ensure_transparent_bg([state.physical_size.0, state.physical_size.1]);
        // Re-create the view each frame — it's free (just a wrapper around the
        // texture handle); allocates no GPU memory. The texture itself is the
        // cached one from ensure_transparent_bg.
        let bg_view = state
            .compositor
            .transparent_bg_texture()
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
            if s.scroll_velocity_px[0].abs() < 0.01 {
                s.scroll_velocity_px[0] = 0.0;
            }
            if s.scroll_velocity_px[1].abs() < 0.01 {
                s.scroll_velocity_px[1] = 0.0;
            }
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

#[cfg(test)]
mod tests {
    use super::{glass_density_attr, glass_level_attr, glass_tone_attr, material_radius_style};
    use ui_glass::LiquidMaterial;
    use ui_tokens::Color;

    #[test]
    fn level_from_blur_profile() {
        // tooltip blur=10 -> subtle, button blur=12 -> subtle
        assert_eq!(glass_level_attr(&LiquidMaterial::tooltip()), "subtle");
        assert_eq!(glass_level_attr(&LiquidMaterial::button()), "subtle");
        // floating blur=18 -> floating
        assert_eq!(glass_level_attr(&LiquidMaterial::floating()), "floating");
        // overlay blur=24 (radius 18) -> overlay
        assert_eq!(glass_level_attr(&LiquidMaterial::overlay()), "overlay");
        // chrome blur=32, radius 0 -> chrome
        assert_eq!(glass_level_attr(&LiquidMaterial::chrome()), "chrome");
    }

    #[test]
    fn tone_from_tint_matches_canonical_tints() {
        let neutral = LiquidMaterial::floating(); // white tint
        assert_eq!(glass_tone_attr(&neutral), "neutral");

        let primary = LiquidMaterial::new().tint(Color::rgba(0, 102, 204, 1.0), 0.7);
        assert_eq!(glass_tone_attr(&primary), "primary");

        let success = LiquidMaterial::new().tint(Color::rgba(36, 138, 61, 1.0), 0.7);
        assert_eq!(glass_tone_attr(&success), "success");

        let warning = LiquidMaterial::new().tint(Color::rgba(176, 105, 0, 1.0), 0.7);
        assert_eq!(glass_tone_attr(&warning), "warning");

        let danger = LiquidMaterial::new().tint(Color::rgba(196, 43, 43, 1.0), 0.7);
        assert_eq!(glass_tone_attr(&danger), "danger");

        let info = LiquidMaterial::new().tint(Color::rgba(20, 118, 191, 1.0), 0.7);
        assert_eq!(glass_tone_attr(&info), "info");

        // An arbitrary tint falls back to neutral.
        let other = LiquidMaterial::new().tint(Color::rgba(1, 2, 3, 1.0), 0.7);
        assert_eq!(glass_tone_attr(&other), "neutral");
    }

    #[test]
    fn density_defaults_to_comfortable() {
        assert_eq!(
            glass_density_attr(&LiquidMaterial::floating()),
            "comfortable"
        );
    }

    #[test]
    fn radius_style_emitted_only_when_positive() {
        let with_radius = LiquidMaterial::new().radius(14.0);
        assert_eq!(material_radius_style(&with_radius), " border-radius: 14px;");

        let no_radius = LiquidMaterial::chrome(); // radius 0
        assert_eq!(material_radius_style(&no_radius), "");
    }
}
