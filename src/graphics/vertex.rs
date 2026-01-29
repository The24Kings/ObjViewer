use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};

pub const VEC3: i32 = 3;
pub const VEC2: i32 = 2;

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
    /// Creates a new vertex with position and normal.
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self {
            position,
            color: Vec3::ONE, // default white
            normal,
            tex_coords: Vec2::ZERO,
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

    /// Creates a vertex with position, color, normal, and texture coordinates.
    pub fn with_all(position: Vec3, color: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Self {
            position,
            color,
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
