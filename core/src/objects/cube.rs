use crate::{
    game::{GameObject, Physical, Renderable, Transform},
    graphics::{Material, Mesh, Vertex},
    objects::calculate_normals,
};
use glam::{Mat4, Vec3};

pub struct Cube {
    pub material: Material,
    pub mesh: Mesh,
    pub transform: Transform,
    pub velocity: Vec3,
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

    fn animate(&mut self, dt: f32) {
        // Spin
        let rotation_x = glam::Quat::from_rotation_x(0.5 * dt as f32);
        let rotation_y = glam::Quat::from_rotation_y(0.5 * dt as f32);
        let rotation_z = glam::Quat::from_rotation_z(0.5 * dt as f32);
        self.transform.rotation = rotation_x * rotation_y * -rotation_z * self.transform.rotation;

        // Bobbing on Y axis using precomputed sin wave
        let speed: f32 = 0.5; // cycles per second
        let samples = self.sin_wave.len() as f32;

        // advance index (wrap-around)
        self.sin_index = (self.sin_index + dt * speed * samples) % samples;
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

impl Physical for Cube {
    fn update(&mut self, dt: f32) {
        // Apply velocity to position
        self.transform.position += self.velocity * dt;
    }

    fn velocity(&self) -> Vec3 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

// Implement the GameObject super-trait for Cube (requires both Renderable + Physical)
impl GameObject for Cube {}

impl Cube {
    pub fn new(material: Material) -> Self {
        let (mut vertices, indices) = Self::data();

        calculate_normals(&mut vertices, &indices);

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
            velocity: Vec3::ZERO,
            sin_wave,
            sin_index: 0.0,
        }
    }

    fn data() -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(6 * 4); // 6 faces * 4 points
        let mut indices: Vec<u32> = Vec::with_capacity(36);

        // Standard UV coordinates for a quad (same texture on each face)
        let uv_bl = glam::Vec2::new(0.0, 1.0); // bottom-left
        let uv_br = glam::Vec2::new(1.0, 1.0); // bottom-right
        let uv_tr = glam::Vec2::new(1.0, 0.0); // top-right
        let uv_tl = glam::Vec2::new(0.0, 0.0); // top-left

        // Colors per face
        let red = Vec3::new(1.0, 0.0, 0.0);
        let green = Vec3::new(0.0, 1.0, 0.0);
        let blue = Vec3::new(0.0, 0.0, 1.0);
        let yellow = Vec3::new(1.0, 1.0, 0.0);
        let magenta = Vec3::new(1.0, 0.0, 1.0);
        let cyan = Vec3::new(0.0, 1.0, 1.0);

        // Helper to push a face (4 verts, color, tex coords, and 6 indices)
        let mut push_face = |positions: &[Vec3], color: Vec3, uvs: &[glam::Vec2]| {
            let base = vertices.len() as u32;

            // push vertex data (position + color + placeholder normal + tex coords)
            for (i, &pos) in positions.iter().enumerate() {
                vertices.push(Vertex {
                    position: pos,
                    color,
                    normal: Vec3::ZERO,
                    tex_coords: uvs[i],
                });
            }

            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 3);
        };

        // Front (+Z)
        push_face(
            &[
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
            ],
            red,
            &[uv_bl, uv_br, uv_tr, uv_tl],
        );

        // Back (-Z)
        push_face(
            &[
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
            ],
            green,
            &[uv_bl, uv_br, uv_tr, uv_tl],
        );

        // Left (-X)
        push_face(
            &[
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, -0.5),
            ],
            blue,
            &[uv_bl, uv_br, uv_tr, uv_tl],
        );

        // Right (+X)
        push_face(
            &[
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, 0.5),
            ],
            yellow,
            &[uv_bl, uv_br, uv_tr, uv_tl],
        );

        // Top (+Y)
        push_face(
            &[
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
            ],
            magenta,
            &[uv_bl, uv_br, uv_tr, uv_tl],
        );

        // Bottom (-Y)
        push_face(
            &[
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(-0.5, -0.5, 0.5),
            ],
            cyan,
            &[uv_bl, uv_br, uv_tr, uv_tl],
        );

        (vertices, indices)
    }
}
