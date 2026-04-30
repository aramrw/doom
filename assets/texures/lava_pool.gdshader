shader_type spatial;

uniform sampler2D lava_texture : filter_nearest; // Force pixelated filtering
uniform float speed = 2.0;
uniform float height = 0.15;

void vertex() {
    float snapped_time = floor(TIME * 8.0) / 8.0;
    float w = sin(snapped_time * speed + VERTEX.x * 3.0 + VERTEX.z * 3.0);
    VERTEX.y += w * height;
    
    // Vertex snapping
    VERTEX.xyz = floor(VERTEX.xyz * 16.0) / 16.0;
}

void fragment() {
    float snapped_time = floor(TIME * 8.0) / 8.0;
    
    // Low-res UV scrolling
    vec2 uv = floor(UV * 64.0) / 64.0;
    uv += vec2(snapped_time * 0.05, snapped_time * 0.05);
    
    vec4 tex = texture(lava_texture, uv);
    
    // Color banding
    vec3 crushed_color = floor(tex.rgb * 5.0) / 5.0;
    
    ALBEDO = crushed_color;
    EMISSION = crushed_color * 1.5;
}