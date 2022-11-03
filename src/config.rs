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
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RSGSSAppConfig {
    pub name: String,
}
impl Default for RSGSSAppConfig {
    fn default() -> Self {
        Self {
            name: String::from("RSGSS-powered Game"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RSGSSWindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
impl Default for RSGSSWindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("RSGSS Window"),
            width: 640,
            height: 480,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct RSGSSConfig {
    pub debug_mode: bool,
    pub window: RSGSSWindowConfig,
    pub app: RSGSSAppConfig,
}

pub fn get_config() -> RSGSSConfig {
    let figment = Figment::new()
        .merge(Toml::file("rsgss.toml"))
        .merge(Env::prefixed("RSGSS_"))
        .merge(Serialized::defaults(RSGSSConfig::default()));

    //#[cfg(feature = "figment-json")]
    //figment.merge(figment::providers::Json::file("rsgss.json"));

    figment.extract().unwrap()
}
