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
#![warn(rust_2018_idioms, clippy::all)]

use glium::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use rsgss::{config::get_config, graphics::Graphics};
use std::{
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

fn main() {
    #[cfg(debug_assertions)]
    let _ = color_eyre::install();

    pollster::block_on(run());
}

async fn run() {
    println!("rsgss v{}", env!("CARGO_PKG_VERSION"));
    let config = get_config();

    if !Path::new("lib").exists() {
        eprintln!("WARNING: Ruby Gems are missing, engine may crash.");
    }

    println!("Creating RGSS Thread...");
    let _rgss_thread = rsgss::rgss::spawn_rgss_thread(config.app.name.clone());
    println!("Created. Creating window...");
    let event_loop = EventLoop::new();
    let graphics = Graphics::create_window(
        config.window.title.clone(),
        config.window.width,
        config.window.height,
        &event_loop,
    );
    println!("Created. Listening to window events..");

    event_loop.run(move |ev, elwt, control_flow| {
        let next_frame_time = Instant::now() + Duration::from_nanos(16_666_667);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            Event::MainEventsCleared => {
                graphics.update();
            }
            _ => (),
        }
    });
}
