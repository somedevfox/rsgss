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

use rsgss::{config::get_config, get_graphics, graphics::Graphics, GRAPHICS};
use std::{
    io::{stderr, Write},
    path::Path,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

fn main() {
    pollster::block_on(run());
}

async fn run() {
    println!("rsgss v{}", env!("CARGO_PKG_VERSION"));
    let config = get_config();

    if !Path::new("lib").exists() {
        writeln!(
            &mut stderr(),
            "WARNING: Ruby Gems are missing, engine may crash."
        )
        .unwrap();
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
    )
    .await;

    let _ = GRAPHICS.set(graphics);
    let graphics = get_graphics();
    println!("Created. Listening to window events..");

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Engine exit was requested!");
                println!("Shutting down...");
                control_flow.set_exit();
            }
            Event::MainEventsCleared => {
                graphics.window.request_redraw();
            }
            Event::RedrawRequested(wid) if wid == graphics.window.id() => {
                /*window.update();
                match window.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost) => window.resize(window.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e)
                }*/
            }
            _ => {}
        }
    });
}
