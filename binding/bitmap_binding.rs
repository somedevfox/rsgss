// Copyright (C) 2022 Lily Lyons
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

use crate::bitmap::Bitmap;
use magnus::{function, Object};
use std::sync::Arc;

#[magnus::wrap(class = "Bitmap", free_immediatly, size)]
#[derive(Clone)]
pub struct BoundBitmap {
    bitmap: Arc<Bitmap>,
}

impl From<Arc<Bitmap>> for BoundBitmap {
    fn from(bitmap: Arc<Bitmap>) -> Self {
        Self { bitmap }
    }
}

impl From<BoundBitmap> for Arc<Bitmap> {
    fn from(bitmap: BoundBitmap) -> Self {
        bitmap.bitmap
    }
}

impl BoundBitmap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            bitmap: Arc::new(Bitmap::new(width, height)),
        }
    }
}

pub fn bind() -> Result<(), magnus::Error> {
    let class = magnus::define_class("Bitmap", Default::default())?;
    class.define_singleton_method("new", function!(BoundBitmap::new, 2))?;

    Ok(())
}
