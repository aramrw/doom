shader_type spatial;
render_mode unshaded, cull_disabled, depth_test_disabled;

uniform sampler2D tex : source_color, filter_nearest;
uniform vec4 outline_color : source_color = vec4(1.0, 0.0, 0.0, 1.0);
uniform float width : hint_range(0.0, 10.0) = 1.0;

void vertex() {
	// 1. Extract the scale from the MODEL_MATRIX
	// We need this because enemies like the Grin are scaled up in the editor!
	float scale_x = length(MODEL_MATRIX[0].xyz);
	float scale_y = length(MODEL_MATRIX[1].xyz);
	float scale_z = length(MODEL_MATRIX[2].xyz);

	// 2. Re-apply the scale to the billboard matrix
	// This ensures the outline quad perfectly matches the sprite's size
	MODELVIEW_MATRIX = VIEW_MATRIX * mat4(
		vec4(normalize(INV_VIEW_MATRIX[0].xyz) * scale_x, 0.0),
		vec4(normalize(INV_VIEW_MATRIX[1].xyz) * scale_y, 0.0),
		vec4(normalize(INV_VIEW_MATRIX[2].xyz) * scale_z, 0.0),
		MODEL_MATRIX[3]
	);
	
	MODELVIEW_NORMAL_MATRIX = mat3(MODELVIEW_MATRIX);
}

void fragment() {
	vec2 uv = UV;
	vec4 color = texture(tex, uv);
	
	// Don't draw over the sprite itself
	if (color.a > 0.5) {
		discard;
	}
	
	vec2 size = vec2(textureSize(tex, 0));
	float a = 0.0;
	
	// Sample neighbors to find the edge
	a = max(a, texture(tex, uv + vec2(width, 0.0) / size).a);
	a = max(a, texture(tex, uv + vec2(-width, 0.0) / size).a);
	a = max(a, texture(tex, uv + vec2(0.0, width) / size).a);
	a = max(a, texture(tex, uv + vec2(0.0, -width) / size).a);
	
	if (a > 0.5) {
		ALBEDO = outline_color.rgb;
		ALPHA = outline_color.a;
	} else {
		discard;
	}
}
