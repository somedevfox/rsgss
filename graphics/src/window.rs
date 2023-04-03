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
use crate::{
    result::{Error, Result},
    viewport::Viewport,
    Rect, Size,
};
use core::fmt::Debug;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::{cell::RefCell, sync::Arc};
use wgpu::AdapterInfo;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize, Position},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub trait Window: Debug + Send + Sync + HasRawWindowHandle + HasRawDisplayHandle {
    fn new(title: impl Into<String>, rect: Rect) -> Result<Self>
    where
        Self: Sized;

    fn dimensions(&self) -> Size;
}

#[cfg(feature = "winit")]
/// Cross-platform handle to a window, providing methods to manipulate it's contents and dimensions using `winit` Rust crate.
///
/// # Example
/// ```
/// use rsgss_graphics::window::WinitWindow;
/// use rsgss_graphics::result::Result;
///
/// fn main() -> Result<()> {
/// 	let window = Window::new(
/// 		"rsgss window",   // Title
/// 		640,			  // Width
/// 		480				  // Height
/// 	)?;
/// 	let size = window.size();
///
/// 	println!("{}x{}", size.0, size.1); // 640x480
///
/// 	Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct WinitWindow {
    pub(crate) handle: winit::window::Window,
}
#[cfg(feature = "winit")]
impl Window for WinitWindow {
    /// Attempt to create a new window with provided title, width and height
    ///
    /// # Errors:
    ///	Throws an [Error::Window] if the window cannot be created. Common reasons may be:
    /// 	- There's no window manager present in the system
    /// 	- Title is too long
    /// 	- Dimensions are not supported by the target Operating System
    fn new(title: impl Into<String>, rect: Rect) -> Result<Self>
    where
        Self: Sized,
    {
        let event_loop = EventLoop::new();
        Ok(Self {
            handle: WindowBuilder::new()
                .with_title(title)
                .with_inner_size(PhysicalSize::new(rect.width, rect.height))
                .with_position(Position::Physical(PhysicalPosition::new(rect.x, rect.y)))
                .build(&event_loop)?,
        })
    }

    /// Returns dimension of a window
    fn dimensions(&self) -> Size {
        let inner_size = self.handle.inner_size();
        Size {
            width: inner_size.width,
            height: inner_size.height,
        }
    }
}
#[cfg(feature = "winit")]
unsafe impl HasRawDisplayHandle for WinitWindow {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.handle.raw_display_handle()
    }
}
#[cfg(feature = "winit")]
unsafe impl HasRawWindowHandle for WinitWindow {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.handle.raw_window_handle()
    }
}

#[derive(Debug)]
pub(crate) struct RenderWindowImpl {
    window: Box<dyn Window>,

    pub(crate) instance: wgpu::Instance,
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_configuration: wgpu::SurfaceConfiguration,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}
impl RenderWindowImpl {
    pub async fn new(
        window: impl Window + 'static,
        power_preference: wgpu::PowerPreference,
    ) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(Error::NoAvailableAdapters)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await?;

        let window_size = window.dimensions();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|format| format.describe().srgb)
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
        };
        surface.configure(&device, &surface_configuration);

        Ok(Self {
            window: Box::new(window),
            instance,
            surface,
            surface_configuration,
            adapter,
            device,
            queue,
        })
    }
}

/// Representation of a drawable surface on an actual window.
///
/// # Example
/// ```
/// use rsgss_graphics::window::{WinitWindow, RenderWindow};
/// use rsgss_graphics::result::Result;
///
/// fn main() -> Result<()> {
/// 	let window = RenderWindow::from_window(WinitWindow::new(
/// 		"rsgss window",   // Title
/// 		640,			  // Width
/// 		480				  // Height
/// 	)?);
///
/// 	Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct RenderWindow {
    pub(crate) inner: Arc<RefCell<RenderWindowImpl>>,

    viewports: Vec<Viewport>,
}

impl<'rw> RenderWindow {
    /// Create a renderable surface on a supplied window with transfering the ownership of the window to [RenderWindow]
    ///
    /// # Errors:
    /// Throws an...\
    /// 	- [`Error::Surface`] if a renderable surface cannot be created\
    /// 	- [`Error::NoAvailableAdapters`] if the system doesn't have a Graphics Processing Unit\
    /// 	- [`Error::RequestDevice`] if couldn't connect to the target Graphics Processing Unit
    ///
    /// # Example (with `winit` flag on and present tokio runtime):
    /// ```
    /// use rsgss_graphics::window::{WinitWindow, RenderWindow};
    ///
    /// #[tokio::main]
    /// fn main() {
    /// 	if let Err(why) = RenderWindow::from_window(WinitWindow::new(
    /// 		"OneShot: Fading Memory",
    /// 		640,
    /// 		480
    /// 	).unwrap()).await {
    /// 		eprintln!("Couldn't initialize graphics back-end: {why}");
    /// 	}
    /// }
    /// ```
    pub async fn from_window(
        window: impl Window + 'static,
        power_preference: wgpu::PowerPreference,
    ) -> Result<Self> {
        let inner = RenderWindowImpl::new(window, power_preference).await?;

        let window = Self {
            inner: Arc::new(RefCell::new(inner)),

            viewports: Vec::new(),
        };
        Ok(window)
    }

    // * Adapter methods
    /// Get list of Graphics Processing Units available in the system
    pub fn enumerate_adapters(&self) -> impl Iterator<Item = AdapterInfo> + '_ {
        let inner = self.inner.borrow();
        inner
            .instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(move |adapter| adapter.is_surface_supported(&inner.surface))
            .map(|adapter| adapter.get_info())
    }
    /// Get information on the current Graphics Processing Unit responsible for rendering
    pub fn get_current_adapter_info(&self) -> AdapterInfo {
        self.inner.borrow().adapter.get_info()
    }
    /// Try to switch current Graphics Process Unit on the go
    ///
    /// # Example:
    /// ```
    /// use rsgss_graphics::window::RenderWindow;
    ///	use rsgss_graphics::wgpu::{AdapterInfo, Backend};
    ///
    /// fn main() {
    /// 	let render_window = RenderWindow::from_window(...);
    ///		let adapter_pcid = render_window
    /// 		.enumerate_adapters()
    /// 		.unwrap()
    /// 		.filter(|adapter| adapter.backend == Backend::Vulkan)
    /// 		.collect::<Vec<AdapterInfo>>()[0]
    /// 		.device;
    ///		render_window.switch_adapter(adapter_pcid);
    /// }
    /// ```
    pub fn switch_adapter(&self, pcie_id: usize) -> Result<()> {
        let inner = self.inner.borrow();
        let mut filtered_adapters = inner
            .instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| adapter.is_surface_supported(&inner.surface))
            .filter(|adapter| adapter.get_info().device == pcie_id)
            .collect::<Vec<wgpu::Adapter>>();
        if filtered_adapters.len() == 0 {
            Err(Error::NoAvailableAdapters)
        } else {
            drop(inner); // Drop immutable borrow
            let mut inner = self.inner.borrow_mut();
            inner.adapter = filtered_adapters.remove(0);
            Ok(())
        }
    }

    // * Viewport methods
    /// Get an immutable reference to a viewport assigned to the window
    pub fn get_window_viewport(&'rw self) -> &'rw Viewport {
        self.viewports.get(0).unwrap()
    }
    /// Get a mutable reference to a viewport assigned to the window
    pub fn get_window_viewport_mut(&'rw mut self) -> &'rw mut Viewport {
        self.get_viewport_mut(0).unwrap()
    }

    /// Get an iterator containing all viewports
    pub fn viewports(&'rw self) -> impl Iterator<Item = &'rw Viewport> {
        self.viewports.iter()
    }
    /// Attempt to get a mutable reference to a viewport under the specified index
    ///
    /// # Example:
    /// ```
    /// use rsgss_graphics::{viewport::Viewport, window::RenderWindow};
    ///
    /// fn main() {
    /// 	let mut render_window = RenderWindow::from_window(...);
    /// 	let viewports = render_window.viewports();
    /// 	for (i, _) in viewports.enumerate() {
    /// 		let mut viewport = render_window.get_viewport_mut(i).unwrap();
    /// 		viewport.rect.x += 50;
    /// 		viewport.rect.y += 50;
    /// 	}
    /// }
    /// ```
    pub fn get_viewport_mut(&'rw mut self, index: usize) -> Option<&'rw mut Viewport> {
        self.viewports.get_mut(index)
    }
    /// Add already existing viewport and return it's reference
    pub fn add_viewport(&'rw mut self, viewport: Viewport) -> &'rw Viewport {
        self.viewports.push(viewport);
        self.viewports.last().unwrap()
    }
    /// Create, assign and return a reference to a viewport
    pub fn create_viewport(
        &'rw mut self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> &'rw Viewport {
        self.viewports.push(Viewport::from_rect(
            self.inner.clone(),
            Rect {
                x,
                y,
                width,
                height,
            },
        ));
        self.viewports.last().unwrap()
    }
    /// Dispose of a viewport under specified index.
    ///
    /// # Notes:
    /// Viewport under index 0 will not be removed, as it is [**window viewport**](RenderWindow::get_window_viewport).
    ///
    /// # Example:
    /// ```
    /// use rsgss_graphics::{viewport::Viewport, window::RenderWindow};
    ///
    /// fn main() {
    /// 	let mut render_window = RenderWindow::from_window(...);
    /// 	let viewports = render_window.viewports().next(); // Ignore first index - it's window viewport
    /// 	for (i, _) in viewports.enumerate() {
    /// 		render_window.remove_viewport(i);
    /// 	}
    /// }
    /// ```
    pub fn remove_viewport(&'rw mut self, index: usize) {
        // Viewport 0 is window viewport and it cannot be removed
        if index != 0 {
            self.viewports.remove(index);
        }
    }

    // * Graphics update methods
    /// Update [wgpu::Surface]'s dimensions
    pub(crate) fn resize(&self, new_width: u32, new_height: u32) {
        let mut inner = self.inner.borrow_mut();
        inner.surface_configuration.width = new_width;
        inner.surface_configuration.height = new_height;
        inner
            .surface
            .configure(&inner.device, &inner.surface_configuration);
    }
    /// Render all viewports
    ///
    /// # Errors
    /// Throws an...\
    /// 	- [Error::Surface] if the rendering back-end somehow recreated a texture after disposing of the old one
    pub fn update(&mut self) -> Result<()> {
        let inner = self.inner.borrow();
        let window_size = inner.window.dimensions();
        if (inner.surface_configuration.width != window_size.width)
            || (inner.surface_configuration.height != window_size.height)
        {
            self.resize(window_size.width, window_size.height);
        }

        let output = inner.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = inner
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        for viewport in &self.viewports {
            viewport.update(&mut encoder, &view);
        }

        inner.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
