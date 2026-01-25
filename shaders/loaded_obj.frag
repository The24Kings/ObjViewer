#version 460

in vec3 f_color;
in vec3 f_normal;

out vec4 o_color;

void main(void) {
	o_color = vec4(f_color, 1.0);
}