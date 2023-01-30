#version 430 core

layout (location = 0) in vec2 uv;
layout (location = 1) in vec2 coords;

out vec2 frag_coords;

void main() {
	gl_Position = vec4(uv, 0.0, 1.0);
	frag_coords = coords;
}