use crate::{
    game::{Renderable, Transform},
    graphics::{Material, Mesh, Vertex},
};
use glam::{Mat4, Vec3};

pub struct Light {
    pub material: Material,
    pub mesh: Mesh,
    pub transform: Transform,
}

impl Renderable for Light {
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
        let dt = dt * 5.0;

        let rotation_x = glam::Quat::from_rotation_x(0.5 * dt as f32);
        let rotation_z = glam::Quat::from_rotation_z(0.5 * dt as f32);
        self.transform.rotation = rotation_x * rotation_z * self.transform.rotation;
    }
}

impl Light {
    pub fn new(material: Material) -> Self {
        let (vertices, indices) = Self::data();

        let mesh = Mesh {
            vao: None,
            vbo: None,
            ibo: None,
            vertices,
            indices,
        };

        Self {
            material,
            mesh,
            transform: Transform::default(),
        }
    }

    fn data() -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(6 * 4); // 6 faces * 4 points
        let mut indices: Vec<u32> = Vec::with_capacity(36);

        // Helper to push a face (4 verts, color, and 6 indices)
        let mut push_face = |positions: &[Vec3], color: Vec3| {
            let base = vertices.len() as u32;

            // push vertex data (position + color + placeholder normal)
            for &pos in positions.iter() {
                vertices.push(Vertex::with_color(pos, color, Vec3::ZERO));
            }

            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 3);
        };

        // Colors per face
        let red = Vec3::new(1.0, 0.0, 0.0);
        let green = Vec3::new(0.0, 1.0, 0.0);
        let blue = Vec3::new(0.0, 0.0, 1.0);
        let yellow = Vec3::new(1.0, 1.0, 0.0);
        let magenta = Vec3::new(1.0, 0.0, 1.0);
        let cyan = Vec3::new(0.0, 1.0, 1.0);

        // Front (+Z)
        push_face(
            &[
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
            ],
            red,
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
        );

        (vertices, indices)
    }
}
