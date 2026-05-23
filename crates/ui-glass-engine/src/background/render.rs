//! Background renderer. Materializes BackgroundSource descriptors into an
//! RGBA8UnormSrgb texture suitable as a glass-pass bg input.

use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::background::{BackgroundSource, Gradient, GradientKind};

const SHADER: &str = include_str!("../shaders/bg_gradient.wgsl");
const MAX_STOPS: usize = 8;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct BgUniforms {
    canvas_size: [f32; 2],
    _pad0: [f32; 2],
    direction: [f32; 2],
    _pad1: [f32; 2],
    center: [f32; 2],
    radius: f32,
    start_angle_rad: f32,
    solid: [f32; 4],
    stop_offsets: [f32; 4],
    stop_offsets2: [f32; 4],
    stop_colors: [[f32; 4]; MAX_STOPS],
}

pub struct BackgroundRenderer {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    bgl: wgpu::BindGroupLayout,
}

impl BackgroundRenderer {
    pub fn new(device: Arc<wgpu::Device>, queue: Arc<wgpu::Queue>) -> Self {
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bg-render-bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        Self { device, queue, bgl }
    }

    pub fn render_to_texture(
        &mut self,
        sources: &[BackgroundSource],
        w: u32,
        h: u32,
    ) -> wgpu::Texture {
        let tex = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("bg-source-tex"),
            size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut first = true;
        for src in sources {
            let (uniforms, kind, stop_count) = self.uniforms_for(src, [w as f32, h as f32]);
            let pipeline = self.build_pipeline(kind, stop_count);
            let buf = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("bg-uniforms"),
                contents: bytemuck::bytes_of(&uniforms),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
            let bg = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("bg-bg"),
                layout: &self.bgl,
                entries: &[wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() }],
            });
            self.run_pass(&mut encoder, &view, &pipeline, &bg, first);
            first = false;
        }
        self.queue.submit(Some(encoder.finish()));
        tex
    }

    #[cfg(any(test, feature = "headless"))]
    pub fn render_to_pixels(&mut self, sources: &[BackgroundSource], w: u32, h: u32) -> Vec<u8> {
        let tex = self.render_to_texture(sources, w, h);
        read_back(&self.device, &self.queue, &tex, w, h)
    }

    fn uniforms_for(&self, src: &BackgroundSource, canvas: [f32; 2]) -> (BgUniforms, u32, u32) {
        let mut u = BgUniforms {
            canvas_size: canvas,
            _pad0: [0.0; 2],
            direction: [1.0, 0.0],
            _pad1: [0.0; 2],
            center: [0.5, 0.5],
            radius: 0.5,
            start_angle_rad: 0.0,
            solid: [0.0, 0.0, 0.0, 0.0],
            stop_offsets: [0.0; 4],
            stop_offsets2: [0.0; 4],
            stop_colors: [[0.0; 4]; MAX_STOPS],
        };

        match src {
            BackgroundSource::Color(c) => {
                u.solid = [c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a];
                (u, 0, 0)
            }
            BackgroundSource::Gradient(g) => {
                let (kind, stops) = self.write_gradient(g, &mut u);
                (u, kind, stops)
            }
            BackgroundSource::Image(_) | BackgroundSource::Mesh(_) => {
                (u, 0, 0)
            }
        }
    }

    fn write_gradient(&self, g: &Gradient, u: &mut BgUniforms) -> (u32, u32) {
        let stops = g.stops();
        let n = stops.len().min(MAX_STOPS);
        for (i, s) in stops.iter().take(n).enumerate() {
            let arr = if i < 4 { &mut u.stop_offsets } else { &mut u.stop_offsets2 };
            arr[i % 4] = s.offset;
            u.stop_colors[i] = [
                s.color.r as f32 / 255.0,
                s.color.g as f32 / 255.0,
                s.color.b as f32 / 255.0,
                s.color.a,
            ];
        }
        match g.kind() {
            GradientKind::Linear { angle_rad } => {
                u.direction = [angle_rad.cos(), angle_rad.sin()];
                (1, n as u32)
            }
            GradientKind::Radial { center, radius } => {
                u.center = center;
                u.radius = radius;
                (2, n as u32)
            }
            GradientKind::Conic { center, start_angle_rad } => {
                u.center = center;
                u.start_angle_rad = start_angle_rad;
                (3, n as u32)
            }
        }
    }

    fn build_pipeline(&self, kind: u32, stop_count: u32) -> wgpu::RenderPipeline {
        let module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bg_gradient.wgsl"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("bg-layout"),
            bind_group_layouts: &[&self.bgl],
            push_constant_ranges: &[],
        });
        let constants: &[(&str, f64)] = &[
            ("KIND", kind as f64),
            ("STOP_COUNT", stop_count.max(1) as f64),
        ];
        self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bg-pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &module, entry_point: Some("vs_main"),
                buffers: &[], compilation_options: wgpu::PipelineCompilationOptions {
                    constants, zero_initialize_workgroup_memory: false,
                },
            },
            fragment: Some(wgpu::FragmentState {
                module: &module, entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions {
                    constants, zero_initialize_workgroup_memory: false,
                },
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }

    fn run_pass(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        pipeline: &wgpu::RenderPipeline,
        bind: &wgpu::BindGroup,
        clear: bool,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("bg-source-pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: if clear { wgpu::LoadOp::Clear(wgpu::Color::BLACK) } else { wgpu::LoadOp::Load },
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None, occlusion_query_set: None,
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind, &[]);
        pass.draw(0..3, 0..1);
    }
}

#[cfg(any(test, feature = "headless"))]
fn read_back(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    tex: &wgpu::Texture, w: u32, h: u32,
) -> Vec<u8> {
    let bpr = ((w * 4 + 255) / 256) * 256;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("bg-readback"),
        size: (bpr * h) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex, mip_level: 0, origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(h),
            },
        },
        wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
    );
    queue.submit(Some(enc.finish()));
    let slice = buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| { tx.send(r).unwrap(); });
    let _ = device.poll(wgpu::PollType::Wait);
    rx.recv().unwrap().unwrap();
    let data = slice.get_mapped_range();
    let mut out = Vec::with_capacity((w * h * 4) as usize);
    for row in 0..h {
        let start = (row * bpr) as usize;
        out.extend_from_slice(&data[start..start + (w * 4) as usize]);
    }
    drop(data);
    buf.unmap();
    out
}
