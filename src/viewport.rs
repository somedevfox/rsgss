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
use crate::{graphics::Renderable, GRAPHICS};

use std::sync::{Arc, Weak};

use parking_lot::RwLock;

pub struct Viewport {
    pub renderable: Vec<Weak<Box<dyn Renderable + Send + Sync>>>,

    pub color: wgpu::Color,

    pub z: i32,
    pub ox: f32,
    pub oy: f32,

    pub width: f32,
    pub height: f32,

    pub visible: bool,
}

impl Viewport {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Arc<RwLock<Self>> {
        println!("[Viewport] Created new Viewport with specified parameters:");
        println!("\tPosition: {};{}", x, y);
        println!("\tDimensions: {}x{}", width, height);
        let viewport = Arc::new(RwLock::new(Self {
            renderable: vec![],

            color: wgpu::Color::WHITE,

            z: 0,
            ox: x,
            oy: y,

            width,
            height,

            visible: true,
        }));

        match GRAPHICS.get() {
            Some(graphics) => graphics.add_viewport(Arc::downgrade(&viewport)),
            None => {
                println!("[Viewport] This viewport has been created on window initialization")
            }
        }

        viewport
    }
    pub fn update(&self) {}
}
