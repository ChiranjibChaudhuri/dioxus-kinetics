// Composite pass. Plan 1 implements: SDF rounded-rect mask, sample blurred
// backdrop, apply tint. Later plans light up REFRACT, DISPERSE, SPECULAR,
// INNER_SHADOW, AMBIENT_MESH, POINTER, SCROLL, TINT_ADAPT via the override
// constants below.

override FEAT_BLUR:         bool = false;
override FEAT_REFRACT:      bool = false;
override FEAT_DISPERSE:     bool = false;
override FEAT_SPECULAR:     bool = false;
override FEAT_INNER_SHADOW: bool = false;
override FEAT_AMBIENT_MESH: bool = false;
override FEAT_POINTER:      bool = false;
override FEAT_SCROLL:       bool = false;
override FEAT_TINT_ADAPT:   bool = false;

struct GlassUniforms {
    rect:               vec4<f32>,
    tint:               vec4<f32>,
    canvas_size:        vec2<f32>,
    pointer:            vec2<f32>,
    scroll_velocity:    vec2<f32>,
    light_dir:          vec2<f32>,
    radius:             f32,
    thickness:          f32,
    blur_radius:        f32,
    saturation:         f32,
    refract_strength:   f32,
    surface_curvature:  f32,
    noise_frequency:    f32,
    noise_seed:         f32,
    dispersion_px:      f32,
    light_intensity:    f32,
    edge_falloff:       f32,
    inner_shadow_px:    f32,
    inner_shadow_alpha: f32,
    adapt_strength:     f32,
    time_seconds:       f32,
    _pad0:              f32,
};

@group(0) @binding(0) var<uniform> u: GlassUniforms;
@group(0) @binding(1) var bg_tex:  texture_2d<f32>;
@group(0) @binding(2) var bg_samp: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    var uv = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );
    var out: VsOut;
    out.pos = vec4<f32>(pos[vid], 0.0, 1.0);
    out.uv  = uv[vid];
    return out;
}

fn rounded_rect_sdf(p: vec2<f32>, half_size: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(r, r);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

fn apply_saturation(c: vec3<f32>, sat: f32) -> vec3<f32> {
    let luma = dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
    return mix(vec3<f32>(luma), c, sat);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // Map the fragment's UV onto the surface rect.
    let frag = in.uv * u.canvas_size;
    let local = frag - (u.rect.xy + u.rect.zw * 0.5);
    let sdf = rounded_rect_sdf(local, u.rect.zw * 0.5, u.radius);
    if (sdf > 0.0) { discard; }

    var bg = textureSample(bg_tex, bg_samp, in.uv);

    // Saturation + tint mix
    var color = apply_saturation(bg.rgb, u.saturation);
    color = mix(color, u.tint.rgb, u.tint.a);

    return vec4<f32>(color, 1.0);
}
