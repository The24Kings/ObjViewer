pub mod material;
pub mod mesh;
pub mod shader;
pub mod shader_paths;
pub mod source;
pub mod texture;
pub mod types;
pub mod vertex;

pub use material::Material;
pub use mesh::Mesh;
pub use shader::Shader;
pub use shader_paths::*;
pub(crate) use source::ShaderSource;
pub use texture::Texture;
pub use types::{
    GlRef, PhysicalRef, RenderableRef, ShaderRef, TextureRef, WindowRef, new_gl_ref,
    new_physical_ref, new_renderable_ref, new_shader_ref, new_texture_ref,
};
pub use vertex::VEC2;
pub use vertex::VEC3;
pub use vertex::Vertex;
