use std::error::Error;
use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;

use crate::{RenderContext, State};

pub trait PlatformBackend: Sized {
    /// Create a new platform backend with wgpu context, window, and all related state.
    fn new(event_loop: &ActiveEventLoop) -> Result<Self, Box<dyn Error>>;

    /// Handle window resize — reconfigure surface and depth texture.
    fn resize(&mut self, size: PhysicalSize<u32>);

    /// Get mutable access to the shared state.
    fn state(&mut self) -> &mut State;

    /// Get mutable access to the render context.
    fn context(&mut self) -> &mut RenderContext;

    /// Borrow state and context simultaneously (separate struct fields).
    fn inner(&mut self) -> (&mut State, &mut RenderContext);
}
