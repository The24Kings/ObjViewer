use crate::graphics::{Material, Mesh, Renderable};

pub struct Triangle {
    pub material: Material,
    pub mesh: Mesh,
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

    fn animate(&mut self, _dt: f32) {
        // No animation for the triangle
    }
}

impl Triangle {
    pub fn new(material: Material) -> Self {
        let vertices: Vec<f32> = vec![
            0.0, 0.5, 0.0, 1.0, 0.0, 0.0, // top (red)
            -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // left (green)
            0.5, -0.5, 0.0, 0.0, 0.0, 1.0, // right (blue)
        ];
        let indices: Vec<u32> = vec![0, 1, 2];

        let mesh = Mesh {
            vao: None,
            vbo: None,
            ibo: None,
            vertices,
            indices,
        };

        Self { material, mesh }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        for i in 0..self.mesh.vertices.len() / 6 {
            self.mesh.vertices[i * 6 + 0] += x;
            self.mesh.vertices[i * 6 + 1] += y;
            self.mesh.vertices[i * 6 + 2] += z;
        }
    }
}
