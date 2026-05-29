//! wgpu pipeline construction for the blur passes and composite pass.

use std::sync::Arc;

const BLUR_SRC: &str = include_str!("shaders/blur.wgsl");
const COMPOSE_SRC: &str = include_str!("shaders/compose.wgsl");

/// Negotiate a render-target texture format that's compatible with the
/// surface's reported capabilities. Used by the host (`ui-glass-dioxus`) to
/// pick a `TextureFormat` that the surface actually advertises, and then
/// feed the same value into `Compositor::with_output_format` so the compose
/// pipeline's `ColorTargetState` matches the surface's output view. Without
/// this matching step Chromium emits a per-frame "Invalid CommandBuffer"
/// warning when the surface's preferred format (e.g. `Bgra8UnormSrgb` on
/// Windows) disagrees with the pipeline's hardcoded format.
///
/// Preference order:
///   1. `Bgra8UnormSrgb` (Windows-friendly sRGB)
///   2. `Rgba8UnormSrgb` (Chromium-friendly sRGB)
///   3. `Bgra8Unorm` (legacy)
///   4. `Rgba8Unorm` (legacy)
///   5. Whatever the surface offers first
///   6. Fallback to `Rgba8UnormSrgb` (engine's historical default)
pub fn negotiate_surface_format(
    surface: &wgpu::Surface<'_>,
    adapter: &wgpu::Adapter,
) -> wgpu::TextureFormat {
    let caps = surface.get_capabilities(adapter);
    let preferred = [
        wgpu::TextureFormat::Bgra8UnormSrgb,
        wgpu::TextureFormat::Rgba8UnormSrgb,
        wgpu::TextureFormat::Bgra8Unorm,
        wgpu::TextureFormat::Rgba8Unorm,
    ];
    preferred
        .iter()
        .copied()
        .find(|f| caps.formats.contains(f))
        .or_else(|| caps.formats.first().copied())
        .unwrap_or(wgpu::TextureFormat::Rgba8UnormSrgb)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BlurDirection {
    Horizontal,
    Vertical,
}

pub fn build_blur_pipeline(
    device: &Arc<wgpu::Device>,
    direction: BlurDirection,
    taps: u32,
) -> wgpu::RenderPipeline {
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("blur.wgsl"),
        source: wgpu::ShaderSource::Wgsl(BLUR_SRC.into()),
    });

    let (dx, dy) = match direction {
        BlurDirection::Horizontal => (1.0, 0.0),
        BlurDirection::Vertical => (0.0, 1.0),
    };

    let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("blur-bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("blur-layout"),
        bind_group_layouts: &[&bind_layout],
        push_constant_ranges: &[],
    });

    let constants: &[(&str, f64)] = &[
        ("BLUR_DIRECTION_X", dx),
        ("BLUR_DIRECTION_Y", dy),
        ("BLUR_TAPS", taps as f64),
    ];

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("blur-pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants,
                zero_initialize_workgroup_memory: false,
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants,
                zero_initialize_workgroup_memory: false,
            },
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComposeKey {
    pub features: ui_glass::GlassFeatures,
}

pub fn compose_bind_group_layout(device: &Arc<wgpu::Device>) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("compose-bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 5,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 6,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

/// Build a compose pipeline targeting the engine's historical default format
/// (`Rgba8UnormSrgb`). Tests and callers that don't care about surface
/// negotiation use this; the runtime path goes through
/// [`build_compose_pipeline_with_format`] so the compose pipeline matches the
/// actual surface format reported by `wgpu::Surface::get_capabilities`.
pub fn build_compose_pipeline(device: &Arc<wgpu::Device>, key: ComposeKey) -> wgpu::RenderPipeline {
    build_compose_pipeline_with_format(device, key, wgpu::TextureFormat::Rgba8UnormSrgb)
}

/// Build a compose pipeline whose color target uses `target_format`. The
/// caller must ensure the output texture view it later binds to this pipeline
/// has the same format — typically the format returned by
/// [`negotiate_surface_format`] for the live `wgpu::Surface`.
pub fn build_compose_pipeline_with_format(
    device: &Arc<wgpu::Device>,
    key: ComposeKey,
    target_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    use ui_glass::GlassFeatures as F;
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("compose.wgsl"),
        source: wgpu::ShaderSource::Wgsl(COMPOSE_SRC.into()),
    });

    let bgl = compose_bind_group_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("compose-layout"),
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });

    let f = key.features;
    let constants: &[(&str, f64)] = &[
        (
            "FEAT_REFRACT",
            if f.contains(F::REFRACT) { 1.0 } else { 0.0 },
        ),
        (
            "FEAT_DISPERSE",
            if f.contains(F::DISPERSE) { 1.0 } else { 0.0 },
        ),
        (
            "FEAT_SPECULAR",
            if f.contains(F::SPECULAR) { 1.0 } else { 0.0 },
        ),
        (
            "FEAT_INNER_SHADOW",
            if f.contains(F::INNER_SHADOW) {
                1.0
            } else {
                0.0
            },
        ),
        (
            "FEAT_AMBIENT_MESH",
            if f.contains(F::AMBIENT_MESH) {
                1.0
            } else {
                0.0
            },
        ),
        (
            "FEAT_POINTER",
            if f.contains(F::POINTER) { 1.0 } else { 0.0 },
        ),
        ("FEAT_SCROLL", if f.contains(F::SCROLL) { 1.0 } else { 0.0 }),
        (
            "FEAT_TINT_ADAPT",
            if f.contains(F::TINT_ADAPT) { 1.0 } else { 0.0 },
        ),
    ];

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("compose-pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants,
                zero_initialize_workgroup_memory: false,
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: target_format,
                blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions {
                constants,
                zero_initialize_workgroup_memory: false,
            },
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlurKey {
    pub direction: BlurDirection,
    pub taps: u32,
}

pub fn blur_bind_group_layout(device: &Arc<wgpu::Device>) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("blur-bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

const MIPMAP_SRC: &str = include_str!("shaders/mipmap.wgsl");

pub fn mipmap_bind_group_layout(device: &Arc<wgpu::Device>) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("mipmap-bgl"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

pub fn build_mipmap_pipeline(device: &Arc<wgpu::Device>) -> wgpu::RenderPipeline {
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("mipmap.wgsl"),
        source: wgpu::ShaderSource::Wgsl(MIPMAP_SRC.into()),
    });

    let bgl = mipmap_bind_group_layout(device);
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("mipmap-layout"),
        bind_group_layouts: &[&bgl],
        push_constant_ranges: &[],
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("mipmap-pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &module,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}

// `cargo build` does NOT validate WGSL — wgpu compiles shaders via `naga` at
// runtime (on a GPU). These tests run `naga`'s WGSL front-end + validator over
// the embedded shader sources at `cargo test` time, so a syntax/type error
// (e.g. a uniform field mismatch with `uniforms.rs`, a bad `vec4` arity, or an
// undefined identifier) fails CI WITHOUT needing a GPU. `wgpu` re-exports the
// exact `naga` it uses, so validation matches the runtime pipeline.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod shader_validation_tests {
    use super::{BLUR_SRC, COMPOSE_SRC, MIPMAP_SRC};
    use wgpu::naga;

    fn validate(label: &str, src: &str) {
        let module = naga::front::wgsl::parse_str(src)
            .unwrap_or_else(|e| panic!("{label} should parse as WGSL: {e:?}"));
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        validator
            .validate(&module)
            .unwrap_or_else(|e| panic!("{label} should pass naga validation: {e:?}"));
    }

    #[test]
    fn compose_shader_is_valid_wgsl() {
        validate("compose.wgsl", COMPOSE_SRC);
    }

    #[test]
    fn blur_shader_is_valid_wgsl() {
        validate("blur.wgsl", BLUR_SRC);
    }

    #[test]
    fn mipmap_shader_is_valid_wgsl() {
        validate("mipmap.wgsl", MIPMAP_SRC);
    }
}
