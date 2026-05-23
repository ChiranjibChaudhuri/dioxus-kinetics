//! Behavioral assertions: each Tier 1 feature should observably change the
//! pixels it touches. These tests don't compare to a golden — they compare
//! "feature on" to "feature off" and assert that *some* fraction of pixels
//! differ. This guards against a feature regressing into a no-op.

mod common;
use common::render_with_material;
use ui_glass::{AmbientMesh, LiquidMaterial};

fn diff_count(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b.iter()).filter(|(x, y)| x.abs_diff(**y) > 1).count()
}

const W: u32 = 96;
const H: u32 = 96;
const MIN_AFFECTED_FRACTION: f64 = 0.02;

fn base() -> LiquidMaterial {
    LiquidMaterial::new()
        .blur(8.0)
        .radius(20.0)
        .tint(ui_tokens::Color::rgba(255, 255, 255, 1.0), 0.1)
}

#[test]
fn refract_changes_output() {
    let off = render_with_material(W, H, base());
    let on = render_with_material(W, H, base().refract(0.5).noise(2.0, 0.0).surface_curvature(0.5).thickness(2.0));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(frac > MIN_AFFECTED_FRACTION, "REFRACT changed only {:.2}% of pixels", frac * 100.0);
}

#[test]
fn disperse_changes_output() {
    let off = render_with_material(W, H, base());
    let on = render_with_material(W, H, base().disperse(4.0));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(frac > MIN_AFFECTED_FRACTION, "DISPERSE changed only {:.2}% of pixels", frac * 100.0);
}

#[test]
fn specular_changes_output() {
    let off = render_with_material(W, H, base());
    let on = render_with_material(W, H, base().specular(45.0_f32.to_radians(), 0.8).edge_falloff(2.0));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(frac > MIN_AFFECTED_FRACTION, "SPECULAR changed only {:.2}% of pixels", frac * 100.0);
}

#[test]
fn inner_shadow_changes_output() {
    let off = render_with_material(W, H, base());
    let on = render_with_material(W, H, base().inner_shadow(8.0, 0.5));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(frac > MIN_AFFECTED_FRACTION, "INNER_SHADOW changed only {:.2}% of pixels", frac * 100.0);
}

#[test]
fn ambient_mesh_changes_output() {
    let off = render_with_material(W, H, base());
    let on = render_with_material(W, H, base().ambient_mesh(AmbientMesh::Aurora));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(frac > MIN_AFFECTED_FRACTION, "AMBIENT_MESH changed only {:.2}% of pixels", frac * 100.0);
}

#[test]
fn tint_adapt_changes_output() {
    let off = render_with_material(W, H, base());
    let on = render_with_material(W, H, base().adapt_to_background(0.5));
    let frac = diff_count(&off, &on) as f64 / off.len() as f64;
    assert!(frac > MIN_AFFECTED_FRACTION, "TINT_ADAPT changed only {:.2}% of pixels", frac * 100.0);
}
