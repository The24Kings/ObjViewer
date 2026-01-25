pub mod material;
pub mod mesh;

use glam::Mat4;

use crate::graphics::{Material, Mesh};

pub trait Renderable {
    fn material(&self) -> &Material;
    fn mesh(&self) -> &Mesh;

    fn material_mut(&mut self) -> &mut Material;
    fn mesh_mut(&mut self) -> &mut Mesh;

    fn model_matrix(&self) -> Mat4 {
        Mat4::IDENTITY
    }

    fn animate(&mut self, _dt: f32);
}
