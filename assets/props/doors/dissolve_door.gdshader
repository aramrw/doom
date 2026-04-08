shader_type spatial;

uniform sampler2D albedo_texture : source_color;
uniform sampler2D noise_texture;
uniform float dissolve_amount : hint_range(0.0, 1.1) = 0.0;
uniform float edge_width : hint_range(0.0, 0.1) = 0.05;
uniform vec4 edge_color : source_color = vec4(1.0, 0.5, 0.0, 1.0); // Orange glow

void fragment() {
    vec4 tex = texture(albedo_texture, UV);
    float noise = texture(noise_texture, UV).r;
    
    // Determine if the pixel is dissolved
    // If noise is less than dissolve_amount, discard it
    if (noise < dissolve_amount) {
        discard;
    }
    
    // Create a glowing edge
    float edge_mask = step(noise, dissolve_amount + edge_width);
    vec3 final_color = mix(tex.rgb, edge_color.rgb, edge_mask * (1.0 - step(noise, dissolve_amount)));
    
    ALBEDO = final_color;
    EMISSION = edge_color.rgb * edge_mask * 2.0; // Make the edge glow
    ALPHA = tex.a;
}