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
//! rsgss - Rust Implementation of the Ruby Game Scripting System
#![warn(missing_docs)]
use bytemuck::{Pod, Zeroable};
use glium::implement_vertex;
use graphics::Graphics;
use once_cell::sync::OnceCell;
use std::mem::size_of;

#[path = "../binding/binding_util.rs"]
pub mod binding_util;
pub mod bitmap;
pub mod config;
pub mod graphics;
pub mod log;
pub mod result;
pub mod rgss;

/*pub static GRAPHICS: OnceCell<Graphics> = OnceCell::new();
pub fn get_graphics() -> &'static Graphics {
    GRAPHICS.get().unwrap()
}*/
