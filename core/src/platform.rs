use std::error::Error;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;

use crate::State;

pub trait PlatformBackend: Sized {
    /// Create a new platform backend with GL context, window, and all related state.
    fn new(event_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>>;

    /// Handle window resize - update surface/canvas dimensions and viewport.
    fn resize(&mut self, size: PhysicalSize<u32>);

    /// Swap buffers after rendering (native) or no-op (wasm).
    fn swap_buffers(&self);

    /// Get mutable access to the shared state.
    fn state(&mut self) -> &mut State;

    /// Get the clear color from the context.
    fn clear_color(&self) -> [f32; 4];

    /// Get delta time since last tick in seconds.
    fn dt(&self) -> f32;

    /// Reset the timing instant for the next frame.
    fn tick(&mut self);

    /// Handle window events for UI (e.g., ImGui on native).
    fn handle_ui_event(&mut self, event: &WindowEvent);

    /// Render the UI (ImGui on native, no-op on wasm).
    fn render_ui(&mut self);

    /// Set the control flow for the event loop.
    fn set_control_flow(&self, event_loop: &ActiveEventLoop);
}
