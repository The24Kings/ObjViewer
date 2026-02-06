#version 300 es
precision mediump float;

uniform mat4 pv;
uniform mat4 model;

in vec3 i_position;
in vec2 i_uv;

out vec2 f_tex_coord;

void main(void) {
    f_tex_coord = i_uv;

    gl_Position = pv * model * vec4(i_position, 1.0);
}