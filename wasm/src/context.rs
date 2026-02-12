//! WASM context (no ImGui).

use std::rc::Rc;
use web_time::Instant;

pub struct WasmContext {
    pub gl: Rc<glow::Context>,
    pub clear_color: [f32; 4],
    instant: Instant,
}

impl WasmContext {
    pub fn new(gl: glow::Context) -> Self {
        Self {
            gl: Rc::new(gl),
            clear_color: [0.0, 0.0, 0.0, 1.0],
            instant: Instant::now(),
        }
    }

    pub fn gl_context(&self) -> Rc<glow::Context> {
        Rc::clone(&self.gl)
    }

    /// Returns delta time since last tick in seconds.
    pub fn dt(&self) -> f32 {
        self.instant.elapsed().as_secs_f32()
    }

    /// Resets the instant for the next frame.
    pub fn tick(&mut self) {
        self.instant = Instant::now();
    }

    /// Returns the current instant (for control flow timing).
    pub fn instant(&self) -> Instant {
        self.instant
    }
}
