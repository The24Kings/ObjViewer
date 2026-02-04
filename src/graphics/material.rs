use glow::{Context, HasContext};
use std::sync::Arc;

use crate::graphics::{Shader, Texture};

pub struct Material {
    pub shader: Arc<Shader>,
    pub texture: Option<Arc<Texture>>,
    default_texture: Arc<Texture>,
}

impl Material {
    pub fn new(gl: Arc<Context>, shader: Arc<Shader>) -> Self {
        let default_texture =
            Arc::new(Texture::white_1x1(gl).expect("Failed to create default white texture"));
        Self {
            shader,
            texture: None,
            default_texture,
        }
    }

    pub fn shader(&self) -> &Shader {
        &self.shader
    }

    pub fn shader_mut(&mut self) -> &mut Shader {
        Arc::get_mut(&mut self.shader).unwrap()
    }

    pub fn texture(&self) -> &Texture {
        match &self.texture {
            Some(tex) => tex,
            None => panic!("No texture attatched to material"),
        }
    }

    pub fn texture_mut(&mut self) -> &mut Texture {
        match &mut self.texture {
            Some(tex) => Arc::get_mut(tex).unwrap(),
            None => panic!("No texture attatched to material"),
        }
    }

    pub fn apply(&self, gl: &Context) {
        self.shader.bind();

        match &self.texture {
            Some(tex) => tex.bind(),
            None => self.default_texture.bind(),
        }

        for (_, loc) in &self.shader.attributes {
            unsafe {
                gl.enable_vertex_attrib_array(*loc);
            }
        }
    }
}
