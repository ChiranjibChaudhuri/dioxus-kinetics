#![forbid(unsafe_code)]

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlipDelta {
    pub translate_x: f32,
    pub translate_y: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

pub fn compute_flip(first: Rect, last: Rect) -> FlipDelta {
    FlipDelta {
        translate_x: first.x - last.x,
        translate_y: first.y - last.y,
        scale_x: if last.width == 0.0 {
            1.0
        } else {
            first.width / last.width
        },
        scale_y: if last.height == 0.0 {
            1.0
        } else {
            first.height / last.height
        },
    }
}
