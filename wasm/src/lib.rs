#![cfg(target_arch = "wasm32")]
// Build: wasm-pack build wasm --target web
// Run (py3): python -m http.server

use core::ViewPort;
use log::info;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::{self, prelude::*};
use web_sys::WebGl2RenderingContext;
use web_time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::web::WindowAttributesExtWebSys;
use winit::window::{Window, WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

const FPS: u32 = 60;

struct State {
    view_port: ViewPort,
}

struct App {
    window: Option<Rc<Window>>,
    state: Option<State>,
    input: WinitInputHelper,
    request_redraw: bool,
    wait_cancelled: bool,
    instant: Instant,
}

impl Default for App {
    fn default() -> Self {
        App {
            window: None,
            state: None,
            input: WinitInputHelper::new(),
            request_redraw: false,
            wait_cancelled: false,
            instant: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        self.input.step();

        self.wait_cancelled = match cause {
            StartCause::WaitCancelled { .. } => true,
            _ => false,
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

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
        let gl = Rc::new(glow::Context::from_webgl2_context(web_gl_context));

        let attributes = WindowAttributes::default()
            .with_title("Obj Viewer")
            .with_canvas(Some(canvas));
        let window = event_loop.create_window(attributes).unwrap();
        let window = Rc::new(window);

        let view_port = ViewPort::new(window.clone(), gl.clone(), (WIDTH, HEIGHT));

        self.window = Some(window.clone());
        self.state = Some(State { view_port })
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.input.process_window_event(&event);
        match event {
            WindowEvent::Resized(size) => {
                self.request_redraw = true;
                if let Some(ref mut state) = self.state {
                    // Update canvas dimensions (equivalent to glSurface.resize on native)
                    let web_window = web_sys::window().unwrap();
                    let document = web_window.document().unwrap();
                    if let Some(canvas) = document.get_element_by_id("canvas") {
                        if let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
                            canvas.set_width(size.width);
                            canvas.set_height(size.height);
                        }
                    }
                    state.view_port.resize(size.width, size.height);
                }
            }
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(ref mut state) = self.state {
                    state.view_port.render();
                }
            }
            _ => (),
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _id: DeviceId, event: DeviceEvent) {
        self.input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.input.end_step();

        if let Some(ref mut state) = self.state {
            let dt = self.instant.elapsed().as_secs_f32();
            state.view_port.handle_input(dt, &self.input, event_loop);
        }

        if self.request_redraw && !self.wait_cancelled {
            self.window.as_ref().unwrap().request_redraw();
            self.request_redraw = false;

            let dt = self.instant.elapsed().as_secs_f32();
            if let Some(ref mut state) = self.state {
                state.view_port.update(dt);
            }
        }

        if !self.wait_cancelled {
            self.instant = Instant::now();
            event_loop.set_control_flow(ControlFlow::WaitUntil(
                self.instant + Duration::from_secs_f64(1.0 / FPS as f64),
            ));
            self.request_redraw = true;
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // if let Some(ref mut state) = self.state {
        //     state.view_port.destroy();
        // }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use std::panic;

    console_log::init_with_level(log::Level::Info).expect("Unable to init logger");
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let event_loop = EventLoop::new().unwrap();
    event_loop
        .run_app(&mut App::default())
        .expect("Failed to run event loop");

    Ok(())
}
