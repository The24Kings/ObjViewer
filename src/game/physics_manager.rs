use std::sync::{Arc, Mutex};

use crate::game::Physical;

/// Manages all physical objects in the scene.
/// Handles physics updates and will support collision detection in the future.
pub struct PhysicsManager {
    pub physical_targets: Vec<Arc<Mutex<dyn Physical>>>,
}

impl PhysicsManager {
    pub fn new() -> Self {
        Self {
            physical_targets: Vec::new(),
        }
    }

    /// Add a physical object to be managed by the physics system
    pub fn add_physical(&mut self, physical: Arc<Mutex<dyn Physical>>) {
        self.physical_targets.push(physical);
    }

    /// Update all physical objects
    pub fn physics_update(&mut self, dt: f32) {
        for physical in &self.physical_targets {
            if let Ok(mut obj) = physical.lock() {
                obj.update(dt);
            }
        }
    }

    // TODO: Add collision detection methods here
    // pub fn check_collisions(&self) -> Vec<CollisionEvent> { ... }
    // pub fn resolve_collisions(&mut self, events: &[CollisionEvent]) { ... }
}

impl Default for PhysicsManager {
    fn default() -> Self {
        Self::new()
    }
}
