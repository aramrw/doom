import re
import sys

def cleanup(filename):
    with open(filename, 'r') as f:
        content = f.read()
    
    # 1. Remove the shader ext_resource
    content = re.sub(r'\[ext_resource type="Shader" path="res://shaders/enemy_visuals.gdshader" id="enemy_shader"\]\n?', '', content)
    
    # 2. Remove the ShaderMaterial sub_resource
    # We need to be careful not to remove the animations property that follows it
    content = re.sub(r'\[sub_resource type="ShaderMaterial" id="ShaderMaterial_enemy"\]\nshader = ExtResource\("enemy_shader"\)\nshader_parameter/hit_flash = 0.0\nshader_parameter/flash_color = Color\(1, 1, 1, 1\)\nshader_parameter/outline_active = false\nshader_parameter/outline_color = Color\(1, 0, 0, 1\)\nshader_parameter/outline_width = 1.0\n', '', content)
    
    # 3. Ensure material_override is gone from AnimatedSprite3D
    content = content.replace(' material_override = SubResource("ShaderMaterial_enemy")', '')
    
    with open(filename, 'w') as f:
        f.write(content)

cleanup('enemies/slitherfist/Slitherfist.tscn')
cleanup('npcs/shotgun_monk.tscn')
