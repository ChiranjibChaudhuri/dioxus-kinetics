//! Background-source descriptors. CPU types only — no wgpu. The renderer that
//! turns these into a texture lives in `background::render`.

use ui_tokens::Color;

pub mod image_cache;
pub mod render;

pub use image_cache::{ImageCache, ImageHandle};

/// A single layer of the background scene. Compositors materialize one of
/// these (or the per-surface variant) into the texture that glass surfaces
/// sample from.
#[derive(Clone, Debug, PartialEq)]
pub enum BackgroundSource {
    Color(Color),
    Gradient(Gradient),
    Image(ImageSource),
    Mesh(MeshKind),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImageSource {
    /// URL or path; resolved through ImageCache.
    Static(String),
    /// Externally-uploaded texture; handle owned by ImageCache.
    Dynamic(ImageHandle),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshKind {
    Aurora,
    Orbs,
    Grain,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientStop {
    pub offset: f32,
    pub color: Color,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Gradient {
    kind: GradientKind,
    stops: Vec<GradientStop>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum GradientKind {
    Linear {
        angle_rad: f32,
    },
    Radial {
        center: [f32; 2],
        radius: f32,
    },
    Conic {
        center: [f32; 2],
        start_angle_rad: f32,
    },
}

impl Gradient {
    pub fn linear(angle_rad: f32, stops: Vec<GradientStop>) -> Self {
        Self {
            kind: GradientKind::Linear { angle_rad },
            stops,
        }
    }

    pub fn radial(center: [f32; 2], radius: f32, stops: Vec<GradientStop>) -> Self {
        Self {
            kind: GradientKind::Radial { center, radius },
            stops,
        }
    }

    pub fn conic(center: [f32; 2], start_angle_rad: f32, stops: Vec<GradientStop>) -> Self {
        Self {
            kind: GradientKind::Conic {
                center,
                start_angle_rad,
            },
            stops,
        }
    }

    pub fn stops(&self) -> &[GradientStop] {
        &self.stops
    }
    pub fn is_linear(&self) -> bool {
        matches!(self.kind, GradientKind::Linear { .. })
    }
    pub fn is_radial(&self) -> bool {
        matches!(self.kind, GradientKind::Radial { .. })
    }
    pub fn is_conic(&self) -> bool {
        matches!(self.kind, GradientKind::Conic { .. })
    }

    pub(crate) fn kind(&self) -> GradientKind {
        self.kind
    }
}

#[derive(Clone, Debug, Default)]
pub struct BackgroundScene {
    pub layers: Vec<BackgroundSource>,
}

impl BackgroundScene {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn layer(mut self, source: BackgroundSource) -> Self {
        self.layers.push(source);
        self
    }
    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }
}
