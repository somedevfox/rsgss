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

use super::color::Color;
use super::rect::Rect;

/// Viewport is a viewing region in an RGSS game, or computer graphics in general.
pub struct Viewport {
    /// Background color of the Viewport.
    pub color: Color,
    /// Size of the Viewport.
    pub rect: Rect,
}

impl Viewport {
    /// Create viewport based on future position and size.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self::new_rect(Rect {
            x,
            y,
            width,
            height,
        })
    }

    /// Create viewport based on future position and size via `Rect` struct.
    pub fn new_rect(rect: Rect) -> Self {
        let color = Color {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 1.,
        };

        Self { color, rect }
    }
}
