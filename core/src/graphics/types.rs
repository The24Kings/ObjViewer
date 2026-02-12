//! Type aliases for smart pointers used throughout the codebase.
//! Uses Rc/RefCell for single-threaded reference counting.

use glow::Context;
use std::cell::RefCell;
use std::rc::Rc;
use winit::window::Window;

use super::{Shader, Texture};
use crate::game::{GameObject, GlobalLight, Physical, Renderable};

/// Reference-counted pointer to a Window
pub type WindowRef = Rc<Window>;

/// Reference-counted pointer to a GL context
pub type GlRef = Rc<Context>;

/// Reference-counted pointer to a Shader
pub type ShaderRef = Rc<Shader>;

/// Reference-counted pointer to a Texture  
pub type TextureRef = Rc<Texture>;

/// Reference-counted pointer to a Renderable trait object
pub type RenderableRef = Rc<RefCell<dyn Renderable>>;

/// Reference-counted pointer to a Physical trait object
pub type PhysicalRef = Rc<RefCell<dyn Physical>>;

// Reference-counted pointer to a Game Object trait object
pub type GameObjectRef = Rc<RefCell<dyn GameObject>>;

// Reference-counted pointer to a Game Object trait object
pub type LightObjectRef = Rc<RefCell<dyn GlobalLight>>;

/// Helper to create a new GlRef
pub fn new_gl_ref(ctx: Context) -> GlRef {
    Rc::new(ctx)
}

/// Helper to create a new ShaderRef
pub fn new_shader_ref(shader: Shader) -> ShaderRef {
    Rc::new(shader)
}

/// Helper to create a new TextureRef
pub fn new_texture_ref(texture: Texture) -> TextureRef {
    Rc::new(texture)
}

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
