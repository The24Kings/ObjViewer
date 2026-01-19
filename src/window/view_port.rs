use glam::Mat4;
use glow::{Context, HasContext};
use log::info;
use std::rc::Rc;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::game::{Camera, Material};
use crate::graphics::{ObjectRenderer, Shader};
use crate::objects::Triangle;

pub struct ViewPort {
    window: Rc<Window>,
    gl: Rc<Context>,
    camera: Camera,
    renderer: ObjectRenderer,
}

impl ViewPort {
    pub fn new(window: Rc<Window>, gl: Rc<Context>, (_width, _height): (u32, u32)) -> Self {
        unsafe {
            // gl.viewport(0, 0, width as i32, height as i32);
            let size = window.inner_size();
            gl.viewport(0, 0, size.width as i32, size.height as i32);
            info!("Initial viewport: {}/{}", size.width, size.height);

            gl.line_width(10.0);
            gl.enable(glow::DEPTH_TEST);
            gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
        }

        let camera = Camera::new(0.1, 100.0);

        let mut renderer = ObjectRenderer::new(gl.clone()).unwrap();

        // Create shader for the triangle
        let mut shader = Shader::new(gl.clone());
        shader
            .add(
                glow::FRAGMENT_SHADER,
                include_str!("../../shaders/loaded_obj.frag"),
            )
            .add(
                glow::VERTEX_SHADER,
                include_str!("../../shaders/loaded_obj.vert"),
            )
            .link();

        let shader_rc = Rc::new(shader);
        // Create triangle material and upload its mesh to the GPU
        let material = Material::new(shader_rc.clone());
        let mut triangle = Triangle::new(material);

        triangle
            .mesh
            .upload(&gl, shader_rc)
            .expect("Failed to upload triangle mesh");

        // Add to renderer targets
        renderer.add_renderable(triangle);

        ViewPort {
            window,
            gl,
            camera,
            renderer,
        }
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {}

    pub fn update(&mut self, _dt: f64, input: &WinitInputHelper, eventLoop: &ActiveEventLoop) {
        if input.key_pressed(KeyCode::Escape) {
            eventLoop.exit();
        }
    }

    pub fn render(&mut self) {
        unsafe {
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        }

        let size = self.window.inner_size();
        let aspect = size.width as f32 / size.height as f32;

        // let projection = Mat4::perspective_rh(
        //     self.camera.frustum.fov.to_radians(),
        //     aspect,
        //     self.camera.frustum.near,
        //     self.camera.frustum.far,
        // );
        let projection = Mat4::orthographic_rh(
            -aspect * 5.0,
            aspect * 5.0,
            -5.0,
            5.0,
            self.camera.frustum.near,
            self.camera.frustum.far,
        );
        // let view = self.camera.getViewMatrix();

        let mvp = Mat4::IDENTITY * projection;
        self.renderer.draw(&mvp);
    }
}
