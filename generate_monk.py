
import os

sprites_dir = "npcs/ShotgunMonk/sprites"
output_file = "npcs/shotgun_monk.tscn"

# Frame A to E are walk
# Angle 1 to 5
animations = {
    "walk_1": ["MARAA1", "MARAB1", "MARAC1", "MARAD1", "MARAE1"],
    "walk_2": ["MARAA8A2", "MARAB8B2", "MARAC8C2", "MARAD8D2", "MARAE8E2"],
    "walk_3": ["MARAA7A3", "MARAB7B3", "MARAC7C3", "MARAD7D3", "MARAE7E3"],
    "walk_4": ["MARAA6A4", "MARAB6B4", "MARAC6C4", "MARAD6D4", "MARAE6E4"],
    "walk_5": ["MARAA5", "MARAB5", "MARAC5", "MARAD5", "MARAE5"],
    "pain": ["MARAF1"],
    "death_1": ["MARAH0", "MARAI0", "MARAJ0", "MARAK0", "MARAL0", "MARAM0", "MARAN0", "MARAO0"]
}

# We also need attack
animations["attack_1"] = ["MARAG1", "MARAP1"]
animations["attack_2"] = ["MARAG8G2", "MARAP8P2"]
animations["attack_3"] = ["MARAG7G3", "MARAP7P3"]
animations["attack_4"] = ["MARAG6G4", "MARAP6P4"]
animations["attack_5"] = ["MARAG5", "MARAP5"]

# Generate TSCN
ext_resources = []
sprite_map = {} # name to id

# Find UIDs
for root, dirs, files in os.walk(sprites_dir):
    for f in files:
        if f.endswith(".png.import"):
            with open(os.path.join(root, f), 'r') as f_in:
                content = f_in.read()
                uid = content.split('uid="')[1].split('"')[0]
                sprite_name = f.replace(".png.import", "")
                sprite_map[sprite_name] = uid

res_id = 2
tscn = '[gd_scene load_steps=100 format=3 uid="uid://shotgunmonk123"]\n\n'
tscn += '[ext_resource type="Script" uid="uid://c0seuvhjuadad" path="res://npcs/base_npc.gd" id="1_base"]\n'
tscn += '[ext_resource type="Script" path="res://enemies/enemy_sounds.gd" id="2_sounds"]\n'

res_id = 3
mapped_res = {} # name to res_id

for anim, frames in animations.items():
    for f in frames:
        if f in sprite_map and f not in mapped_res:
            tscn += f'[ext_resource type="Texture2D" uid="{sprite_map[f]}" path="res://npcs/ShotgunMonk/sprites/{f}.png" id="{res_id}_tex"]\n'
            mapped_res[f] = res_id
            res_id += 1

tscn += '\n[sub_resource type="CapsuleShape3D" id="CapsuleShape3D_1"]\nradius = 0.4\nheight = 1.8\n'
tscn += '\n[sub_resource type="SpriteFrames" id="SpriteFrames_1"]\nanimations = ['

anim_list = []
for anim, frames in animations.items():
    anim_str = '{\n"frames": ['
    frame_list = []
    for f in frames:
        if f in mapped_res:
            frame_list.append(f'{{\n"duration": 1.0,\n"texture": ExtResource("{mapped_res[f]}_tex")\n}}')
    anim_str += ",\n".join(frame_list)
    anim_str += f'],\n"loop": {"true" if "walk" in anim else "false"},\n"name": &"{anim}",\n"speed": 8.0\n}}'
    anim_list.append(anim_str)

tscn += ",\n".join(anim_list)
tscn += ']\n'

tscn += """
[node name="ShotgunMonk" type="CharacterBody3D" groups=["Enemies", "NPCs"]]
collision_layer = 4
collision_mask = 3
script = ExtResource("1_base")
follows_player = true
attacks_enemies = true
npc_display_name = "Shotgun Monk"
dialogue_lines = PackedStringArray("I'll cover you.", "Keep moving.")

[node name="CollisionShape3D" type="CollisionShape3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0.9, 0)
shape = SubResource("CapsuleShape3D_1")

[node name="AnimatedSprite3D" type="AnimatedSprite3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0.9, 0)
pixel_size = 0.03
billboard = 2
shaded = true
texture_filter = 0
sprite_frames = SubResource("SpriteFrames_1")
animation = &"walk_1"

[node name="NavigationAgent3D" type="NavigationAgent3D" parent="."]

[node name="RayCast3D" type="RayCast3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1.2, 0)
target_position = Vector3(0, 0, -10)
collision_mask = 3

[node name="EnemySounds" type="AudioStreamPlayer3D" parent="."]
bus = &"SfxBus"
script = ExtResource("2_sounds")
death_folder = "res://npcs/ShotgunMonk/sounds/death"
hurt_folder = "res://npcs/ShotgunMonk/sounds/hurt"
taunt_folder = "res://npcs/ShotgunMonk/sounds/taunt"

[node name="InteractionZone" type="Area3D" parent="."]
collision_layer = 4
collision_mask = 2

[node name="CollisionShape3D" type="CollisionShape3D" parent="InteractionZone"]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0.9, 0)
shape = SubResource("CapsuleShape3D_1")
"""

with open(output_file, 'w') as f_out:
    f_out.write(tscn)
print(f"Generated {output_file}")
