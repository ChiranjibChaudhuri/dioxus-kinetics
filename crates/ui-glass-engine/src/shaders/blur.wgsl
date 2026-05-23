// Separable Gaussian blur. Direction is set via the `BLUR_DIRECTION_X` /
// `BLUR_DIRECTION_Y` override pair: (1.0, 0.0) for horizontal, (0.0, 1.0) for
// vertical. The pipeline cache instantiates each direction once.

override BLUR_DIRECTION_X: f32 = 1.0;
override BLUR_DIRECTION_Y: f32 = 0.0;
override BLUR_TAPS: u32 = 13u;

struct BlurUniforms {
    canvas_size: vec2<f32>,
    blur_radius_px: f32,
    _pad: f32,
};

@group(0) @binding(0) var<uniform> u: BlurUniforms;
@group(0) @binding(1) var src_tex: texture_2d<f32>;
@group(0) @binding(2) var src_samp: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    // Full-screen triangle: covers NDC -1..1 with 3 verts.
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    var uv  = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );
    var out: VsOut;
    out.pos = vec4<f32>(pos[vid], 0.0, 1.0);
    out.uv  = uv[vid];
    return out;
}

fn gaussian_weight(i: i32, sigma: f32) -> f32 {
    let x = f32(i);
    return exp(-0.5 * (x * x) / (sigma * sigma));
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let dir = vec2<f32>(BLUR_DIRECTION_X, BLUR_DIRECTION_Y) / u.canvas_size;
    let radius = max(u.blur_radius_px, 0.0);
    let sigma = max(radius * 0.5, 0.5);
    let half_taps = i32(BLUR_TAPS / 2u);

    var acc = vec4<f32>(0.0);
    var weight_sum = 0.0;
    for (var i = -half_taps; i <= half_taps; i = i + 1) {
        let w = gaussian_weight(i, sigma);
        let offset = dir * f32(i) * radius;
        acc = acc + textureSample(src_tex, src_samp, in.uv + offset) * w;
        weight_sum = weight_sum + w;
    }
    return acc / max(weight_sum, 1e-5);
}
