use glow::{Context, HasContext};
use std::rc::Rc;

use crate::graphics::Shader;

pub struct Material {
    pub shader: Rc<Shader>,
}

impl Material {
    pub fn new(shader: Rc<Shader>) -> Self {
        Self { shader }
    }

    pub fn shader(&self) -> &Shader {
        &self.shader
    }

    pub fn shader_mut(&mut self) -> &mut Shader {
        Rc::get_mut(&mut self.shader).unwrap()
    }

    pub fn apply(&self, gl: &Context) {
        self.shader.bind();

        for (_, loc) in &self.shader.attributes {
            unsafe {
                gl.enable_vertex_attrib_array(*loc);
            }
        }
    }
}
