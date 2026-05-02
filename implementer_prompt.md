You are an expert Godot engine developer. Your task is to implement Task 1 from the Slitherfist integration plan.

Task 1: Initialize Slitherfist Scene Assets and Components
Modify: enemies/slitherfist/Slitherfist.tscn

Step 1: Update unique IDs and fix component references in Slitherfist.tscn. Ensure unique_id values are valid integers and that HealthComponent and AttackComponent are correctly linked to their respective scripts (res://components/HealthComponent.gd and res://components/AttackComponent.gd).
Step 2: Assign proper EnemySounds resource for Slitherfist. For now, check if there's a Slitherfist-specific sounds folder, otherwise use a placeholder.

Context: 
- Current Slitherfist.tscn:
[gd_scene format=3 uid="uid://d9f9j1v1l2d3e4f"]

[ext_resource type="Script" uid="uid://s1i2t3h4e5r6f" path="res://enemies/slitherfist/Slitherfist.gd" id="1_slitherfist"]
[ext_resource type="Script" path="res://enemies/enemy_sounds.gd" id="2_jgin6"] 
[ext_resource type="Script" uid="uid://h3a4l5t6h7c" path="res://components/HealthComponent.gd" id="3_health_component"]
[ext_resource type="Script" uid="uid://a8t9t0a1c2k" path="res://components/AttackComponent.gd" id="4_attack_component"]

[sub_resource type="CapsuleShape3D" id="CapsuleShape3D_ej5ua"]
height = 2.3876038

[sub_resource type="SpriteFrames" id="SpriteFrames_lch36"]

[node name="Slitherfist" type="CharacterBody3D" unique_id=1006558139] 
collision_layer = 4
collision_mask = 3
floor_stop_on_slope = false
safe_margin = 0.5
script = ExtResource("1_slitherfist")

[node name="HealthComponent" type="Node" parent="." unique_id=4000000000]
script = ExtResource("3_health_component")

[node name="AttackComponent" type="Node" parent="." unique_id=5000000000]
script = ExtResource("4_attack_component")

[node name="CollisionShape3D" type="CollisionShape3D" parent="." unique_id=1779757227]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, -0.026836932, 1.5509595, 0.0030199215)
shape = SubResource("CapsuleShape3D_ej5ua")

[node name="AnimatedSprite3D" type="AnimatedSprite3D" parent="." unique_id=2131804395]
transform = Transform3D(2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 1.0706522, 0)
billboard = 2
shaded = true
texture_filter = 0
sprite_frames = SubResource("SpriteFrames_lch36")

[node name="NavigationAgent3D" type="NavigationAgent3D" parent="." unique_id=255344233]
simplify_path = true

[node name="RayCast3D" type="RayCast3D" parent="." unique_id=1035914153]
collision_mask = 3

[node name="EnemySounds" type="AudioStreamPlayer3D" parent="." unique_id=22765739]
pitch_scale = 5.0
bus = &"SfxBus"
script = ExtResource("2_jgin6")
death_folder = "res://enemies/Grin/sounds/death" 
taunt_folder = "res://enemies/Grin/sounds/taunt" 
pitch_step = 10.0

