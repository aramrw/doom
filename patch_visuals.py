import re

with open('npcs/shotgun_monk.tscn', 'r') as f:
    content = f.read()

# Add shader ext_resource
shader_res = '[ext_resource type="Shader" path="res://shaders/enemy_visuals.gdshader" id="enemy_shader"]\n'
if 'res://shaders/enemy_visuals.gdshader' not in content:
    last_ext_idx = content.rfind('[ext_resource')
    end_of_line = content.find('\n', last_ext_idx) + 1
    content = content[:end_of_line] + shader_res + content[end_of_line:]

# Add ShaderMaterial sub_resource
sub_res = """
[sub_resource type="ShaderMaterial" id="ShaderMaterial_enemy"]
shader = ExtResource("enemy_shader")
shader_parameter/hit_flash = 0.0
shader_parameter/flash_color = Color(1, 1, 1, 1)
shader_parameter/outline_active = false
shader_parameter/outline_color = Color(1, 0, 0, 1)
shader_parameter/outline_width = 1.0
"""
if 'id="ShaderMaterial_enemy"' not in content:
    last_sub_idx = content.rfind('[sub_resource')
    end_of_line = content.find('\n', last_sub_idx) + 1
    content = content[:end_of_line] + sub_res + content[end_of_line:]

# Apply material to AnimatedSprite3D
# Find AnimatedSprite3D node and add material_override
sprite_pattern = r'(\[node name="AnimatedSprite3D".*?\])'
sprite_match = re.search(sprite_pattern, content)
if sprite_match:
    node_str = sprite_match.group(1)
    if 'material_override' not in node_str:
        new_node_str = node_str.replace(']', ' material_override = SubResource("ShaderMaterial_enemy")]')
        content = content.replace(node_str, new_node_str)

with open('npcs/shotgun_monk.tscn', 'w') as f:
    f.write(content)
