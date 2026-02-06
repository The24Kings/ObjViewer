#![allow(non_snake_case)]
use glam::{Mat4, Vec2, Vec3, Vec4};
use glow::{HasContext, Program, UniformLocation};
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use tracing::error;

use crate::gl_check_error;
use crate::graphics::GlRef;
use crate::graphics::ShaderSource;

#[derive(Clone)]
pub struct Shader {
    gl: GlRef,
    pub(crate) handle: Program,
    pub(crate) attributes: HashMap<&'static str, u32>, // Name and Location
    pub(crate) sources: Vec<ShaderSource>,
    destroyed: bool,
}

// Create a basic loaded object shader
#[macro_export]
macro_rules! loaded_shader {
    ($gl:expr) => {{
        let mut shader = crate::graphics::Shader::new($gl.clone());
        shader
            .add(
                glow::FRAGMENT_SHADER,
                crate::graphics::LOADED_OBJ_FRAG_SRC,
                crate::graphics::LOADED_OBJ_FRAG_PATH,
            )
            .add(
                glow::VERTEX_SHADER,
                crate::graphics::LOADED_OBJ_VERT_SRC,
                crate::graphics::LOADED_OBJ_VERT_PATH,
            )
            .link();

        shader.add_attribute("i_position");
        shader.add_attribute("i_color");
        shader.add_attribute("i_normal");
        shader.add_attribute("i_uv");

        shader
    }};
}

// Create a basic loaded object shader
#[macro_export]
macro_rules! default_frag {
    ($gl:expr, $handle:expr) => {{
        ShaderSource::new(
            $gl.clone(),
            $handle,
            glow::FRAGMENT_SHADER,
            crate::graphics::DEFAULT_FRAG_SRC,
            crate::graphics::DEFAULT_FRAG_PATH,
        )
        .expect("Default fragment shader failed")
    }};
}

// Create a basic loaded object shader
#[macro_export]
macro_rules! default_vert {
    ($gl:expr, $handle:expr) => {{
        ShaderSource::new(
            $gl.clone(),
            $handle,
            glow::VERTEX_SHADER,
            crate::graphics::DEFAULT_VERT_SRC,
            crate::graphics::DEFAULT_VERT_PATH,
        )
        .expect("Default vertex shader failed")
    }};
}

#[allow(dead_code)]
impl Shader {
    pub fn new(renderer: GlRef) -> Self {
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
    pub fn add(&mut self, shader_type: u32, source: &str, filepath: &'static str) -> &mut Self {
        let src = ShaderSource::new(self.gl.clone(), self.handle, shader_type, source, filepath);

        let src = match src {
            Ok(src) => src,
            Err(e) => {
                error!("Failed to compiled shader, falling back to default: {e}");

                match shader_type {
                    glow::FRAGMENT_SHADER => default_frag!(self.gl.clone(), self.handle),
                    glow::VERTEX_SHADER => default_vert!(self.gl.clone(), self.handle),
                    _ => panic!("Unsupported default shader type"),
                }
            }
        };

        self.sources.push(src);

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
                let e = self.gl.get_program_info_log(self.handle);
                panic!("Shader failed to link: {e}");
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

    //TODO: Add a fallback for when loading the shader fails, don't just unwrap Nothing and Crash
    #[cfg(not(target_arch = "wasm32"))]
    pub fn reload_shader(
        gl: GlRef,
        shader: &mut Shader,
        vertex: &'static str,
        fragment: &'static str,
    ) {
        let vert = fs::read_to_string(vertex).expect("Failed to read vertex shader!");
        let frag = fs::read_to_string(fragment).expect("Failed to read fragment shader!");

        let mut reloaded_shader = Shader::new(gl.clone());
        reloaded_shader
            .add(glow::VERTEX_SHADER, vert.as_str(), vertex)
            .add(glow::FRAGMENT_SHADER, frag.as_str(), fragment)
            .link();

        reloaded_shader.add_attribute("i_position");
        reloaded_shader.add_attribute("i_color");
        reloaded_shader.add_attribute("i_normal");
        reloaded_shader.add_attribute("i_uv");

        if reloaded_shader.is_linked() {
            let old_handle = shader.handle;
            shader.handle = reloaded_shader.handle;

            // Prevent the temporary `reloaded_shader` from deleting its program in Drop.
            // Safety: The handle is passed to the Shader object which will delete the shader on the GPU when Dropped
            // All sources are deleted after linking.
            std::mem::forget(reloaded_shader);

            unsafe {
                gl.delete_program(old_handle);
            }
        }
    }

    /// Add attribute to the shader
    pub fn add_attribute(&mut self, name: &'static str) {
        let loc = self.getAttribLocation(name);

        match loc {
            Some(loc) => self.attributes.insert(name, loc),
            None => return, // panic!("Attribute not found")
        };
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

    fn getUniformLocation(&self, name: &str) -> Option<UniformLocation> {
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
