shader_type spatial;
render_mode cull_disabled, depth_draw_opaque; // Removed 'unshaded'

uniform sampler2D tex : source_color, filter_nearest;
uniform float hit_flash : hint_range(0.0, 1.0) = 0.0;
uniform vec4 flash_color : source_color = vec4(1.0, 1.0, 1.0, 1.0);

// Outline settings
uniform bool outline_active = false;
uniform vec4 outline_color : source_color = vec4(1.0, 0.0, 0.0, 1.0);
uniform float outline_width : hint_range(0.0, 10.0) = 1.0;

void vertex() {
	// Proven billboard logic
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
	vec2 uv = UV;
	vec4 color = texture(tex, uv);
	
	// 1. Calculate Outline
	float outline_alpha = 0.0;
	if (outline_active && color.a <= 0.5) {
		vec2 size = vec2(textureSize(tex, 0));
		outline_alpha = max(outline_alpha, texture(tex, uv + vec2(outline_width, 0.0) / size).a);
		outline_alpha = max(outline_alpha, texture(tex, uv + vec2(-outline_width, 0.0) / size).a);
		outline_alpha = max(outline_alpha, texture(tex, uv + vec2(0.0, outline_width) / size).a);
		outline_alpha = max(outline_alpha, texture(tex, uv + vec2(0.0, -outline_width) / size).a);
	}

	// 2. Draw Logic
	if (color.a > 0.5) {
		// Main Sprite
		ALBEDO = color.rgb;
		// Apply hit flash as EMISSION so it bypasses shading and looks bright
		EMISSION = flash_color.rgb * hit_flash * 10.0;
		ALPHA = color.a;
	} else if (outline_alpha > 0.5) {
		// Outline (Unshaded for visibility)
		ALBEDO = outline_color.rgb;
		EMISSION = outline_color.rgb;
		ALPHA = outline_color.a;
	} else {
		discard;
	}
}
