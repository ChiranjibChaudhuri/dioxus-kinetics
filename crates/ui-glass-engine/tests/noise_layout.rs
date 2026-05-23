use ui_glass_engine::noise::generate_noise_rgba;

#[test]
fn noise_returns_correct_byte_count() {
    let pixels = generate_noise_rgba(256, 256, 42);
    assert_eq!(pixels.len(), 256 * 256 * 4);
}

#[test]
fn noise_is_deterministic_for_same_seed() {
    let a = generate_noise_rgba(64, 64, 7);
    let b = generate_noise_rgba(64, 64, 7);
    assert_eq!(a, b);
}

#[test]
fn noise_differs_across_seeds() {
    let a = generate_noise_rgba(64, 64, 1);
    let b = generate_noise_rgba(64, 64, 2);
    assert_ne!(a, b);
}

#[test]
fn noise_covers_dynamic_range() {
    let p = generate_noise_rgba(128, 128, 99);
    let min = *p.iter().min().unwrap();
    let max = *p.iter().max().unwrap();
    assert!(min < 64, "noise floor too high: min={min}");
    assert!(max > 192, "noise ceiling too low: max={max}");
}
