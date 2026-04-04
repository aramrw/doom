shader_type spatial;
render_mode unshaded, blend_mix, depth_draw_opaque, cull_back, alpha_to_coverage;

// 'TEXTURE' is the built-in for the sprite's texture in some contexts, 
// but in Spatial shaders for Sprite3D, we usually use albedo_texture
uniform sampler2D albedo_texture : source_color, filter_nearest;
uniform vec4 flash_color : source_color = vec4(1.0, 1.0, 1.0, 1.0);
uniform float flash_intensity : hint_range(0.0, 1.0) = 0.0;

void fragment() {
	vec4 color = texture(albedo_texture, UV);
	// Ensure we only flash the non-transparent parts
	vec3 flashed_rgb = mix(color.rgb, flash_color.rgb, flash_intensity);
	
	ALBEDO = flashed_rgb;
	ALPHA = color.a;
}