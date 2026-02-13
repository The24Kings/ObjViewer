use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

/// A vertex with position, color, normal, and texture coordinates.
///
/// Memory layout is `#[repr(C)]` for GPU compatibility:
/// - `position`: 12 bytes (3 × f32)
/// - `color`: 12 bytes (3 × f32)
/// - `normal`: 12 bytes (3 × f32)
/// - `tex_coords`: 8 bytes (2 × f32)
/// - Total stride: 44 bytes
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec3,
    pub normal: Vec3,
    pub tex_coords: Vec2,
}

impl Vertex {
    /// Describes the memory layout for the wgpu vertex buffer.
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // position: vec3<f32> @ location(0)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // color: vec3<f32> @ location(1)
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // normal: vec3<f32> @ location(2)
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // tex_coords: vec2<f32> @ location(3)
                wgpu::VertexAttribute {
                    offset: 36,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }

    /// Creates a new vertex with position, normal, color, and texture.
    pub fn new(position: Vec3, color: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Self {
            position,
            color,
            normal,
            tex_coords,
        }
    }

    /// Creates a vertex with specified position, color, and normal.
    pub fn with_color(position: Vec3, color: Vec3, normal: Vec3) -> Self {
        Self {
            position,
            color,
            normal,
            tex_coords: Vec2::ZERO,
        }
    }

    /// Creates a vertex with position, normal, and texture coordinates.
    pub fn with_texture(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Self {
            position,
            color: Vec3::ONE,
            normal,
            tex_coords,
        }
    }

    /// Tints the vertex color by adding the given color values.
    pub fn add_color(&mut self, tint: Vec3) {
        self.color += tint;
    }

    /// Tints the vertex color by multiplying with the given color values.
    pub fn multiply_color(&mut self, tint: Vec3) {
        self.color *= tint;
    }

    /// Sets the vertex color.
    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }

    /// Sets the normal vector.
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            color: Vec3::ONE,
            normal: Vec3::ZERO,
            tex_coords: Vec2::ZERO,
        }
    }
}
