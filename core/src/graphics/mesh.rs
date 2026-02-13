//! wgpu mesh: vertex + index buffers on the GPU.

use bytemuck::cast_slice;
use std::sync::Arc;
use wgpu::util::DeviceExt;

use crate::graphics::Vertex;

#[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    /// Upload vertex/index data to the GPU. Call once after construction.
    pub fn upload(&mut self, device: &Arc<wgpu::Device>) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        self.vertex_buffer = Some(vertex_buffer);

        if !self.indices.is_empty() {
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Mesh Index Buffer"),
                contents: cast_slice(&self.indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            self.index_buffer = Some(index_buffer);
        }
    }

    pub fn is_uploaded(&self) -> bool {
        self.vertex_buffer.is_some()
    }

    /// Issue draw commands into an existing render pass.
    pub fn draw(&self, rpass: &mut wgpu::RenderPass<'_>) {
        let vb = self
            .vertex_buffer
            .as_ref()
            .expect("Mesh not uploaded to GPU");

        rpass.set_vertex_buffer(0, vb.slice(..));

        if let Some(ib) = &self.index_buffer {
            rpass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint32);
            rpass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
        } else {
            rpass.draw(0..self.vertices.len() as u32, 0..1);
        }
    }
}

