#version 430 core

precision mediump float;

uniform vec3 color;

in float frag_opacity;

out vec4 pixel_color;

void main() {
	pixel_color = vec4(color, frag_opacity);
}