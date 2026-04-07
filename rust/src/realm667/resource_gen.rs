use std::path::Path;
use std::fs;
use std::collections::HashMap;
use crate::realm667::actor::ActorDefinition;

pub struct ResourceGenerator;

impl ResourceGenerator {
    pub fn generate_weapon_resources(
        actor: &ActorDefinition, 
        actor_root: &Path, 
        rel_base: &str,
        label_sprites: &HashMap<String, Vec<(String, i32)>>
    ) {
        let weapon_name = actor.name.to_lowercase();
        let rel_path = rel_base.trim_end_matches('/');
        
        // Determine weapon type
        let mut projectile_name = None;
        let is_projectile = actor.states.iter().any(|(label, frames)| {
            if label != "Fire" && label != "Hold" && label != "AltFire" { return false; }
            frames.iter().any(|f| {
                if let Some(action) = &f.action {
                    let name = action.name.as_str();
                    if name == "A_FireProjectile" || name == "A_FireCustomMissile" {
                        if !action.args.is_empty() {
                            if let crate::realm667::actor::GZValue::String(p) = &action.args[0] {
                                projectile_name = Some(p.clone());
                            } else if let crate::realm667::actor::GZValue::Identifier(p) = &action.args[0] {
                                projectile_name = Some(p.clone());
                            }
                        }
                        return true;
                    }
                    return name.contains("Missile") || name.contains("Projectile");
                }
                false
            })
        });

        // 1. Generate Effects
        let effect_path = actor_root.join(format!("{}_fire_effect.tres", weapon_name));
        let effect_content = if is_projectile {
            let p_name = projectile_name.unwrap_or_else(|| "Unknown".to_string());
            let projectile_res_path = format!("{}/{}.tscn", rel_path, p_name.to_lowercase());
            format!(
r#"[gd_resource type="Resource" script_class="ProjectileEffect" format=3]
[ext_resource type="Script" path="res://weapons/effects/projectile_effect.gd" id="1_script"]
[ext_resource type="PackedScene" path="{}" id="2_proj"]
[resource]
script = ExtResource("1_script")
projectile_scene = ExtResource("2_proj")
speed = 50.0
damage = 20
"#, projectile_res_path)
        } else {
            format!(
r#"[gd_resource type="Resource" script_class="HitscanEffect" format=3]
[ext_resource type="Script" path="res://weapons/effects/hitscan_effect.gd" id="1_script"]
[resource]
script = ExtResource("1_script")
damage = 15
spread_angle = 2.0
"#)
        };
        let _ = fs::write(&effect_path, effect_content);

        let sound_path = actor_root.join(format!("{}_fire_sound.tres", weapon_name));
        let sound_content = 
r#"[gd_resource type="Resource" script_class="SoundEffect" format=3]
[ext_resource type="Script" path="res://weapons/effects/sound_effect.gd" id="1_script"]
[resource]
script = ExtResource("1_script")
pitch_randomness = 0.05
"#;
        let _ = fs::write(&sound_path, sound_content);

        // 2. Generate Action Step
        let step_path = actor_root.join(format!("{}_fire_step.tres", weapon_name));
        let step_content = format!(
r#"[gd_resource type="Resource" script_class="ActionStep" format=3]
[ext_resource type="Script" path="res://weapons/action_step.gd" id="1_script"]
[ext_resource type="Resource" path="{}/{}_fire_effect.tres" id="2_effect"]
[ext_resource type="Resource" path="{}/{}_fire_sound.tres" id="3_sound"]
[resource]
script = ExtResource("1_script")
frame_index = 0
effects = [ExtResource("2_effect"), ExtResource("3_sound")]
"#, rel_path, weapon_name, rel_path, weapon_name);
        let _ = fs::write(&step_path, step_content);

        // 3. Generate Action
        let action_path = actor_root.join(format!("{}_fire_action.tres", weapon_name));
        let action_content = format!(
r#"[gd_resource type="Resource" script_class="WeaponAction" format=3]
[ext_resource type="Script" path="res://weapons/weapon_action.gd" id="1_script"]
[ext_resource type="Resource" path="{}/{}_fire_step.tres" id="2_step"]
[resource]
script = ExtResource("1_script")
animation_name = "shoot"
steps = [ExtResource("2_step")]
loop = false
consumes_ammo = true
"#, rel_path, weapon_name);
        let _ = fs::write(&action_path, action_content);

        // 4. Generate SpriteFrames
        let sprite_frames_path = actor_root.join(format!("{}_spriteframes.tres", weapon_name));
        let mut sf_content = String::from("[gd_resource type=\"SpriteFrames\" load_steps=2 format=3]\n\n");
        
        let mut ext_resources = Vec::new();
        let mut animations = Vec::new();
        let mut id_counter = 1;

        // Sort keys for consistent generation
        let mut keys: Vec<_> = label_sprites.keys().collect();
        keys.sort();

        for anim_name in keys {
            let sprites = label_sprites.get(anim_name).unwrap();
            if sprites.is_empty() { continue; }

            let mut frames = Vec::new();
            for (sprite_rel_path, duration) in sprites {
                let id = format!("{}_ext", id_counter);
                ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, id));
                let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                id_counter += 1;
            }
            
            animations.push(format!(
r#"{{
"frames": [{}],
"loop": {},
"name": &"{}",
"speed": 35.0
}}"#, frames.join(", "), if anim_name == "idle" { "true" } else { "false" }, anim_name));
        }

        sf_content.push_str(&ext_resources.join("\n"));
        sf_content.push_str("\n\n[resource]\nanimations = [");
        sf_content.push_str(&animations.join(", "));
        sf_content.push_str("]\n");
        
        let _ = fs::write(&sprite_frames_path, sf_content);

        // 5. Generate WeaponData
        let wpdata_path = actor_root.join(format!("{}_wpdata.tres", weapon_name));
        let wpdata_content = format!(
r#"[gd_resource type="Resource" script_class="WeaponData" format=3]
[ext_resource type="Script" path="res://weapons/weapon_data.gd" id="1_script"]
[ext_resource type="SpriteFrames" path="{}/{}_spriteframes.tres" id="2_sprites"]
[ext_resource type="Resource" path="{}/{}_fire_action.tres" id="3_fire"]
[resource]
script = ExtResource("1_script")
item_name = "{}"
sprite_frames = ExtResource("2_sprites")
sprite_offset = Vector2(0, 0)
flip_h = false
actions = {{ "primary": ExtResource("3_fire") }}
ammo_give = {}
"#, rel_path, weapon_name, rel_path, weapon_name, weapon_name, 
            actor.properties.get("Weapon.AmmoGive")
                .map(|v| v.to_string_lossy())
                .unwrap_or_else(|| "30".to_string()));
        
        let _ = fs::write(&wpdata_path, wpdata_content);

        // 6. Generate Pickup Scene
        let pickup_path = actor_root.join(format!("{}_pickup.tscn", weapon_name));
        let pickup_content = format!(
r#"[gd_scene load_steps=6 format=3]

[ext_resource type="Script" path="res://items/pickup.gd" id="1_pickup"]
[ext_resource type="Resource" path="{}/{}_wpdata.tres" id="2_data"]
[ext_resource type="SpriteFrames" path="{}/{}_spriteframes.tres" id="3_sprites"]

[sub_resource type="SphereShape3D" id="SphereShape3D_1"]
radius = 0.5

[node name="Pickup" type="Area3D"]
collision_layer = 4
collision_mask = 2
script = ExtResource("1_pickup")
data = ExtResource("2_data")
mode = 2

[node name="CollisionShape3D" type="CollisionShape3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0.5, 0)
shape = SubResource("SphereShape3D_1")

[node name="AnimatedSprite3D" type="AnimatedSprite3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0.5, 0)
billboard = 2
texture_filter = 0
sprite_frames = ExtResource("3_sprites")
autoplay = "ground"
"#, rel_path, weapon_name, rel_path, weapon_name);
        
        let _ = fs::write(&pickup_path, pickup_content);
    }

    fn get_direction_sprites(sprites: &[(String, i32)], dir: usize) -> Vec<(String, i32)> {
        let dir_char = std::char::from_digit(dir as u32, 10).unwrap();
        sprites.iter().filter(|(s, _)| {
            let stem = Path::new(s).file_stem().unwrap_or_default().to_str().unwrap_or_default().to_uppercase();
            if stem.ends_with('0') { return true; }
            
            // Typical Doom sprite: XXXXA1 or XXXXA2A8
            // We look at the characters after the frame char.
            // A frame char is usually at index 4 (0-indexed).
            if stem.len() >= 6 {
                let rotations = &stem[5..];
                rotations.contains(dir_char)
            } else {
                false
            }
        }).cloned().collect()
    }

    pub fn generate_enemy_resources(
        actor: &ActorDefinition, 
        actor_root: &Path, 
        rel_base: &str,
        label_sprites: &HashMap<String, Vec<(String, i32)>>
    ) {
        let enemy_name = actor.name.to_lowercase();
        let rel_path = rel_base.trim_end_matches('/');

        // Properties with scaling (Doom units/tics to Godot meters/seconds)
        let health = actor.properties.get("Health")
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| "100".to_string());
        
        let speed_val: f32 = actor.properties.get("Speed")
            .and_then(|v| v.to_string_lossy().parse().ok())
            .unwrap_or(8.0) / 40.0 * 35.0; // Approx meters per second
        
        let pain_chance = actor.properties.get("PainChance")
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| "50".to_string());
        
        let damage = actor.properties.get("Damage")
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| "5".to_string());
        
        let meleerange_val: f32 = actor.properties.get("MeleeRange")
            .and_then(|v| v.to_string_lossy().parse().ok())
            .unwrap_or(64.0) / 40.0; // 64 units default
        
        let reactiontime_val: f32 = actor.properties.get("ReactionTime")
            .and_then(|v| v.to_string_lossy().parse().ok())
            .unwrap_or(8.0) as f32 / 35.0; // 8 tics default
        
        let radius = actor.properties.get("Radius")
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| "20.0".to_string());
        let height = actor.properties.get("Height")
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| "56.0".to_string());

        let r_val: f32 = radius.parse().unwrap_or(20.0) / 40.0;
        let h_val: f32 = height.parse().unwrap_or(56.0) / 40.0;

        // 1. Generate SpriteFrames
        let sprite_frames_path = actor_root.join(format!("{}_spriteframes.tres", enemy_name));
        let mut sf_content = String::from("[gd_resource type=\"SpriteFrames\" load_steps=2 format=3]\n\n");
        
        let mut ext_resources = Vec::new();
        let mut animations = Vec::new();
        let mut id_counter = 1;
        let mut texture_to_id = HashMap::new();

        let mut keys: Vec<_> = label_sprites.keys().collect();
        keys.sort();

        for anim_name in keys {
            let sprites = label_sprites.get(anim_name).unwrap();
            if sprites.is_empty() { continue; }

            if anim_name == "walk" || anim_name == "attack" {
                for i in 1..=5 {
                    let dir_sprites = Self::get_direction_sprites(sprites, i);
                    let active_sprites = if dir_sprites.is_empty() { sprites } else { &dir_sprites };
                    
                    let mut frames = Vec::new();
                    for (sprite_rel_path, duration) in active_sprites {
                        let id = texture_to_id.entry(sprite_rel_path.clone()).or_insert_with(|| {
                            let new_id = format!("{}_ext", id_counter);
                            ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, new_id));
                            id_counter += 1;
                            new_id
                        });
                        let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                    }

                    animations.push(format!(
r#"{{
"frames": [{}],
"loop": true,
"name": &"{}_{}",
"speed": 35.0
}}"#, frames.join(", "), anim_name, i));
                }
            } else if anim_name == "pain" {
                 let mut frames = Vec::new();
                 for (sprite_rel_path, duration) in sprites {
                    let id = texture_to_id.entry(sprite_rel_path.clone()).or_insert_with(|| {
                        let new_id = format!("{}_ext", id_counter);
                        ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, new_id));
                        id_counter += 1;
                        new_id
                    });
                    let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                 }
                 animations.push(format!(
r#"{{
"frames": [{}],
"loop": false,
"name": &"pain_1",
"speed": 35.0
}}"#, frames.join(", ")));
            } else if anim_name == "death" {
                 let mut frames = Vec::new();
                 for (sprite_rel_path, duration) in sprites {
                    let id = texture_to_id.entry(sprite_rel_path.clone()).or_insert_with(|| {
                        let new_id = format!("{}_ext", id_counter);
                        ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, new_id));
                        id_counter += 1;
                        new_id
                    });
                    let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                 }
                 animations.push(format!(
r#"{{
"frames": [{}],
"loop": false,
"name": &"death_1",
"speed": 35.0
}}"#, frames.join(", ")));
                 if !label_sprites.contains_key("xdeath") {
                     animations.push(format!(
r#"{{
"frames": [{}],
"loop": false,
"name": &"death_2",
"speed": 35.0
}}"#, frames.join(", ")));
                 }
            } else if anim_name == "xdeath" {
                 let mut frames = Vec::new();
                 for (sprite_rel_path, duration) in sprites {
                    let id = texture_to_id.entry(sprite_rel_path.clone()).or_insert_with(|| {
                        let new_id = format!("{}_ext", id_counter);
                        ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, new_id));
                        id_counter += 1;
                        new_id
                    });
                    let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                 }
                 animations.push(format!(
r#"{{
"frames": [{}],
"loop": false,
"name": &"death_2",
"speed": 35.0
}}"#, frames.join(", ")));
            } else {
                let mut frames = Vec::new();
                for (sprite_rel_path, duration) in sprites {
                    let id = texture_to_id.entry(sprite_rel_path.clone()).or_insert_with(|| {
                        let new_id = format!("{}_ext", id_counter);
                        ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, new_id));
                        id_counter += 1;
                        new_id
                    });
                    let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                }
                animations.push(format!(
r#"{{
"frames": [{}],
"loop": {},
"name": &"{}",
"speed": 35.0
}}"#, frames.join(", "), if anim_name == "idle" { "true" } else { "false" }, anim_name));
            }
        }

        sf_content.push_str(&ext_resources.join("\n"));
        sf_content.push_str("\n\n[resource]\nanimations = [");
        sf_content.push_str(&animations.join(", "));
        sf_content.push_str("]\n");
        let _ = fs::write(&sprite_frames_path, sf_content);

        // 2. Generate TSCN
        let tscn_path = actor_root.join(format!("{}.tscn", enemy_name));
        let tscn_content = format!(
r#"[gd_scene load_steps=6 format=3]

[ext_resource type="Script" path="res://enemies/grin/doom_enemy.gd" id="1_script"]
[ext_resource type="Script" path="res://enemies/enemy_sounds.gd" id="2_sounds"]
[ext_resource type="SpriteFrames" path="{}/{}_spriteframes.tres" id="3_sprites"]

[sub_resource type="CapsuleShape3D" id="CapsuleShape3D_1"]
radius = {:.4}
height = {:.4}

[node name="DoomEnemy" type="CharacterBody3D"]
floor_stop_on_slope = false
safe_margin = 0.5
script = ExtResource("1_script")
speed = {:.4}
meleerange = {:.4}
damage = {}
reactiontime = {:.4}
pain_chance = {}
health = {}

[node name="CollisionShape3D" type="CollisionShape3D" parent="."]
transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, {:.4}, 0)
shape = SubResource("CapsuleShape3D_1")

[node name="AnimatedSprite3D" type="AnimatedSprite3D" parent="."]
transform = Transform3D(2, 0, 0, 0, 2, 0, 0, 0, 2, 0, {:.4}, 0)
billboard = 2
shaded = true
texture_filter = 0
sprite_frames = ExtResource("3_sprites")

[node name="NavigationAgent3D" type="NavigationAgent3D" parent="."]
simplify_path = true

[node name="RayCast3D" type="RayCast3D" parent="."]

[node name="EnemySounds" type="AudioStreamPlayer3D" parent="."]
bus = &"SfxBus"
script = ExtResource("2_sounds")
death_folder = "{}/sounds/{}/death"
hurt_folder = "{}/sounds/{}/hurt"
taunt_folder = "{}/sounds/{}/taunt"
"#, 
            rel_path, enemy_name, 
            r_val, h_val,
            speed_val, meleerange_val, damage, reactiontime_val, pain_chance, health,
            h_val / 2.0, // collision y transform
            h_val / 2.0, // sprite y transform
            rel_path, enemy_name, rel_path, enemy_name, rel_path, enemy_name
        );
        let _ = fs::write(&tscn_path, tscn_content);
    }

    pub fn generate_projectile_resources(
        actor: &ActorDefinition,
        actor_root: &Path,
        rel_base: &str,
        label_sprites: &HashMap<String, Vec<(String, i32)>>,
    ) {
        let proj_name = actor.name.to_lowercase();
        let rel_path = rel_base.trim_end_matches('/');

        let radius = actor.properties.get("Radius")
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| "3.0".to_string());
        let height = actor.properties.get("Height")
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| "3.0".to_string());
        let speed = actor.properties.get("Speed")
            .map(|v| v.to_string_lossy())
            .unwrap_or_else(|| "20.0".to_string());
        
        let r_val: f32 = radius.parse().unwrap_or(3.0) / 40.0;
        let _h_val: f32 = height.parse().unwrap_or(3.0) / 40.0;

        // 1. Generate SpriteFrames
        let sprite_frames_path = actor_root.join(format!("{}_spriteframes.tres", proj_name));
        let mut sf_content = String::from("[gd_resource type=\"SpriteFrames\" load_steps=2 format=3]\n\n");
        let mut ext_resources = Vec::new();
        let mut animations = Vec::new();
        let mut id_counter = 1;

        let mut keys: Vec<_> = label_sprites.keys().collect();
        keys.sort();

        for anim_name in keys {
            let sprites = label_sprites.get(anim_name).unwrap();
            if sprites.is_empty() { continue; }
            
            let mut frames = Vec::new();
            for (sprite_rel_path, duration) in sprites {
                let id = format!("{}_ext", id_counter);
                ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, id));
                let dur = if *duration < 0 { 1.0 } else { *duration as f32 };
                frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                id_counter += 1;
            }
            
            animations.push(format!(
r#"{{
"frames": [{}],
"loop": {},
"name": &"{}",
"speed": 35.0
}}"#, frames.join(", "), if anim_name == "idle" || anim_name == "spawn" { "true" } else { "false" }, anim_name.to_lowercase()));
        }

        sf_content.push_str(&ext_resources.join("\n"));
        sf_content.push_str("\n\n[resource]\nanimations = [");
        sf_content.push_str(&animations.join(", "));
        sf_content.push_str("]\n");
        let _ = fs::write(&sprite_frames_path, sf_content);

        let autoplay_anim = if label_sprites.contains_key("spawn") {
            "spawn"
        } else if label_sprites.contains_key("death") {
            "death"
        } else {
            label_sprites.keys().next().map(|s| s.as_str()).unwrap_or("")
        };

        // 2. Generate TSCN
        let tscn_path = actor_root.join(format!("{}.tscn", proj_name));
        let tscn_content = format!(
r#"[gd_scene load_steps=5 format=3]

[ext_resource type="Script" path="res://weapons/projectiles/base_projectile.gd" id="1_script"]
[ext_resource type="SpriteFrames" path="{}/{}_spriteframes.tres" id="2_sprites"]

[sub_resource type="SphereShape3D" id="SphereShape3D_1"]
radius = {:.4}

[node name="Projectile" type="Area3D"]
script = ExtResource("1_script")
speed = {}
damage = 20

[node name="CollisionShape3D" type="CollisionShape3D" parent="."]
shape = SubResource("SphereShape3D_1")

[node name="AnimatedSprite3D" type="AnimatedSprite3D" parent="."]
transform = Transform3D(1.5, 0, 0, 0, 1.5, 0, 0, 0, 1.5, 0, 0, 0)
billboard = 2
shaded = true
texture_filter = 0
sprite_frames = ExtResource("2_sprites")
autoplay = "{}"

[node name="AudioStreamPlayer3D" type="AudioStreamPlayer3D" parent="."]
bus = &"SfxBus"
"#, rel_path, proj_name, r_val, speed, autoplay_anim);
        let _ = fs::write(&tscn_path, tscn_content);
    }
}
