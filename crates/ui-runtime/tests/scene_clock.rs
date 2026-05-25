use dioxus::prelude::*;
use ui_runtime::scene_clock::{SceneClock, SceneState};

fn empty_app() -> Element {
    rsx! { div {} }
}

/// Wraps a test body in a Dioxus runtime + root scope so `Signal::new`
/// (which requires both) succeeds. We use `VirtualDom::new(...)` together
/// with `in_runtime` and `in_scope(ScopeId::ROOT, ...)` instead of the
/// SSR-probe pattern from `hooks_ssr.rs` because these tests call
/// `SceneClock` methods directly; driving them through an SSR probe would
/// require re-rendering after each mutation and asserting on DOM scrape,
/// which is more brittle than direct method calls + signal peek. The
/// VirtualDom is dropped after `body` returns, which also cleans up the
/// signal storage.
fn with_runtime<R>(body: impl FnOnce() -> R) -> R {
    let dom = VirtualDom::new(empty_app);
    dom.in_runtime(|| dom.in_scope(ScopeId::ROOT, body))
}

#[test]
fn new_clock_starts_paused_at_zero() {
    with_runtime(|| {
        let clock = SceneClock::new(1_000.0, 60, false);
        assert_eq!(clock.peek_elapsed_ms(), 0.0);
        assert_eq!(clock.peek_state(), SceneState::Paused);
        assert!(!clock.peek_reduced());
    });
}

#[test]
fn reduced_constructor_settles_immediately() {
    with_runtime(|| {
        let clock = SceneClock::new(1_000.0, 60, true);
        assert!(clock.peek_reduced());
        assert_eq!(clock.peek_state(), SceneState::Settled);
        assert!((clock.peek_elapsed_ms() - 1_000.0).abs() < f32::EPSILON);
    });
}

#[test]
fn seek_ms_clamps_low() {
    with_runtime(|| {
        let clock = SceneClock::new(1_000.0, 60, false);
        clock.seek_ms(-50.0);
        assert_eq!(clock.peek_elapsed_ms(), 0.0);
        assert_eq!(clock.peek_state(), SceneState::Paused);
    });
}

#[test]
fn seek_ms_clamps_high_and_settles() {
    with_runtime(|| {
        let clock = SceneClock::new(1_000.0, 60, false);
        clock.seek_ms(2_000.0);
        assert!((clock.peek_elapsed_ms() - 1_000.0).abs() < f32::EPSILON);
        assert_eq!(clock.peek_state(), SceneState::Settled);
    });
}

#[test]
fn seek_progress_maps_to_duration() {
    with_runtime(|| {
        let clock = SceneClock::new(2_000.0, 60, false);
        clock.seek_progress(0.5);
        assert!((clock.peek_elapsed_ms() - 1_000.0).abs() < 1e-3);
    });
}

#[test]
fn settle_jumps_to_duration_ms() {
    with_runtime(|| {
        let clock = SceneClock::new(750.0, 60, false);
        clock.settle();
        assert!((clock.peek_elapsed_ms() - 750.0).abs() < f32::EPSILON);
        assert_eq!(clock.peek_state(), SceneState::Settled);
    });
}

#[test]
fn frame_clock_derives_from_elapsed_and_fps() {
    with_runtime(|| {
        let clock = SceneClock::new(1_000.0, 30, false);
        clock.seek_ms(500.0);
        let fc = clock.frame_clock();
        assert_eq!(fc.frame, 15);
        assert_eq!(fc.fps, 30);
    });
}
