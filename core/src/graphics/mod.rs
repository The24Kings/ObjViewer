pub mod material;
pub mod mesh;
pub mod shader;
pub mod shader_paths;
pub mod texture;
pub mod types;
pub mod vertex;

pub use material::Material;
pub use mesh::Mesh;
pub use shader::Shader;
pub use shader_paths::*;
pub use texture::Texture;
pub use types::{
    GpuContext, PhysicalRef, RenderableRef, WindowRef,
    new_game_obj_ref, new_light_obj_ref, new_physical_ref, new_renderable_ref,
};
pub use types::LightObjectRef;
pub use vertex::Vertex;
