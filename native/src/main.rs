#![allow(non_snake_case)]

mod context;

use std::error::Error;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Duration;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin_winit::{DisplayBuilder, GlWindow};
use log::info;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use time::{UtcOffset, format_description::parse};
use tracing_subscriber::fmt::time::OffsetTime;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::x11::EventLoopBuilderExtX11;
use winit::window::WindowAttributes;
use winit_input_helper::WinitInputHelper;

use app::{App, FPS, HEIGHT, WIDTH};
use core::{PlatformBackend, State, ViewPort};

use context::NativeContext;

/// Native platform backend using glutin/OpenGL.
pub struct NativeBackend {
    state: State,
    context: NativeContext,
}

impl PlatformBackend for NativeBackend {
    fn new(event_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>> {
        let attributes = WindowAttributes::default()
            .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
            .with_title("Obj Viewer");

        let template = ConfigTemplateBuilder::new();
        let displayBuilder = DisplayBuilder::new().with_window_attributes(Some(attributes));

        let (window, glConfig) = displayBuilder
            .build(event_loop, template, |configs| {
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
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                major: 4,
                minor: 1,
            })))
            .build(rwh);

        let (window, gl, glSurface, glContext) = unsafe {
            let glContext = glDisplay
                .create_context(&glConfig, &context_attributes)
                .unwrap();
            let window = Rc::new(window.unwrap());

            let surface_attributes = window.build_surface_attributes(Default::default()).unwrap();
            let glSurface = glDisplay
                .create_window_surface(&glConfig, &surface_attributes)
                .unwrap();

            let glContext = glContext.make_current(&glSurface).unwrap();
            let gl = glow::Context::from_loader_function_cstr(|s| glDisplay.get_proc_address(s));

            (window, gl, glSurface, glContext)
        };

        // ImGui context
        let mut context = NativeContext::new(gl, glSurface, glContext);
        context.attach_window(&window);
        info!("Imgui initialized");

        let gl = context.gl_context().unwrap();
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
        self.context.glSurface.resize(
            &self.context.glContext,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );
        self.state.view_port.resize(width, height);
    }

    fn swap_buffers(&self) {
        self.context
            .glSurface
            .swap_buffers(&self.context.glContext)
            .unwrap();
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

    fn handle_ui_event(&mut self, event: &WindowEvent) {
        self.context.handle_event(&self.state.window, event);
    }

    fn render_ui(&mut self) {
        let input_cursor = self.state.input.cursor().unwrap_or((0.0, 0.0));
        let window_size = self.state.window.inner_size();

        self.context.render_ui(
            &self.state.window,
            &mut self.state.view_port,
            input_cursor,
            (window_size.width, window_size.height),
        );
    }

    fn set_control_flow(&self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            self.context.instant() + Duration::from_secs_f64(1.0 / FPS as f64),
        ));
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

    let event_loop = EventLoop::builder()
        .with_x11()
        .build()
        .unwrap();
    App::<NativeBackend>::run(event_loop);
}
