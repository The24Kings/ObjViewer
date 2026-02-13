//! Material = shader pipeline + texture + bind group.

use std::rc::Rc;
use std::sync::Arc;

use crate::graphics::{Shader, Texture};

/// A material owns a shader pipeline reference, a texture, and a
/// pre-built bind group that ties the uniform buffer + texture together.
#[derive(Clone)]
pub struct Material {
    pub shader: Rc<Shader>,
    pub texture: Texture,
    pub bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
}

impl Material {
    /// Build a material.  `uniform_size` is the byte size of the uniform
    /// struct that will be written each frame (e.g. `size_of::<ObjUniforms>()`).
    pub fn new(
        device: &Arc<wgpu::Device>,
        shader: Rc<Shader>,
        texture: Texture,
        uniform_size: u64,
    ) -> Self {
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Material Uniform Buffer"),
            size: uniform_size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: &shader.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        Self {
            shader,
            texture,
            bind_group,
            uniform_buffer,
        }
    }
}
