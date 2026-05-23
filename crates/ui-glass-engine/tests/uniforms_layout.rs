use ui_glass_engine::GlassUniforms;

#[test]
fn uniforms_size_is_multiple_of_16_bytes() {
    let size = std::mem::size_of::<GlassUniforms>();
    assert_eq!(size % 16, 0, "uniform struct must be 16-byte aligned for wgpu");
}

#[test]
fn uniforms_zeroed_construction_compiles() {
    let _u: GlassUniforms = bytemuck::Zeroable::zeroed();
}

#[test]
fn uniforms_default_has_unit_thickness() {
    let u = GlassUniforms::default();
    assert_eq!(u.thickness, 1.0);
}
