#version 460

uniform mat4 vp;

in vec3 i_position;
in vec3 i_color;

out vec3 f_color;

void main(void) {	
	gl_Position = vp * vec4(i_position, 1.0);
	f_color = i_color;
}