use dear_imgui_rs::{TreeNodeFlags, Ui};
use glam::{Mat4, Vec2, Vec3, vec2, vec4};
use log::info;
use std::rc::Rc;
use std::sync::Arc;
use winit::dpi::PhysicalPosition;
use winit::event::MouseButton;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;
use winit::window::CursorGrabMode;
use winit_input_helper::WinitInputHelper;

use crate::game::render_manager::{LightUniforms, ObjUniforms};
use crate::game::{Camera, PhysicsManager, Projection, RenderManager};
use crate::graphics::types::{LightObjectRef, new_light_obj_ref};
use crate::graphics::{
    GpuContext, LIGHT_CUBE_WGSL, LOADED_OBJ_WGSL, Material, Shader, Texture, WindowRef,
    new_game_obj_ref,
};
use crate::objects::{Cube, Light};

pub struct ViewPort {
    window: WindowRef,
    gpu: GpuContext,
    depth_texture: wgpu::TextureView,

    camera: Camera,
    enable_2d: bool,
    capture_mouse: bool,
    last_mouse_pos: Vec2,

    render_manager: RenderManager,
    physics_manager: PhysicsManager,
    projection_matrix: Mat4,
    view_matrix: Mat4,
    sun: LightObjectRef,
}

impl ViewPort {
    pub fn new(
        window: WindowRef,
        gpu: GpuContext,
        surface_format: wgpu::TextureFormat,
        (width, height): (u32, u32),
    ) -> Self {
        info!("Initial viewport: {}/{}", width, height);

        let mut camera = Camera::new(0.1, 100.0);
        let mut renderer = RenderManager::new(gpu.queue.clone());
        let mut physics_manager = PhysicsManager::new();

        // Light Source
        let light_shader = Rc::new(Shader::new(
            &gpu.device,
            LIGHT_CUBE_WGSL,
            surface_format,
            "light_cube",
        ));

        let light_texture = Texture::from_bytes(
            &gpu.device,
            &gpu.queue,
            include_bytes!("objects/textures/redstone_lamp.png"),
            Some("redstone_lamp"),
        )
        .expect("Failed to load light texture");

        let light_material = Material::new(
            &gpu.device,
            light_shader.clone(),
            light_texture,
            std::mem::size_of::<LightUniforms>() as u64,
        );

        let mut light = Light::new(light_material);
        light.mesh.upload(&gpu.device);
        light.transform.position = Vec3::new(1.0, 1.0, 1.0);
        light.transform.scale = Vec3::new(0.25, 0.25, 0.25);

        let light_ref = new_light_obj_ref(light);
        renderer.add_renderable(light_ref.clone());

        // Test Cube
        let obj_shader = Rc::new(Shader::new(
            &gpu.device,
            LOADED_OBJ_WGSL,
            surface_format,
            "loaded_obj",
        ));

        let default_texture = Texture::white_1x1(&gpu.device, &gpu.queue);

        let obj_material = Material::new(
            &gpu.device,
            obj_shader.clone(),
            default_texture,
            std::mem::size_of::<ObjUniforms>() as u64,
        );

        let mut cube = Cube::new(obj_material);
        cube.mesh.upload(&gpu.device);

        let cube_ref = new_game_obj_ref(cube);
        renderer.add_renderable(cube_ref.clone());
        physics_manager.add_physical(cube_ref);

        camera.transform.position = Vec3::new(0.0, 0.0, 5.0);

        let aspect = width as f32 / height as f32;
        let projection_matrix =
            camera.get_camera_projection_matrix(Projection::Perspective(aspect));

        let depth_texture = Self::create_depth_texture(&gpu.device, width, height);

        ViewPort {
            window,
            gpu,

            camera,
            render_manager: renderer,
            physics_manager,
            enable_2d: false,
            capture_mouse: false,
            last_mouse_pos: Vec2::ZERO,

            projection_matrix,
            view_matrix: Mat4::IDENTITY,
            sun: light_ref,

            depth_texture,
        }
    }

    fn create_depth_texture(
        device: &Arc<wgpu::Device>,
        width: u32,
        height: u32,
    ) -> wgpu::TextureView {
        let size = wgpu::Extent3d {
            width: width.max(1),
            height: height.max(1),
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("depth_texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };
        device
            .create_texture(&desc)
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    // Set projection matrix based on current window size, fov, and mode (2D/3D)
    fn set_projection_matrix(&mut self) {
        let size = self.window.inner_size();
        if size.width == 0 || size.height == 0 {
            return;
        }

        let aspect = size.width as f32 / size.height as f32;

        let projection = if self.enable_2d {
            Projection::Orthographic(aspect)
        } else {
            Projection::Perspective(aspect)
        };

        self.projection_matrix = self.camera.get_camera_projection_matrix(projection);
    }

    fn update_mouse_capture_state(&mut self) {
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

        if self.capture_mouse {
            self.handle_mouse(input);
        }
    }

    fn normalize_cursor(&mut self, cursor: Vec2) -> Vec3 {
        let size = self.window.inner_size();
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
            if input.mouse_pressed(MouseButton::Left) {
                if let Some(cursor) = input.cursor() {
                    self.last_mouse_pos = vec2(cursor.0, cursor.1);
                }
            }

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
        info!("Resized viewport: {}/{}", width, height);
        self.depth_texture = Self::create_depth_texture(&self.gpu.device, width, height);
        self.set_projection_matrix();
    }

    pub fn update(&mut self, dt: f32) {
        self.physics_manager.update(dt);
        self.render_manager.update(dt);
    }

    pub fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        clear_color: [f32; 4],
    ) {
        self.view_matrix = self.camera.get_camera_view_matrix();
        let pv = self.projection_matrix * self.view_matrix;

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("scene_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: clear_color[0] as f64,
                        g: clear_color[1] as f64,
                        b: clear_color[2] as f64,
                        a: clear_color[3] as f64,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            multiview_mask: None,
            occlusion_query_set: None,
        });

        self.render_manager
            .draw(&mut rpass, &pv, &self.camera, &self.sun);
    }

    pub fn gui(&mut self, ui: &mut Ui) {
        ui.window("Viewport").build(|| {
            if ui.collapsing_header("Camera", TreeNodeFlags::COLLAPSING_HEADER) {
                ui.text(format!(
                    "Position: ({}, {}, {})",
                    self.camera.transform.position.x,
                    self.camera.transform.position.y,
                    self.camera.transform.position.z
                ));

                ui.separator();

                ui.text(format!("Pitch: {}", self.camera.pitch));
                ui.text(format!("Yaw: {}", self.camera.yaw));
                ui.text(format!("Zoom: {}", self.camera.frustum.fov));

                ui.separator();

                if ui.small_button("Reset##Camera") {
                    self.camera.transform.position = Vec3::new(0.0, 0.0, 5.0);
                    self.camera.frustum.fov = 45.0;
                    self.set_projection_matrix();
                }
            }

            if ui.collapsing_header("Sun", TreeNodeFlags::COLLAPSING_HEADER) {
                let mut sun = self.sun.borrow_mut();
                let transform = sun.transform_mut();

                let mut position = transform.position.to_array();

                if ui.input_float3("Position", &mut position).build() {
                    transform.position = position.into();
                }

                ui.separator();

                ui.slider_f32("Ambient", &mut sun.ambient_mut(), 0.0, 1.0);
                ui.slider_f32("Specular", &mut sun.specular_mut(), 0.0, 1.0);

                ui.separator();

                if ui.small_button("Reset##Sun") {
                    sun.transform_mut().position = Vec3::new(1.0, 1.0, 1.0);
                    *sun.ambient_mut() = 0.2;
                    *sun.specular_mut() = 0.5;
                }
            }
        });
    }
}
