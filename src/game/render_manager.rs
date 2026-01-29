use glam::{Mat4, Vec3};
use glow::Context;
use std::sync::{Arc, Mutex};

use crate::{game::Camera, game::Renderable};

pub struct RenderManager {
    gl: Arc<Context>,
    pub render_targets: Vec<Arc<Mutex<dyn Renderable>>>,
}

impl RenderManager {
    pub fn new(gl: Arc<Context>) -> Result<Self, String> {
        Ok(Self {
            gl,
            render_targets: Vec::new(),
        })
    }

    pub fn add_renderable(&mut self, renderable: Arc<Mutex<dyn Renderable>>) {
        self.render_targets.push(renderable);
    }

    // Animation and other updates
    pub fn update(&mut self, dt: f32) {
        for renderable in &self.render_targets {
            if let Ok(mut obj) = renderable.lock() {
                obj.animate(dt);
            }
        }
    }

    pub fn draw(&mut self, model: &Mat4, camera: &Camera) {
        for renderable in &self.render_targets {
            if let Ok(obj) = renderable.lock() {
                let material = obj.material();
                let mesh = obj.mesh();

                material.apply(&self.gl);

                // Set uniforms
                material.shader.setUniform4fm("pv", model);
                material.shader.setUniform4fm("model", &obj.model_matrix());

                material.shader.setUniform1f("ambient", 0.2);
                material.shader.setUniform1f("specular", 0.5);
                material //FIXME: Actually have a light source in the view_port
                    .shader
                    .setUniform3fv("light_pos", &Vec3::new(1.0, 1.0, 1.0));
                material
                    .shader
                    .setUniform3fv("view_pos", &camera.transform.position);

                // Draw mesh
                mesh.draw(&self.gl);
            }
        }
    }
}
