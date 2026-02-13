use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use std::sync::Arc;

use crate::game::Camera;
use crate::graphics::RenderableRef;
use crate::graphics::types::LightObjectRef;

/// Uniform data for the loaded-object shader (Phong lighting).
/// Must match the WGSL `Uniforms` struct in loaded_obj.wgsl.
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ObjUniforms {
    pub pv: Mat4,
    pub model: Mat4,
    pub light_pos: Vec3,
    pub ambient: f32,
    pub view_pos: Vec3,
    pub specular: f32,
}

/// Uniform data for the light-cube shader (no lighting).
/// Must match the WGSL `Uniforms` struct in light_cube.wgsl.
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct LightUniforms {
    pub pv: Mat4,
    pub model: Mat4,
}

pub struct RenderManager {
    pub render_targets: Vec<RenderableRef>,
    queue: Arc<wgpu::Queue>,
}

impl RenderManager {
    pub fn new(queue: Arc<wgpu::Queue>) -> Self {
        Self {
            render_targets: Vec::new(),
            queue,
        }
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

    /// Record draw commands into the given render pass.
    ///
    /// For each renderable we write its uniform data and issue draw calls.
    pub fn draw(
        &self,
        rpass: &mut wgpu::RenderPass<'_>,
        pv: &Mat4,
        camera: &Camera,
        sun: &LightObjectRef,
    ) {
        let sun_ref = sun.borrow();

        for renderable in &self.render_targets {
            let obj = renderable.borrow();
            let material = obj.material();
            let mesh = obj.mesh();

            // Determine uniform size: if this material's pipeline matches
            // the light pipeline it uses `LightUniforms`, else `ObjUniforms`.
            let uniform_size = material.uniform_buffer.size();

            if uniform_size == std::mem::size_of::<LightUniforms>() as u64 {
                let uniforms = LightUniforms {
                    pv: *pv,
                    model: obj.model_matrix(),
                };
                self.queue
                    .write_buffer(&material.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));
            } else {
                let uniforms = ObjUniforms {
                    pv: *pv,
                    model: obj.model_matrix(),
                    light_pos: sun_ref.transform().position,
                    ambient: sun_ref.ambient(),
                    view_pos: camera.transform.position,
                    specular: sun_ref.specular(),
                };

                self.queue
                    .write_buffer(&material.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));
            }

            rpass.set_pipeline(&material.shader.pipeline);
            rpass.set_bind_group(0, &material.bind_group, &[]);
            mesh.draw(rpass);
        }
    }
}
