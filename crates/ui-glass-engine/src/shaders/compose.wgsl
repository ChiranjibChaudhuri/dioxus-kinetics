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
@group(0) @binding(3) var noise_tex: texture_2d<f32>;
@group(0) @binding(4) var noise_samp: sampler;

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

    // Surface normal approximation from rounded-rect SDF.
    let half_size = u.rect.zw * 0.5;
    let edge_dist = half_size - abs(local);
    let normal = normalize(vec2<f32>(
        sign(local.x) * (1.0 - smoothstep(0.0, half_size.x * 0.5, edge_dist.x)),
        sign(local.y) * (1.0 - smoothstep(0.0, half_size.y * 0.5, edge_dist.y))
    ) + vec2<f32>(1e-5));

    // Refraction-displaced UV.
    var sample_uv = in.uv;
    if (FEAT_REFRACT) {
        let noise = textureSample(noise_tex, noise_samp,
            in.uv * u.noise_frequency + vec2<f32>(u.noise_seed, 0.0)
        ).xy * 2.0 - 1.0;
        let disp = (normal * u.surface_curvature + noise) * u.refract_strength * u.thickness;
        sample_uv = in.uv + disp * 0.01;
    }

    var bg = textureSample(bg_tex, bg_samp, sample_uv);

    // Chromatic dispersion: re-sample R and B with extra offset along normal.
    if (FEAT_DISPERSE) {
        let d = normal * (u.dispersion_px / u.canvas_size);
        let r_sample = textureSample(bg_tex, bg_samp, sample_uv + d).r;
        let b_sample = textureSample(bg_tex, bg_samp, sample_uv - d).b;
        bg = vec4<f32>(r_sample, bg.g, b_sample, bg.a);
    }

    // Saturation + tint mix
    var color = apply_saturation(bg.rgb, u.saturation);
    color = mix(color, u.tint.rgb, u.tint.a);

    // Virtual light specular along normal.
    if (FEAT_SPECULAR) {
        let n_dot_l = max(dot(normal, u.light_dir), 0.0);
        let spec = pow(n_dot_l, 16.0) * u.light_intensity;
        // Concentrate the highlight near the edge.
        let edge_mask = smoothstep(0.0, max(u.edge_falloff, 0.5), -sdf);
        color = color + vec3<f32>(spec * (1.0 - edge_mask));
    }

    return vec4<f32>(color, 1.0);
}
