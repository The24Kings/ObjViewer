// ── Uniforms ────────────────────────────────────────────────────────────────

struct Uniforms {
    pv:    mat4x4<f32>,
    model: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> u: Uniforms;
@group(0) @binding(1) var t_diffuse: texture_2d<f32>;
@group(0) @binding(2) var s_diffuse: sampler;

// ── Vertex ──────────────────────────────────────────────────────────────────

struct VertexInput {
    @location(0) position:   vec3<f32>,
    @location(1) color:      vec3<f32>,
    @location(2) normal:     vec3<f32>,
    @location(3) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.uv = in.tex_coords;
    out.clip_position = u.pv * u.model * vec4<f32>(in.position, 1.0);
    return out;
}

// ── Fragment ────────────────────────────────────────────────────────────────

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uv);
}
