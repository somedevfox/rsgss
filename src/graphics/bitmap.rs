use std::sync::Arc;

use wgpu::util::DeviceExt;

use super::{Graphics, Window, Vertex, shader::{Shader, BITMAP_SHADER}};

pub struct Bitmap {
    pub(crate) surface: Arc<wgpu::Surface>,
    pub(crate) buffer: Arc<wgpu::Buffer>,
    pub(crate) render_pipeline: Arc<wgpu::RenderPipeline>,

    pub shader: Shader
}

impl Bitmap {
    pub fn new(width: u32, height: u32) -> Self {
        let window = Window::get();
        let graphics = Graphics::get();

        let surface = unsafe { graphics.instance.create_surface(window.raw.as_ref()) };
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&graphics.adapter)[0],
            width, height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto
        };
        surface.configure(&graphics.device, &surface_configuration);

        let vertices = &[
            Vertex { position: [0., 0., 0.], color: [0., 0., 0.] }
        ];
        
        let buffer = graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX
        });

        let shader = BITMAP_SHADER.get().unwrap().clone();

        let render_pipeline = graphics.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&graphics.render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader.module(),
                entry_point: "vs_main",
                buffers: &[Vertex::desc()]
            },
            fragment: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, 
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            multiview: None
        });

        Self {
            surface: Arc::new(surface),
            buffer: Arc::new(buffer),
            render_pipeline: Arc::new(render_pipeline),

            shader
        }
    }
}