shader_type canvas_item;

uniform sampler2D screen_texture : hint_screen_texture, repeat_disable, filter_nearest;

// Adjust these in the Godot Inspector!
uniform float pixel_size : hint_range(1.0, 16.0) = 1.5; // Makes pixels chunky. Set to 1.0 if you don't need it.
uniform float color_depth : hint_range(1.0, 255.0) = 32.0; // Lower = more retro color banding. Higher = modern colors.

void fragment() {
    // 1. CHUNKY PIXELS: Calculate the grid for the retro pixelated look
    vec2 size = vec2(textureSize(screen_texture, 0));
    vec2 uv = floor(SCREEN_UV * size / pixel_size) * pixel_size / size;
    
    // Grab the pixel from the game world
    vec4 original_color = texture(screen_texture, uv);
    
    // 2. COLOR CRUSH: Reduce the color depth for that old-school monitor feel, 
    // but keep the actual color hues of your original game!
    vec3 crushed_color = round(original_color.rgb * color_depth) / color_depth;
    
    // Output the final retro pixel
    COLOR = vec4(crushed_color, original_color.a);
}