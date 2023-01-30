#version 430 core

struct Dot {
	vec2 coords;
	float radius;
	vec3 color;
	float brightness;
};

layout(std430, binding = 0) buffer obj_buffer {
	uint num_dots;
	Dot dots[];
};

in vec2 frag_coords;

out vec4 pixel_color;

void main() {
	pixel_color = vec4(mod(abs(frag_coords.x), 50.0) / 50.0, mod(abs(frag_coords.y), 50.0) / 50.0, float(num_dots), 1.0);
}