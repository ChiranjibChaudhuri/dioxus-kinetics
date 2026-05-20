use ui_layout::{compute_flip, Rect};

#[test]
fn flip_delta_moves_from_last_box_back_to_first_box() {
    let first = Rect::new(10.0, 20.0, 100.0, 50.0);
    let last = Rect::new(30.0, 45.0, 200.0, 100.0);
    let delta = compute_flip(first, last);

    assert_eq!(delta.translate_x, -20.0);
    assert_eq!(delta.translate_y, -25.0);
    assert_eq!(delta.scale_x, 0.5);
    assert_eq!(delta.scale_y, 0.5);
}

#[test]
fn zero_sized_last_box_uses_identity_scale() {
    let first = Rect::new(0.0, 0.0, 100.0, 50.0);
    let last = Rect::new(0.0, 0.0, 0.0, 0.0);
    let delta = compute_flip(first, last);

    assert_eq!(delta.scale_x, 1.0);
    assert_eq!(delta.scale_y, 1.0);
}

#[test]
fn non_finite_rect_values_produce_finite_delta() {
    let first = Rect::new(f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::NAN);
    let last = Rect::new(f32::INFINITY, f32::NEG_INFINITY, f32::NAN, f32::INFINITY);
    let delta = compute_flip(first, last);

    assert!(delta.translate_x.is_finite());
    assert!(delta.translate_y.is_finite());
    assert!(delta.scale_x.is_finite());
    assert!(delta.scale_y.is_finite());
    assert_eq!(delta.translate_x, 0.0);
    assert_eq!(delta.translate_y, 0.0);
    assert_eq!(delta.scale_x, 1.0);
    assert_eq!(delta.scale_y, 1.0);
}

#[test]
fn tiny_non_zero_last_dimensions_use_computed_scale() {
    let first = Rect::new(0.0, 0.0, 100.0, 50.0);
    let last = Rect::new(0.0, 0.0, 0.0005, -0.0005);
    let delta = compute_flip(first, last);

    assert_eq!(delta.scale_x, first.width / last.width);
    assert_eq!(delta.scale_y, first.height / last.height);
}
