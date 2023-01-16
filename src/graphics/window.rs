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

use std::{sync::Arc, num::NonZeroU32};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use wgpu::{Instance, Backends};
use winit::{
	window::WindowBuilder,
	event_loop::EventLoop
};

pub static mut WINDOW: OnceCell<Window> = OnceCell::new();

#[derive(Debug)]
pub struct Window {
	pub(crate) raw: Arc<winit::window::Window>,
	pub(crate) event_loop: Arc<EventLoop<()>>
}
unsafe impl Send for Window {}
unsafe impl Sync for Window {}
impl Window {
	pub fn get() -> &'static Self {
		unsafe { WINDOW.get() }.unwrap()
	}
	pub fn get_mut() -> &'static mut Self {
		unsafe { WINDOW.get_mut() }.unwrap()
	}

	pub fn new(title: impl Into<String>) -> Self {
		let event_loop = EventLoop::new();
		let window = WindowBuilder::new()
			.with_title(title.into())
			.build(&event_loop)
			.unwrap();

		Self {
			raw: Arc::new(window),
			event_loop: Arc::new(event_loop)
		}
	}
}
