//! Type aliases for smart pointers used throughout the codebase.
//! Uses Rc/RefCell for single-threaded reference counting.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use winit::window::Window;

use crate::game::{GameObject, GlobalLight, Physical, Renderable};

/// Reference-counted pointer to a Window (Arc required by wgpu Surface).
pub type WindowRef = Arc<Window>;

/// Shared GPU context: device + queue.
/// Uses Arc so it can be shared across modules (single-threaded, but Arc
/// is required by wgpu's Send/Sync constraints).
#[derive(Clone)]
pub struct GpuContext {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
}

/// Reference-counted pointer to a Renderable trait object
pub type RenderableRef = Rc<RefCell<dyn Renderable>>;

/// Reference-counted pointer to a Physical trait object
pub type PhysicalRef = Rc<RefCell<dyn Physical>>;

// Reference-counted pointer to a Game Object trait object
pub type GameObjectRef = Rc<RefCell<dyn GameObject>>;

// Reference-counted pointer to a Light Object trait object
pub type LightObjectRef = Rc<RefCell<dyn GlobalLight>>;

/// Helper to create a new RenderableRef
pub fn new_renderable_ref<T: Renderable + 'static>(renderable: T) -> RenderableRef {
    Rc::new(RefCell::new(renderable))
}

/// Helper to create a new PhysicalRef
pub fn new_physical_ref<T: Physical + 'static>(physical: T) -> PhysicalRef {
    Rc::new(RefCell::new(physical))
}

pub fn new_game_obj_ref<T: GameObject + 'static>(object: T) -> GameObjectRef {
    Rc::new(RefCell::new(object))
}

pub fn new_light_obj_ref<T: GlobalLight + 'static>(object: T) -> LightObjectRef {
    Rc::new(RefCell::new(object))
}
