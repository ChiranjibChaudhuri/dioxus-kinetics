//! Scene Dioxus component: hosts a SceneClock, provides a
//! SceneContext, emits hyperframes-compatible data attributes. The
//! transport UI (scrubber, play/pause) and adapter fan-out arrive in
//! later tasks; this module ships the shell.

use std::rc::Rc;

use dioxus::prelude::*;
use ui_runtime::frame_adapter::FrameAdapterRegistry;
use ui_runtime::reduced_motion::use_reduced_motion;
use ui_runtime::scene_clock::{SceneClock, SceneState};

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
    children: Element,
) -> Element {
    let _ = controls; // wired in Task 11
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

    use_effect(move || {
        if autoplay && !reduced {
            clock.play();
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
    let reduced_attr = if *reduced_signal.read() { "true" } else { "false" };
    let aspect = format!("aspect-ratio: {} / {}", width, height);

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
        }
    }
}
