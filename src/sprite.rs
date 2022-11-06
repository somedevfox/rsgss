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

use std::{num::NonZeroU32, sync::Arc};

use parking_lot::RwLock;
use wgpu::util::DeviceExt;

use crate::{
    bitmap::Bitmap,
    get_graphics,
    graphics::Renderable,
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
        let graphics = get_graphics();

        let x = 0.0;
        let y = 0.0;

        Arc::new(RwLock::new(Self {
            x,
            y,
            ox: x,
            oy: y,

            buffer: None,
            vertices_len: 0u32,

            viewport,
            bitmap: None,
        }))
    }

    /*pub fn get_bitmap(&self) -> Arc<Option<Arc<Bitmap>>> {
        self.bitmap
    }*/
    pub fn set_bitmap(&mut self, bitmap: Arc<Bitmap>) -> Arc<Bitmap> {
        let vertices: &[Vertex] = &[
            Vertex {
                position: [self.x, self.y, 0.],
                color: [0., 0., 0., 0.],
            },
            Vertex {
                position: [self.x + bitmap.size.width as f32, self.y, 0.],
                color: [0., 0., 0., 0.],
            },
            Vertex {
                position: [
                    self.x + bitmap.size.width as f32,
                    self.y - bitmap.size.height as f32,
                    0.,
                ],
                color: [0., 0., 0., 0.],
            },
            Vertex {
                position: [self.x, self.y - bitmap.size.height as f32, 0.],
                color: [0., 0., 0., 0.],
            },
        ];
        let vertex_buffer =
            get_graphics()
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        self.buffer = Some(vertex_buffer);
        self.vertices_len = vertices.len() as u32;

        bitmap
    }
}

impl Renderable for Sprite {
    fn render<'render>(
        &'render self,
        rpass: &mut wgpu::RenderPass<'render>,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        if self.bitmap.is_none() {
            return Err(Error::NothingToRender);
        }
        let bitmap = self.bitmap.as_ref().unwrap();

        if bitmap.img.is_some() {
            let img = bitmap.img.as_ref().unwrap();
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &bitmap.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &img,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: NonZeroU32::new(4 * bitmap.size.width),
                    rows_per_image: NonZeroU32::new(bitmap.size.height),
                },
                bitmap.size,
            );
        }
        rpass.set_bind_group(0, &bitmap.bind_group, &[]);

        Ok(())
    }
}
