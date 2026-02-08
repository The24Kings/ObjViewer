use bytemuck::{cast_slice, offset_of};
use glow::{Buffer, Context, HasContext, VertexArray};
use std::mem::size_of;

use crate::gl_check_error;
use crate::graphics::{ShaderRef, VEC3};
use crate::graphics::{VEC2, Vertex};

#[derive(Clone)]

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

    pub fn upload(&mut self, gl: &Context, shader: ShaderRef) -> Result<(), String> {
        unsafe {
            let vao = gl
                .create_vertex_array()
                .expect("Failed to create vertex array");

            let vbo = gl.create_buffer().expect("Failed to create vertex buffer");

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            gl_check_error!(gl);

            // Upload vertex data
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                cast_slice(&self.vertices),
                glow::STATIC_DRAW,
            );

            gl_check_error!(gl);

            // Upload index data if present
            if !self.indices.is_empty() {
                let ibo = gl.create_buffer().expect("Failed to create index buffer");

                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
                gl.buffer_data_u8_slice(
                    glow::ELEMENT_ARRAY_BUFFER,
                    cast_slice(&self.indices),
                    glow::STATIC_DRAW,
                );
                gl_check_error!(gl);

                self.ibo = Some(ibo);
            }

            self.vao = Some(vao);
            self.vbo = Some(vbo);

            let stride = size_of::<Vertex>() as i32;

            // Setup vertex attributes
            for (name, loc) in &shader.attributes {
                let (offset, size) = match *name {
                    "i_position" => (offset_of!(Vertex, position), VEC3),
                    "i_color" => (offset_of!(Vertex, color), VEC3),
                    "i_normal" => (offset_of!(Vertex, normal), VEC3),
                    "i_uv" => (offset_of!(Vertex, tex_coords), VEC2),
                    _ => {
                        return Err(format!("Unknown attribute name: {}", name));
                    }
                };
                gl.enable_vertex_attrib_array(*loc);
                gl.vertex_attrib_pointer_f32(*loc, size, glow::FLOAT, false, stride, offset as i32);
                gl_check_error!(gl);
            }

            gl.bind_vertex_array(None);
            gl.bind_buffer(glow::ARRAY_BUFFER, None);

            Ok(())
        }
    }
}
