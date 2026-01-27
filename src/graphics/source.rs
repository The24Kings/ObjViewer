use glow::{Context, HasContext, NativeProgram, NativeShader};
use limited_gl::gl_check_error;
use std::io::{Error, ErrorKind};
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct ShaderSource {
    gl: Rc<Context>,
    pub shader_type: u32,
    pub handle: NativeShader,
    pub filepath: &'static str,
    destroyed: bool,
}

#[allow(dead_code)]
impl ShaderSource {
    pub(crate) fn new(
        renderer: Rc<Context>,
        program: NativeProgram,
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
