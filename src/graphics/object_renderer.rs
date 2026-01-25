use glam::{Mat4, Vec3};
use glow::Context;
use std::rc::Rc;

use crate::game::{Camera, Renderable};

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

    pub fn draw(&mut self, model: &Mat4, camera: &Camera) {
        for renderable in &self.render_targets {
            let material = renderable.material();
            let mesh = renderable.mesh();

            material.apply(&self.gl);

            // Set uniforms
            material.shader.setUniform4fm("pv", model);
            material
                .shader
                .setUniform4fm("model", &renderable.model_matrix());

            material.shader.setUniform1f("ambient", 0.2);
            material.shader.setUniform1f("specular", 0.5);
            material //FIXME: Actually have a light source in the view_port
                .shader
                .setUniform3fv("light_pos", &Vec3::new(3.0, 5.0, 2.0));
            material
                .shader
                .setUniform3fv("view_pos", &camera.transform.position);

            // Draw mesh
            mesh.draw(&self.gl);
        }
    }
}
