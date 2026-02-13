#![cfg(target_arch = "wasm32")]
// Build: wasm-pack build wasm --target web
// Run (py3): python3 -m http.server

use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;

use wasm_bindgen::{self, prelude::*};
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::web::WindowAttributesExtWebSys;
use winit::window::{Window, WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;

use app::{FPS, HEIGHT, WIDTH};
use core::graphics::GpuContext;
use core::{RenderContext, State, ViewPort};

/// Fully initialized WASM app window (created asynchronously).
struct WasmBackend {
    state: State,
    context: RenderContext,
}

/// Application handler that creates the window synchronously,
/// finishes GPU init asynchronously via `spawn_local`, and picks up the
/// result on the next event loop iteration.
#[derive(Default)]
struct App {
    backend: Option<WasmBackend>,
}

thread_local! {
    static READY: RefCell<Option<WasmBackend>> = const { RefCell::new(None) };
}

impl WasmBackend {
    /// Async GPU setup given an already-created window + surface.
    async fn init(
        window: Arc<Window>,
        surface: wgpu::Surface<'static>,
        instance: wgpu::Instance,
    ) -> Result<Self, JsValue> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| JsValue::from_str(&format!("request_adapter: {e}")))?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("ObjViewer Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                trace: wgpu::Trace::Off,
            })
            .await
            .map_err(|e| JsValue::from_str(&format!("request_device: {e}")))?;

        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let physical = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: physical.width.max(1),
            height: physical.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let gpu = GpuContext {
            device: device.clone(),
            queue: queue.clone(),
        };

        let mut context = RenderContext::new(
            device.clone(),
            queue.clone(),
            surface,
            surface_config,
            surface_format,
        );
        context.attach_window(&window);

        let view_port = ViewPort::new(window.clone(), gpu, surface_format, (WIDTH, HEIGHT));

        let state = State {
            window,
            input: WinitInputHelper::new(),
            view_port,
            request_redraw: false,
            wait_cancelled: false,
        };

        Ok(Self { state, context })
    }
}

impl ApplicationHandler for App {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        // Pick up async init result
        if self.backend.is_none() {
            if let Some(w) = READY.with(|c| c.borrow_mut().take()) {
                self.backend = Some(w);
            }
        }
        if let Some(app) = &mut self.backend {
            app.state.input.step();
            app.state.wait_cancelled = matches!(cause, StartCause::WaitCancelled { .. });
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.backend.is_some() {
            return;
        }

        // Create window + surface synchronously
        let web_window = web_sys::window().unwrap();
        let document = web_window.document().unwrap();
        let canvas: web_sys::HtmlCanvasElement = document
            .get_element_by_id("canvas")
            .expect("canvas not found")
            .dyn_into()
            .expect("element is not a canvas");
        canvas.set_width(WIDTH);
        canvas.set_height(HEIGHT);

        let window: Arc<Window> = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Obj Viewer")
                        .with_canvas(Some(canvas)),
                )
                .expect("create_window"),
        );

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance
            .create_surface(window.clone())
            .expect("create_surface");

        // Finish GPU init asynchronously on the JS microtask queue
        let redraw_window = window.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match WasmBackend::init(window, surface, instance).await {
                Ok(app) => {
                    READY.with(|c| *c.borrow_mut() = Some(app));
                    redraw_window.request_redraw();
                }
                Err(e) => {
                    web_sys::console::error_1(&JsValue::from_str(&format!(
                        "Failed to init GPU: {:?}",
                        e
                    )));
                }
            }
        });
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // Pick up async init if just completed
        if self.backend.is_none() {
            if let Some(w) = READY.with(|c| c.borrow_mut().take()) {
                self.backend = Some(w);
            }
        }
        let Some(app) = &mut self.backend else {
            return;
        };

        // Resize before split-borrow
        if let WindowEvent::Resized(size) = event {
            let (w, h) = (size.width.max(1), size.height.max(1));
            let web_window = web_sys::window().unwrap();
            let document = web_window.document().unwrap();
            if let Some(canvas) = document.get_element_by_id("canvas") {
                if let Ok(canvas) = canvas.dyn_into::<web_sys::HtmlCanvasElement>() {
                    canvas.set_width(w);
                    canvas.set_height(h);
                }
            }
            app.context.resize(w, h);
            app.state.view_port.resize(w, h);
            app.state.request_redraw = true;
        }

        app.state.input.process_window_event(&event);
        app.context.handle_event(&app.state.window, &event);

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let input_cursor = app.state.input.cursor().unwrap_or((0.0, 0.0));
                let window_size = app.state.window.inner_size();
                app.context.render_frame(
                    &app.state.window,
                    &mut app.state.view_port,
                    input_cursor,
                    (window_size.width, window_size.height),
                );
            }
            _ => (),
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _id: DeviceId, event: DeviceEvent) {
        if let Some(app) = &mut self.backend {
            app.state.input.process_device_event(&event);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        // Pick up async init if just completed
        if self.backend.is_none() {
            if let Some(w) = READY.with(|c| c.borrow_mut().take()) {
                self.backend = Some(w);
            }
        }
        let Some(app) = &mut self.backend else {
            return;
        };

        let dt = app.context.dt();
        app.state.input.end_step();

        let input = app.state.input.clone();
        app.state.view_port.handle_input(dt, &input, event_loop);

        if app.state.request_redraw && !app.state.wait_cancelled {
            app.state.window.request_redraw();
            app.state.request_redraw = false;
            app.state.view_port.update(dt);
        }

        if !app.state.wait_cancelled {
            app.context.tick();
            event_loop.set_control_flow(ControlFlow::WaitUntil(
                app.context.instant() + Duration::from_secs_f64(1.0 / FPS as f64),
            ));
            app.state.request_redraw = true;
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);

    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("failed to run app");
}
