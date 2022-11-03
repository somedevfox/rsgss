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

use crate::sprite::Sprite;
use magnus::{function, method, Module, Object, RTypedData, Value};
use parking_lot::RwLock;
use std::sync::Arc;

use super::viewport_binding::BoundViewport;

#[magnus::wrap(class = "Sprite", free_immediatly, size)]
struct BoundSprite {
    sprite: Arc<RwLock<Sprite>>,
}

impl BoundSprite {
    pub fn new_rb(args: &[Value]) -> Result<RTypedData, magnus::Error> {
        let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;
        let (viewport_val,): (Option<Value>,) = args.optional;

        let viewport = viewport_val
            .map(|v| v.try_convert::<&BoundViewport>())
            .transpose()?;

        let rb_self = RTypedData::from_value(
            Self {
                sprite: Sprite::new(viewport.cloned().map(Into::into)),
            }
            .into(),
        )
        .unwrap();

        rb_self.ivar_set("@viewport", viewport_val)?;
        rb_self.ivar_set("@bitmap", ())?;

        Ok(rb_self)
    }

    pub fn set_viewport(rb_self: Value, viewport: Value) -> Result<(), magnus::Error> {
        let rb_self = RTypedData::from_value(rb_self).unwrap();

        rb_self.try_convert::<&Self>()?.sprite.write().viewport = viewport
            .try_convert::<Option<&_>>()?
            .cloned()
            .map(BoundViewport::into);

        rb_self.ivar_set("@viewport", viewport)?;

        Ok(())
    }
}

pub fn bind() -> Result<(), magnus::Error> {
    let class = magnus::define_class("Sprite", Default::default())?;
    class.define_singleton_method("new", function!(BoundSprite::new_rb, -1))?;
    class.define_method("viewport=", method!(BoundSprite::set_viewport, 1))?;
    class.define_attr("viewport", magnus::Attr::Read)?;
    class.define_attr("bitmap", magnus::Attr::Read)?;

    Ok(())
}
