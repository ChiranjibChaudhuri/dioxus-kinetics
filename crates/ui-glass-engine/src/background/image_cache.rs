//! Texture cache for `Image::Static(...)` and external `Image::Dynamic(handle)`.
//! Real impl lands in Task 5; this stub keeps the public types defined so the
//! BackgroundSource enum compiles.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageHandle(pub u64);

pub struct ImageCache;
