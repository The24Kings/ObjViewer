//! Game objects module.
//!
//! This module contains all renderable and physical objects in the scene.
//!
//! ## Architecture Note
//!
//! The current trait-based approach (`Renderable`, `Physical`, `GameObject`) is designed
//! for learning OpenGL fundamentals and understanding how rendering/physics systems interact.
//!
//! If the project grows to have many objects with varied components, consider migrating to
//! an Entity-Component-System (ECS) architecture using libraries like:
//! - `hecs` - A fast, minimal ECS library
//! - `bevy_ecs` - Bevy's ECS, usable standalone
//! - `specs` - Parallel ECS with good Rust integration
//!
//! ECS provides better cache locality, easier composition, and more flexible entity management
//! at the cost of a steeper learning curve.

pub mod cube;
pub mod triangle;
pub mod light;

pub use cube::Cube;
pub use triangle::Triangle;
pub use light::Light;

use crate::graphics::Vertex;
use glam::Vec3;

pub fn calculate_normals(vertices: &mut Vec<Vertex>, indices: &Vec<u32>) {
    let vertex_count = vertices.len();

    // Accumulate face normals into each vertex
    let mut accum_normals: Vec<Vec3> = vec![Vec3::ZERO; vertex_count];

    // Iterate triangles and accumulate face normals into vertex normals
    for tri in indices.chunks(3) {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;

        let p0 = vertices[i0].position;
        let p1 = vertices[i1].position;
        let p2 = vertices[i2].position;

        // Edge vectors
        let u = p1 - p0;
        let v = p2 - p0;

        // Face normal = u × v
        let face_normal = u.cross(v);

        accum_normals[i0] += face_normal;
        accum_normals[i1] += face_normal;
        accum_normals[i2] += face_normal;
    }

    // Normalize and write back into vertices
    for (i, vertex) in vertices.iter_mut().enumerate() {
        let normal = accum_normals[i];
        vertex.normal = if normal.length_squared() > 0.0 {
            normal.normalize()
        } else {
            Vec3::ZERO
        };
    }
}