override MESH_KIND: u32 = 0u; // 0=Aurora, 1=Orbs, 2=Grain

struct MeshUniforms {
    canvas_size: vec2<f32>,
    time_seconds: f32,
    _pad: f32,
};

@group(0) @binding(0) var<uniform> u: MeshUniforms;

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

fn hash21(p: vec2<f32>) -> f32 {
    let q = vec2<f32>(dot(p, vec2<f32>(127.1, 311.7)), dot(p, vec2<f32>(269.5, 183.3)));
    return fract(sin(dot(q, vec2<f32>(43758.5453, 12345.6789))) * 43758.5453);
}

fn aurora(uv: vec2<f32>, t: f32) -> vec3<f32> {
    let y = uv.y;
    let b1 = exp(-pow((y - 0.3 + 0.05 * sin(t)), 2.0) * 25.0);
    let b2 = exp(-pow((y - 0.6 + 0.05 * sin(t + 1.7)), 2.0) * 25.0);
    let b3 = exp(-pow((y - 0.9 + 0.05 * sin(t + 3.4)), 2.0) * 25.0);
    let c1 = vec3<f32>(0.18, 0.55, 0.85) * b1;
    let c2 = vec3<f32>(0.85, 0.35, 0.55) * b2;
    let c3 = vec3<f32>(0.40, 0.85, 0.55) * b3;
    return c1 + c2 + c3 + vec3<f32>(0.05);
}

fn orbs(uv: vec2<f32>, t: f32) -> vec3<f32> {
    let c0 = vec2<f32>(0.5 + 0.3 * cos(t),       0.5 + 0.3 * sin(t));
    let c1 = vec2<f32>(0.5 + 0.3 * cos(t + 2.0), 0.5 + 0.3 * sin(t + 2.0));
    let c2 = vec2<f32>(0.5 + 0.3 * cos(t + 4.0), 0.5 + 0.3 * sin(t + 4.0));
    let r = 0.25;
    let f0 = exp(-pow(length(uv - c0) / r, 2.0));
    let f1 = exp(-pow(length(uv - c1) / r, 2.0));
    let f2 = exp(-pow(length(uv - c2) / r, 2.0));
    return vec3<f32>(0.30, 0.55, 0.95) * f0
         + vec3<f32>(0.90, 0.45, 0.65) * f1
         + vec3<f32>(0.55, 0.95, 0.65) * f2
         + vec3<f32>(0.05);
}

fn grain(uv: vec2<f32>, t: f32) -> vec3<f32> {
    let base = vec3<f32>(0.15);
    let n = hash21(uv * 256.0 + vec2<f32>(t));
    return base + vec3<f32>(n) * 0.4;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let t = u.time_seconds * 0.3;
    var c: vec3<f32>;
    if (MESH_KIND == 0u) { c = aurora(in.uv, t); }
    else if (MESH_KIND == 1u) { c = orbs(in.uv, t); }
    else { c = grain(in.uv, t); }
    return vec4<f32>(c, 1.0);
}
