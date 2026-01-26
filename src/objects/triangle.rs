use glam::Mat4;
use crate::{game::Transform, graphics::{Material, Mesh, Renderable}, objects::calculate_normals};

pub struct Triangle {
    pub material: Material,
    pub mesh: Mesh,
    pub transform: Transform,
}

impl Renderable for Triangle {
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
        let rotation_y = glam::Quat::from_rotation_y(0.5 * dt as f32);
        self.transform.rotation = rotation_y * self.transform.rotation;
    }
}

impl Triangle {
    pub fn new(material: Material) -> Self {
        let mut vertices: Vec<f32> = vec![
            0.0, 0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, // top (red)
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, // left (green)
            0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, // right (blue)
        ];
        let indices: Vec<u32> = vec![0, 1, 2];

        calculate_normals(&mut vertices, &indices);

        let mesh = Mesh {
            vao: None,
            vbo: None,
            ibo: None,
            vertices,
            indices,
        };

        Self { material, mesh, transform: Transform::default() }
    }
}
