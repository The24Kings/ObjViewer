// ============================================================================
// Native shaders (OpenGL 3.3+)
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    // Default shader
    pub const DEFAULT_FRAG_SRC: &str =
        include_str!("../../../resources/shaders/native/default.frag");
    pub const DEFAULT_VERT_SRC: &str =
        include_str!("../../../resources/shaders/native/default.vert");
    pub const DEFAULT_FRAG_PATH: &str = "resources/shaders/native/default.frag";
    pub const DEFAULT_VERT_PATH: &str = "resources/shaders/native/default.vert";

    // Light cube shader
    pub const LIGHT_CUBE_FRAG_SRC: &str =
        include_str!("../../../resources/shaders/native/light_cube.frag");
    pub const LIGHT_CUBE_VERT_SRC: &str =
        include_str!("../../../resources/shaders/native/light_cube.vert");
    pub const LIGHT_CUBE_FRAG_PATH: &str = "resources/shaders/native/light_cube.frag";
    pub const LIGHT_CUBE_VERT_PATH: &str = "resources/shaders/native/light_cube.vert";

    // Loaded obj shader
    pub const LOADED_OBJ_FRAG_SRC: &str =
        include_str!("../../../resources/shaders/native/loaded_obj.frag");
    pub const LOADED_OBJ_VERT_SRC: &str =
        include_str!("../../../resources/shaders/native/loaded_obj.vert");
    pub const LOADED_OBJ_FRAG_PATH: &str = "resources/shaders/native/loaded_obj.frag";
    pub const LOADED_OBJ_VERT_PATH: &str = "resources/shaders/native/loaded_obj.vert";
}

// ============================================================================
// Web shaders (GLSL ES 3.00 / WebGL2)
// ============================================================================

#[cfg(target_arch = "wasm32")]
pub mod web {
    // Default shader
    pub const DEFAULT_FRAG_SRC: &str = include_str!("../../../resources/shaders/web/default.frag");
    pub const DEFAULT_VERT_SRC: &str = include_str!("../../../resources/shaders/web/default.vert");
    pub const DEFAULT_FRAG_PATH: &str = "resources/shaders/web/default.frag";
    pub const DEFAULT_VERT_PATH: &str = "resources/shaders/web/default.vert";

    // Light cube shader
    pub const LIGHT_CUBE_FRAG_SRC: &str =
        include_str!("../../../resources/shaders/web/light_cube.frag");
    pub const LIGHT_CUBE_VERT_SRC: &str =
        include_str!("../../../resources/shaders/web/light_cube.vert");
    pub const LIGHT_CUBE_FRAG_PATH: &str = "resources/shaders/web/light_cube.frag";
    pub const LIGHT_CUBE_VERT_PATH: &str = "resources/shaders/web/light_cube.vert";

    // Loaded obj shader
    pub const LOADED_OBJ_FRAG_SRC: &str =
        include_str!("../../../resources/shaders/web/loaded_obj.frag");
    pub const LOADED_OBJ_VERT_SRC: &str =
        include_str!("../../../resources/shaders/web/loaded_obj.vert");
    pub const LOADED_OBJ_FRAG_PATH: &str = "resources/shaders/web/loaded_obj.frag";
    pub const LOADED_OBJ_VERT_PATH: &str = "resources/shaders/web/loaded_obj.vert";
}

// ============================================================================
// Platform-agnostic re-exports
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
pub use web::*;
