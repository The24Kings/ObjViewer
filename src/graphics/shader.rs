use glam::{Mat4, Vec2, Vec3, Vec4};
use glow::{Context, HasContext, NativeUniformLocation, Program};
use limited_gl::gl_check_error;
use std::{collections::HashMap, rc::Rc};

use crate::graphics::ShaderSource;

#[derive(Clone)]
pub struct Shader {
    gl: Rc<Context>,
    pub handle: Program,
    pub attributes: HashMap<&'static str, u32>, // Name and Location
    sources: Vec<ShaderSource>,
    destroyed: bool,
}

#[allow(dead_code)]
impl Shader {
    pub fn new(renderer: Rc<Context>) -> Self {
        unsafe {
            let program = renderer.create_program().expect("Failed to create program");

            gl_check_error!(&renderer);

            Self {
                gl: renderer,
                handle: program,
                attributes: HashMap::new(),
                sources: Vec::new(),
                destroyed: false,
            }
        }
    }

    /// Compile Shader and attach to the program
    pub fn add(&mut self, shader_type: u32, source: &str) -> &mut Self {
        self.sources.push(ShaderSource::new(
            self.gl.clone(),
            self.handle,
            shader_type,
            source,
        ));

        self
    }

    pub fn is_linked(&self) -> bool {
        unsafe { self.gl.get_program_link_status(self.handle) }
    }

    /// Link shader to the program
    pub fn link(&mut self) -> &mut Self {
        unsafe {
            self.gl.link_program(self.handle);

            gl_check_error!(&self.gl);

            if !self.is_linked() {
                let error = self.gl.get_program_info_log(self.handle);
                panic!("Failed to link shader: {}", error);
            }
        }

        for source in &mut self.sources {
            source.delete();
        }

        self
    }

    // Use the shader
    pub fn bind(&self) {
        if self.destroyed {
            return;
        }

        unsafe {
            self.gl.use_program(Some(self.handle));
            gl_check_error!(&self.gl);
        }
    }

    /// Add attribute to the shader
    pub fn add_attribute(&mut self, name: &'static str) {
        self.attributes
            .insert(name, self.getAttribLocation(name).unwrap());
    }

    /// Remove shader from GPU memory
    pub fn delete(&mut self) {
        if self.destroyed {
            return;
        }

        unsafe {
            self.gl.delete_program(self.handle);
        }

        self.destroyed = true;
    }

    fn getAttribLocation(&self, name: &str) -> Option<u32> {
        unsafe { self.gl.get_attrib_location(self.handle, name) }
    }

    fn getUniformLocation(&self, name: &str) -> Option<NativeUniformLocation> {
        unsafe { self.gl.get_uniform_location(self.handle, name) }
    }

    pub fn setUniform1i(&self, name: &str, value: i32) {
        unsafe {
            self.gl
                .uniform_1_i32(self.getUniformLocation(name).as_ref(), value);
        }
    }

    pub fn setUniform1ui(&self, name: &str, value: u32) {
        unsafe {
            self.gl
                .uniform_1_u32(self.getUniformLocation(name).as_ref(), value);
        }
    }

    pub fn setUniform1f(&self, name: &str, value: f32) {
        unsafe {
            self.gl
                .uniform_1_f32(self.getUniformLocation(name).as_ref(), value);
        }
    }

    pub fn setUniform2fv(&self, name: &str, value: &Vec2) {
        self.setUniform2f(name, value.x, value.y);
    }

    pub fn setUniform2f(&self, name: &str, x: f32, y: f32) {
        unsafe {
            self.gl
                .uniform_2_f32(self.getUniformLocation(name).as_ref(), x, y);
        }
    }

    pub fn setUniform3fv(&self, name: &str, value: &Vec3) {
        self.setUniform3f(name, value.x, value.y, value.z);
    }

    pub fn setUniform3f(&self, name: &str, x: f32, y: f32, z: f32) {
        unsafe {
            self.gl
                .uniform_3_f32(self.getUniformLocation(name).as_ref(), x, y, z);
        }
    }

    pub fn setUniform4fv(&self, name: &str, value: &Vec4) {
        self.setUniform4f(name, value.x, value.y, value.z, value.w);
    }

    pub fn setUniform4f(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            self.gl
                .uniform_4_f32(self.getUniformLocation(name).as_ref(), x, y, z, w);
        }
    }

    pub fn setUniform4fm(&self, name: &str, mat: &Mat4) {
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(
                self.getUniformLocation(name).as_ref(),
                false,
                &mat.to_cols_array(),
            );
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.delete();
    }
}
