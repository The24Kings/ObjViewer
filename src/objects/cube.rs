use crate::{
    game::Transform,
    graphics::{Material, Mesh, Renderable},
};
use glam::Mat4;

pub struct Cube {
    pub material: Material,
    pub mesh: Mesh,
    pub transform: Transform,
    sin_wave: Vec<f32>,
    sin_index: f32,
}

impl Renderable for Cube {
    fn material(&self) -> &Material {
        &self.material
    }

    fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    fn model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            self.transform.scale,
            self.transform.rotation,
            self.transform.position,
        )
    }

    fn animate(&mut self, delta: f32) {
        // Spin
        let rotation_y = glam::Quat::from_rotation_y(0.5 * delta as f32);
        let rotation_x = glam::Quat::from_rotation_x(0.5 * delta as f32);
        self.transform.rotation = rotation_x * rotation_y * self.transform.rotation;

        // Bobbing on Y axis using precomputed sin wave
        let speed: f32 = 0.5; // cycles per second
        let samples = self.sin_wave.len() as f32;

        // advance index (wrap-around)
        self.sin_index = (self.sin_index + delta * speed * samples) % samples;
        let idx_f = self.sin_index;

        // Get current and next index for lerp
        let i0 = idx_f.floor() as usize % self.sin_wave.len();
        let i1 = (i0 + 1) % self.sin_wave.len();

        // Get the floating point (percentage between the two points)
        let frac = idx_f - idx_f.floor();

        // Interpolation
        let s0 = self.sin_wave[i0];
        let s1 = self.sin_wave[i1];
        let sinv = s0 + (s1 - s0) * frac;

        // Map sinv (-1..1) to desired Y range
        let amplitude: f32 = 0.5; // max offset from center
        let base_y: f32 = 0.0; // center position
        self.transform.position.y = base_y + sinv * amplitude;
    }
}

impl Cube {
    pub fn new(material: Material) -> Self {
        let (vertices, indices) = Self::data();
        let mesh = Mesh {
            vao: None,
            vbo: None,
            ibo: None,
            vertices,
            indices,
        };

        // Generate Sin wave 0->2PI (one cycle)
        let samples: usize = 256;
        let sin_wave: Vec<f32> = (0..samples)
            .map(|i| {
                let t = i as f32 / (samples - 1) as f32 * std::f32::consts::TAU;
                t.sin()
            })
            .collect();

        Self {
            material,
            mesh,
            transform: Transform::default(),
            sin_wave,
            sin_index: 0.0,
        }
    }

    //TODO: Convert `vertices` to a Vertex Struct
    fn data() -> (Vec<f32>, Vec<u32>) {
        let mut vertices: Vec<f32> = Vec::with_capacity(9 * 6 * 4); // pos, color, normal - 6 faces * 4 points
        let mut indices: Vec<u32> = Vec::with_capacity(36);

        // Helper to push a face (4 verts, color, and 6 indices)
        let mut push_face = |positions: &[(f32, f32, f32)], color: (f32, f32, f32)| {
            // base vertex index (each vertex has 9 floats: pos(3), color(3), normal(3))
            let base = (vertices.len() / 9) as u32;

            // push vertex data (position + color + placeholder normal)
            for &(x, y, z) in positions.iter() {
                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
                vertices.push(color.0);
                vertices.push(color.1);
                vertices.push(color.2);
                // placeholder normal (will accumulate later)
                vertices.push(0.0);
                vertices.push(0.0);
                vertices.push(0.0);
            }

            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 3);
        };

        // Colors per face
        let red = (1.0, 0.0, 0.0);
        let green = (0.0, 1.0, 0.0);
        let blue = (0.0, 0.0, 1.0);
        let yellow = (1.0, 1.0, 0.0);
        let magenta = (1.0, 0.0, 1.0);
        let cyan = (0.0, 1.0, 1.0);

        // Front (+Z)
        push_face(
            &[
                (-0.5, -0.5, 0.5),
                (0.5, -0.5, 0.5),
                (0.5, 0.5, 0.5),
                (-0.5, 0.5, 0.5),
            ],
            red,
        );

        // Back (-Z)
        push_face(
            &[
                (0.5, -0.5, -0.5),
                (-0.5, -0.5, -0.5),
                (-0.5, 0.5, -0.5),
                (0.5, 0.5, -0.5),
            ],
            green,
        );

        // Left (-X)
        push_face(
            &[
                (-0.5, -0.5, -0.5),
                (-0.5, -0.5, 0.5),
                (-0.5, 0.5, 0.5),
                (-0.5, 0.5, -0.5),
            ],
            blue,
        );

        // Right (+X)
        push_face(
            &[
                (0.5, -0.5, 0.5),
                (0.5, -0.5, -0.5),
                (0.5, 0.5, -0.5),
                (0.5, 0.5, 0.5),
            ],
            yellow,
        );

        // Top (+Y)
        push_face(
            &[
                (-0.5, 0.5, 0.5),
                (0.5, 0.5, 0.5),
                (0.5, 0.5, -0.5),
                (-0.5, 0.5, -0.5),
            ],
            magenta,
        );

        // Bottom (-Y)
        push_face(
            &[
                (-0.5, -0.5, -0.5),
                (0.5, -0.5, -0.5),
                (0.5, -0.5, 0.5),
                (-0.5, -0.5, 0.5),
            ],
            cyan,
        );

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

        (vertices, indices)
    }
}
