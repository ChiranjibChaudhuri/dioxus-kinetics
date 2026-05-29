//! Reads pointer/scroll/reduced-motion and drives the idle/visibility gating
//! for the render loop. Pointer/scroll/visibility listeners flip a shared
//! "dirty" flag so the frame loop can render only when something actually
//! changed instead of every rAF tick forever.

use ui_glass_engine::motion::MotionInputs;

/// Shared dirty flag wiring the pointer/scroll/visibility listeners to the
/// render loop. `Rc<Cell<bool>>` (single-threaded; the Dioxus web renderer
/// runs on one task). Listeners set it `true`; the frame loop reads + clears
/// it. See [`attach_listeners`] and the component's frame loop.
#[cfg(target_arch = "wasm32")]
pub type DirtyFlag = std::rc::Rc<std::cell::Cell<bool>>;

/// Non-web placeholder so component code can hold and flip a `DirtyFlag`
/// uniformly across targets. The frame loop / listeners are web-only, so this
/// flag is never observed off the web, but keeping the same `Rc<Cell<bool>>`
/// shape lets the component body (hook + region effect) compile without `cfg`
/// gymnastics around the flag itself.
#[cfg(not(target_arch = "wasm32"))]
pub type DirtyFlag = std::rc::Rc<std::cell::Cell<bool>>;

#[derive(Clone, Copy, Debug, Default)]
pub struct MotionState {
    pub pointer_px: [f32; 2],
    pub scroll_velocity_px: [f32; 2],
    pub reduced_motion: bool,
}

impl MotionState {
    /// Build a `MotionInputs` snapshot. `start_time_ms` is the loop start in
    /// performance.now() units; the resulting `time_seconds` is relative.
    pub fn to_motion_inputs(self, start_time_ms: f64) -> MotionInputs {
        let now = web_sys_performance_now();
        let elapsed_s = ((now - start_time_ms) / 1000.0) as f32;
        MotionInputs::new()
            .with_pointer(self.pointer_px)
            .with_scroll_velocity(self.scroll_velocity_px)
            .with_time(elapsed_s)
            .with_reduced_motion(self.reduced_motion)
    }

    /// Whether the surface still has scroll motion that needs to keep
    /// rendering frames as it decays. Pure helper used by the render loop's
    /// idle gating. Pointer movement is event-driven (flips the dirty flag),
    /// so it is not part of this "ongoing animation" test.
    pub fn has_residual_motion(self) -> bool {
        self.scroll_velocity_px[0] != 0.0 || self.scroll_velocity_px[1] != 0.0
    }

    /// Whether the render loop should render this tick given the current
    /// motion + the dirty flag. Under `reduced_motion` we never keep the loop
    /// "live" off residual scroll velocity — only an explicit dirty mark
    /// (e.g. a prop change or visibility regain) triggers a single repaint.
    pub fn should_render(self, dirty: bool) -> bool {
        if dirty {
            return true;
        }
        if self.reduced_motion {
            return false;
        }
        self.has_residual_motion()
    }
}

#[cfg(target_arch = "wasm32")]
fn web_sys_performance_now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn web_sys_performance_now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

/// Attach pointer + scroll + matchMedia + visibilitychange listeners that
/// update the given motion_state signal and the shared `dirty` flag. Returns
/// a `MotionListenersGuard` whose drop removes the listeners. Web-only.
///
/// Every input that changes what the surface should look like flips `dirty`
/// so the render loop wakes for at least one frame:
/// - `pointermove` (pointer-reactive surfaces),
/// - `scroll` (kicks off velocity that then decays over subsequent frames),
/// - `visibilitychange` -> visible (repaint after a hidden stretch).
#[cfg(target_arch = "wasm32")]
pub fn attach_listeners(
    canvas: &web_sys::HtmlCanvasElement,
    mut motion_state: dioxus::prelude::Signal<MotionState>,
    dirty: DirtyFlag,
) -> MotionListenersGuard {
    use dioxus::prelude::WritableExt;
    use gloo_events::EventListener;
    use wasm_bindgen::JsCast;

    let canvas_for_pointer = canvas.clone();
    let dirty_pointer = dirty.clone();
    let pointer = EventListener::new(canvas, "pointermove", move |evt| {
        if let Some(e) = evt.dyn_ref::<web_sys::PointerEvent>() {
            let rect = canvas_for_pointer.get_bounding_client_rect();
            let x = (e.client_x() as f64 - rect.left()) as f32;
            let y = (e.client_y() as f64 - rect.top()) as f32;
            motion_state.with_mut(|s| s.pointer_px = [x, y]);
            dirty_pointer.set(true);
        }
    });

    let window = web_sys::window().expect("window");
    let scroll_state = std::rc::Rc::new(std::cell::RefCell::new(0.0f64));
    let scroll_state_clone = scroll_state.clone();
    let dirty_scroll = dirty.clone();
    let scroll = EventListener::new(&window, "scroll", move |_| {
        if let Some(w) = web_sys::window() {
            let y = w.scroll_y().unwrap_or(0.0);
            let mut prev = scroll_state_clone.borrow_mut();
            let dy = (y - *prev) as f32;
            *prev = y;
            motion_state.with_mut(|s| s.scroll_velocity_px = [0.0, dy]);
            if dy != 0.0 {
                dirty_scroll.set(true);
            }
        }
    });

    // Visibility: when the tab/document becomes visible again, request one
    // repaint so the surface is fresh after a hidden stretch. The scheduler
    // already skips work while hidden, but we mark dirty on regain so an
    // otherwise-idle (settled) surface paints once.
    let visibility = window.document().map(|doc| {
        let doc_for_listener = doc.clone();
        let dirty_vis = dirty.clone();
        let target: &web_sys::EventTarget = doc.as_ref();
        EventListener::new(target, "visibilitychange", move |_| {
            if !doc_for_listener.hidden() {
                dirty_vis.set(true);
            }
        })
    });

    // Reduced-motion preference
    let media = window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten();
    if let Some(mql) = &media {
        let initial = mql.matches();
        motion_state.with_mut(|s| s.reduced_motion = initial);
    }
    let dirty_media = dirty.clone();
    let media_listener = media.as_ref().map(|mql| {
        let mql_target: &web_sys::EventTarget = mql.as_ref();
        EventListener::new(mql_target, "change", move |evt| {
            if let Some(e) = evt.dyn_ref::<web_sys::MediaQueryListEvent>() {
                let on = e.matches();
                motion_state.with_mut(|s| s.reduced_motion = on);
                // Repaint once so the surface reflects the new preference even
                // if it is otherwise idle.
                dirty_media.set(true);
            }
        })
    });

    MotionListenersGuard {
        _pointer: pointer,
        _scroll: scroll,
        _visibility: visibility,
        _media: media_listener,
    }
}

#[cfg(target_arch = "wasm32")]
pub struct MotionListenersGuard {
    _pointer: gloo_events::EventListener,
    _scroll: gloo_events::EventListener,
    _visibility: Option<gloo_events::EventListener>,
    _media: Option<gloo_events::EventListener>,
}

#[cfg(test)]
mod tests {
    use super::MotionState;

    #[test]
    fn no_motion_has_no_residual() {
        let s = MotionState::default();
        assert!(!s.has_residual_motion());
    }

    #[test]
    fn scroll_velocity_counts_as_residual() {
        let s = MotionState {
            scroll_velocity_px: [0.0, 3.0],
            ..MotionState::default()
        };
        assert!(s.has_residual_motion());
    }

    #[test]
    fn should_render_when_dirty_even_if_idle() {
        let s = MotionState::default();
        assert!(s.should_render(true));
    }

    #[test]
    fn idle_clean_surface_does_not_render() {
        let s = MotionState::default();
        assert!(!s.should_render(false));
    }

    #[test]
    fn residual_scroll_keeps_rendering_until_settled() {
        let s = MotionState {
            scroll_velocity_px: [0.0, 1.5],
            ..MotionState::default()
        };
        assert!(s.should_render(false));
    }

    #[test]
    fn reduced_motion_ignores_residual_scroll() {
        // Under reduced-motion, residual scroll velocity must NOT keep the
        // loop live; only an explicit dirty mark paints a single frame.
        let s = MotionState {
            scroll_velocity_px: [0.0, 5.0],
            reduced_motion: true,
            ..MotionState::default()
        };
        assert!(!s.should_render(false));
        assert!(s.should_render(true));
    }
}
