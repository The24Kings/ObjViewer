use crate::{
    game::Transform,
    graphics::{Material, Mesh, Renderable},
};
use glam::Mat4;

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
        let rotation_x = glam::Quat::from_rotation_x(0.5 * dt as f32);
        let rotation_y = glam::Quat::from_rotation_y(0.5 * dt as f32);
        let rotation_z = glam::Quat::from_rotation_z(0.5 * dt as f32);
        self.transform.rotation = rotation_x * rotation_y * -rotation_z * self.transform.rotation;
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

        (vertices, indices)
    }
}
