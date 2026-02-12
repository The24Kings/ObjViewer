use log::{error, info};
use std::marker::PhantomData;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
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
        if let Some(ref mut backend) = self.backend {
            let shared = backend.state();
            shared.input.step();
            shared.wait_cancelled = matches!(cause, StartCause::WaitCancelled { .. });
        }
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.backend.is_none() {
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
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(ref mut backend) = self.backend {
            // Process input and UI events
            backend.state().input.process_window_event(&event);
            backend.handle_ui_event(&event);

            match event {
                WindowEvent::Resized(size) => {
                    backend.resize(size);
                }
                WindowEvent::CloseRequested => {
                    info!("The close button was pressed; stopping");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    let clear_color = backend.clear_color();
                    backend.state().view_port.render(clear_color);

                    backend.render_ui();

                    // Swap
                    backend.state().window.pre_present_notify();
                    backend.swap_buffers();
                }
                _ => (),
            }
        }
    }

    fn device_event(&mut self, _event_loop: &ActiveEventLoop, _id: DeviceId, event: DeviceEvent) {
        if let Some(ref mut backend) = self.backend {
            backend.state().input.process_device_event(&event);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(ref mut backend) = self.backend {
            let dt = backend.dt();
            let state = backend.state();

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
                backend.tick();
                backend.set_control_flow(event_loop);
                backend.state().request_redraw = true;
            }
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // Cleanup if needed
    }
}
