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

use crate::get_graphics;
use magnus::{define_module, function};

fn graphics_update() {
    get_graphics().update();
}

fn graphics_frame_rate() -> u8 {
    *get_graphics().frame_rate.read()
}

fn graphics_frame_rate_set(frame_rate: u8) {
    *get_graphics().frame_rate.write() = frame_rate;
}

pub fn bind() -> Result<(), magnus::Error> {
    let module = define_module("Graphics")?;
    module.define_module_function("update", function!(graphics_update, 0))?;
    module.define_module_function("frame_rate", function!(graphics_frame_rate, 0))?;
    module.define_module_function("frame_rate=", function!(graphics_frame_rate_set, 1))?;
    Ok(())
}
