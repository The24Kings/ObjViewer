#![allow(non_snake_case)]

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use log::info;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::rc::Rc;
use std::time::{Duration, Instant};
use time::{UtcOffset, format_description::parse};
use tracing_subscriber::fmt::time::OffsetTime;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;

use crate::window::ViewPort;

mod game;
mod graphics;
mod objects;
mod window;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

const FPS: u32 = 60;

struct State {
    glSurface: Surface<WindowSurface>,
    glContext: PossiblyCurrentContext,
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
    fn new_events(&mut self, _eventLoop: &ActiveEventLoop, cause: StartCause) {
        self.input.step();

        self.wait_cancelled = match cause {
            StartCause::WaitCancelled { .. } => true,
            _ => false,
        }
    }

    fn resumed(&mut self, eventLoop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let attributes = WindowAttributes::default()
            .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
            .with_title("Obj Viewer");

        let template = ConfigTemplateBuilder::new();
        let displayBuilder = DisplayBuilder::new().with_window_attributes(Some(attributes));

        let (window, glConfig) = displayBuilder
            .build(eventLoop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();
        let rwh: Option<RawWindowHandle> = window
            .as_ref()
            .and_then(|w| w.window_handle().map(Into::into).ok());

        let glDisplay = glConfig.display();
        let contextAttributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                major: 4,
                minor: 1,
            })))
            .build(rwh);

        let (window, gl, glSurface, glContext) = unsafe {
            let notCurrentGlContext = glDisplay
                .create_context(&glConfig, &contextAttributes)
                .unwrap();
            let window = Rc::new(window.unwrap());

            let surfaceAttributes = window.build_surface_attributes(Default::default()).unwrap();
            let glSurface = glDisplay
                .create_window_surface(&glConfig, &surfaceAttributes)
                .unwrap();

            let glContext = notCurrentGlContext.make_current(&glSurface).unwrap();
            let gl = Rc::new(glow::Context::from_loader_function_cstr(|s| {
                glDisplay.get_proc_address(s)
            }));

            (window, gl, glSurface, glContext)
        };

        let view_port = ViewPort::new(window.clone(), gl.clone(), (WIDTH, HEIGHT));

        self.window = Some(window.clone());
        self.state = Some(State {
            glSurface,
            glContext,
            view_port,
        });
    }

    fn window_event(&mut self, eventLoop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.input.process_window_event(&event);
        match event {
            WindowEvent::Resized(size) => {
                self.request_redraw = true;
                if let Some(ref mut state) = self.state {
                    state.view_port.resize(size.width, size.height);
                }
            }
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                eventLoop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(ref mut state) = self.state {
                    state.view_port.render();
                    state.glSurface.swap_buffers(&state.glContext).unwrap();
                }
            }
            _ => (),
        }
    }

    fn device_event(&mut self, _eventLoop: &ActiveEventLoop, _id: DeviceId, event: DeviceEvent) {
        self.input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, eventLoop: &ActiveEventLoop) {
        self.input.end_step();

        if self.request_redraw && !self.wait_cancelled {
            self.window.as_ref().unwrap().request_redraw();
            self.request_redraw = false;

            let dt = self.instant.elapsed().as_secs_f64();
            // info!("Delta time: {}s", dt);
            if let Some(ref mut state) = self.state {
                state.view_port.update(dt, &self.input, eventLoop);
            }
        }

        if !self.wait_cancelled {
            eventLoop.set_control_flow(ControlFlow::WaitUntil(
                Instant::now() + Duration::from_secs_f64(1.0 / FPS as f64),
            ));
            self.request_redraw = true;
            self.instant = Instant::now();
        }
    }

    fn exiting(&mut self, _eventLoop: &ActiveEventLoop) {
        // if let Some(ref mut state) = self.state {
        //     state.view_port.destroy();
        // }
    }
}

fn main() {
    // Setup tracing subscriber for logging
    let timer = parse("[year]-[month padding:zero]-[day padding:zero] [hour]:[minute]:[second]")
        .expect("Tracing time format is invalid");
    let time_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let timer = OffsetTime::new(time_offset, timer);

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_target(false)
        .with_timer(timer)
        .with_file(true)
        .with_ansi(true)
        .compact()
        .init();

    let eventLoop = EventLoop::new().unwrap();
    eventLoop
        .run_app(&mut App::default())
        .expect("Failed to run event loop");
}
