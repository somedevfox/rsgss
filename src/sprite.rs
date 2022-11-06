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

use std::sync::Arc;

use parking_lot::RwLock;
use wgpu::util::DeviceExt;

use crate::{
    bitmap::Bitmap,
    get_graphics,
    graphics::{Renderable, RenderableTrait},
    result::{Error, Result},
    viewport::Viewport,
    Vertex,
};

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub ox: f32,
    pub oy: f32,

    buffer: Option<wgpu::Buffer>,
    vertices_len: u32,

    bitmap: Option<Arc<Bitmap>>,
    pub viewport: Option<Arc<RwLock<Viewport>>>, // The sprite should keep the viewport alive, however the viewport should NOT keep the sprite alive
}
impl Sprite {
    pub fn new(viewport: Option<Arc<RwLock<Viewport>>>) -> Arc<RwLock<Self>> {
        // let graphics = get_graphics();

        let x = 0.0;
        let y = 0.0;

        let _self = Arc::new(RwLock::new(Self {
            x,
            y,
            ox: x,
            oy: y,

            buffer: None,
            vertices_len: 0u32,

            viewport: viewport.clone(),
            bitmap: None,
        }));

        if let Some(viewport) = viewport {
            viewport
                .write()
                .renderable
                .push(Renderable::from(Arc::downgrade(&_self)));
        }

        _self
    }

    /*pub fn get_bitmap(&self) -> Arc<Option<Arc<Bitmap>>> {
        self.bitmap
    }*/

    pub fn set_bitmap(&mut self, bitmap: Option<Arc<Bitmap>>) {
        let width = bitmap.as_ref().map_or(0, |b| b.size.width) as f32;
        let height = bitmap.as_ref().map_or(0, |b| b.size.height) as f32;
        let vertices: [Vertex; 4] = [
            Vertex {
                position: [self.x, self.y, 0.],
                color: [0., 0., 0., 0.],
            },
            Vertex {
                position: [self.x + width, self.y, 0.],
                color: [0., 0., 0., 0.],
            },
            Vertex {
                position: [self.x + width, self.y - height, 0.],
                color: [0., 0., 0., 0.],
            },
            Vertex {
                position: [self.x, self.y - height, 0.],
                color: [0., 0., 0., 0.],
            },
        ];
        let vertex_buffer =
            get_graphics()
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        self.buffer = Some(vertex_buffer);
        self.vertices_len = vertices.len() as u32;

        self.bitmap = bitmap;
    }
}

impl RenderableTrait for Sprite {
    fn render<'render>(
        &'render self,
        rpass: &mut wgpu::RenderPass<'render>,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        match self.bitmap {
            None => Err(Error::NothingToRender),
            Some(ref bitmap) => {
                rpass.set_bind_group(0, &bitmap.bind_group, &[]);
                rpass.set_vertex_buffer(0, self.buffer.as_ref().unwrap().slice(..));
                rpass.draw_indexed(0..self.vertices_len, 0, 0..1);

                Ok(())
            }
        }
    }
}
