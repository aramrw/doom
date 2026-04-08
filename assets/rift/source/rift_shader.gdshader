shader_type spatial;
render_mode cull_disabled, depth_draw_opaque;

// 1. Your original textures
uniform sampler2D albedo_texture : source_color, filter_linear_mipmap; 
uniform sampler2D alpha_texture : hint_default_white, filter_linear_mipmap; 
uniform sampler2D emission_texture : source_color, filter_linear_mipmap;

// A slider to boost your original emission texture's brightness
uniform float base_emission_energy : hint_range(0.0, 10.0) = 1.0; 

// 2. The noise map that dictates how the tear spreads
uniform sampler2D noise_texture; 

// 3. The slider you will animate (0.0 is closed, 1.0 is fully open)
uniform float progress : hint_range(0.0, 1.0) = 0.0; 

// 4. The glowing color of the tearing edge
uniform vec4 edge_color : source_color = vec4(0.2, 0.6, 1.0, 1.0);
uniform float edge_thickness : hint_range(0.0, 0.2) = 0.05;
uniform float edge_glow_energy : hint_range(0.0, 10.0) = 2.5;

void fragment() {
    // Read your base textures
    vec4 base_color = texture(albedo_texture, UV);
    float base_alpha = texture(alpha_texture, UV).r; // Reads the grayscale alpha mask
    vec3 base_emission = texture(emission_texture, UV).rgb; // Reads the original glow
    
    // If the pixel is supposed to be fully transparent based on your alpha map, discard it immediately.
    // (This stops the glowing edge from appearing in the empty spaces of your mesh).
    if (base_alpha < 0.1) {
        discard;
    }
    
    // Read the noise value
    float noise = texture(noise_texture, UV).r;
    
    // THE TEAR: Delete pixels that haven't materialized yet
    if (noise > progress) {
        discard;
    }
    
    float distance_to_edge = progress - noise;
    
    // THE GLOWING EDGE
    if (distance_to_edge < edge_thickness && progress < 1.0) {
        ALBEDO = edge_color.rgb;
        EMISSION = edge_color.rgb * edge_glow_energy; 
    } else {
        // NORMAL APPEARANCE: Use your original textures
        ALBEDO = base_color.rgb;
        EMISSION = base_emission * base_emission_energy;
    }
    
    // Apply the alpha from your alpha texture
    ALPHA = base_alpha;
}