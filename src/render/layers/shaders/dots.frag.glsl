#version 430 core

precision highp float;

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

vec4 get_light_color(vec4 light) {
	return vec4(light.rgb, 1.0 - exp(-light.a));
}

float bloom_strength(float dist, float brightness) {
	if (dist <= 0) return brightness;
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

const vec4 AMBIENT_LIGHT = vec4(1.0, 1.0, 1.0, 0.2);

vec4 get_cast_light() {
	vec4 result = vec4(0.0, 0.0, 0.0, 0.0);
	for (uint i = 0; i < num_dots; i++) {
		Dot current_dot = dots[i];

		float signed_dist = signed_distance(frag_coords, current_dot);
		vec4 current_light = bloom(signed_dist, current_dot);
		result = blend_light(result, current_light);
	}
	return result;
}

vec4 get_light(vec4 cast_light) {
	return blend_light(cast_light, AMBIENT_LIGHT);
}

const vec3 BACKGROUND_COLOR = vec3(0.0, 0.0, 0.0);

vec3 get_object_color() {
	for (uint i = 0; i < num_dots; i++) {
		Dot current_dot = dots[i];
		float signed_dist = signed_distance(frag_coords, current_dot);
		if (signed_dist <= 0) {
			return current_dot.color;
		}
	}
	return BACKGROUND_COLOR;
}

vec3 shade(vec3 obj_color, vec4 light_color) {
	return light_color.a * light_color.rgb * obj_color;
}

vec3 fog(vec3 shaded_color, vec4 cast_light_color) {
	return mix(shaded_color, cast_light_color.rgb, cast_light_color.a);
}

void main() {
	vec4 cast_light = get_cast_light();
	vec4 cast_light_color = get_light_color(cast_light);
	vec4 light_color = get_light_color(get_light(cast_light));
	vec3 shaded_color = shade(get_object_color(), light_color);
	vec3 final_color = fog(shaded_color, cast_light_color);
	pixel_color = vec4(final_color, 1.0);
}