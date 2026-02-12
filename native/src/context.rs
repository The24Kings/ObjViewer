//! Native ImGui context wrapper.

use std::rc::Rc;
use std::time::Instant;

use dear_imgui_glow::GlowRenderer;
use dear_imgui_winit::WinitPlatform;
use glutin::context::PossiblyCurrentContext;
use glutin::surface::{Surface, WindowSurface};
use winit::event::WindowEvent;
use winit::window::Window;

use core::ViewPort;

pub struct NativeContext {
    pub context: dear_imgui_rs::Context,
    pub platform: WinitPlatform,
    pub renderer: GlowRenderer,
    pub clear_color: [f32; 4],

    pub glSurface: Surface<WindowSurface>,
    pub glContext: PossiblyCurrentContext,

    instant: Instant,
}

impl NativeContext {
    pub fn new(
        gl: glow::Context,
        glSurface: Surface<WindowSurface>,
        glContext: PossiblyCurrentContext,
    ) -> Self {
        let mut context = dear_imgui_rs::Context::create();
        context
            .set_ini_filename(Some("imgui.ini"))
            .expect("Failed to create imgui ini");

        let platform = WinitPlatform::new(&mut context);

        let mut renderer = GlowRenderer::new(gl, &mut context).unwrap();
        renderer.set_framebuffer_srgb_enabled(false);
        renderer.new_frame().expect("Error preparing imgui frame");

        Self {
            context,
            platform,
            renderer,
            clear_color: [0.0, 0.0, 0.0, 1.0],
            glSurface,
            glContext,
            instant: Instant::now(),
        }
    }

    pub fn attach_window(&mut self, window: &Window) {
        self.platform.attach_window(
            window,
            dear_imgui_winit::HiDpiMode::Default,
            &mut self.context,
        );
    }

    pub fn gl_context(&self) -> Option<&Rc<glow::Context>> {
        self.renderer.gl_context()
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

    /// Handle window events for ImGui.
    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) {
        self.platform
            .handle_window_event(&mut self.context, window, event);
    }

    /// Render the ImGui UI.
    pub fn render_ui(
        &mut self,
        window: &Rc<Window>,
        view_port: &mut ViewPort,
        input_cursor: (f32, f32),
        window_size: (u32, u32),
    ) {
        let dt = self.dt();

        // UI setup
        self.context.io_mut().set_delta_time(dt);
        self.tick();

        self.platform.prepare_frame(window, &mut self.context);

        let ui = self.context.frame();

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

        // Render viewport UI
        view_port.gui(ui);

        self.platform.prepare_render_with_ui(&ui, window);

        let draw_data = self.context.render();
        self.renderer
            .new_frame()
            .expect("Error preparing imgui frame");
        self.renderer
            .render(&draw_data)
            .expect("Failed to render imgui");
    }
}
