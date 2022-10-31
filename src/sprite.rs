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

use std::sync::{Weak, Arc};

use parking_lot::RwLock;

use crate::{viewport::Viewport, bitmap::Bitmap};

pub struct Sprite {
	pub bitmap: Option<Bitmap>,

	//pub viewport: Weak<RwLock<Viewport>>,
}
impl Sprite {
	pub fn new() -> Self {
		Self {
			bitmap: None
		}
	}

	pub fn from_viewport(viewport: Arc<RwLock<Viewport>>) -> Self {
		Self {
			bitmap: None,
			//viewport: Arc::downgrade(&viewport)
		}
	}
}