#version 300 es
precision mediump float;

in vec2 f_tex_coord;

uniform sampler2D u_texture;

out vec4 o_color;

void main(void) {
	o_color = texture(u_texture, f_tex_coord) * vec4(1.0, 1.0, 1.0, 1.0);
}