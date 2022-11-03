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

use crate::{bitmap::Bitmap, viewport::Viewport};

pub struct Sprite {
    pub bitmap: Option<Arc<Bitmap>>,
    pub viewport: Option<Arc<RwLock<Viewport>>>, // The sprite should keep the viewport alive, however the viewport should NOT keep the sprite alive
}
impl Sprite {
    pub fn new(viewport: Option<Arc<RwLock<Viewport>>>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            viewport,
            bitmap: None,
        }))
    }
}
