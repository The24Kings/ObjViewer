#![allow(dead_code)]
use glow::{HasContext, Program, Shader};
use std::io::{Error, ErrorKind};

use crate::gl_check_error;
use crate::graphics::GlRef;

#[derive(Clone)]
pub(crate) struct ShaderSource {
    gl: GlRef,
    pub shader_type: u32,
    pub handle: Shader,
    pub filepath: &'static str,
    destroyed: bool,
}

#[allow(dead_code)]
impl ShaderSource {
    pub(crate) fn new(
        renderer: GlRef,
        program: Program,
        shader_type: u32,
        source: &str,
        filepath: &'static str,
    ) -> Result<Self, Error> {
        unsafe {
            let shader = renderer
                .create_shader(shader_type)
                .expect("Failed to create program");

            renderer.shader_source(shader, source);
            renderer.compile_shader(shader);

            gl_check_error!(&renderer);

            if !renderer.get_shader_compile_status(shader) {
                let e = renderer.get_shader_info_log(shader);
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unable to compile shader: {e}"),
                ));
            }

            renderer.attach_shader(program, shader);

            gl_check_error!(&renderer);

            Ok(Self {
                gl: renderer,
                shader_type,
                handle: shader,
                filepath,
                destroyed: false,
            })
        }
    }

    /// Remove shader from GPU memory
    pub(crate) fn delete(&mut self) {
        if self.destroyed {
            return;
        }

        unsafe {
            self.gl.delete_shader(self.handle);
        }

        self.destroyed = true;
    }
}

impl Drop for ShaderSource {
    fn drop(&mut self) {
        self.delete();
    }
}
