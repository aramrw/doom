shader_type spatial;
render_mode unshaded, cull_disabled, depth_test_disabled;

uniform sampler2D tex : source_color, filter_nearest;
uniform float hit_flash : hint_range(0.0, 1.0) = 0.0;

void vertex() {
	// Re-use the proven billboard logic from the outline shader
	float scale_x = length(MODEL_MATRIX[0].xyz);
	float scale_y = length(MODEL_MATRIX[1].xyz);
	float scale_z = length(MODEL_MATRIX[2].xyz);

	MODELVIEW_MATRIX = VIEW_MATRIX * mat4(
		vec4(normalize(INV_VIEW_MATRIX[0].xyz) * scale_x, 0.0),
		vec4(normalize(INV_VIEW_MATRIX[1].xyz) * scale_y, 0.0),
		vec4(normalize(INV_VIEW_MATRIX[2].xyz) * scale_z, 0.0),
		MODEL_MATRIX[3]
	);
	
	MODELVIEW_NORMAL_MATRIX = mat3(MODELVIEW_MATRIX);
}

void fragment() {
	vec4 color = texture(tex, UV);
	if (color.a < 0.1) discard;
	
	ALBEDO = vec3(1.0, 1.0, 1.0);
	ALPHA = color.a * hit_flash;
}
