#version 330

uniform mat4 pv;
uniform mat4 model;

in vec3 i_position;

void main(void) {
    gl_Position = pv * model * vec4(i_position, 1.0);
}