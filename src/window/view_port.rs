use glam::{Mat4, Vec2, Vec3, vec2, vec4};
use glow::{Context, HasContext};
use log::info;
use std::sync::{Arc, Mutex};
use winit::dpi::PhysicalPosition;
use winit::event::MouseButton;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::{CursorGrabMode, Window};
use winit_input_helper::WinitInputHelper;

use crate::game::{Camera, PhysicsManager, Projection, RenderManager};
use crate::graphics::{Material, Shader};
use crate::loaded_shader;
use crate::objects::{Cube, Light};

pub struct ViewPort {
    window: Arc<Window>,
    gl: Arc<Context>,
    camera: Camera,
    enable_2d: bool,
    capture_mouse: bool,
    last_mouse_pos: Vec2,
    render_manager: RenderManager,
    physics_manager: PhysicsManager,
    projection_matrix: Mat4,
    view_matrix: Mat4,
}

impl ViewPort {
    pub fn new(window: Arc<Window>, gl: Arc<Context>, (_width, _height): (u32, u32)) -> Self {
        unsafe {
            let size = window.inner_size();
            gl.viewport(0, 0, size.width as i32, size.height as i32);
            info!("Initial viewport: {}/{}", size.width, size.height);

            gl.enable(glow::DEPTH_TEST);
            gl.polygon_mode(glow::FRONT_AND_BACK, glow::FILL);
        }

        let mut camera = Camera::new(0.1, 100.0);
        let mut renderer = RenderManager::new(gl.clone()).unwrap();
        let mut physics_manager = PhysicsManager::new();

        let light_shader = Arc::new({
            let mut shader = crate::graphics::Shader::new(gl.clone());
            shader
                .add(
                    glow::FRAGMENT_SHADER,
                    include_str!("../../shaders/light_cube.frag"),
                    "shaders/light_cube.frag",
                )
                .add(
                    glow::VERTEX_SHADER,
                    include_str!("../../shaders/light_cube.vert"),
                    "shaders/light_cube.vert",
                )
                .link();

            shader.add_attribute("i_position");

            shader
        });
        let light_material = Material::new(light_shader.clone());

        let mut light = Light::new(light_material);
        light
            .mesh
            .upload(&gl, light_shader)
            .expect("Failed to upload mesh");

        light.transform.position = Vec3::new(1.0, 1.0, 1.0);
        light.transform.scale = Vec3::new(0.25, 0.25, 0.25);

        let light = Arc::new(Mutex::new(light));
        renderer.add_renderable(light);

        let obj_shader = Arc::new(loaded_shader!(gl.clone()));
        let obj_material = Material::new(obj_shader.clone());

        let mut cube = Cube::new(obj_material);
        cube.mesh
            .upload(&gl, obj_shader)
            .expect("Failed to upload mesh");

        // Cube has both rendering and physics
        let cube = Arc::new(Mutex::new(cube));
        renderer.add_renderable(cube.clone());
        physics_manager.add_physical(cube);

        camera.transform.position = Vec3::new(0.0, 0.0, 5.0);

        ViewPort {
            window,
            gl,

            camera,
            render_manager: renderer,
            physics_manager,
            enable_2d: false,
            capture_mouse: false,
            last_mouse_pos: Vec2::ZERO,

            projection_matrix: Mat4::IDENTITY,
            view_matrix: Mat4::IDENTITY,
        }
    }

    // Set projection matrix based on current window size, fov, and mode (2D/3D)
    fn set_projection_matrix(&mut self) {
        let size = self.window.inner_size();
        let aspect = size.width as f32 / size.height as f32;

        let projection = if self.enable_2d {
            Projection::Orthographic(aspect * -1.0, aspect, -1.0, 1.0)
        } else {
            Projection::Perspective(aspect)
        };

        self.projection_matrix = self.camera.get_camera_projection_matrix(projection);
    }

    fn update_mouse_capture_state(&mut self) {
        info!("Capturing mouse: {}", self.capture_mouse);

        //FIXME: Sometimes you can get it into a state where you can move and the mouse is still visible
        if self.capture_mouse {
            _ = self
                .window
                .set_cursor_grab(CursorGrabMode::Confined)
                .or_else(|_| self.window.set_cursor_grab(CursorGrabMode::Locked));
            self.window.set_cursor_visible(false);
        }

        if self.enable_2d || !self.capture_mouse {
            _ = self.window.set_cursor_grab(CursorGrabMode::None);
            self.window.set_cursor_visible(true);
        }
    }

    pub fn handle_input(
        &mut self,
        _dt: f32,
        input: &WinitInputHelper,
        event_loop: &ActiveEventLoop,
    ) {
        if input.key_pressed(KeyCode::Escape) {
            event_loop.exit();
        }
        if input.key_pressed(KeyCode::F1) {
            self.enable_2d = !self.enable_2d;
            self.set_projection_matrix();
        }
        if input.key_pressed(KeyCode::F2) {
            self.capture_mouse = !self.capture_mouse;
            self.update_mouse_capture_state();
        }
        if input.key_pressed(KeyCode::KeyR) {
            info!("Reloading Shaders");

            self.render_manager.render_targets.iter().for_each(|o| {
                if let Ok(mut obj) = o.lock() {
                    let to_reload: Vec<(u32, &str)> = obj
                        .material()
                        .shader
                        .sources
                        .iter()
                        .map(|s| (s.shader_type, s.filepath))
                        .collect();

                    let shader = obj.material_mut().shader_mut();

                    to_reload.chunks(2).for_each(|c| {
                        let mut vertex = "";
                        let mut fragment = "";

                        c.iter().for_each(|s| {
                            let shader_type = s.0;
                            let path = s.1;

                            match shader_type {
                                glow::VERTEX_SHADER => vertex = path,
                                glow::FRAGMENT_SHADER => fragment = path,
                                _ => panic!("Unsupported shader type"),
                            }
                        });

                        Shader::reload_shader(self.gl.clone(), shader, vertex, fragment);
                    });
                }
            });
        }

        if self.capture_mouse {
            self.handle_mouse(input);
        }
    }

    fn normalize_cursor(&mut self, cursor: Vec2) -> Vec3 {
        let size = self.window.inner_size();
        // https://antongerdelan.net/opengl/raycasting.html
        let ndc = vec2(
            (2.0 * cursor.x) / size.width as f32 - 1.0,
            1.0 - (2.0 * cursor.y) / size.height as f32,
        );
        let clip = vec4(ndc.x, ndc.y, -1.0, 1.0);
        let mut eye = self.projection_matrix.inverse() * clip;
        eye.z = -1.0;
        eye.w = 0.0;
        (self.view_matrix.inverse() * eye).truncate()
    }

    fn handle_mouse(&mut self, input: &WinitInputHelper) {
        let scroll_diff = {
            let d = input.scroll_diff();
            vec2(d.0, d.1)
        };
        let mouse_diff = {
            let d = input.mouse_diff();
            vec2(d.0, d.1)
        };

        if scroll_diff.y != 0.0 {
            self.camera.frustum.zoom(-scroll_diff.y);
            self.set_projection_matrix();
        }

        if self.enable_2d {
            // Get the initial mouse position on first press
            if input.mouse_pressed(MouseButton::Left) {
                if let Some(cursor) = input.cursor() {
                    self.last_mouse_pos = vec2(cursor.0, cursor.1);
                }
            }

            // Handle moving mouse (diff from origin)
            if input.mouse_held(MouseButton::Left) {
                if let Some(cursor) = input.cursor() {
                    let current = vec2(cursor.0, cursor.1);
                    let diff = self.last_mouse_pos - current;

                    if diff.length() > 0.0 {
                        let lastMouseWorldPos =
                            self.normalize_cursor(self.last_mouse_pos).truncate();
                        let diffWorldSpace =
                            self.normalize_cursor(self.last_mouse_pos + diff).truncate();

                        let diff = lastMouseWorldPos - diffWorldSpace;

                        self.camera.transform.position.x -= diff.x;
                        self.camera.transform.position.y -= diff.y;
                    }

                    self.last_mouse_pos = current;
                }
            }
        } else {
            if !(mouse_diff.length() > 0.0) {
                return;
            }

            self.camera.turn(mouse_diff.x, -mouse_diff.y);
            let size = self.window.inner_size();
            _ = self
                .window
                .set_cursor_position(PhysicalPosition::new(size.width / 2, size.height / 2));
        }
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {
        self.set_projection_matrix();
    }

    pub fn update(&mut self, dt: f32) {
        // Update physics before rendering
        self.physics_manager.update(dt);
        self.render_manager.update(dt);
    }

    pub fn render(&mut self) {
        unsafe {
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        }

        // Pass projection * view (vp); each renderable supplies its own model matrix.
        self.view_matrix = self.camera.get_camera_view_matrix();
        let pv = self.projection_matrix * self.view_matrix;
        self.render_manager.draw(&pv, &self.camera);
    }
}
