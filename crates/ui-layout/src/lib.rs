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

const MIN_DIMENSION: f32 = 0.001;

/// Computes the FLIP delta from `last` back to `first`, assuming a top-left transform origin.
pub fn compute_flip(first: Rect, last: Rect) -> FlipDelta {
    let first = sanitize_rect(first);
    let last = sanitize_rect(last);

    FlipDelta {
        translate_x: finite_translation(first.x - last.x),
        translate_y: finite_translation(first.y - last.y),
        scale_x: compute_scale(first.width, last.width),
        scale_y: compute_scale(first.height, last.height),
    }
}

fn sanitize_rect(rect: Rect) -> Rect {
    Rect {
        x: finite_or_zero(rect.x),
        y: finite_or_zero(rect.y),
        width: finite_or_zero(rect.width),
        height: finite_or_zero(rect.height),
    }
}

fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() {
        value
    } else {
        0.0
    }
}

fn finite_translation(value: f32) -> f32 {
    if value.is_finite() {
        value
    } else {
        0.0
    }
}

fn compute_scale(first_dimension: f32, last_dimension: f32) -> f32 {
    if last_dimension.abs() < MIN_DIMENSION {
        return 1.0;
    }

    let scale = first_dimension / last_dimension;

    if scale.is_finite() {
        scale
    } else {
        1.0
    }
}
