use crate::{
    game::{GlobalLight, Physical, Renderable, Transform},
    graphics::{Material, Mesh, Vertex},
};
use glam::{Mat4, Vec3};

#[derive(Clone)]
pub struct Light {
    pub material: Material,
    pub mesh: Mesh,
    pub transform: Transform,

    ambient: f32,
    specular: f32,
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

impl Physical for Light {
    fn update(&mut self, dt: f32) {
        self.transform.position += Vec3::ZERO * dt;
    }

    fn velocity(&self) -> Vec3 {
        Vec3::ZERO
    }

    fn set_velocity(&mut self, _: Vec3) {}

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}

impl GlobalLight for Light {
    fn ambient(&self) -> f32 {
        self.ambient
    }

    fn specular(&self) -> f32 {
        self.specular
    }

    fn ambient_mut(&mut self) -> &mut f32 {
        &mut self.ambient
    }

    fn specular_mut(&mut self) -> &mut f32 {
        &mut self.specular
    }
}

impl Light {
    pub fn new(material: Material) -> Self {
        let (vertices, indices) = Self::data();

        let mesh = Mesh {
            vertex_buffer: None,
            index_buffer: None,
            vertices,
            indices,
        };

        Self {
            material,
            mesh,
            transform: Transform::default(),

            ambient: 0.2,
            specular: 0.5,
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
