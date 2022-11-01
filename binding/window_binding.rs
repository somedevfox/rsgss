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

use magnus::define_module;

/*pub fn window_viewport() -> &'static Viewport {
    &get_graphics().window_viewport.upgrade().unwrap().read().borrow()
}*/

pub fn bind() -> Result<(), magnus::Error> {
    let _module = define_module("Window")?;
    //module.define_module_function("viewport", function!(window_viewport, 0))?;

    Ok(())
}
