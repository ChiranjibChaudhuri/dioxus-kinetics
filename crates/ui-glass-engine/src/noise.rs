//! CPU noise generator. Produces a 2D RGBA8 texture combining gradient noise
//! at three frequency bands (R/G/B channels). Alpha is 255. The shader uses
//! the R channel as primary displacement and G as detail.
//!
//! This is intentionally simple and self-contained: no external noise crates.
//! Quality is "good enough for a 256×256 turbulence texture", not photoreal.

/// Generate an RGBA8 noise texture of the given size and seed.
pub fn generate_noise_rgba(w: u32, h: u32, seed: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity((w * h * 4) as usize);
    for y in 0..h {
        for x in 0..w {
            let fx = x as f32 / w as f32;
            let fy = y as f32 / h as f32;

            let r = (gradient_noise(fx * 4.0, fy * 4.0, seed) * 0.5 + 0.5).clamp(0.0, 1.0);
            let g = (gradient_noise(fx * 12.0, fy * 12.0, seed.wrapping_add(1)) * 0.5 + 0.5)
                .clamp(0.0, 1.0);
            let b = (gradient_noise(fx * 2.0, fy * 2.0, seed.wrapping_add(2)) * 0.5 + 0.5)
                .clamp(0.0, 1.0);

            out.push((r * 255.0) as u8);
            out.push((g * 255.0) as u8);
            out.push((b * 255.0) as u8);
            out.push(255);
        }
    }
    out
}

fn gradient_noise(x: f32, y: f32, seed: u64) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let xf = x - xi as f32;
    let yf = y - yi as f32;
    let u = smoothstep(xf);
    let v = smoothstep(yf);

    let g00 = rand_unit(xi, yi, seed);
    let g10 = rand_unit(xi + 1, yi, seed);
    let g01 = rand_unit(xi, yi + 1, seed);
    let g11 = rand_unit(xi + 1, yi + 1, seed);

    let x1 = lerp(g00, g10, u);
    let x2 = lerp(g01, g11, u);
    lerp(x1, x2, v)
}

fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn rand_unit(x: i32, y: i32, seed: u64) -> f32 {
    let mut h = seed
        .wrapping_add((x as i64 as u64).wrapping_mul(73856093))
        .wrapping_add((y as i64 as u64).wrapping_mul(19349663));
    h ^= h >> 13;
    h = h.wrapping_mul(1274126177);
    h ^= h >> 16;
    (h as u32 as f32 / u32::MAX as f32) * 2.0 - 1.0
}
