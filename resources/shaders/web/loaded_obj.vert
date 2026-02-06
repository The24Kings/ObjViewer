#version 300 es
precision mediump float;

uniform mat4 pv;
uniform mat4 model;

in vec3 i_position;
in vec3 i_color;
in vec3 i_normal;
in vec2 i_uv;

out vec3 f_pos;
out vec3 f_color;
out vec3 f_normal;
out vec2 f_uv;

void main(void) {
    f_pos = vec3(model * vec4(i_position, 1.0));
    f_color = i_color;
    f_normal = mat3(transpose(inverse(model))) * i_normal;
    f_uv = i_uv;

    gl_Position = pv * vec4(f_pos, 1.0);
}