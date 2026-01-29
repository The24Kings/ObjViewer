use glam::Vec3;

use crate::game::Transform;

/// Trait for objects that participate in physics simulation.
/// Implement this for any object that needs velocity, collision, or physics updates.
pub trait Physical: Send + Sync {
    /// Called each frame to update physics state (velocity, acceleration, etc.)
    fn update(&mut self, dt: f32);

    /// Get the current velocity of the object
    fn velocity(&self) -> Vec3;

    /// Set the velocity of the object
    fn set_velocity(&mut self, velocity: Vec3);

    /// Get a reference to the object's transform
    fn transform(&self) -> &Transform;

    /// Get a mutable reference to the object's transform
    fn transform_mut(&mut self) -> &mut Transform;
}
