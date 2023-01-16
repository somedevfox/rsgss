use std::{sync::Arc, borrow::Cow};

use once_cell::sync::OnceCell;

use super::Graphics;

pub static BITMAP_SHADER: OnceCell<Shader> = OnceCell::new();

#[derive(Clone, Debug)]
pub struct Shader {
    module: Arc<wgpu::ShaderModule>,
}

impl Shader {
    pub fn new(source: impl Into<String>) -> Self {
        let graphics = Graphics::get();
        let module = graphics.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&source.into()))
        });

        Self {
            module: Arc::new(module)
        }
    }

    pub fn module(&self) -> Arc<wgpu::ShaderModule> {
        self.module.clone()
    }
}