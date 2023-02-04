#version 430 core

layout (location = 0) in vec2 center;
layout (location = 1) in vec2 offs_position;
layout (location = 2) in float rotation;
layout (location = 3) in float opacity;

out float frag_opacity;

void main() {
	// mat2 rot_matrix = mat2(
	// 	cos(rotation), sin(rotation), 
	// 	-sin(rotation), cos(rotation)
	// );
	// vec2 rot_offs_position = rot_matrix * offs_position;
	// gl_Position = vec4(center + offs_position, 0.0, 1.0);
	gl_Position = vec4(center + offs_position, 0.0, 1.0);
	frag_opacity = opacity;

}