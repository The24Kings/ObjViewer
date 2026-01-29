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

/// A super-trait for objects that need both rendering and physics capabilities.
/// Objects like `Cube` implement this. Render-only objects (like `Light`) only
/// implement `Renderable`. Invisible physical objects only implement `Physical`.
pub trait GameObject: Renderable + Physical {}
