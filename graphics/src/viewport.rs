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
use crate::{sprite::Sprite, window::RenderWindowImpl, Color, Rect, RenderWindow, Renderable};
use std::{any::TypeId, cell::RefCell, sync::Arc};

#[derive(Debug)]
pub struct Viewport {
    pub rect: Rect,
    pub color: Color,
    pub visible: bool,

    renderable: Vec<Box<dyn Renderable>>,
    window: Arc<RefCell<RenderWindowImpl>>,
}

impl Viewport {
    pub(crate) fn from_rect(window: Arc<RefCell<RenderWindowImpl>>, rect: Rect) -> Self {
        Self {
            rect,
            color: Color::default(),
            visible: true,
            renderable: Vec::new(),
            window: window,
        }
    }

    pub fn add(&mut self, renderable: impl Renderable) {
        self.renderable.push(Box::new(renderable));
    }

    pub fn sprites<'s>(&self) -> impl Iterator<Item = &'s Box<Sprite>> + '_ {
        self.renderable
            .iter()
            .filter(|r| r.type_id() == TypeId::of::<Sprite>())
            .map(|r| unsafe { core::mem::transmute::<_, &Box<Sprite>>(r) })
    }

    pub fn update(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.color.red as f64,
                            g: self.color.green as f64,
                            b: self.color.blue as f64,
                            a: self.color.alpha as f64,
                        }),
                        store: false,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_viewport(
                self.rect.x as f32,
                self.rect.y as f32,
                self.rect.width as f32,
                self.rect.height as f32,
                0.,
                1.,
            );
            for renderable in &self.renderable {
                let _ = renderable.render(&mut render_pass);
            }
        }
    }
}
