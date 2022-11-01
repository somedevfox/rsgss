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

use crate::viewport::Viewport;
use magnus::{define_class, function, method, Module, Object};
use parking_lot::RwLock;

#[magnus::wrap(class = "Viewport", free_immediatly, size)]
struct BoundViewport {
    viewport: Arc<RwLock<Viewport>>,
}

impl BoundViewport {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            viewport: Viewport::new(x, y, width, height),
        }
    }

    fn update(&self) {
        self.viewport.read().update();
    }
}

pub fn bind() -> Result<(), magnus::Error> {
    let class = define_class("Viewport", Default::default())?;
    class.define_singleton_method("new", function!(BoundViewport::new, 4))?;
    class.define_method("update", method!(BoundViewport::update, 0))?;
    Ok(())
}
