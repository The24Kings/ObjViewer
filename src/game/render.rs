pub mod material;
pub mod mesh;

use crate::game::Material;
use crate::game::Mesh;
use glam::Mat4;

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
