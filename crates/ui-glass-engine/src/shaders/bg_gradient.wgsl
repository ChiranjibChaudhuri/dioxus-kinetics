override KIND: u32 = 0u; // 0=Color, 1=Linear, 2=Radial, 3=Conic
override STOP_COUNT: u32 = 2u;

struct BgUniforms {
    canvas_size: vec2<f32>,
    _pad0: vec2<f32>,
    direction: vec2<f32>,
    _pad1: vec2<f32>,
    center: vec2<f32>,
    radius: f32,
    start_angle_rad: f32,
    solid: vec4<f32>,
    stop_offsets: vec4<f32>,
    stop_offsets2: vec4<f32>,
    stop_colors_0: vec4<f32>,
    stop_colors_1: vec4<f32>,
    stop_colors_2: vec4<f32>,
    stop_colors_3: vec4<f32>,
    stop_colors_4: vec4<f32>,
    stop_colors_5: vec4<f32>,
    stop_colors_6: vec4<f32>,
    stop_colors_7: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u: BgUniforms;

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

fn stop_offset(i: u32) -> f32 {
    if (i == 0u) { return u.stop_offsets.x; }
    if (i == 1u) { return u.stop_offsets.y; }
    if (i == 2u) { return u.stop_offsets.z; }
    if (i == 3u) { return u.stop_offsets.w; }
    if (i == 4u) { return u.stop_offsets2.x; }
    if (i == 5u) { return u.stop_offsets2.y; }
    if (i == 6u) { return u.stop_offsets2.z; }
    return u.stop_offsets2.w;
}

fn stop_color(i: u32) -> vec4<f32> {
    if (i == 0u) { return u.stop_colors_0; }
    if (i == 1u) { return u.stop_colors_1; }
    if (i == 2u) { return u.stop_colors_2; }
    if (i == 3u) { return u.stop_colors_3; }
    if (i == 4u) { return u.stop_colors_4; }
    if (i == 5u) { return u.stop_colors_5; }
    if (i == 6u) { return u.stop_colors_6; }
    return u.stop_colors_7;
}

fn sample_gradient(t: f32) -> vec4<f32> {
    let tc = clamp(t, 0.0, 1.0);
    if (STOP_COUNT <= 1u) { return stop_color(0u); }
    for (var i: u32 = 1u; i < STOP_COUNT; i = i + 1u) {
        let a = stop_offset(i - 1u);
        let b = stop_offset(i);
        if (tc <= b) {
            let span = max(b - a, 1e-5);
            let f = (tc - a) / span;
            return mix(stop_color(i - 1u), stop_color(i), f);
        }
    }
    return stop_color(STOP_COUNT - 1u);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    if (KIND == 0u) {
        return u.solid;
    }
    if (KIND == 1u) {
        let p = in.uv - vec2<f32>(0.5);
        let t = dot(p, u.direction) + 0.5;
        return sample_gradient(t);
    }
    if (KIND == 2u) {
        let d = length(in.uv - u.center) / max(u.radius, 1e-5);
        return sample_gradient(d);
    }
    let v = in.uv - u.center;
    let angle = atan2(v.y, v.x) - u.start_angle_rad;
    let t = (angle / 6.28318) + 0.5;
    return sample_gradient(t - floor(t));
}
