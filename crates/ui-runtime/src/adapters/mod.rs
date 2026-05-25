mod sequence;
pub use sequence::SequenceAdapter;

#[cfg(target_arch = "wasm32")]
mod waapi;
#[cfg(not(target_arch = "wasm32"))]
#[path = "waapi_stub.rs"]
mod waapi;
pub use waapi::WaapiAdapter;

#[cfg(target_arch = "wasm32")]
mod css_keyframes;
#[cfg(not(target_arch = "wasm32"))]
#[path = "css_keyframes_stub.rs"]
mod css_keyframes;
pub use css_keyframes::CssKeyframesAdapter;
