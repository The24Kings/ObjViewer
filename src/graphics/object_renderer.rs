use glam::Mat4;
use glow::{Context, HasContext};
use std::rc::Rc;

use crate::game::Renderable;

pub struct ObjectRenderer {
    gl: Rc<Context>,
    render_targets: Vec<Box<dyn Renderable>>,
}

#[allow(dead_code)]
impl ObjectRenderer {
    pub fn new(gl: Rc<Context>) -> Result<Self, String> {
        Ok(Self {
            gl,
            render_targets: Vec::new(),
        })
    }

    pub fn add_renderable<T: 'static + Renderable>(&mut self, renderable: T) {
        self.render_targets.push(Box::new(renderable));
    }

    pub fn update(&self) {}

    pub fn draw(&mut self, pvMatrix: &Mat4) {
        for renderable in &self.render_targets {
            let material = renderable.material();
            let mesh = renderable.mesh();

            material.apply(&self.gl);

            // Set uniforms
            if let Some(loc) = material.shader.getUniformLocation("pvMatrix") {
                unsafe {
                    self.gl
                        .uniform_matrix_4_f32_slice(Some(&loc), false, pvMatrix.as_ref());
                }
            }

            // Draw mesh
            mesh.draw(&self.gl);
        }
    }
}
