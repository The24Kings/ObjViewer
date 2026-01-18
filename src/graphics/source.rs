use glow::{Context, HasContext, NativeProgram, NativeShader};
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct ShaderSource {
    gl: Rc<Context>,
    pub handle: NativeShader,
    destroyed: bool,
}

#[allow(dead_code)]
impl ShaderSource {
    pub(crate) fn new(
        renderer: Rc<Context>,
        program: NativeProgram,
        shader_type: u32,
        source: &str,
    ) -> Self {
        unsafe {
            let shader = renderer
                .create_shader(shader_type)
                .expect("Failed to create program");

            renderer.shader_source(shader, source);
            renderer.compile_shader(shader);

            if !renderer.get_shader_compile_status(shader) {
                let error = renderer.get_shader_info_log(shader);
                panic!("Failed to compile shader: {}", error);
            }

            renderer.attach_shader(program, shader);

            Self {
                gl: renderer,
                handle: shader,
                destroyed: false,
            }
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
