//! Orders the render passes: bg → blur H → blur V → composite into the
//! output target.

use std::sync::Arc;
use wgpu::util::DeviceExt;

use crate::pipeline::{
    blur_bind_group_layout, build_blur_pipeline, build_compose_pipeline, compose_bind_group_layout,
    BlurDirection, ComposeKey,
};
use crate::uniforms::{BlurUniforms, GlassUniforms};

/// One end-to-end pass: input bg texture → output RGBA8 texture, with the
/// material's blur radius applied via two separable passes and the composite
/// shader sampling the blurred result.
pub fn render_glass_to_texture(
    device: &Arc<wgpu::Device>,
    queue: &Arc<wgpu::Queue>,
    bg_view: &wgpu::TextureView,
    output_view: &wgpu::TextureView,
    uniforms: &GlassUniforms,
    compose_key: ComposeKey,
) {
    let (w, h) = (uniforms.canvas_size[0] as u32, uniforms.canvas_size[1] as u32);

    // Two scratch textures for separable blur.
    let make_scratch = |label: &str| {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
    };
    let scratch_h = make_scratch("blur-h");
    let scratch_v = make_scratch("blur-v");
    let scratch_h_view = scratch_h.create_view(&wgpu::TextureViewDescriptor::default());
    let scratch_v_view = scratch_v.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("linear-clamp"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    // Blur uniform buffer
    let blur_u = BlurUniforms::new(uniforms.canvas_size, uniforms.blur_radius);
    let blur_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("blur-uniforms"),
        contents: bytemuck::bytes_of(&blur_u),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let blur_bgl = blur_bind_group_layout(device);
    let bg_h = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("blur-h-bg"),
        layout: &blur_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: blur_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(bg_view) },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
        ],
    });
    let bg_v = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("blur-v-bg"),
        layout: &blur_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: blur_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&scratch_h_view) },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
        ],
    });

    let blur_h_pipeline = build_blur_pipeline(device, BlurDirection::Horizontal, 13);
    let blur_v_pipeline = build_blur_pipeline(device, BlurDirection::Vertical, 13);

    // Compose uniform + bind
    let compose_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("compose-uniforms"),
        contents: bytemuck::bytes_of(uniforms),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let compose_bgl = compose_bind_group_layout(device);
    let compose_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("compose-bg"),
        layout: &compose_bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: compose_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&scratch_v_view) },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&sampler) },
        ],
    });

    let compose_pipeline = build_compose_pipeline(device, compose_key);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("glass-frame"),
    });

    fn run_pass(
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        pipeline: &wgpu::RenderPipeline,
        bind: &wgpu::BindGroup,
        label: &str,
        clear: bool,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if clear {
                        wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT)
                    } else {
                        wgpu::LoadOp::Load
                    },
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind, &[]);
        pass.draw(0..3, 0..1);
    }

    run_pass(&mut encoder, &scratch_h_view, &blur_h_pipeline, &bg_h, "blur-h", true);
    run_pass(&mut encoder, &scratch_v_view, &blur_v_pipeline, &bg_v, "blur-v", true);
    run_pass(&mut encoder, output_view, &compose_pipeline, &compose_bg, "compose", true);

    queue.submit(Some(encoder.finish()));
}
