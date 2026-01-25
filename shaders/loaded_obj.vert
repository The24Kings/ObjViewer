#version 460

uniform mat4 pv;
uniform mat4 model;

in vec3 i_position;
in vec3 i_color;
in vec3 i_normal;

out vec3 f_color;
out vec3 f_normal;

void main(void) {
    gl_Position = pv * model * vec4(i_position, 1.0);
    f_color = i_color;
    f_normal = i_normal;
}