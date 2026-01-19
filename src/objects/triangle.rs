use crate::game::{Material, Mesh, Renderable};

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
}
