use glam::Mat4;
use glow::Context;
use std::rc::Rc;

use crate::game::Renderable;

pub struct ObjectRenderer {
    gl: Rc<Context>,
    pub render_targets: Vec<Box<dyn Renderable>>,
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

    // Animation and other updates
    pub fn update(&mut self, dt: f32) {
        for renderable in &mut self.render_targets {
            renderable.animate(dt);
        }
    }

    pub fn draw(&mut self, pv: &Mat4) {
        for renderable in &self.render_targets {
            let material = renderable.material();
            let mesh = renderable.mesh();

            material.apply(&self.gl);

            // Set uniforms
            material.shader.setUniform4fm("pv", pv);
            material
                .shader
                .setUniform4fm("model", &renderable.model_matrix());

            // Draw mesh
            mesh.draw(&self.gl);
        }
    }
}
