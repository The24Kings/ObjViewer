use crate::graphics::PhysicalRef;

pub struct PhysicsManager {
    pub physical_targets: Vec<PhysicalRef>,
}

impl PhysicsManager {
    pub fn new() -> Self {
        Self {
            physical_targets: Vec::new(),
        }
    }

    pub fn add_physical(&mut self, physical: PhysicalRef) {
        self.physical_targets.push(physical);
    }

    pub fn update(&mut self, dt: f32) {
        for physical in &self.physical_targets {
            physical.borrow_mut().update(dt);
        }
    }

    // TODO: Add collision detection methods here
    // pub fn check_collisions(&self) -> Vec<CollisionEvent> { ... }
    // pub fn resolve_collisions(&mut self, events: &[CollisionEvent]) { ... }
}
