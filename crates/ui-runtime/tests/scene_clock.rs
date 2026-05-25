use std::time::Duration;

use dioxus::core::{Runtime, RuntimeGuard};
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

/// Async variant of `with_runtime` for tokio tests that need Signal access
/// across `.await` points. Holds the `RuntimeGuard` on the thread-local
/// stack for the lifetime of `body`. Safe because the tests run on a
/// `current_thread` tokio runtime inside a `LocalSet`, so the task never
/// migrates between threads. `Signal::new` calls inside `body` must be
/// bracketed with `enter(|| ...)` (which pushes `ScopeId::ROOT`) since
/// `in_scope` only accepts a sync `FnOnce`; subsequent `peek`/`set` calls
/// reuse the signal's stored origin scope and need only the live runtime.
async fn with_runtime_async<F, Fut, O>(body: F) -> O
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = O>,
{
    let dom = VirtualDom::new(empty_app);
    let _runtime_guard = RuntimeGuard::new(dom.runtime());
    body().await
}

/// Run `f` with `ScopeId::ROOT` active on the current Dioxus runtime so
/// `Signal::new` inside `f` finds an owner. Must be called from inside
/// `with_runtime_async` (i.e. with a `RuntimeGuard` already pushed).
fn enter<R>(f: impl FnOnce() -> R) -> R {
    Runtime::current().in_scope(ScopeId::ROOT, f)
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

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn play_advances_elapsed_until_settled() {
    with_runtime_async(|| async {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let clock = enter(|| SceneClock::new(80.0, 60, false));
                clock.play();
                // Yield once so the spawned frame-loop task gets polled
                // (and registers its interval at virtual t=0) before we
                // advance virtual time. Without this, `advance` jumps
                // ahead before the task has captured its `last` Instant,
                // so the first measured `dt_ms` is 0.
                tokio::task::yield_now().await;
                // Each native scheduler tick is ~16ms; advance enough virtual
                // time to cross duration_ms = 80ms.
                tokio::time::advance(Duration::from_millis(200)).await;
                tokio::task::yield_now().await;
                assert!(clock.peek_elapsed_ms() >= 80.0 - 1e-3);
                assert_eq!(clock.peek_state(), SceneState::Settled);
            })
            .await;
    })
    .await;
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn pause_stops_advance() {
    with_runtime_async(|| async {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let clock = enter(|| SceneClock::new(10_000.0, 60, false));
                clock.play();
                // Yield so the frame-loop task starts before we advance.
                tokio::task::yield_now().await;
                tokio::time::advance(Duration::from_millis(40)).await;
                tokio::task::yield_now().await;
                let mid = clock.peek_elapsed_ms();
                assert!(
                    mid > 0.0,
                    "play should have advanced before pause; got {mid}"
                );
                clock.pause();
                tokio::time::advance(Duration::from_millis(200)).await;
                tokio::task::yield_now().await;
                assert!(
                    (clock.peek_elapsed_ms() - mid).abs() < 5.0,
                    "pause should freeze elapsed_ms; was {mid} now {}",
                    clock.peek_elapsed_ms()
                );
            })
            .await;
    })
    .await;
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn reduced_clock_play_is_noop() {
    with_runtime_async(|| async {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let clock = enter(|| SceneClock::new(500.0, 60, true));
                clock.play();
                tokio::time::advance(Duration::from_millis(200)).await;
                tokio::task::yield_now().await;
                assert!((clock.peek_elapsed_ms() - 500.0).abs() < f32::EPSILON);
                assert_eq!(clock.peek_state(), SceneState::Settled);
            })
            .await;
    })
    .await;
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn play_from_settled_rewinds_and_replays() {
    with_runtime_async(|| async {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let clock = enter(|| SceneClock::new(80.0, 60, false));
                clock.settle();
                assert_eq!(clock.peek_state(), SceneState::Settled);
                clock.play();
                tokio::task::yield_now().await;
                // After play() on a Settled clock, elapsed_ms must rewind to 0
                // and state must transition to Playing before the first tick.
                assert_eq!(clock.peek_elapsed_ms(), 0.0);
                assert_eq!(clock.peek_state(), SceneState::Playing);
                tokio::time::advance(Duration::from_millis(200)).await;
                tokio::task::yield_now().await;
                assert_eq!(clock.peek_state(), SceneState::Settled);
            })
            .await;
    })
    .await;
}
