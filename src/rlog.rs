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

use std::{env, sync::{Arc, atomic::{AtomicUsize, Ordering}}, str::FromStr};
use log::{LevelFilter, SetLoggerError, Level};
use owo_colors::{OwoColorize, colors::{Cyan, Red}, Color, Style, Rgb};

use crate::config::LogConfig;

static MAX_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn max_target_width(target: &str) -> usize { // https://github.com/seanmonstar/pretty-env-logger/blob/master/src/lib.rs#L230
	let max_width = MAX_WIDTH.load(Ordering::Relaxed);
	if max_width < target.len() {
		MAX_WIDTH.store(target.len(), Ordering::Relaxed);
		target.len()
	} else {
		max_width
	}
}

fn parse_format_string(fmt: impl ToString, record: &log::Record<'_>, style: Style) -> String {
	let target = record.target();
	let max_width = max_target_width(target);
	
	let space_amount = max_width - target.len();
	let mut padding = String::new();
	for _i in 0..space_amount {
		padding.push(' ');
	}
	
	let mut level = record.level().to_string();
	if level == "INFO" || level == "WARN" {
		level.push(' ');
	}

	fmt.to_string()
	   .replace("%l", &level.style(style).to_string())
	   .replace("%t", &format!("{}{}", record.target(), padding))
	   .replace("%m", &record.args().to_string())
}

struct Logger {
	pub config: LogConfig
}

impl log::Log for Logger {
	fn enabled(&self, _metadata: &log::Metadata) -> bool {
		_metadata.level() <= log::max_level().to_level().unwrap_or(Level::Trace)
	}

	fn log(&self, record: &log::Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let colors = &self.config.colors;

		let color = match record.level() {
			Level::Trace => Rgb(colors.trace[0], colors.trace[1], colors.trace[2]),
			Level::Info => Rgb(colors.info[0], colors.info[1], colors.info[2]),
			Level::Error => Rgb(colors.error[0], colors.error[1], colors.info[2]),
			Level::Warn => Rgb(colors.warn[0], colors.warn[1], colors.warn[2]),
			Level::Debug => Rgb(colors.debug[0], colors.debug[1], colors.debug[2])
		};
		let style = Style::new().color(color).bold();

		println!("{}", parse_format_string(self.config.format.clone(), record, style));
	}

	fn flush(&self) {
		
	}
}

pub fn init(config: LogConfig) {
	try_init(config).unwrap()
}

pub fn try_init(config: LogConfig) -> Result<(), log::SetLoggerError> {
	let level = env::var("RUST_LOG").unwrap_or(String::from("trace"));
	let level = LevelFilter::from_str(level.as_str()).unwrap();

	log::set_max_level(level);

	let logger = Logger {
		config
	};

	log::set_boxed_logger(Box::new(logger))
}