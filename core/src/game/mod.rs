pub mod camera;
pub mod physics;
pub mod physics_manager;
pub mod render;
pub mod render_manager;
pub mod transform;

pub use camera::Camera;
pub use camera::Frustum;
pub use camera::Projection;
pub use physics::Physical;
pub use physics_manager::PhysicsManager;
pub use render::Renderable;
pub use render_manager::RenderManager;
pub use transform::Transform;

// Re-export type aliases from graphics for convenience
pub use crate::graphics::{GpuContext, PhysicalRef, RenderableRef};

pub trait GameObject: Renderable + Physical {}

pub trait GlobalLight: Renderable + Physical {
    fn ambient(&self) -> f32 {
        1.0
    }

    fn specular(&self) -> f32 {
        0.0
    }

    fn ambient_mut(&mut self) -> &mut f32;

    fn specular_mut(&mut self) -> &mut f32;
}
