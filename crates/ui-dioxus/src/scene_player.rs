//! Scene Dioxus component: hosts a SceneClock, provides a
//! SceneContext, emits hyperframes-compatible data attributes. The
//! transport UI (scrubber, play/pause) and adapter fan-out arrive in
//! later tasks; this module ships the shell.

use std::rc::Rc;

use dioxus::prelude::*;
use ui_composition::{ClipFill, FrameClip};
use ui_runtime::frame_adapter::FrameAdapterRegistry;
use ui_runtime::reduced_motion::use_reduced_motion;
use ui_runtime::scene_clock::{SceneClock, SceneState};
use ui_runtime::scene_driver::SceneDriver;

/// Sub-microsecond nudge applied to the Clip query time at the exact
/// scene-duration boundary so a clip whose `start + duration` equals
/// the parent scene's `duration_ms` remains visible at the settled
/// terminal frame. The 1 µs value is far below any realistic frame
/// quantum (16.67 ms at 60 fps), so it cannot flip the active flag
/// between frames sampled at frame boundaries.
const TERMINAL_BOUNDARY_EPSILON_MS: f32 = 0.001;

#[derive(Clone, Copy)]
pub struct SceneContext {
    pub clock: SceneClock,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub duration_ms: f32,
    pub adapters: Signal<FrameAdapterRegistry>,
    pub id_signal: Signal<Rc<str>>,
}

#[component]
pub fn Scene(
    id: String,
    width: u32,
    height: u32,
    duration_ms: f32,
    fps: Option<u32>,
    autoplay: Option<bool>,
    controls: Option<bool>,
    driver: Option<SceneDriver>,
    children: Element,
) -> Element {
    let fps = fps.unwrap_or(60).max(1);
    let autoplay = autoplay.unwrap_or(true);
    let reduced = use_reduced_motion();

    let clock = use_hook(|| SceneClock::new(duration_ms, fps, reduced));
    let registry = use_hook(FrameAdapterRegistry::default);
    let id_rc: Rc<str> = Rc::from(id.as_str());
    let id_signal = use_hook(|| Signal::new(id_rc.clone()));
    let adapters_signal = use_hook(|| Signal::new(registry.clone()));

    use_context_provider(|| SceneContext {
        clock,
        width,
        height,
        fps,
        duration_ms,
        adapters: adapters_signal,
        id_signal,
    });

    // Fan seek out to every registered adapter whenever elapsed_ms or
    // state changes. We read both signals so settle transitions emit
    // one final broadcast even if elapsed_ms is unchanged.
    use_effect(move || {
        let ms = *clock.elapsed_ms.read();
        let _ = *clock.state.read();
        let reduced = *clock.reduced.read();
        adapters_signal.read().broadcast_seek(ms, reduced);
    });

    let driver_for_effect = driver.clone();
    use_effect(move || {
        if reduced {
            return;
        }
        if let Some(d) = driver_for_effect.clone() {
            clock.play_with(d);
        } else if autoplay {
            clock.play();
        }
    });

    // Keep the clock's `reduced` flag in sync with the surrounding
    // `ReducedMotion` context. The Scene's clock was constructed once at
    // mount via `use_hook`, so without this sync the gallery's runtime
    // motion toggle (which flips the `ReducedMotion` context) would never
    // propagate to `clock.reduced` and the scene would keep playing.
    //
    // We read `use_reduced_motion()` inside the effect so the effect
    // subscribes to the `ReducedMotion` context directly — when the
    // context value flips, this effect re-runs and reconciles the clock.
    use_effect(move || {
        let now_reduced = use_reduced_motion();
        let was_reduced = *clock.reduced.peek();
        if now_reduced != was_reduced {
            let mut s = clock.reduced;
            s.set(now_reduced);
            if now_reduced {
                // Force-settle and drop any active autoplay handle so the
                // scrubber disables and the scene snaps to its final
                // frame immediately.
                clock.pause();
                clock.settle();
            }
        }
    });

    let elapsed = clock.elapsed_ms;
    let state = clock.state;
    let reduced_signal = clock.reduced;

    let state_attr = match *state.read() {
        SceneState::Paused => "paused",
        SceneState::Playing => "playing",
        SceneState::Settled => "settled",
    };
    let elapsed_attr = format!("{}", *elapsed.read() as i64);
    let duration_attr = format!("{}", duration_ms as i64);
    let reduced_attr = if *reduced_signal.read() {
        "true"
    } else {
        "false"
    };
    let aspect = format!("aspect-ratio: {} / {}", width, height);

    let show_transport = controls.unwrap_or(false);
    let duration_attr_for_input = duration_ms.max(0.0);
    let reduced_now = *reduced_signal.read();

    let play_label = if matches!(*state.read(), SceneState::Playing) {
        "Pause"
    } else {
        "Play"
    };

    let scrubber_value = format!("{}", *elapsed.read() as i64);
    let scrubber_max = format!("{}", duration_attr_for_input as i64);
    let time_text = format!(
        "{:.2}s / {:.2}s",
        *elapsed.read() / 1000.0,
        duration_ms / 1000.0
    );

    let scrubber_disabled = reduced_now;

    rsx! {
        section {
            class: "ui-scene-stage",
            style: "{aspect}",
            "data-composition-id": "{id}",
            "data-width": "{width}",
            "data-height": "{height}",
            "data-fps": "{fps}",
            "data-duration-ms": "{duration_attr}",
            "data-elapsed-ms": "{elapsed_attr}",
            "data-state": "{state_attr}",
            "data-reduced": "{reduced_attr}",
            {children}
            if show_transport {
                div { class: "ui-scene-transport",
                    button {
                        class: "ui-scene-play",
                        r#type: "button",
                        disabled: reduced_now,
                        onclick: move |_| {
                            if matches!(*state.read(), SceneState::Playing) {
                                clock.pause();
                            } else {
                                clock.play();
                            }
                        },
                        "{play_label}"
                    }
                    input {
                        class: "ui-scene-scrubber",
                        r#type: "range",
                        min: "0",
                        max: "{scrubber_max}",
                        step: "1",
                        value: "{scrubber_value}",
                        aria_disabled: if scrubber_disabled { "true" } else { "false" },
                        oninput: move |evt| {
                            if reduced_now {
                                return;
                            }
                            if let Ok(ms) = evt.value().parse::<f32>() {
                                clock.seek_ms(ms);
                            }
                        },
                    }
                    span { class: "ui-scene-time", "{time_text}" }
                    if reduced_now {
                        span { class: "ui-scene-reduced-tag",
                            "Reduced motion · settled state"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Clip(start_ms: f32, duration_ms: f32, fill: Option<ClipFill>, children: Element) -> Element {
    let fill = fill.unwrap_or(ClipFill::None);
    let ctx = try_consume_context::<SceneContext>();
    let Some(ctx) = ctx else {
        // Orphan clip (no Scene ancestor): render children, flag for diagnostics.
        return rsx! {
            div {
                class: "ui-scene-clip ui-scene-clip--orphan",
                "data-clip-orphan": "true",
                "data-clip-active": "true",
                {children}
            }
        };
    };

    let frame_clip = FrameClip::new(start_ms.max(0.0) as u32, duration_ms.max(0.0) as u32, fill);
    let elapsed = ctx.clock.elapsed_ms;
    let raw_ms = *elapsed.read();
    // `FrameClip::active_at_ms` uses an exclusive end (`ms < end`). When
    // the scene is at its terminal frame (settled, where
    // `elapsed_ms == duration_ms`), the last clip — whose
    // `start + duration == scene.duration_ms` — would otherwise be
    // reported inactive. Nudge the query time back by a sub-millisecond
    // epsilon at the terminal boundary so the final composed frame
    // matches the final played frame.
    let query_ms = if raw_ms >= ctx.duration_ms && ctx.duration_ms > 0.0 {
        raw_ms - TERMINAL_BOUNDARY_EPSILON_MS
    } else {
        raw_ms
    };
    let active = frame_clip.active_at_ms(query_ms);

    let style = if active {
        "opacity: 1"
    } else {
        match fill {
            ClipFill::None => "opacity: 0; visibility: hidden; pointer-events: none",
            ClipFill::HoldStart | ClipFill::HoldEnd | ClipFill::HoldBoth => "opacity: 1",
        }
    };
    let fill_attr = match fill {
        ClipFill::None => "none",
        ClipFill::HoldStart => "hold-start",
        ClipFill::HoldEnd => "hold-end",
        ClipFill::HoldBoth => "hold-both",
    };

    rsx! {
        div {
            class: "ui-scene-clip",
            style: "{style}",
            "data-start-ms": "{start_ms}",
            "data-duration-ms": "{duration_ms}",
            "data-fill": "{fill_attr}",
            "data-clip-active": "{active}",
            {children}
        }
    }
}
