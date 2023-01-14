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
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
pub use window::Window;

use std::sync::Arc;
use glium::{glutin::event_loop::EventLoop, Display, Frame};

pub trait Renderable {
	fn render(&self, glium: Frame);
}

pub static GRAPHICS: OnceCell<Graphics> = OnceCell::new();

pub struct GL;
impl GL {
	pub fn device(&self) -> String {
		let display = Window::get().display.read();
		display.get_opengl_renderer_string().to_string()
	}

	pub fn vendor(&self) -> String {
		let display = Window::get().display.read();
		display.get_opengl_vendor_string().to_string()
	}

	pub fn api(&self) -> glium::Api {
		let display = Window::get().display.read();
		let version = display.get_opengl_version();
		version.0
	}

	pub fn version(&self) -> (u8, u8) {
		let display = Window::get().display.read();
		let version = display.get_opengl_version();
		(version.1, version.2)
	}
}

pub struct Graphics {
	pub gl: GL
}
unsafe impl Sync for Graphics {}
unsafe impl Send for Graphics {}
impl Graphics {
	pub fn new() -> Self {
		Self {
			gl: GL
		}
	}

	
}