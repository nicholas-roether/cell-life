#version 330 core

in vec2 frag_coords;

out vec4 pixel_color;

void main() {
	pixel_color = vec4(frag_coords, 0.0, 1.0);
}