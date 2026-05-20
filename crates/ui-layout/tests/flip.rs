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
