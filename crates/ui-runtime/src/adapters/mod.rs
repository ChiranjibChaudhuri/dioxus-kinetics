mod sequence;
pub use sequence::SequenceAdapter;

#[cfg(target_arch = "wasm32")]
mod waapi;
#[cfg(not(target_arch = "wasm32"))]
#[path = "waapi_stub.rs"]
mod waapi;
pub use waapi::WaapiAdapter;
