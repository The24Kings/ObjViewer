use glam::{Mat4, Vec2, Vec3, vec2, vec4};
use glow::HasContext;
use tracing::{error, info};
use winit::dpi::PhysicalPosition;
use winit::event::MouseButton;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;
use winit_input_helper::WinitInputHelper;

use crate::game::{Camera, PhysicsManager, Projection, RenderManager};
#[cfg(not(target_arch = "wasm32"))]
use crate::graphics::Shader;
use crate::graphics::types::GameObjectRef;
use crate::graphics::{
    GlRef, LIGHT_CUBE_FRAG_PATH, LIGHT_CUBE_FRAG_SRC, LIGHT_CUBE_VERT_PATH, LIGHT_CUBE_VERT_SRC,
    Material, ShaderRef, Texture, TextureRef, WindowRef, new_game_obj_ref, new_shader_ref,
    new_texture_ref,
};
use crate::loaded_shader;
use crate::objects::{Cube, Light};

pub struct ViewPort {
    window: WindowRef,
    gl: GlRef,
    camera: Camera,
    enable_2d: bool,
    capture_mouse: bool,
    last_mouse_pos: Vec2,
    render_manager: RenderManager,
    physics_manager: PhysicsManager,
    projection_matrix: Mat4,
    view_matrix: Mat4,
    sun: GameObjectRef,
}

impl ViewPort {
    pub fn new(window: WindowRef, gl: GlRef, (width, height): (u32, u32)) -> Self {
        unsafe {
            info!("Initial viewport: {}/{}", width, height);

            gl.viewport(0, 0, width as i32, height as i32);
            gl.enable(glow::DEPTH_TEST);
        }

        let mut camera = Camera::new(0.1, 100.0);
        let mut renderer = RenderManager::new(gl.clone()).unwrap();
        let mut physics_manager = PhysicsManager::new();

        let light_shader: ShaderRef = {
            let mut shader = crate::graphics::Shader::new(gl.clone());
            let _ = shader.add(
                glow::FRAGMENT_SHADER,
                LIGHT_CUBE_FRAG_SRC,
                LIGHT_CUBE_FRAG_PATH,
            );
            let _ = shader.add(
                glow::VERTEX_SHADER,
                LIGHT_CUBE_VERT_SRC,
                LIGHT_CUBE_VERT_PATH,
            );
            let _ = shader.link();

            shader.add_attribute("i_position");
            shader.add_attribute("i_uv");

            new_shader_ref(shader)
        };
        let mut light_material = Material::new(gl.clone(), light_shader.clone());
        let light_texture: TextureRef = {
            let tex = Texture::from_bytes(
                gl.clone(),
                include_bytes!("objects/textures/redstone_lamp.png"),
            )
            .expect("Failed to load texture");
            new_texture_ref(tex)
        };
        light_material.texture = Some(light_texture);

        let mut light = Light::new(light_material);
        light
            .mesh
            .upload(&gl, light_shader)
            .expect("Failed to upload mesh");

        light.transform.position = Vec3::new(1.0, 1.0, 1.0);
        light.transform.scale = Vec3::new(0.25, 0.25, 0.25);

        let light_ref = new_game_obj_ref(light);
        renderer.add_renderable(light_ref.clone());

        let obj_shader: ShaderRef = {
            let shader = loaded_shader!(gl.clone());
            new_shader_ref(shader)
        };
        let obj_material = Material::new(gl.clone(), obj_shader.clone());

        let mut cube = Cube::new(obj_material);
        cube.mesh
            .upload(&gl, obj_shader)
            .expect("Failed to upload mesh");

        let cube_ref = new_game_obj_ref(cube);

        renderer.add_renderable(cube_ref.clone());
        physics_manager.add_physical(cube_ref);

        camera.transform.position = Vec3::new(0.0, 0.0, 5.0);

        // Calculate initial projection matrix using the passed dimensions
        let aspect = width as f32 / height as f32;
        let projection_matrix =
            camera.get_camera_projection_matrix(Projection::Perspective(aspect));

        ViewPort {
            window,
            gl,

            camera,
            render_manager: renderer,
            physics_manager,
            enable_2d: false,
            capture_mouse: false,
            last_mouse_pos: Vec2::ZERO,

            projection_matrix,
            view_matrix: Mat4::IDENTITY,
            sun: light_ref,
        }
    }

    // Set projection matrix based on current window size, fov, and mode (2D/3D)
    fn set_projection_matrix(&mut self) {
        let size = self.window.inner_size();
        if size.width == 0 || size.height == 0 {
            return;
        }

        let aspect = size.width as f32 / size.height as f32;

        let projection = if self.enable_2d {
            Projection::Orthographic(aspect * -1.0, aspect, -1.0, 1.0)
        } else {
            Projection::Perspective(aspect)
        };

        self.projection_matrix = self.camera.get_camera_projection_matrix(projection);
    }

    fn update_mouse_capture_state(&mut self) {
        // Only confine and hide cursor in 3D mode with capture enabled
        let confine = self.capture_mouse && !self.enable_2d;
        _ = if confine {
            self.window
                .set_cursor_grab(CursorGrabMode::Confined)
                .or_else(|_| self.window.set_cursor_grab(CursorGrabMode::Locked))
        } else {
            self.window.set_cursor_grab(CursorGrabMode::None)
        };
        self.window.set_cursor_visible(!confine);
    }

    pub fn handle_input(
        &mut self,
        _dt: f32,
        input: &WinitInputHelper,
        _event_loop: &ActiveEventLoop,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        if input.key_pressed(KeyCode::Escape) {
            _event_loop.exit();
        }
        if input.key_pressed(KeyCode::F1) {
            self.capture_mouse = !self.capture_mouse;
            self.update_mouse_capture_state();
            info!("Capturing mouse: {}", self.capture_mouse);
        }
        if input.key_pressed(KeyCode::F2) {
            self.enable_2d = true;
            self.set_projection_matrix();
            self.update_mouse_capture_state();
        }
        if input.key_pressed(KeyCode::F3) {
            self.enable_2d = false;
            self.set_projection_matrix();
            self.update_mouse_capture_state();
        }
        #[cfg(not(target_arch = "wasm32"))]
        if input.key_pressed(KeyCode::KeyR) {
            info!("Reloading Shaders");

            self.render_manager.render_targets.iter().for_each(|o| {
                let mut obj = o.borrow_mut();
                let shader = obj.material_mut().shader_mut();

                match Shader::reload_shader(self.gl.clone(), shader) {
                    Ok(_) => info!("Successfully reloaded shader: {:?}", shader.handle),
                    Err(e) => error!("Failed to reload shader: {}", e),
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

                    if !(diff.length() > 0.0) {
                        return;
                    }

                    let last_world_pos = self.normalize_cursor(self.last_mouse_pos).truncate();
                    let diff_world_pos =
                        self.normalize_cursor(self.last_mouse_pos + diff).truncate();

                    let diff = last_world_pos - diff_world_pos;

                    self.sun.borrow_mut().transform_mut().position.x += diff.x;
                    self.sun.borrow_mut().transform_mut().position.y += diff.y;

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

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
            info!("Resized viewport: {}/{}", width, height);
        }
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
        self.render_manager.draw(&pv, &self.camera, &self.sun);
    }
}
