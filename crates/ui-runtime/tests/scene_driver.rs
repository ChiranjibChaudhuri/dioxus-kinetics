use ui_runtime::scene_driver::{SceneDriver, ScrollObserverConfig};

#[test]
fn autoplay_is_default() {
    let d = SceneDriver::default();
    assert!(matches!(d, SceneDriver::Autoplay));
}

#[test]
fn manual_is_distinct_from_autoplay() {
    let m = SceneDriver::Manual;
    let a = SceneDriver::Autoplay;
    assert_ne!(m, a);
}

#[test]
fn scroll_carries_observer_config() {
    let config = ScrollObserverConfig {
        trigger_selector: "#hero".to_string(),
        start_offset_px: Some(100.0),
        end_offset_px: Some(0.0),
    };
    let d = SceneDriver::Scroll(config.clone());
    match d {
        SceneDriver::Scroll(c) => {
            assert_eq!(c.trigger_selector, "#hero");
            assert_eq!(c.start_offset_px, Some(100.0));
        }
        _ => panic!("expected Scroll"),
    }
}

mod helpers {
    use dioxus::core::{Runtime, RuntimeGuard};
    use dioxus::prelude::*;

    fn empty_app() -> Element {
        rsx! { div {} }
    }

    /// Async variant of `with_runtime` for tokio tests that need Signal
    /// access across `.await` points. Mirrors the helper in
    /// `tests/scene_clock.rs`.
    pub async fn with_runtime_async<F, Fut, O>(body: F) -> O
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
    /// `with_runtime_async`.
    pub fn enter<R>(f: impl FnOnce() -> R) -> R {
        Runtime::current().in_scope(ScopeId::ROOT, f)
    }
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn manual_driver_skips_autoplay() {
    use std::time::Duration;
    use ui_runtime::scene_clock::{SceneClock, SceneState};

    helpers::with_runtime_async(|| async {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let clock = helpers::enter(|| SceneClock::new(1_000.0, 60, false));
                clock.play_with(SceneDriver::Manual);
                tokio::task::yield_now().await;
                tokio::time::advance(Duration::from_millis(200)).await;
                tokio::task::yield_now().await;
                assert_eq!(clock.peek_elapsed_ms(), 0.0);
                assert_eq!(clock.peek_state(), SceneState::Paused);
            })
            .await;
    })
    .await;
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn autoplay_driver_advances_as_before() {
    use std::time::Duration;
    use ui_runtime::scene_clock::{SceneClock, SceneState};

    helpers::with_runtime_async(|| async {
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let clock = helpers::enter(|| SceneClock::new(80.0, 60, false));
                clock.play_with(SceneDriver::Autoplay);
                tokio::task::yield_now().await;
                tokio::time::advance(Duration::from_millis(200)).await;
                tokio::task::yield_now().await;
                assert!(clock.peek_elapsed_ms() >= 80.0 - 1e-3);
                assert_eq!(clock.peek_state(), SceneState::Settled);
            })
            .await;
    })
    .await;
}
