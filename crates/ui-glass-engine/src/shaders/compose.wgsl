// Composite pass. Plan 1 implements: SDF rounded-rect mask, sample blurred
// backdrop, apply tint. Later plans light up REFRACT, DISPERSE, SPECULAR,
// INNER_SHADOW, AMBIENT_MESH, POINTER, SCROLL, TINT_ADAPT via the override
// constants below.

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
    surface_alpha:      f32,
};

@group(0) @binding(0) var<uniform> u: GlassUniforms;
@group(0) @binding(1) var bg_tex:  texture_2d<f32>;
@group(0) @binding(2) var bg_samp: sampler;
@group(0) @binding(3) var noise_tex: texture_2d<f32>;
@group(0) @binding(4) var noise_samp: sampler;
@group(0) @binding(5) var mipped_bg:   texture_2d<f32>;
@group(0) @binding(6) var mipped_samp: sampler;

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

fn orb(p: vec2<f32>, center: vec2<f32>, radius: f32) -> f32 {
    let d = length(p - center);
    return exp(-(d * d) / (radius * radius));
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
    var disp = vec2<f32>(0.0);
    if (FEAT_REFRACT) {
        let noise = textureSample(noise_tex, noise_samp,
            in.uv * u.noise_frequency + vec2<f32>(u.noise_seed, 0.0)
        ).xy * 2.0 - 1.0;
        disp = (normal * u.surface_curvature + noise) * u.refract_strength * u.thickness;
    }
    if (FEAT_POINTER) {
        // Pointer is in surface-local normalized space (-1..1).
        let pull = (u.pointer - vec2<f32>(local.x / half_size.x, local.y / half_size.y));
        disp = disp + pull * 0.5;
    }
    if (FEAT_SCROLL) {
        disp = disp + u.scroll_velocity * 0.02;
    }
    sample_uv = in.uv + disp * 0.01;

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

    if (FEAT_TINT_ADAPT) {
        // Sample a high mip of the mipmapped background (built per-frame by
        // Compositor::materialize_mipped_bg). Roughly a 64×-downscaled average.
        let avg = textureSampleLevel(mipped_bg, mipped_samp, sample_uv, 6.0).rgb;
        color = mix(color, color * avg, clamp(u.adapt_strength, 0.0, 1.0));
    }

    // Virtual light specular along normal. When the pointer feature is on the
    // light direction leans toward the cursor, so the highlight tracks it and
    // the glass reads as a dynamic, hand-lit material instead of a static one.
    if (FEAT_SPECULAR) {
        var light = u.light_dir;
        if (FEAT_POINTER) {
            light = normalize(mix(u.light_dir, normalize(u.pointer + vec2<f32>(1e-5)), 0.4));
        }
        let n_dot_l = max(dot(normal, light), 0.0);
        let spec = pow(n_dot_l, 4.0) * u.light_intensity;
        // Concentrate the highlight near the edge.
        let edge_mask = smoothstep(0.0, max(u.edge_falloff, 0.5), -sdf);
        color = color + vec3<f32>(spec * (1.0 - edge_mask));
    }

    if (FEAT_INNER_SHADOW) {
        let inner = smoothstep(-max(u.inner_shadow_px, 0.5), 0.0, sdf);
        color = color * (1.0 - inner * u.inner_shadow_alpha);
    }

    if (FEAT_AMBIENT_MESH) {
        // Animated mesh contribution: three soft color centers orbiting slowly.
        let t = u.time_seconds * 0.2;
        let p = local / max(half_size.x, 1.0);
        let c0 = vec3<f32>(0.30, 0.55, 0.95) * orb(p, vec2<f32>(cos(t),       sin(t)),       0.6);
        let c1 = vec3<f32>(0.90, 0.45, 0.65) * orb(p, vec2<f32>(cos(t + 2.1), sin(t + 2.1)), 0.6);
        let c2 = vec3<f32>(0.55, 0.95, 0.65) * orb(p, vec2<f32>(cos(t + 4.2), sin(t + 4.2)), 0.6);
        color = color + (c0 + c1 + c2) * 0.18;
    }

    // Premultiplied-alpha output: the compose pipeline blends with
    // PREMULTIPLIED_ALPHA_BLENDING, so multiply rgb by alpha here. surface_alpha
    // defaults to 1.0 (fully opaque — identical to the historical output); a
    // lower value composites the glass translucently over the page backdrop.
    let a = clamp(u.surface_alpha, 0.0, 1.0);
    return vec4<f32>(color * a, a);
}
