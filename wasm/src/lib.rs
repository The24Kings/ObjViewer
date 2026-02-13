#![cfg(target_arch = "wasm32")]
// Build: wasm-pack build wasm --target web
// Run (py3): python3 -m http.server

mod context;

use std::error::Error;
use std::rc::Rc;
use std::time::Duration;

use glow::HasContext;
use log::info;
use wasm_bindgen::{self, prelude::*};
use web_sys::WebGl2RenderingContext;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::web::WindowAttributesExtWebSys;
use winit::window::WindowAttributes;
use winit_input_helper::WinitInputHelper;

use app::{App, FPS, HEIGHT, WIDTH};
use core::{PlatformBackend, State, ViewPort};

use context::WasmContext;

/// WASM platform backend using WebGL2.
pub struct WasmBackend {
    state: State,
    context: WasmContext,
}

impl PlatformBackend for WasmBackend {
    fn new(event_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>> {
        event_loop.set_control_flow(ControlFlow::Poll);

        let web_window = web_sys::window().unwrap();
        let document = web_window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        canvas.set_width(WIDTH);
        canvas.set_height(HEIGHT);

        let web_gl_context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        let gl = glow::Context::from_webgl2_context(web_gl_context);

        let attributes = WindowAttributes::default()
            .with_title("Obj Viewer")
            .with_canvas(Some(canvas));
        let window = event_loop.create_window(attributes).unwrap();
        let window = Rc::new(window);

        info!("WebGL2 context initialized");

        let context = WasmContext::new(gl);

        let gl = context.gl_context();
        let view_port = ViewPort::new(window.clone(), gl.clone(), (WIDTH, HEIGHT));

        let state = State {
            window,
            input: WinitInputHelper::new(),
            view_port,
            request_redraw: false,
            wait_cancelled: false,
        };

        Ok(Self { state, context })
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.state.request_redraw = true;
        let (width, height) = (size.width.max(1), size.height.max(1));

        // Update canvas dimensions
        let web_window = web_sys::window().unwrap();
        let document = web_window.document().unwrap();
        if let Some(canvas) = document.get_element_by_id("canvas") {
            if let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
                canvas.set_width(width);
                canvas.set_height(height);
            }
        }

        unsafe {
            self.context.gl.viewport(0, 0, width as i32, height as i32);
        }

        self.state.view_port.resize(width, height);
    }

    fn swap_buffers(&self) {
        // No-op for WebGL
    }

    fn state(&mut self) -> &mut State {
        &mut self.state
    }

    fn clear_color(&self) -> [f32; 4] {
        self.context.clear_color
    }

    fn dt(&self) -> f32 {
        self.context.dt()
    }

    fn tick(&mut self) {
        self.context.tick();
    }

    fn handle_ui_event(&mut self, _event: &WindowEvent) {
        // No-op for wasm
    }

    fn render_ui(&mut self) {
        // No-op for wasm
    }

    fn set_control_flow(&self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            self.context.instant() + Duration::from_secs_f64(1.0 / FPS as f64),
        ));
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use std::panic;

    console_log::init_with_level(log::Level::Info).expect("Unable to init logger");
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let event_loop = EventLoop::new().unwrap();
    App::<WasmBackend>::run(event_loop);

    Ok(())
}
