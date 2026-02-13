//! wgpu texture wrapper.

use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use image::ImageReader;

#[derive(Clone, Copy, Default)]
pub enum FilterMode {
    #[default]
    Nearest,
    Linear,
}

#[derive(Clone, Copy, Default)]
pub enum WrapMode {
    #[default]
    Repeat,
    ClampToEdge,
    MirroredRepeat,
}

/// A GPU texture + view + sampler, ready to be bound.
#[derive(Clone)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    /// Create from raw RGBA8 bytes.
    pub fn from_rgba(
        device: &Arc<wgpu::Device>,
        queue: &Arc<wgpu::Queue>,
        data: &[u8],
        width: u32,
        height: u32,
        filter: FilterMode,
        wrap: WrapMode,
        label: Option<&str>,
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let wgpu_filter = match filter {
            FilterMode::Nearest => wgpu::FilterMode::Nearest,
            FilterMode::Linear => wgpu::FilterMode::Linear,
        };
        let wgpu_wrap = match wrap {
            WrapMode::Repeat => wgpu::AddressMode::Repeat,
            WrapMode::ClampToEdge => wgpu::AddressMode::ClampToEdge,
            WrapMode::MirroredRepeat => wgpu::AddressMode::MirrorRepeat,
        };

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("texture_sampler"),
            address_mode_u: wgpu_wrap,
            address_mode_v: wgpu_wrap,
            address_mode_w: wgpu_wrap,
            mag_filter: wgpu_filter,
            min_filter: wgpu_filter,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            width,
            height,
        }
    }

    /// Load from embedded image bytes (PNG, JPEG, etc.).
    pub fn from_bytes(
        device: &Arc<wgpu::Device>,
        queue: &Arc<wgpu::Queue>,
        data: &[u8],
        label: Option<&str>,
    ) -> Result<Self, String> {
        let img = image::load_from_memory(data)
            .map_err(|e| format!("Failed to decode image: {e}"))?
            .to_rgba8();

        Ok(Self::from_rgba(
            device,
            queue,
            img.as_raw(),
            img.width(),
            img.height(),
            FilterMode::Nearest,
            WrapMode::Repeat,
            label,
        ))
    }

    /// Load from file path (native only).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_file(
        device: &Arc<wgpu::Device>,
        queue: &Arc<wgpu::Queue>,
        path: &str,
    ) -> Result<Self, String> {
        let img = ImageReader::open(path)
            .map_err(|e| format!("Failed to open '{path}': {e}"))?
            .decode()
            .map_err(|e| format!("Failed to decode '{path}': {e}"))?
            .to_rgba8();

        Ok(Self::from_rgba(
            device,
            queue,
            img.as_raw(),
            img.width(),
            img.height(),
            FilterMode::Nearest,
            WrapMode::Repeat,
            Some(path),
        ))
    }

    /// 1×1 white pixel — used as default when no texture is assigned.
    pub fn white_1x1(device: &Arc<wgpu::Device>, queue: &Arc<wgpu::Queue>) -> Self {
        Self::from_rgba(
            device,
            queue,
            &[255, 255, 255, 255],
            1,
            1,
            FilterMode::Nearest,
            WrapMode::Repeat,
            Some("white_1x1"),
        )
    }
}
