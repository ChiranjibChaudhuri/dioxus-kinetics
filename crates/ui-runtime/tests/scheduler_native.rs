#![cfg(not(target_arch = "wasm32"))]

use std::sync::{Arc, Mutex};
use std::time::Duration;
use ui_runtime::scheduler::{spawn_frame_loop, ControlFlow};

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn frame_loop_invokes_callback_until_stop() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let counter = Arc::new(Mutex::new(0u32));
            let counter_clone = counter.clone();
            let handle = spawn_frame_loop(move |_dt| {
                let mut c = counter_clone.lock().unwrap();
                *c += 1;
                if *c >= 3 {
                    ControlFlow::Stop
                } else {
                    ControlFlow::Continue
                }
            });

            for _ in 0..6 {
                tokio::time::advance(Duration::from_millis(16)).await;
                tokio::task::yield_now().await;
            }

            assert_eq!(*counter.lock().unwrap(), 3);
            drop(handle);
        })
        .await;
}

#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn dropping_handle_stops_the_loop() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let counter = Arc::new(Mutex::new(0u32));
            let counter_clone = counter.clone();
            let handle = spawn_frame_loop(move |_dt| {
                *counter_clone.lock().unwrap() += 1;
                ControlFlow::Continue
            });

            tokio::time::advance(Duration::from_millis(16)).await;
            tokio::task::yield_now().await;
            let after_one = *counter.lock().unwrap();

            drop(handle);

            for _ in 0..4 {
                tokio::time::advance(Duration::from_millis(16)).await;
                tokio::task::yield_now().await;
            }

            assert_eq!(*counter.lock().unwrap(), after_one);
        })
        .await;
}

#[tokio::test(flavor = "current_thread")]
async fn spawn_inside_runtime_does_not_panic() {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let counter = Arc::new(Mutex::new(0u32));
            let counter_clone = counter.clone();
            let _handle = spawn_frame_loop(move |_dt| {
                *counter_clone.lock().unwrap() += 1;
                ControlFlow::Stop
            });
        })
        .await;
}
