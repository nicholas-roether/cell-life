#version 430 core

precision mediump float;

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

vec4 blend_light(vec4 color1, vec4 color2) {
	float alpha_sum = color1.a + color2.a;
	if (alpha_sum == 0) {
		return vec4(0.0, 0.0, 0.0, 0.0);
	}
	
	return vec4(
		mix(
			color1.rgb,
			color2.rgb,
			color2.a / alpha_sum
		),
		alpha_sum
	);
}

vec4 normalize_alpha(vec4 color) {
	return vec4(color.rgb, 1.0 - exp(-color.a));
}

const float BLOOM_DROPOFF = 0.9;

float bloom_strength(float dist, float brightness) {
	return brightness / (dist + 1);
}

float signed_distance(vec2 pos, Dot dot_obj) {
	return distance(pos, dot_obj.coords) - dot_obj.radius;
}

vec4 bloom(float dist, Dot dot_obj) {
	return vec4(
		dot_obj.color,
		bloom_strength(dist, dot_obj.brightness)
	);
}

void main() {
	vec4 light_color = vec4(0.0, 0.0, 0.0, 1.0);

	for (uint i = 0; i < num_dots; i++) {
		Dot current_dot = dots[i];

		float signed_dist = signed_distance(frag_coords, current_dot);
		if (signed_dist <= 0) {
			pixel_color = vec4(current_dot.color, 1.0);
			return;
		}
		vec4 current_light_color = bloom(signed_dist, current_dot);
		light_color = blend_light(light_color, current_light_color);
	}
	
	pixel_color = normalize_alpha(light_color);

	// pixel_color = normalize_alpha(blend_light(vec4(1.0, 1.0, 1.0, 1.0), vec4(0.0, 0.0, 0.0, 0.5)));
}