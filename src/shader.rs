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

use crate::{
    get_graphics,
    result::{Error, Result},
};
use std::borrow::Cow;

#[derive(PartialEq, Eq)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

#[allow(dead_code)]
pub struct Shader {
    pub name: String,
    pub source: String,

    pub render_pipeline: wgpu::RenderPipeline,
    shader_module: wgpu::ShaderModule,
}

impl Shader {
    pub fn from_file(shader_type: ShaderType, name: &str, filename: String) -> Result<Self> {
        match std::fs::read_to_string(filename) {
            Ok(f) => Self::from_string(shader_type, name, f),
            Err(e) => Err(Error::IoError(e)),
        }
    }

    pub fn from_string(shader_type: ShaderType, name: &str, code: String) -> Result<Self> {
        let graphics = get_graphics();

        let shader_module = graphics
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(&format!("`{}` Shader", name)),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&code)),
            });
        let render_pipeline_layout =
            graphics
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(&format!("`{}` Render Pipeline Layout", name)),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let mut fragment_state: Option<wgpu::FragmentState> = None;
        if shader_type == ShaderType::Fragment {
            fragment_state = Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[],
            });
        }

        let render_pipeline =
            graphics
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(&format!("`{}` Render Pipeline", name)),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader_module,
                        entry_point: "vs_main",
                        buffers: &[],
                    },
                    fragment: fragment_state,
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

        Ok(Self {
            name: name.to_string(),
            source: code,

            render_pipeline,
            shader_module,
        })
    }

    pub fn from_str(shader_type: ShaderType, name: &str, code: &str) -> Result<Self> {
        Self::from_string(shader_type, name, code.to_string())
    }
}
