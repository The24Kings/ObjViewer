use log::{error, info};
use std::marker::PhantomData;
use std::time::Duration;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

use core::PlatformBackend;

pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;
pub const FPS: u32 = 60;

/// Generic application struct that works with any platform backend.
pub struct App<P: PlatformBackend> {
    backend: Option<P>,
    _marker: PhantomData<P>,
}

impl<P: PlatformBackend> App<P> {
    pub fn new() -> Self {
        Self {
            backend: None,
            _marker: PhantomData,
        }
    }

    pub fn run(event_loop: EventLoop<()>) {
        event_loop
            .run_app(&mut Self::new())
            .expect("Failed to run event loop");
    }
}

impl<P: PlatformBackend> Default for App<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: PlatformBackend> ApplicationHandler for App<P> {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        let backend = match &mut self.backend {
            Some(backend) => backend,
            None => return,
        };

        let shared = backend.state();
        shared.input.step();
        shared.wait_cancelled = matches!(cause, StartCause::WaitCancelled { .. });
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.backend.is_none() {
            return;
        }

        match P::new(event_loop) {
            Ok(mut backend) => {
                backend.state().window.request_redraw();
                self.backend = Some(backend);
                info!("App state created");
            }
            Err(e) => {
                error!("Error creating AppState: {}", e);
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let backend = match &mut self.backend {
            Some(backend) => backend,
            None => return,
        };

        // Resize must happen before we split-borrow state & context
        if let WindowEvent::Resized(size) = event {
            backend.resize(size);
        }

        let (state, context) = backend.inner();

        // Process input and UI events
        state.input.process_window_event(&event);
        context.handle_event(&state.window, &event);

        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let input_cursor = state.input.cursor().unwrap_or((0.0, 0.0));
                let window_size = state.window.inner_size();

                context.render_frame(
                    &state.window,
                    &mut state.view_port,
                    input_cursor,
                    (window_size.width, window_size.height),
                );
            }
            _ => (),
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _id: DeviceId, event: DeviceEvent) {
        let backend = match &mut self.backend {
            Some(backend) => backend,
            None => return,
        };

        backend.state().input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        let backend = match &mut self.backend {
            Some(backend) => backend,
            None => return,
        };

        let (state, context) = backend.inner();

        let dt = context.dt();

        state.input.end_step();

        let input = state.input.clone();
        state.view_port.handle_input(dt, &input, event_loop);

        let request_redraw = state.request_redraw;
        let wait_cancelled = state.wait_cancelled;

        if request_redraw && !wait_cancelled {
            state.window.request_redraw();
            state.request_redraw = false;
            state.view_port.update(dt);
        }

        if !wait_cancelled {
            context.tick();
            event_loop.set_control_flow(ControlFlow::WaitUntil(
                context.instant() + Duration::from_secs_f64(1.0 / FPS as f64),
            ));
            state.request_redraw = true;
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // Cleanup if needed
    }
}
