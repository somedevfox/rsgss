// Copyright (C) 2023 Egor Poleshko
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

#[cfg(feature = "log")]
#[derive(Clone, Deserialize, Serialize)]
pub struct ColorConfig {
	pub debug: [u8; 3],
	pub info: [u8; 3],
	pub trace: [u8; 3],
	pub warn: [u8; 3],
	pub error: [u8; 3]
}
#[cfg(feature = "log")]
impl Default for ColorConfig {
	fn default() -> Self {
		Self {
			debug: [0, 128, 0],
			info: [0, 255, 255],
			trace: [128, 0, 0],
			warn: [255, 255, 0],
			error: [255, 0, 0]
		}
	}
}

#[cfg(feature = "log")]
#[derive(Clone, Deserialize, Serialize)]
pub struct LogConfig {
	pub colors: ColorConfig,
	pub format: String
}

impl Default for LogConfig {
	fn default() -> Self {
		Self {
			colors: Default::default(),
			format: String::from("%l | %t > %m")
		}
	}
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AppConfig {
	pub title: String
}
impl Default for AppConfig {
	fn default() -> Self {
		Self {
			title: String::from("rsgss game")
		}
	}
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Config {
	#[cfg(feature = "log")]
	pub log: LogConfig,
	pub app: AppConfig
}