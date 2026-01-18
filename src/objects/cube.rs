use crate::game::{Material, Mesh, Renderable};

pub struct Cube {
    pub material: Material,
    pub mesh: Mesh,
}

impl Renderable for Cube {
    fn material(&self) -> &Material {
        &self.material
    }

    fn mesh(&self) -> &Mesh {
        &self.mesh
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

        Self { material, mesh }
    }

    /// Simple translation of the cube by modifying its vertex positions
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        for i in 0..self.mesh.vertices.len() / 6 {
            self.mesh.vertices[i * 6 + 0] += x;
            self.mesh.vertices[i * 6 + 1] += y;
            self.mesh.vertices[i * 6 + 2] += z;
        }
    }

    fn data() -> (Vec<f32>, Vec<u32>) {
        let mut vertices: Vec<f32> = Vec::with_capacity(24 * 6);
        let mut indices: Vec<u32> = Vec::with_capacity(36);

        // Helper to push a face (4 verts, color, and 6 indices)
        let mut push_face = |positions: &[(f32, f32, f32)], color: (f32, f32, f32)| {
            let base = (vertices.len() / 6) as u32;
            for &(x, y, z) in positions.iter() {
                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
                vertices.push(color.0);
                vertices.push(color.1);
                vertices.push(color.2);
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
