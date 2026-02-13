//! WGSL shader sources — platform-agnostic (wgpu compiles WGSL everywhere).

/// Light-cube shader (textured passthrough, no lighting).
pub const LIGHT_CUBE_WGSL: &str = include_str!("../../../resources/shaders/light_cube.wgsl");

/// Loaded-object shader (Phong lighting + texture).
pub const LOADED_OBJ_WGSL: &str = include_str!("../../../resources/shaders/loaded_obj.wgsl");
