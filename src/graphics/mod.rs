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
use glium::{
    glutin::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder},
    implement_vertex, Display, Surface,
};
use std::{borrow::Cow, process::exit, sync::Arc};

pub mod color;
pub mod rect;
pub mod sprite;
pub mod viewport;

pub use sprite::Sprite;
pub use viewport::Viewport;

use crate::graphics::color::Color;

/// Struct responsible for processing images, pixel data and rendering.
pub struct Graphics {
    pub display: Display,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// Representation of a point in 3D Space.
pub struct Vertex {
    /// Position of the Vertex on three-dimensional plane.
    pub position: [f32; 3],
    /// Color of the Vertex.
    pub color: Color,
}
implement_vertex!(Vertex, position);

impl Graphics {
    pub fn create_window(
        title: String,
        width: u32,
        height: u32,
        event_loop: &EventLoop<()>,
    ) -> Self {
        let native_window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height));

        let ctx = glium::glutin::ContextBuilder::new();
        let display = match Display::new(native_window, ctx, event_loop) {
            Ok(dp) => dp,
            Err(e) => {
                eprintln!("Fatal: {}", e.to_string());
                exit(1);
            }
        };
        Self { display }
    }

    pub fn update(&self) {
        let mut target = self.display.draw();
        target.clear_color(0., 0., 1., 0.);
        let triangle = vec![
            Vertex {
                position: [-0.5, -0.5, 1.],
                color: Color::from_rgb(0, 0, 0),
            },
            Vertex {
                position: [0., 0.5, 1.],
                color: Color::from_rgb(0, 0, 0),
            },
            Vertex {
                position: [0.5, -0.25, 1.],
                color: Color::from_rgb(0, 0, 0),
            },
        ];
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &triangle).unwrap();
        let vertex_shader_src = r#"
            in vec3 position;

            void main() {
                gl_Position = vec4(position, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            out vec4 color;
            
            void main() {
                color = vec4(0.51, 0.05, 0.66, 1.0);
            }
        "#;
        let program = glium::Program::from_source(
            &self.display,
            vertex_shader_src,
            fragment_shader_src,
            None,
        )
        .unwrap();
        target
            .draw(
                &vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();

        target.finish().unwrap();
    }
}
