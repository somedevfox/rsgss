/*
   Kitsune Standard License Version 1.0

   Copyright (c) 2023, Poleshko Egor Ivanovich, all rights reserved.

   Redistribution and use in source and binary forms, with or without modification,
   are permitted provided that the following conditions are met:
       1. Redistributions of source code must retain the above copyright notice,
           this list of conditions and the following disclaimer.
       2. All advertising materials mentioning features or use of this Software must
           display the following acknowledgement: This product includes software developed
           by Poleshko Egor Ivanovich.
       3. Redistributions in binary form must reproduce the above copyright notice or name of
           this Software ("rsgss") or trademark on the start up sequence of the distribution,
           unless waiver was granted by specific prior written permission.
       4. Redistributions in binary form must reproduce the above copyright notice, this list of
           conditions and the following disclaimer in the documentation and/or other materials
           provided with the distribution.
       5. Neither the name of the Poleshko Egor Ivanovich nor the names of it's contributors
           may be used to endorse or promote products derived from this software without
           specific prior written permission.
       6. Redistributions in source form must be made publicly available. This does not apply to
           any other software linked with the distribution.
       7. Redistributions in source and binary forms must state changes made to the Software.
       8. Redistributions in binary form must include the instructions on how to install
           and build the distribution.

   THIS SOFTWARE IS PROVIDED BY Poleshko Egor Ivanovich "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
   INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A
   PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL  COPYRIGHT HOLDER BE LIABLE FOR ANY
   DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
   LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
   BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
   STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
   OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

pub mod config;

use rsgss_graphics::{
    wgpu,
    window::{Window, WinitWindow},
    Rect, RenderWindow,
};
use std::env;

use crate::config::AppConfiguration;

fn backend_to_string(backend: wgpu::Backend) -> String {
    String::from(match backend {
        wgpu::Backend::Empty => "Empty",
        wgpu::Backend::Vulkan => "Vulkan",
        wgpu::Backend::Metal => "Apple Metal",
        wgpu::Backend::Dx12 => "Microsoft DirectX 12",
        wgpu::Backend::Dx11 => "Microsoft DirectX 11",
        wgpu::Backend::Gl => "OpenGL",
        wgpu::Backend::BrowserWebGpu => "WebGL",
    })
}

fn log_adapter(adapter_info: wgpu::AdapterInfo) {
    info!("\tAdapter {}:", adapter_info.device);
    info!("\t\tType: {:?}", adapter_info.device_type);
    info!("\t\tBack-end: {}", backend_to_string(adapter_info.backend));
    info!("\t\tName: {}", adapter_info.name);
    info!(
        "\t\tDriver: {} {}",
        adapter_info.driver, adapter_info.driver_info
    );
}

#[tokio::main]
async fn main() {
    // * Get config
    let config = AppConfiguration::get();
    info!("Successfully opened and parsed the configuration file");

    // * Initialize logger
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Trace)
        .filter_module("wgpu", log::LevelFilter::Off)
        .init();
    info!("{}", env!("CARGO_PKG_VERSION"));

    info!(
        "Spawning a window with `{}x{}` size",
        config.window.width, config.window.height
    );
    match WinitWindow::new(
        config.window.title,
        Rect {
            x: 0,
            y: 0,
            width: config.window.width,
            height: config.window.height,
        },
    ) {
        Ok(window) => {
            match RenderWindow::from_window(window, config.renderer.power_preference).await {
                Ok(render_window) => {
                    info!("Successfully created new window and initialized graphics back-end");
                    info!("Available adapters:");
                    for adapter in render_window.enumerate_adapters() {
                        log_adapter(adapter);
                    }

                    info!("Selected adapter:");
                    log_adapter(render_window.get_current_adapter_info());
                }
                Err(why) => {
                    error!("Couldn't initialize graphics back-end: {why}");
                    rfd::MessageDialog::new()
                        .set_title("rsgss")
                        .set_level(rfd::MessageLevel::Error)
                        .set_description(
                            format!("Couldn't initialize graphics back-end: {why}").as_str(),
                        )
                        .show();
                }
            }
        }
        Err(why) => {
            error!("Couldn't spawn a new window: {why}");
            rfd::MessageDialog::new()
                .set_title("rsgss")
                .set_level(rfd::MessageLevel::Error)
                .set_description(format!("Couldn't create a new window: {why}").as_str())
                .show();
        }
    }
}
