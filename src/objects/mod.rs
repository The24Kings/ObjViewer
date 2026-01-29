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

pub fn calculate_normals(vertices: &mut Vec<f32>, indices: &Vec<u32>) {
    assert!(vertices.len() % 9 == 0);
    
    // Recompute per-vertex normals by accumulating triangle normals.
        let vertex_count = vertices.len() / 9;

        // Extract positions into a separate array to avoid simultaneous mutable
        // and immutable borrows of `vertices` while we accumulate normals.
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertex_count);
        for i in 0..vertex_count {
            let off = i * 9;
            positions.push([vertices[off], vertices[off + 1], vertices[off + 2]]);
        }

        let mut accum_normals: Vec<[f32; 3]> = vec![[0.0; 3]; vertex_count];

        // Iterate triangles and accumulate face normals into vertex normals
        for tri in indices.chunks(3) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;

            let [x0, y0, z0] = positions[i0];
            let [x1, y1, z1] = positions[i1];
            let [x2, y2, z2] = positions[i2];

            let ux = x1 - x0;
            let uy = y1 - y0;
            let uz = z1 - z0;
            let vx = x2 - x0;
            let vy = y2 - y0;
            let vz = z2 - z0;

            // face normal = u x v
            let nx = uy * vz - uz * vy;
            let ny = uz * vx - ux * vz;
            let nz = ux * vy - uy * vx;

            accum_normals[i0][0] += nx;
            accum_normals[i0][1] += ny;
            accum_normals[i0][2] += nz;

            accum_normals[i1][0] += nx;
            accum_normals[i1][1] += ny;
            accum_normals[i1][2] += nz;

            accum_normals[i2][0] += nx;
            accum_normals[i2][1] += ny;
            accum_normals[i2][2] += nz;
        }

        // Normalize and write back into vertices
        for i in 0..vertex_count {
            let nx = accum_normals[i][0];
            let ny = accum_normals[i][1];
            let nz = accum_normals[i][2];
            let len = (nx * nx + ny * ny + nz * nz).sqrt();
            let (nx, ny, nz) = if len != 0.0 {
                (nx / len, ny / len, nz / len)
            } else {
                (0.0, 0.0, 0.0)
            };
            let off = i * 9 + 6;
            vertices[off] = nx;
            vertices[off + 1] = ny;
            vertices[off + 2] = nz;
        }
}