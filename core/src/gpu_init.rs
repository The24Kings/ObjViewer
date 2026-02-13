//! Shared async GPU initialization.
//!
//! Both native and wasm call `init_gpu` to create a wgpu Device + Queue.
//! Native wraps the call with `pollster::block_on`, wasm uses
//! `wasm_bindgen_futures::spawn_local`.

use std::sync::Arc;

/// Everything needed to start rendering after GPU init.
pub struct GpuHandle {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub surface: wgpu::Surface<'static>,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface_format: wgpu::TextureFormat,
}

/// Create a wgpu device, queue, and surface for the given window.
///
/// This is `async` so it works on both native (via `pollster::block_on`)
/// and wasm (via `wasm_bindgen_futures`).
pub async fn init_gpu(window: Arc<winit::window::Window>, width: u32, height: u32) -> GpuHandle {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = instance
        .create_surface(window.clone())
        .expect("Failed to create wgpu surface");

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find a suitable GPU adapter");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("ObjViewer Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                .using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::default(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            trace: wgpu::Trace::Off,
        })
        .await
        .expect("Failed to create wgpu device");

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(surface_caps.formats[0]);

    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width,
        height,
        present_mode: wgpu::PresentMode::AutoVsync,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &surface_config);

    GpuHandle {
        device: Arc::new(device),
        queue: Arc::new(queue),
        surface,
        surface_config,
        surface_format,
    }
}
