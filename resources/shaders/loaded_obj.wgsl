// ── Uniforms ────────────────────────────────────────────────────────────────

struct Uniforms {
    pv:        mat4x4<f32>,
    model:     mat4x4<f32>,
    light_pos: vec3<f32>,
    ambient:   f32,
    view_pos:  vec3<f32>,
    specular:  f32,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var t_diffuse:  texture_2d<f32>;
@group(0) @binding(2) var s_diffuse:  sampler;

// ── Vertex ──────────────────────────────────────────────────────────────────

struct VertexInput {
    @location(0) position:   vec3<f32>,
    @location(1) color:      vec3<f32>,
    @location(2) normal:     vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) color:     vec3<f32>,
    @location(2) normal:    vec3<f32>,
    @location(3) uv:        vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_pos = u.model * vec4<f32>(in.position, 1.0);
    out.world_pos = world_pos.xyz;
    out.color     = in.color;
    // Normal matrix = transpose(inverse(model)) — use the 3×3 upper-left.
    // For uniform scale this is equivalent to mat3x3(model).
    let normal_mat = mat3x3<f32>(
        u.model[0].xyz,
        u.model[1].xyz,
        u.model[2].xyz,
    );
    out.normal    = normalize(normal_mat * in.normal);
    out.uv        = in.tex_coords;
    out.clip_position = u.pv * world_pos;

    return out;
}

// ── Fragment ────────────────────────────────────────────────────────────────

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Ambient
    let ambient = u.ambient * in.color;

    // Diffuse
    let normal    = normalize(in.normal);
    let light_dir = normalize(u.light_pos - in.world_pos);
    let diff      = max(dot(normal, light_dir), 0.0);
    let diffuse   = diff * in.color;

    // Specular (Blinn-Phong)
    let view_dir    = normalize(u.view_pos - in.world_pos);
    let reflect_dir = reflect(-light_dir, normal);
    let spec        = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    let specular    = u.specular * spec * in.color;

    let result = ambient + diffuse + specular;
    let tex    = textureSample(t_diffuse, s_diffuse, in.uv);

    return vec4<f32>(result, 1.0) * tex;
}
