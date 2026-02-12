pub mod game;
pub mod graphics;
pub mod objects;
pub mod platform;
mod view_port;

use std::rc::Rc;

use glow::{Context, HasContext};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

pub use self::platform::PlatformBackend;
pub use self::view_port::ViewPort;

/// Shared state that exists on all platforms.
pub struct State {
    pub window: Rc<Window>,
    pub input: WinitInputHelper,
    pub view_port: ViewPort,
    pub request_redraw: bool,
    pub wait_cancelled: bool,
}

// Helper to check for GL errors at runtime. Mirrors the behavior of the
// C-style `glCheckError()` helper: it polls `gl.get_error()` and prints
// any found errors with the source file and line number.
pub fn gl_check_error_impl(gl: &Context, file: &'static str, line: u32) -> u32 {
    let mut last_error = glow::NO_ERROR;
    unsafe {
        loop {
            let err = gl.get_error();
            if err == glow::NO_ERROR {
                break;
            }
            last_error = err;
            let error_str = match err {
                glow::INVALID_ENUM => "INVALID_ENUM",
                glow::INVALID_VALUE => "INVALID_VALUE",
                glow::INVALID_OPERATION => "INVALID_OPERATION",
                glow::STACK_OVERFLOW => "STACK_OVERFLOW",
                glow::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                glow::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                glow::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                _ => "UNKNOWN_ERROR",
            };
            eprintln!("GL error: {} | {} ({})", error_str, file, line);
        }
    }
    last_error
}

// Macro wrapper so callers can write `gl_check_error!(gl)` and get file/line.
#[macro_export]
macro_rules! gl_check_error {
    ($gl:expr) => {
        $crate::gl_check_error_impl(&$gl, file!(), line!())
    };
}
