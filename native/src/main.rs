use std::error::Error;
use std::sync::Arc;

use log::info;
use time::{UtcOffset, format_description::parse};
use tracing_subscriber::fmt::time::OffsetTime;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowAttributes;
use winit_input_helper::WinitInputHelper;

use app::{App, HEIGHT, WIDTH};
use core::gpu_init::init_gpu;
use core::graphics::GpuContext;
use core::{PlatformBackend, RenderContext, State, ViewPort};

/// Native platform backend using wgpu.
pub struct NativeBackend {
    state: State,
    context: RenderContext,
}

impl PlatformBackend for NativeBackend {
    fn new(event_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>> {
        let attributes = WindowAttributes::default()
            .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
            .with_title("Obj Viewer");

        let window = event_loop.create_window(attributes)?;
        let window = Arc::new(window);

        // Initialize wgpu (synchronous on native via pollster)
        let gpu_handle = pollster::block_on(init_gpu(window.clone(), WIDTH, HEIGHT));
        info!("wgpu device initialized");

        let gpu = GpuContext {
            device: gpu_handle.device.clone(),
            queue: gpu_handle.queue.clone(),
        };

        // Shared render context (ImGui + surface)
        let mut context = RenderContext::new(
            gpu_handle.device.clone(),
            gpu_handle.queue.clone(),
            gpu_handle.surface,
            gpu_handle.surface_config,
            gpu_handle.surface_format,
        );
        context.attach_window(&window);
        info!("ImGui initialized");

        let view_port = ViewPort::new(
            window.clone(),
            gpu,
            gpu_handle.surface_format,
            (WIDTH, HEIGHT),
        );

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
        self.context.resize(width, height);
        self.state.view_port.resize(width, height);
    }

    fn state(&mut self) -> &mut State {
        &mut self.state
    }

    fn context(&mut self) -> &mut RenderContext {
        &mut self.context
    }

    fn inner(&mut self) -> (&mut State, &mut RenderContext) {
        (&mut self.state, &mut self.context)
    }
}

fn main() {
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

    let event_loop = EventLoop::new().unwrap();
    App::<NativeBackend>::run(event_loop);
}
