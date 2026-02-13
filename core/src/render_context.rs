//! Shared ImGui + wgpu render context used by both native and wasm backends.

use std::sync::Arc;

use dear_imgui_wgpu::WgpuRenderer;
use dear_imgui_winit::WinitPlatform;
use web_time::Instant;
use winit::event::WindowEvent;
use winit::window::Window;

use crate::ViewPort;
use crate::graphics::WindowRef;

pub struct RenderContext {
    pub imgui: dear_imgui_rs::Context,
    pub platform: WinitPlatform,
    pub renderer: WgpuRenderer,
    pub clear_color: [f32; 4],

    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,

    instant: Instant,
}

impl RenderContext {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        surface: wgpu::Surface<'static>,
        surface_config: wgpu::SurfaceConfiguration,
        surface_format: wgpu::TextureFormat,
    ) -> Self {
        let mut imgui = dear_imgui_rs::Context::create();

        // On native, persist layout to imgui.ini; on wasm, skip file I/O.
        #[cfg(not(target_arch = "wasm32"))]
        imgui
            .set_ini_filename(Some("imgui.ini"))
            .expect("Failed to set imgui ini filename");
        #[cfg(target_arch = "wasm32")]
        imgui
            .set_ini_filename(None::<String>)
            .expect("Failed to set imgui ini filename");

        let platform = WinitPlatform::new(&mut imgui);

        let init_info = dear_imgui_wgpu::WgpuInitInfo::new(
            device.as_ref().clone(),
            queue.as_ref().clone(),
            surface_format,
        );

        let renderer =
            WgpuRenderer::new(init_info, &mut imgui).expect("Failed to initialize imgui renderer");

        Self {
            imgui,
            platform,
            renderer,
            clear_color: [0.0, 0.0, 0.0, 1.0],
            device,
            queue,
            surface,
            surface_config,
            instant: Instant::now(),
        }
    }

    /// Attach a window to the ImGui platform layer (call once after creation).
    pub fn attach_window(&mut self, window: &Window) {
        self.platform.attach_window(
            window,
            dear_imgui_winit::HiDpiMode::Default,
            &mut self.imgui,
        );
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    /// Returns delta time since last tick in seconds.
    pub fn dt(&self) -> f32 {
        self.instant.elapsed().as_secs_f32()
    }

    /// Resets the instant for the next frame.
    pub fn tick(&mut self) {
        self.instant = Instant::now();
    }

    /// Returns the current instant (for control flow timing).
    pub fn instant(&self) -> Instant {
        self.instant
    }

    /// Forward a window event to ImGui.
    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) {
        self.platform
            .handle_window_event(&mut self.imgui, window, event);
    }

    /// Acquire surface texture, render 3D scene + ImGui, present.
    pub fn render_frame(
        &mut self,
        window: &WindowRef,
        view_port: &mut ViewPort,
        input_cursor: (f32, f32),
        window_size: (u32, u32),
    ) {
        // Acquire next surface texture
        let output = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::OutOfMemory) => panic!("Ran out of GPU memory"),
            Err(_) => {
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let dt = self.dt();

        // Scene render pass
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("scene_encoder"),
            });

        view_port.render(&mut encoder, &view, self.clear_color);

        // ImGui
        self.imgui.io_mut().set_delta_time(dt);
        self.tick();

        self.platform.prepare_frame(window, &mut self.imgui);

        let ui = self.imgui.frame();

        // App Info window
        ui.window("App Info").build(|| {
            let ui_width = ui.window_width();
            ui.text(format!("ImGUI FPS: {:.2}", ui.io().framerate()));
            ui.text(format!("ImGUI dt: {}", dt));
            ui.separator();

            ui.text(format!(
                "Mouse Position: ({:.2},{:.2})",
                input_cursor.0, input_cursor.1
            ));
            ui.text(format!(
                "Window Size: ({},{})",
                window_size.0, window_size.1
            ));
            ui.separator();

            let item_width = ui.push_item_width(ui_width * 0.6);
            ui.color_edit4("Clear Color", &mut self.clear_color);
            item_width.end();
        });

        view_port.gui(ui);

        self.platform.prepare_render_with_ui(&ui, window);

        let draw_data = self.imgui.render();

        // ImGui render pass (LoadOp::Load to keep scene)
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("imgui_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                multiview_mask: None,
                occlusion_query_set: None,
            });

            self.renderer
                .render_draw_data(draw_data, &mut rpass)
                .expect("Failed to render imgui");
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
