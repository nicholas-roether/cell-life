#version 330 core

in vec2 frag_coords;

out vec4 pixel_color;

void main() {
	pixel_color = vec4(mod(abs(frag_coords.x), 50.0) / 50.0, mod(abs(frag_coords.y), 50.0) / 50.0, 0.0, 1.0);
}