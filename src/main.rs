#[cfg(feature = "log")]
#[macro_use] extern crate log;
#[macro_use] extern crate serde;

use std::{process, thread};

#[cfg(feature = "config")]
use figment::{Figment, providers::{Serialized, Toml, Format}};
use rsgss::graphics::{Window, Graphics, window::WINDOW, GRAPHICS};
use crate::config::Config;

pub const CONFIG_FILE_NAME: &str = "rsgss.toml";

#[cfg(feature = "log")]
pub mod rlog;
pub mod config;

fn main() {
    pollster::block_on(run());
}

async fn run() {
    #[allow(unused_assignments)]
    let mut config = Config::default();

    #[cfg(feature = "config")]
    {
        config = match Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file(CONFIG_FILE_NAME))
            .extract()
            {
                Ok(conf) => conf,
                Err(err) => {
                    eprintln!("Error while trying to get {}: {}", CONFIG_FILE_NAME, err.to_string());
                    process::exit(1);
                }
            };
    }

    #[cfg(feature = "log")]
    rlog::init(config.log);

    info!("{}@{} by {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS").replace(":", ", "));

    trace!("Spawning a window...");
    let window = Window::new(config.app.title);
    unsafe { WINDOW.set(window) }.unwrap();
    let graphics = Graphics::new().await;
    unsafe { GRAPHICS.set(graphics) }.unwrap();
    let graphics = Graphics::get();

    let gpu = graphics.get_graphics_unit_info();
    info!("Graphics Processing Unit Information:");
    info!("\tBackend: {:?}", gpu.backend);
    info!("\tName: {}", gpu.name);
    info!("\tType: {:?}", gpu.device_type);
    info!("\tDriver: {} {}", gpu.driver, gpu.driver_info);

    let rgss_thread = thread::Builder::new()
        .name("rgss thread".into())
        .spawn(move || {
            info!("Hello from a different thread!");
        })
        .unwrap();

    rgss_thread.join().unwrap();
}