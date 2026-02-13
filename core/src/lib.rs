pub mod game;
pub mod gpu_init;
pub mod graphics;
pub mod objects;
pub mod platform;
pub mod render_context;
mod view_port;

use winit_input_helper::WinitInputHelper;

use crate::graphics::WindowRef;

pub use self::platform::PlatformBackend;
pub use self::render_context::RenderContext;
pub use self::view_port::ViewPort;

/// Shared state that exists on all platforms.
pub struct State {
    pub window: WindowRef,
    pub input: WinitInputHelper,
    pub view_port: ViewPort,
    pub request_redraw: bool,
    pub wait_cancelled: bool,
}
