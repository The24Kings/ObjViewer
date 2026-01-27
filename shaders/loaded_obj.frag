#version 330

in vec3 f_pos;
in vec3 f_color;
in vec3 f_normal;

uniform float ambient;
uniform float specular;
uniform vec3 light_pos;
uniform vec3 view_pos;

out vec4 o_color;

void main(void) {
	// Ambient
	vec3 ambient = ambient * f_color;
	
	// Diffuse
	vec3 normal = normalize(f_normal);
	vec3 light_dir = normalize(light_pos - f_pos);
	float diff = max(dot(normal, light_dir), 0.0);
	vec3 diffuse = diff * f_color;

	// Specular
	vec3 view_dir = normalize(view_pos - f_pos);
	vec3 reflect_dir = reflect(-light_dir, normal);
	float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32);
	vec3 specular = specular * spec * f_color;

	vec3 result = ambient + diffuse + specular;

	o_color = vec4(result, 1.0) * vec4(f_color, 1.0);
	// o_color = vec4(f_color, 1.0);
}