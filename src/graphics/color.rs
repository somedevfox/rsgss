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

#[derive(Copy, Clone, Debug)]
/// Representation of the RGBA color model.
pub struct Color {
    /// Red channel.
    pub red: u8,
    /// Blue channel.
    pub blue: u8,
    /// Green channel.
    pub green: u8,
    /// Alpha channel. (Controls opacity)
    pub alpha: f32,
}

impl Color {
    pub fn from_rgb(r: u8, b: u8, g: u8) -> Self {
        Self {
            red: r,
            blue: b,
            green: g,
            alpha: 1.,
        }
    }

    pub fn from_rgba(r: u8, b: u8, g: u8, a: f32) -> Self {
        Self {
            red: r,
            blue: b,
            green: g,
            alpha: 1.,
        }
    }
}
