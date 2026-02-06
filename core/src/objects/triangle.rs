use crate::{
    game::{Renderable, Transform},
    graphics::{Material, Mesh, Vertex},
    objects::calculate_normals,
};
use glam::{Mat4, Vec3};

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
        let mut vertices: Vec<Vertex> = vec![
            Vertex::with_color(Vec3::new(0.0, 0.5, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO),   // top (red)
            Vertex::with_color(Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::ZERO), // left (green)
            Vertex::with_color(Vec3::new(0.5, -0.5, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::ZERO),  // right (blue)
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

        Self {
            material,
            mesh,
            transform: Transform::default(),
        }
    }
}
