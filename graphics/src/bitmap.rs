use core::num;

/*
   Kitsune Standard License Version 1.0

   Copyright (c) 2023, Poleshko Egor Ivanovich, all rights reserved.

   Redistribution and use in source and binary forms, with or without modification,
   are permitted provided that the following conditions are met:
       1. Redistributions of source code must retain the above copyright notice,
           this list of conditions and the following disclaimer.
       2. All advertising materials mentioning features or use of this Software must
           display the following acknowledgement: This product includes software developed
           by Poleshko Egor Ivanovich.
       3. Redistributions in binary form must reproduce the above copyright notice or name of
           this Software ("rsgss") or trademark on the start up sequence of the distribution,
           unless waiver was granted by specific prior written permission.
       4. Redistributions in binary form must reproduce the above copyright notice, this list of
           conditions and the following disclaimer in the documentation and/or other materials
           provided with the distribution.
       5. Neither the name of the Poleshko Egor Ivanovich nor the names of it's contributors
           may be used to endorse or promote products derived from this software without
           specific prior written permission.
       6. Redistributions in source form must be made publicly available. This does not apply to
           any other software linked with the distribution.
       7. Redistributions in source and binary forms must state changes made to the Software.
       8. Redistributions in binary form must include the instructions on how to install
           and build the distribution.

   THIS SOFTWARE IS PROVIDED BY Poleshko Egor Ivanovich "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
   INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
   PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL  COPYRIGHT HOLDER BE LIABLE FOR ANY
   DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
   BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
   STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
use crate::{result::Error, window::RenderWindowImpl, Rect, RenderWindow};
use image::{io::Reader, ImageBuffer, Rgba};
use std::{cell::RefCell, sync::Arc};

pub(crate) type RgbaImageBuffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

#[derive(Debug)]
pub struct Bitmap {
    pub rect: Rect,

    data: RgbaImageBuffer,
    window: Arc<RefCell<RenderWindowImpl>>,

    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
}

impl Bitmap {
    fn create_texture(device: &wgpu::Device, buffer: &RgbaImageBuffer) -> wgpu::Texture {
        let size = wgpu::Extent3d {
            width: buffer.width(),
            height: buffer.height(),
            depth_or_array_layers: 1,
        };

        device.create_texture(&wgpu::TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        })
    }
    fn create_sampler(device: &wgpu::Device) -> wgpu::Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        })
    }

    pub fn new(renderer: &RenderWindow, width: u32, height: u32) -> Self {
        let inner = renderer.inner.borrow();

        let data = ImageBuffer::new(width, height);
        let texture = Self::create_texture(&inner.device, &data);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            rect: Rect {
                x: 0,
                y: 0,
                width,
                height,
            },
            data,
            window: renderer.inner.clone(),

            texture,
            view,
            sampler: Self::create_sampler(&inner.device),
        }
    }
    pub fn from_filename(renderer: &RenderWindow, name: impl Into<String>) -> Result<Self, Error> {
        let inner = renderer.inner.borrow();

        let image = Reader::open(name.into())?.decode()?;
        let data = image.into_rgba8();
        let texture = Self::create_texture(&inner.device, &data);
        let mut bitmap = Self::new(renderer, data.width(), data.height());

        bitmap.data = data;
        bitmap.texture = texture;

        Ok(bitmap)
    }

    pub(crate) fn write_texture(&self) {
        self.window.borrow().queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: num::NonZeroU32::new(4 * self.rect.width),
                rows_per_image: num::NonZeroU32::new(self.rect.height),
            },
            wgpu::Extent3d {
                width: self.rect.width,
                height: self.rect.height,
                depth_or_array_layers: 1,
            },
        )
    }
}
