// Copyright (C) 2022 Egor Poleshko
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

use std::{
    sync::{Arc, Weak},
    time::{Duration, Instant},
};

use parking_lot::RwLock;
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::viewport::Viewport;

pub struct Graphics {
    pub window: Window,

    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,

    pub size: PhysicalSize<u32>,
    pub title: String,

    pub viewports: RwLock<Vec<Weak<RwLock<Viewport>>>>,

    last_frame: RwLock<Instant>,
}
impl Graphics {
    pub async fn create_window(
        title: String,
        width: u32,
        height: u32,
        event_loop: &EventLoop<()>,
    ) -> Self {
        // Create Window
        let size = PhysicalSize::new(width, height);
        let window = WindowBuilder::new()
            .with_inner_size(size.clone())
            .with_title(title.clone())
            .build(event_loop)
            .unwrap();

        // Initialize WebGPU Instance
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        // rsgss wasm32 support one day maybe?
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &surface_config);

        let window_viewport = Viewport::new(0.0, 0.0, width as f32, height as f32);
        window_viewport.write().color.r = 0.0;
        window_viewport.write().color.g = 0.0;
        window_viewport.write().color.b = 0.0;
        println!("{:?}", window_viewport.read().color);
        let window_viewport = Arc::downgrade(&window_viewport);

        Self {
            window,

            surface,
            device,
            queue,
            surface_config,

            size,
            title,

            viewports: RwLock::new(vec![window_viewport]),

            last_frame: RwLock::new(Instant::now()),
        }
    }

    pub fn update(&self) {
        let mut last_frame = self.last_frame.write();
        let current_frame = Instant::now();
        let delta = current_frame.duration_since(*last_frame);
        println!("[Frame delta] {:#?}", delta);
        std::thread::sleep(Duration::from_secs_f64(1. / 60.).saturating_sub(delta));

        *last_frame = current_frame;

        self.viewports.write().retain(|viewport| {
            if let Some(viewport) = viewport.upgrade() {
                let viewport = viewport.read();
                if viewport.visible {
                    let output = self.surface.get_current_texture().unwrap();
                    let view = output
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let mut encoder = self
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                    {
                        let mut render_pass =
                            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: None,
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(viewport.color),
                                        store: true,
                                    },
                                })],
                                depth_stencil_attachment: None,
                            });
                        render_pass.set_viewport(
                            viewport.ox,
                            viewport.oy,
                            viewport.width,
                            viewport.height,
                            0.0,
                            0.0,
                        );
                    }

                    self.queue.submit(std::iter::once(encoder.finish()));
                    output.present();
                }

                true
            } else {
                println!("[Viewport] Has been garbage collected");

                false
            }
        });
    }

    pub fn add_viewport(&self, viewport: Weak<RwLock<Viewport>>) {
        self.viewports.write().push(viewport);
    }
}

pub trait Renderable {
    fn render(&mut self) -> (wgpu::BindGroup, wgpu::Buffer);
}
