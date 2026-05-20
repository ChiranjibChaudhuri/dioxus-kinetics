#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Composition {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

impl Composition {
    pub fn new(id: impl Into<String>, width: u32, height: u32, frame_count: u32) -> Self {
        Self {
            id: id.into(),
            width,
            height,
            frame_count,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderTrack {
    pub composition_id: String,
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

impl RenderTrack {
    pub fn from_composition(composition: &Composition) -> Self {
        Self {
            composition_id: composition.id.clone(),
            width: composition.width,
            height: composition.height,
            frame_count: composition.frame_count,
        }
    }

    pub fn aspect_ratio(&self) -> &'static str {
        match (self.width, self.height) {
            (1920, 1080) | (1280, 720) => "16:9",
            (1080, 1920) => "9:16",
            (1080, 1080) => "1:1",
            _ => "custom",
        }
    }
}
