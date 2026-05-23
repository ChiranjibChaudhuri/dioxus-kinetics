use ui_glass_engine::capabilities::{Capabilities, Tier};

#[test]
fn high_contrast_overrides_to_solid() {
    let caps = Capabilities {
        has_webgpu: true,
        has_webgl2: true,
        has_backdrop_filter: true,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: true,
    };
    assert_eq!(caps.best_tier(), Tier::SolidCss);
}

#[test]
fn reduced_transparency_overrides_to_solid() {
    let caps = Capabilities {
        has_webgpu: true,
        has_webgl2: true,
        has_backdrop_filter: true,
        reduced_motion: false,
        reduced_transparency: true,
        high_contrast: false,
    };
    assert_eq!(caps.best_tier(), Tier::SolidCss);
}

#[test]
fn webgpu_preferred_when_available() {
    let caps = Capabilities {
        has_webgpu: true,
        has_webgl2: true,
        has_backdrop_filter: true,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    };
    assert_eq!(caps.best_tier(), Tier::WgpuWebGpu);
}

#[test]
fn falls_through_to_webgl2_when_no_webgpu() {
    let caps = Capabilities {
        has_webgpu: false,
        has_webgl2: true,
        has_backdrop_filter: true,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    };
    assert_eq!(caps.best_tier(), Tier::WgpuWebGl2);
}

#[test]
fn falls_through_to_svg_when_only_backdrop_filter() {
    let caps = Capabilities {
        has_webgpu: false,
        has_webgl2: false,
        has_backdrop_filter: true,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    };
    assert_eq!(caps.best_tier(), Tier::SvgFilter);
}

#[test]
fn solid_when_nothing_available() {
    let caps = Capabilities {
        has_webgpu: false,
        has_webgl2: false,
        has_backdrop_filter: false,
        reduced_motion: false,
        reduced_transparency: false,
        high_contrast: false,
    };
    assert_eq!(caps.best_tier(), Tier::SolidCss);
}
