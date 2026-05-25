#![cfg(not(target_arch = "wasm32"))]

use crate::scene_driver::ScrollObserverConfig;

/// Cleanup handle returned by `install_scroll_driver`. On native this
/// is a no-op marker; on web it holds the IntersectionObserver and the
/// scroll event listener closure, both of which clean up on Drop.
pub struct ScrollDriverHandle {
    _private: (),
}

impl Drop for ScrollDriverHandle {
    fn drop(&mut self) {
        // No-op on native.
    }
}

/// Installs the scroll driver for the given `config`, invoking
/// `on_progress(f32)` on every progress update. Returns a handle that
/// disconnects the observer + listener when dropped.
///
/// On native targets the function is a no-op and returns a handle that
/// holds the clock at progress 0 for its lifetime.
pub fn install_scroll_driver(
    _config: &ScrollObserverConfig,
    _on_progress: impl FnMut(f32) + 'static,
) -> ScrollDriverHandle {
    ScrollDriverHandle { _private: () }
}
