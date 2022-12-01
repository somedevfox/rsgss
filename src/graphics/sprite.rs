use super::{rect::Rect, viewport::Viewport};

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
pub struct Sprite {
    pub rect: Rect,
}

impl Sprite {
    /// Create a new `Sprite` and attach it to a `Viewport`.
    pub fn new(viewport: &Viewport) -> Self {
        let rect = Rect {
            x: 0,
            y: 0,
            width: viewport.rect.width,
            height: viewport.rect.height,
        };
        Self { rect }
    }
}
