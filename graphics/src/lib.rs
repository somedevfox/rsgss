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
//! Graphics rendering library for the `rsgss` game engine.
#![warn(clippy::all)]
#![deny(missing_debug_implementations)]

#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate bytemuck;

pub mod bitmap;
pub mod result;
pub mod sprite;
pub mod viewport;
pub mod window;

pub use wgpu;
pub use window::RenderWindow;

use core::{any::Any, fmt::Debug, mem};
use result::Error;

/// Rectangle struct
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Representation of a red-green-blue additive color model with an alpha channel
#[derive(Debug, Default, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

/// Representation of width and height as a 32-bit structure
#[derive(Debug, Default, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, Pod, Zeroable)]
pub(crate) struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub trait Renderable: Debug + Any + Send {
    fn render(&self, render_pass: &mut wgpu::RenderPass) -> Result<(), Error>;
}
