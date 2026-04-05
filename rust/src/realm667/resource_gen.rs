use std::path::Path;
use std::fs;
use crate::realm667::actor::ActorDefinition;

pub struct ResourceGenerator;

impl ResourceGenerator {
    pub fn generate_weapon_resources(actor: &ActorDefinition, actor_root: &Path) {
        let weapon_name = actor.name.to_lowercase();
        let rel_path = format!("res://weapons/{}", weapon_name); // Simplified for now
        
        // 1. Generate Effects
        let effect_path = actor_root.join(format!("{}_fire_effect.tres", weapon_name));
        let effect_content = 
r#"[gd_resource type="Resource" script_class="HitscanEffect" format=3]
[ext_resource type="Script" path="res://weapons/effects/hitscan_effect.gd" id="1_script"]
[resource]
script = ExtResource("1_script")
damage = 15
spread_angle = 2.0
"#;
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
loop = true
consumes_ammo = true
"#, rel_path, weapon_name);
        let _ = fs::write(&action_path, action_content);

        // 4. Generate SpriteFrames
        let sprite_frames_path = actor_root.join(format!("{}_spriteframes.tres", weapon_name));
        let sprite_frames_content = 
r#"[gd_resource type="SpriteFrames" format=3]
[resource]
animations = [{
"frames": [],
"loop": true,
"name": &"idle",
"speed": 5.0
}, {
"frames": [],
"loop": false,
"name": &"reload",
"speed": 5.0
}, {
"frames": [],
"loop": false,
"name": &"shoot",
"speed": 10.0
}]
"#;
        let _ = fs::write(&sprite_frames_path, sprite_frames_content);

        // 5. Generate WeaponData (The Final Product)
        let wpdata_path = actor_root.join(format!("{}_wpdata.tres", weapon_name));
        let wpdata_content = format!(
r#"[gd_resource type="Resource" script_class="WeaponData" format=3]
[ext_resource type="Script" path="res://weapons/weapon_data.gd" id="1_script"]
[ext_resource type="SpriteFrames" path="{}/{}_spriteframes.tres" id="2_sprites"]
[ext_resource type="Resource" path="{}/{}_fire_action.tres" id="3_fire"]
[resource]
script = ExtResource("1_script")
weapon_name = "{}"
sprite_frames = ExtResource("2_sprites")
actions = {{ "primary": ExtResource("3_fire") }}
max_bullets = {}
"#, rel_path, weapon_name, rel_path, weapon_name, weapon_name, 
            actor.properties.get("Weapon.AmmoGive").unwrap_or(&"30".to_string()));
        
        let _ = fs::write(&wpdata_path, wpdata_content);
    }
}
