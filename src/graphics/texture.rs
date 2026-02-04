use glow::{Context, HasContext, NativeTexture};
use image::ImageReader;
use std::sync::Arc;

#[derive(Clone)]
pub struct Texture {
    gl: Arc<Context>,
    pub(crate) handle: NativeTexture,
    pub unit: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Default)]
pub enum FilterMode {
    #[default]
    Nearest, // Pixelated (good for pixel art)
    Linear, // Smooth (good for photos/realistic textures)
}

#[derive(Clone, Copy, Default)]
pub enum WrapMode {
    #[default]
    Repeat,
    ClampToEdge,
    MirroredRepeat,
}

pub struct TextureBuilder {
    gl: Arc<Context>,
    unit: i32,
    filter: FilterMode,
    wrap: WrapMode,
}

impl TextureBuilder {
    pub fn new(gl: Arc<Context>) -> Self {
        Self {
            gl,
            unit: 0,
            filter: FilterMode::default(),
            wrap: WrapMode::default(),
        }
    }

    pub fn unit(mut self, unit: i32) -> Self {
        self.unit = unit;
        self
    }

    pub fn filter(mut self, filter: FilterMode) -> Self {
        self.filter = filter;
        self
    }

    pub fn wrap(mut self, wrap: WrapMode) -> Self {
        self.wrap = wrap;
        self
    }

    /// Load texture from file path
    pub fn load_file(self, path: &str) -> Result<Texture, String> {
        let img = ImageReader::open(path)
            .map_err(|e| format!("Failed to open '{}': {}", path, e))?
            .decode()
            .map_err(|e| format!("Failed to decode '{}': {}", path, e))?
            .to_rgba8();

        self.load_rgba(&img.as_raw(), img.width(), img.height())
    }

    /// Load texture from raw RGBA bytes
    pub fn load_rgba(self, data: &[u8], width: u32, height: u32) -> Result<Texture, String> {
        unsafe {
            let texture = self.gl.create_texture().map_err(|e| e)?;
            let pixels = glow::PixelUnpackData::Slice(Some(data));

            self.gl.active_texture(glow::TEXTURE0 + self.unit as u32);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                pixels,
            );

            let filter = match self.filter {
                FilterMode::Nearest => glow::NEAREST as i32,
                FilterMode::Linear => glow::LINEAR as i32,
            };

            let wrap = match self.wrap {
                WrapMode::Repeat => glow::REPEAT as i32,
                WrapMode::ClampToEdge => glow::CLAMP_TO_EDGE as i32,
                WrapMode::MirroredRepeat => glow::MIRRORED_REPEAT as i32,
            };

            self.gl
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, filter);
            self.gl
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, filter);
            self.gl
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, wrap);
            self.gl
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, wrap);

            Ok(Texture {
                gl: self.gl,
                handle: texture,
                unit: self.unit,
                width,
                height,
            })
        }
    }
}

impl Texture {
    /// Quick load with default settings (unit 0, nearest filter, repeat wrap)
    pub fn from_file(gl: Arc<Context>, path: &str) -> Result<Texture, String> {
        TextureBuilder::new(gl).load_file(path)
    }

    /// Start building a texture with custom settings
    pub fn builder(gl: Arc<Context>) -> TextureBuilder {
        TextureBuilder::new(gl)
    }

    /// Create a default 1x1 white texture (RGBA: 255, 255, 255, 255)
    pub fn white_1x1(gl: Arc<Context>) -> Result<Texture, String> {
        let white_pixel: [u8; 4] = [255, 255, 255, 255];
        TextureBuilder::new(gl)
            .filter(FilterMode::Nearest)
            .wrap(WrapMode::Repeat)
            .load_rgba(&white_pixel, 1, 1)
    }

    /// Bind this texture to its assigned texture unit
    pub fn bind(&self) {
        unsafe {
            self.gl.active_texture(glow::TEXTURE0 + self.unit as u32);
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.handle));
        }
    }
}
