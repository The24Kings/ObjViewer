use glam::Mat4;

use crate::game::Camera;
use crate::graphics::types::GameObjectRef;
use crate::graphics::{GlRef, RenderableRef};

pub struct RenderManager {
    gl: GlRef,
    pub render_targets: Vec<RenderableRef>,
}

impl RenderManager {
    pub fn new(gl: GlRef) -> Result<Self, String> {
        Ok(Self {
            gl,
            render_targets: Vec::new(),
        })
    }

    pub fn add_renderable(&mut self, renderable: RenderableRef) {
        self.render_targets.push(renderable);
    }

    // Animation and other updates
    pub fn update(&mut self, dt: f32) {
        for renderable in &self.render_targets {
            renderable.borrow_mut().animate(dt);
        }
    }

    pub fn draw(&mut self, model: &Mat4, camera: &Camera, sun: &GameObjectRef) {
        for renderable in &self.render_targets {
            let obj = renderable.borrow();
            let material = obj.material();
            let mesh = obj.mesh();

            material.apply(&self.gl);

            // Set uniforms
            material.shader.setUniform4fm("pv", model);
            material.shader.setUniform4fm("model", &obj.model_matrix());
            material.shader.setUniform1i("u_texture", 0); // Replace in the future with tex.unit for PBR

            material.shader.setUniform1f("u_ambient", 0.2);
            material.shader.setUniform1f("u_specular", 0.5);
            material
                .shader
                .setUniform3fv("u_light_pos", &sun.borrow().transform().position);
            material
                .shader
                .setUniform3fv("u_view_pos", &camera.transform.position);

            // Draw mesh
            mesh.draw(&self.gl);
        }
    }
}
