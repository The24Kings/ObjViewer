use bytemuck::cast_slice;
use glow::{Buffer, Context, HasContext, VertexArray};
use limited_gl::gl_check_error;
use std::mem::{offset_of, size_of};
use std::sync::Arc;

use crate::graphics::{Shader, VEC3};
use crate::graphics::{VEC2, Vertex};

pub struct Mesh {
    pub vao: Option<VertexArray>,
    pub vbo: Option<Buffer>,
    pub ibo: Option<Buffer>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn draw(&self, gl: &Context) {
        if !self.is_uploaded() {
            panic!("Mesh not uploaded to GPU");
        }

        unsafe {
            gl.bind_vertex_array(self.vao);

            gl_check_error!(gl);

            if let Some(_) = self.ibo {
                gl.draw_elements(
                    glow::TRIANGLES,
                    self.indices.len() as i32,
                    glow::UNSIGNED_INT,
                    0,
                );
            } else {
                gl.draw_arrays(glow::TRIANGLES, 0, self.vertices.len() as i32);
            }

            gl.bind_vertex_array(None);
        }
    }

    fn is_uploaded(&self) -> bool {
        self.vao.is_some() && self.vbo.is_some()
    }

    pub fn upload(&mut self, gl: &Context, shader: Arc<Shader>) -> Result<(), String> {
        unsafe {
            let vao = gl
                .create_vertex_array()
                .expect("Failed to create vertex array");

            let vbo = gl
                .create_named_buffer()
                .expect("Failed to create vertex buffer");

            gl.bind_vertex_array(Some(vao));

            gl_check_error!(gl);

            let stride = size_of::<Vertex>() as i32;
            gl.vertex_array_vertex_buffer(vao, 0, Some(vbo), 0, stride);

            gl_check_error!(gl);

            // Upload vertex data
            gl.named_buffer_data_u8_slice(vbo, cast_slice(&self.vertices), glow::STATIC_DRAW);

            gl_check_error!(gl);

            // Upload index data if present
            if !self.indices.is_empty() {
                let ibo = gl
                    .create_named_buffer()
                    .expect("Failed to create index buffer");

                gl.vertex_array_element_buffer(vao, Some(ibo));
                gl.named_buffer_data_u8_slice(ibo, cast_slice(&self.indices), glow::STATIC_DRAW);
                gl_check_error!(gl);

                self.ibo = Some(ibo);
            }

            self.vao = Some(vao);
            self.vbo = Some(vbo);

            // Setup vertex attributes
            for (name, loc) in &shader.attributes {
                let (offset, size) = match *name {
                    "i_position" => (offset_of!(Vertex, position), VEC3),
                    "i_color" => (offset_of!(Vertex, color), VEC3),
                    "i_normal" => (offset_of!(Vertex, normal), VEC3),
                    "i_tex_coords" => (offset_of!(Vertex, tex_coords), VEC2),
                    _ => {
                        return Err(format!("Unknown attribute name: {}", name));
                    }
                };
                gl.vertex_array_attrib_format_f32(
                    vao,
                    *loc,
                    size,
                    glow::FLOAT,
                    false,
                    offset as u32,
                );
                gl.vertex_array_attrib_binding_f32(vao, *loc, 0);
                gl.enable_vertex_array_attrib(vao, *loc);
                gl_check_error!(gl);
            }

            Ok(())
        }
    }
}
