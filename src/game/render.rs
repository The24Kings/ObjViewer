use glam::Mat4;

use crate::graphics::{Material, Mesh};

/// Trait for objects that can be rendered.
/// Implement this for any object that has a mesh and material.
pub trait Renderable: Send + Sync {
    fn material(&self) -> &Material;
    fn mesh(&self) -> &Mesh;

    fn material_mut(&mut self) -> &mut Material;
    fn mesh_mut(&mut self) -> &mut Mesh;

    fn model_matrix(&self) -> Mat4 {
        Mat4::IDENTITY
    }

    fn animate(&mut self, _dt: f32);
}
