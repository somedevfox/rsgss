// Copyright (C) 2023 Egor Poleshko
//
// This file is part of rsgss.
//
// rsgss is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rsgss is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rsgss.  If not, see <http://www.gnu.org/licenses/>.

pub mod window;
pub mod viewport;
pub mod shader;
pub mod bitmap;
pub use window::Window;

use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use std::sync::Arc;
use core::mem;

use self::shader::{BITMAP_SHADER, Shader};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
	position: [f32; 3],
	color: [f32; 3]
}
impl Vertex {
	pub(crate) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		wgpu::VertexBufferLayout { 
			array_stride: mem::size_of::<Self>() as wgpu::BufferAddress, 
			step_mode: wgpu::VertexStepMode::Vertex, 
			attributes: &[
				wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
			]
		}
	}
}

pub trait Renderable {
    fn render(&self);
}

pub static mut GRAPHICS: OnceCell<Graphics> = OnceCell::new();

#[derive(Debug)]
pub struct Graphics {
    pub(crate) instance: Arc<wgpu::Instance>,
    pub(crate) adapter: Arc<wgpu::Adapter>,
    pub(crate) device: Arc<wgpu::Device>,
	pub(crate) queue: Arc<wgpu::Queue>,

	pub(crate) render_pipeline_layout: Arc<wgpu::PipelineLayout>
}
unsafe impl Sync for Graphics {}
unsafe impl Send for Graphics {}
impl Graphics {
    pub fn get() -> &'static Self {
        unsafe { GRAPHICS.get() }.unwrap()
    }

    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let mut adapter_options = wgpu::RequestAdapterOptions::default();
        adapter_options.power_preference = wgpu::PowerPreference::HighPerformance;
        let adapter = instance.request_adapter(&adapter_options).await.unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .unwrap();

		let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
			label: None, 
			bind_group_layouts: &[], 
			push_constant_ranges: &[] 
		});

		BITMAP_SHADER.set(Shader::new(include_str!("../../shaders/bitmap.wgsl"))).unwrap();

        Self {
            instance: Arc::new(instance),
            adapter: Arc::new(adapter),
            device: Arc::new(device),
			queue: Arc::new(queue),

			render_pipeline_layout: Arc::new(render_pipeline_layout)
        }
    }

    pub fn get_graphics_unit_info(&self) -> wgpu::AdapterInfo {
        self.adapter.get_info()
    }
}
