use std::sync::{Arc, Mutex};

use crate::game::Physical;

pub struct PhysicsManager {
    pub physical_targets: Vec<Arc<Mutex<dyn Physical>>>,
}

impl PhysicsManager {
    pub fn new() -> Self {
        Self {
            physical_targets: Vec::new(),
        }
    }

    pub fn add_physical(&mut self, physical: Arc<Mutex<dyn Physical>>) {
        self.physical_targets.push(physical);
    }

    pub fn update(&mut self, dt: f32) {
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
