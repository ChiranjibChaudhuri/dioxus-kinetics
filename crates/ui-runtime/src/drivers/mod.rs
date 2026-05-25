#[cfg(target_arch = "wasm32")]
mod scroll;
#[cfg(not(target_arch = "wasm32"))]
#[path = "scroll_stub.rs"]
mod scroll;
pub use scroll::{install_scroll_driver, ScrollDriverHandle};
