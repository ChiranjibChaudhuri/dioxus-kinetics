#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Composition {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub frame_count: u32,
}

impl Composition {
    pub fn new(id: impl Into<String>, width: u32, height: u32, fps: u32, frame_count: u32) -> Self {
        Self {
            id: id.into(),
            width,
            height,
            fps,
            frame_count,
        }
    }
}
