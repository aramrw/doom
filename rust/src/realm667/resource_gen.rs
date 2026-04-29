use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use crate::realm667::actor::ActorDefinition;

fn with_atomic_gen<F>(actor_root: &Path, f: F) -> std::io::Result<()>
where F: FnOnce(&Path) -> std::io::Result<()>
{
    let temp_dir = actor_root.join("temp_gen");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    let result = f(&temp_dir);

    if result.is_ok() {
        for entry in fs::read_dir(&temp_dir)? {
            let entry = entry?;
            let dest = actor_root.join(entry.file_name());
            fs::rename(entry.path(), dest)?;
        }
    }

    let _ = fs::remove_dir_all(&temp_dir);
    result
}

pub struct ResourceGenerator;

impl ResourceGenerator {
    pub fn generate_weapon_resources(
        actor: &ActorDefinition, 
        actor_root: &Path, 
        rel_base: &str,
        label_sprites: &HashMap<String, Vec<(String, i32)>>,
        sounds_map: &HashMap<String, String>,
        mod_name: &str
    ) -> std::io::Result<()> {
        with_atomic_gen(actor_root, |target_dir| {
            let weapon_name = actor.name.to_lowercase();
            let rel_path = rel_base.trim_end_matches('/');
            
            // 1. Process Actions (States)
            let mut fire_steps = Vec::new();
            let mut fire_ext_resources = Vec::new();
            let mut step_id_counter = 1;

            if let Some(states) = actor.states.get("Fire") {
                for (frame_idx, frame) in states.iter().enumerate() {
                    if frame.actions.is_empty() {
                        continue;
                    }

                    let mut effect_ext_resources = Vec::new();
                    let mut effect_array_items = Vec::new();

                    for (action_idx, action) in frame.actions.iter().enumerate() {
                        let lower_name = action.name.to_lowercase();
                        let effect_filename = format!("{}_f{}_a{}_effect.tres", weapon_name, frame_idx, action_idx);
                        let effect_path = target_dir.join(&effect_filename);
                        
                        let (_script_path, effect_content) = if lower_name == "a_firebullets" || lower_name == "a_custompunch" || lower_name == "ca_quakeaxechop" {
                            let path = "res://weapons/effects/hitscan_effect.gd";
                            
                            let (damage, range) = if lower_name == "ca_quakeaxechop" {
                                 ("25".to_string(), "3.5".to_string())
                            } else if lower_name == "a_custompunch" {
                                 (action.args.get(0).map(|v| v.to_string_lossy()).unwrap_or_else(|| "5".to_string()), "3.5".to_string())
                            } else {
                                (action.args.get(0).map(|v| v.to_string_lossy()).unwrap_or_else(|| "5".to_string()), "100.0".to_string())
                            };
                            let spread = action.args.get(2).map(|v| v.to_string_lossy()).unwrap_or_else(|| "2.0".to_string());
                            let pellets = action.args.get(1).map(|v| v.to_string_lossy()).unwrap_or_else(|| "1".to_string());

                            (path, format!(
    r#"[gd_resource type="Resource" script_class="HitscanEffect" format=3]
    [ext_resource type="Script" path="{}" id="1_script"]
    [resource]
    script = ExtResource("1_script")
    damage = {}
    spread_angle = {}
    pellets = {}
    range_distance = {}
    "#, path, damage, spread, pellets, range))

                        } else if lower_name == "a_fireprojectile" || lower_name == "a_firecustommissile" || lower_name == "a_custommissile" {
                            let path = "res://weapons/effects/projectile_effect.gd";
                            
                            let p_name = action.args.get(0).map(|v| v.to_string_lossy().to_lowercase()).unwrap_or_else(|| "unknown".to_string());
                            let projectile_res_path = format!("{}/{}.tscn", rel_path, p_name);

                            (path, format!(
    r#"[gd_resource type="Resource" script_class="ProjectileEffect" format=3]
    [ext_resource type="Script" path="{}" id="1_script"]
    [ext_resource type="PackedScene" path="{}" id="2_proj"]
    [resource]
    script = ExtResource("1_script")
    projectile_scene = ExtResource("2_proj")
    speed = 50.0
    damage = 20
    "#, path, projectile_res_path))

                        } else if lower_name == "a_playsound" || lower_name == "a_startsound" || lower_name == "a_playweaponsound" {
                            let path = "res://weapons/effects/sound_effect.gd";
                            
                            let sound_alias = action.args.get(0).map(|v| v.to_string_lossy().to_uppercase()).unwrap_or_else(|| "NONE".to_string());
                            let mut sound_ext_res = "".to_string();
                            let mut sound_property = "".to_string();

                            if let Some(sound_file) = sounds_map.get(&sound_alias) {
                                let sound_ext = Path::new(sound_file).extension().unwrap_or_default().to_str().unwrap_or("ogg").to_lowercase();
                                let sound_filename = format!("{}.{}", sound_alias.to_lowercase().replace('/', "_"), sound_ext);
                                let sound_rel_res = format!("{}/sounds/{}/{}", rel_path, mod_name, sound_filename);
                                sound_ext_res = format!("[ext_resource type=\"AudioStream\" path=\"{}\" id=\"2_sound\"]\n", sound_rel_res);
                                sound_property = "sound = ExtResource(\"2_sound\")\n".to_string();
                            }

                            (path, format!(
    r#"[gd_resource type="Resource" script_class="SoundEffect" format=3]
    [ext_resource type="Script" path="{}" id="1_script"]
    {}
    [resource]
    script = ExtResource("1_script")
    {}pitch_randomness = 0.05
    "#, path, sound_ext_res, sound_property))

                        } else if lower_name == "a_quake" || lower_name == "a_recoil" {
                            let path = "res://weapons/effects/camera_shake_effect.gd";
                            
                            (path, format!(
    r#"[gd_resource type="Resource" script_class="CameraShakeEffect" format=3]
    [ext_resource type="Script" path="{}" id="1_script"]
    [resource]
    script = ExtResource("1_script")
    trauma_amount = 0.2
    "#, path))

                        } else if lower_name == "a_gunflash" {
                            let path = "res://weapons/effects/visual_effect.gd";
                            
                            (path, format!(
    r#"[gd_resource type="Resource" script_class="VisualEffect" format=3]
    [ext_resource type="Script" path="{}" id="1_script"]
    [resource]
    script = ExtResource("1_script")
    muzzle_flash = true
    "#, path))

                        } else {
                            let path = "res://weapons/effects/raw_zscript_effect.gd";
                            
                            let args_str = action.args.iter()
                                .map(|v| format!("\"{}\"", v.to_string_lossy()))
                                .collect::<Vec<_>>()
                                .join(", ");

                            (path, format!(
    r#"[gd_resource type="Resource" script_class="RawZScriptEffect" format=3]
    [ext_resource type="Script" path="{}" id="1_script"]
    [resource]
    script = ExtResource("1_script")
    function_name = "{}"
    arguments = [{}]
    "#, path, action.name, args_str))
                        };

                        if !effect_content.is_empty() {
                            fs::write(&effect_path, effect_content)?;
                            let res_id = format!("eff_{}_{}", frame_idx, action_idx);
                            effect_ext_resources.push(format!("[ext_resource type=\"Resource\" path=\"{}/{}\" id=\"{}\"]", rel_path, effect_filename, res_id));
                            effect_array_items.push(format!("ExtResource(\"{}\")", res_id));
                        }
                    }

                    if !effect_array_items.is_empty() {
                        let step_filename = format!("{}_f{}_step.tres", weapon_name, frame_idx);
                        let step_path = target_dir.join(&step_filename);
                        let step_id = format!("step_{}", step_id_counter);
                        
                        let step_content = format!(
    r#"[gd_resource type="Resource" script_class="ActionStep" format=3]
    [ext_resource type="Script" path="res://weapons/action_step.gd" id="1_script"]
    [ext_resource type="Script" path="res://weapons/effects/weapon_effect.gd" id="2_base"]
    {}
    [resource]
    script = ExtResource("1_script")
    frame_index = {}
    effects = Array[ExtResource("2_base")]([{}])
    "#, effect_ext_resources.join("\n"), frame_idx, effect_array_items.join(", "));
                        
                        fs::write(&step_path, step_content)?;
                        fire_ext_resources.push(format!("[ext_resource type=\"Resource\" path=\"{}/{}\" id=\"{}\"]", rel_path, step_filename, step_id));
                        fire_steps.push(format!("ExtResource(\"{}\")", step_id));
                        step_id_counter += 1;
                    }
                }
            }

            // 3. Generate WeaponAction
            let consumes_ammo = !actor.flags.contains(&"WEAPON.MELEEWEAPON".to_string());
            let action_path = target_dir.join(format!("{}_fire_action.tres", weapon_name));
            let action_content = format!(
    r#"[gd_resource type="Resource" script_class="WeaponAction" format=3]
    [ext_resource type="Script" path="res://weapons/weapon_action.gd" id="1_script"]
    {}
    [resource]
    script = ExtResource("1_script")
    animation_name = "shoot"
    steps = [{}]
    loop = false
    consumes_ammo = {}
    "#, fire_ext_resources.join("\n"), fire_steps.join(", "), consumes_ammo);
            fs::write(&action_path, action_content)?;

            // 4. Generate SpriteFrames
            let sprite_frames_path = target_dir.join(format!("{}_spriteframes.tres", weapon_name));
            let mut sf_content = String::from("[gd_resource type=\"SpriteFrames\" format=3]\n\n");
            
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
                    let dur = if *duration <= 0 { 1.0 } else { *duration as f32 };
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
            
            fs::write(&sprite_frames_path, sf_content)?;

            // 5. Generate WeaponData
            let wpdata_path = target_dir.join(format!("{}_wpdata.tres", weapon_name));
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
            
            fs::write(&wpdata_path, wpdata_content)?;

            // 6. Generate Pickup Scene
            let pickup_path = target_dir.join(format!("{}_pickup.tscn", weapon_name));
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
            
            fs::write(&pickup_path, pickup_content)?;
            Ok(())
        })
    }

    fn get_direction_sprites(sprites: &[(String, i32)], dir: usize) -> Vec<(String, i32)> {
        let dir_char = std::char::from_digit(dir as u32, 10).unwrap();
        sprites.iter().filter(|(s, _)| {
            let stem = Path::new(s).file_stem().unwrap_or_default().to_str().unwrap_or_default().to_uppercase();
            if stem.ends_with('0') { return true; }
            
            // Standard Doom sprite: PREFIX + FRAME + ROTATION (e.g., XXXXA1 or XXXXA2A8)
            // or multi-frame: XXXXA1B1 (A=frame1, 1=rot1, B=frame2, 1=rot2)
            if stem.len() >= 6 {
                // Check all characters from index 5 onwards for the direction digit
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
    ) -> std::io::Result<()> {
        with_atomic_gen(actor_root, |target_dir| {
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

            let use_gravity = !actor.flags.contains(&"NOGRAVITY".to_string());

            // 0. Process ActionSteps (similar to weapons)
            let mut action_ext_resources = Vec::new();
            let mut step_id_counter = 1;
            let mut label_to_steps = HashMap::new();

            // Process all labels
            for (label, frames) in &actor.states {
                let mut steps = Vec::new();
                for (frame_idx, frame) in frames.iter().enumerate() {
                    if frame.actions.is_empty() {
                        continue;
                    }

                    let mut effect_ext_resources = Vec::new();
                    let mut effect_array_items = Vec::new();

                    for (action_idx, action) in frame.actions.iter().enumerate() {
                        let _lower_name = action.name.to_lowercase();
                        // If it's a custom method call, we should expand it into its constituent actions
                        let mut actual_actions = Vec::new();
                        if let Some(method_body) = actor.methods.get(&action.name) {
                            actual_actions.extend(method_body.clone());
                        } else {
                            actual_actions.push(action.clone());
                        }

                        for (sub_idx, sub_action) in actual_actions.iter().enumerate() {
                            let sub_lower_name = sub_action.name.to_lowercase();
                            let effect_filename = format!("{}_{}_f{}_a{}_s{}_effect.tres", enemy_name, label.to_lowercase(), frame_idx, action_idx, sub_idx);
                            let effect_path = target_dir.join(&effect_filename);

                            let (_script_path, effect_content) = if sub_lower_name == "a_spawnprojectile" || sub_lower_name == "a_fireprojectile" || sub_lower_name == "a_firecustommissile" {
                                 let path = "res://weapons/effects/projectile_effect.gd";
                                 let p_name = sub_action.args.get(0).map(|v| v.to_string_lossy().to_lowercase()).unwrap_or_else(|| "unknown".to_string());
                                 let projectile_res_path = format!("{}/{}.tscn", rel_path, p_name);

                                 (path, format!(
    r#"[gd_resource type="Resource" script_class="ProjectileEffect" format=3]
    [ext_resource type="Script" path="{}" id="1_script"]
    [ext_resource type="PackedScene" path="{}" id="2_proj"]
    [resource]
    script = ExtResource("1_script")
    projectile_scene = ExtResource("2_proj")
    speed = 15.0
    damage = 20
    "#, path, projectile_res_path))
                            } else if sub_lower_name == "a_playsound" || sub_lower_name == "a_startsound" {
                                let path = "res://weapons/effects/sound_effect.gd";
                                let sound_alias = sub_action.args.get(0).map(|v| v.to_string_lossy().to_uppercase()).unwrap_or_else(|| "NONE".to_string());
                                // For enemies, sounds are already extracted to sounds/mod/alias.ogg
                                let sound_rel_res = format!("{}/sounds/{}/{}.ogg", rel_path, enemy_name, sound_alias.to_lowercase());
                                
                                (path, format!(
    r#"[gd_resource type="Resource" script_class="SoundEffect" format=3]
    [ext_resource type="Script" path="{}" id="1_script"]
    [ext_resource type="AudioStream" path="{}" id="2_sound"]
    [resource]
    script = ExtResource("1_script")
    sound = ExtResource("2_sound")
    pitch_randomness = 0.05
    "#, path, sound_rel_res))
                            } else {
                                // Raw fallback
                                 let path = "res://weapons/effects/raw_zscript_effect.gd";
                                 let args_str = sub_action.args.iter()
                                    .map(|v| format!("\"{}\"", v.to_string_lossy()))
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                 (path, format!(
    r#"[gd_resource type="Resource" script_class="RawZScriptEffect" format=3]
    [ext_resource type="Script" path="{}" id="1_script"]
    [resource]
    script = ExtResource("1_script")
    function_name = "{}"
    arguments = [{}]
    "#, path, sub_action.name, args_str))
                            };

                            fs::write(&effect_path, effect_content)?;
                            let res_id = format!("eff_{}_{}_{}_{}", label.to_lowercase(), frame_idx, action_idx, sub_idx);
                            effect_ext_resources.push(format!("[ext_resource type=\"Resource\" path=\"{}/{}\" id=\"{}\"]", rel_path, effect_filename, res_id));
                            effect_array_items.push(format!("ExtResource(\"{}\")", res_id));
                        }
                    }

                    if !effect_array_items.is_empty() {
                        let step_filename = format!("{}_{}_f{}_step.tres", enemy_name, label.to_lowercase(), frame_idx);
                        let step_path = target_dir.join(&step_filename);
                        let step_id = format!("step_{}", step_id_counter);
                        
                        let step_content = format!(
    r#"[gd_resource type="Resource" script_class="ActionStep" format=3]
    [ext_resource type="Script" path="res://weapons/action_step.gd" id="1_script"]
    [ext_resource type="Script" path="res://weapons/effects/weapon_effect.gd" id="2_base"]
    {}
    [resource]
    script = ExtResource("1_script")
    frame_index = {}
    effects = Array[ExtResource("2_base")]([{}])
    "#, effect_ext_resources.join("\n"), frame_idx, effect_array_items.join(", "));
                        
                        fs::write(&step_path, step_content)?;
                        action_ext_resources.push(format!("[ext_resource type=\"Resource\" path=\"{}/{}\" id=\"{}\"]", rel_path, step_filename, step_id));
                        steps.push(format!("ExtResource(\"{}\")", step_id));
                        step_id_counter += 1;
                    }
                }
                if !steps.is_empty() {
                    label_to_steps.insert(label.clone(), steps);
                }
            }

            // 1. Generate SpriteFrames
            let sprite_frames_path = target_dir.join(format!("{}_spriteframes.tres", enemy_name));
            let mut sf_content = String::from("[gd_resource type=\"SpriteFrames\" format=3]\n\n");
            
            let mut ext_resources = Vec::new();
            let mut animations = Vec::new();
            let mut id_counter = 1;
            let mut texture_to_id = HashMap::new();

            let mut keys: Vec<_> = label_sprites.keys().collect();
            keys.sort();

            for anim_name in keys {
                let sprites = label_sprites.get(anim_name).unwrap();
                if sprites.is_empty() { continue; }

                if anim_name == "walk" || anim_name == "attack" || anim_name == "idle" || anim_name == "raise" {
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
                            let dur = if *duration <= 0 { 1.0 } else { *duration as f32 };
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
                        let dur = if *duration <= 0 { 1.0 } else { *duration as f32 };
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
                        let dur = if *duration <= 0 { 1.0 } else { *duration as f32 };
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
                        let dur = if *duration <= 0 { 1.0 } else { *duration as f32 };
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
                        let dur = if *duration <= 0 { 1.0 } else { *duration as f32 };
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
            fs::write(&sprite_frames_path, sf_content)?;

            let autoplay_anim = if label_sprites.contains_key("idle") {
                "idle_1"
            } else if label_sprites.contains_key("walk") {
                "walk_1"
            } else {
                label_sprites.keys().next().map(|s| s.as_str()).unwrap_or("")
            };

            // 1.5 Generate WeaponActions for the enemy
            let mut action_tres_ext = Vec::new();
            let mut action_props = Vec::new();
            
            let action_labels = ["Missile", "Melee"];
            for label in action_labels {
                if let Some(steps) = label_to_steps.get(label) {
                    let action_filename = format!("{}_{}_action.tres", enemy_name, label.to_lowercase());
                    let action_path = target_dir.join(&action_filename);
                    let action_id = format!("action_{}", label.to_lowercase());
                    
                    let action_content = format!(
    r#"[gd_resource type="Resource" script_class="WeaponAction" format=3]
    [ext_resource type="Script" path="res://weapons/weapon_action.gd" id="1_script"]
    {}
    [resource]
    script = ExtResource("1_script")
    animation_name = "attack"
    steps = [{}]
    loop = false
    consumes_ammo = false
    "#, action_ext_resources.join("\n"), steps.join(", "));
                    
                    fs::write(&action_path, action_content)?;
                    action_tres_ext.push(format!("[ext_resource type=\"Resource\" path=\"{}/{}\" id=\"{}\"]", rel_path, action_filename, action_id));
                    action_props.push(format!("{}_action = ExtResource(\"{}\")", label.to_lowercase(), action_id));
                }
            }

            // 2. Generate TSCN
            let tscn_path = target_dir.join(format!("{}.tscn", enemy_name));
            let tscn_content = format!(
    r#"[gd_scene load_steps=6 format=3]

    [ext_resource type="Script" path="res://enemies/grin/doom_enemy.gd" id="1_script"]
    [ext_resource type="Script" path="res://enemies/enemy_sounds.gd" id="2_sounds"]
    [ext_resource type="SpriteFrames" path="{}/{}_spriteframes.tres" id="3_sprites"]
    {}

    [sub_resource type="CapsuleShape3D" id="CapsuleShape3D_1"]
    radius = {:.4}
    height = {:.4}

    [node name="{}" type="CharacterBody3D" groups=["Enemies"]]
    floor_stop_on_slope = false
    safe_margin = 0.5
    script = ExtResource("1_script")
    speed = {:.4}
    meleerange = {:.4}
    damage = {}
    reactiontime = {:.4}
    pain_chance = {}
    health = {}
    use_gravity = {}
    {}

    [node name="CollisionShape3D" type="CollisionShape3D" parent="."]
    transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, {:.4}, 0)
    shape = SubResource("CapsuleShape3D_1")

    [node name="AnimatedSprite3D" type="AnimatedSprite3D" parent="."]
    transform = Transform3D(2, 0, 0, 0, 2, 0, 0, 0, 2, 0, {:.4}, 0)
    billboard = 2
    shaded = true
    texture_filter = 0
    sprite_frames = ExtResource("3_sprites")
    autoplay = "{}"

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
                action_tres_ext.join("\n"),
                r_val, h_val,
                enemy_name,
                speed_val, meleerange_val, damage, reactiontime_val, pain_chance, health,
                use_gravity,
                action_props.join("\n"),
                h_val / 2.0, // collision y transform
                h_val / 2.0, // sprite y transform
                autoplay_anim,
                rel_path, enemy_name, rel_path, enemy_name, rel_path, enemy_name
            );
            fs::write(&tscn_path, tscn_content)?;
            Ok(())
        })
    }

    pub fn generate_projectile_resources(
        actor: &ActorDefinition,
        actor_root: &Path,
        rel_base: &str,
        label_sprites: &HashMap<String, Vec<(String, i32)>>,
    ) -> std::io::Result<()> {
        with_atomic_gen(actor_root, |target_dir| {
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
            let sprite_frames_path = target_dir.join(format!("{}_spriteframes.tres", proj_name));
            let mut sf_content = String::from("[gd_resource type=\"SpriteFrames\" format=3]\n\n");
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
                    let dur = if *duration <= 0 { 1.0 } else { *duration as f32 };
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
            fs::write(&sprite_frames_path, sf_content)?;

            let autoplay_anim = if label_sprites.contains_key("spawn") {
                "spawn"
            } else if label_sprites.contains_key("death") {
                "death"
            } else {
                label_sprites.keys().next().map(|s| s.as_str()).unwrap_or("")
            };

            // 2. Generate TSCN
            let tscn_path = target_dir.join(format!("{}.tscn", proj_name));
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
            fs::write(&tscn_path, tscn_content)?;
            Ok(())
        })
    }

    pub fn generate_prop_resources(
        actor: &ActorDefinition,
        actor_root: &Path,
        rel_base: &str,
        label_sprites: &HashMap<String, Vec<(String, i32)>>,
    ) -> std::io::Result<()> {
        with_atomic_gen(actor_root, |target_dir| {
            let prop_name = actor.name.to_lowercase();
            let rel_path = rel_base.trim_end_matches('/');

            let radius = actor.properties.get("Radius")
                .map(|v| v.to_string_lossy())
                .unwrap_or_else(|| "16.0".to_string());
            let height = actor.properties.get("Height")
                .map(|v| v.to_string_lossy())
                .unwrap_or_else(|| "64.0".to_string());
            let is_solid = actor.flags.contains(&"Solid".to_string());

            let r_val: f32 = radius.parse().unwrap_or(16.0) / 40.0;
            let h_val: f32 = height.parse().unwrap_or(64.0) / 40.0;

            // 1. Generate SpriteFrames
            let sprite_frames_path = target_dir.join(format!("{}_spriteframes.tres", prop_name));
            let mut sf_content = String::from("[gd_resource type=\"SpriteFrames\" format=3]\n\n");
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
                    let dur = if *duration <= 0 { 1.0 } else { *duration as f32 };
                    frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                    id_counter += 1;
                }
                
                animations.push(format!(
    r#"{{
    "frames": [{}],
    "loop": true,
    "name": &"{}",
    "speed": 35.0
    }}"#, frames.join(", "), anim_name.to_lowercase()));
            }

            sf_content.push_str(&ext_resources.join("\n"));
            sf_content.push_str("\n\n[resource]\nanimations = [");
            sf_content.push_str(&animations.join(", "));
            sf_content.push_str("]\n");
            fs::write(&sprite_frames_path, sf_content)?;

            let autoplay_anim = if label_sprites.contains_key("idle") {
                "idle".to_string()
            } else if label_sprites.contains_key("spawn") {
                "spawn".to_string()
            } else if !label_sprites.is_empty() {
                label_sprites.keys().next().unwrap().to_lowercase()
            } else {
                "default".to_string()
            };

            // 2. Generate TSCN
            let tscn_path = target_dir.join(format!("{}.tscn", prop_name));
            
            let mut tscn_content = format!(
    r#"[gd_scene load_steps=5 format=3]

    [ext_resource type="Script" path="res://assets/base_prop.gd" id="1_script"]
    [ext_resource type="SpriteFrames" path="{}/{}_spriteframes.tres" id="2_sprites"]

    [sub_resource type="CapsuleShape3D" id="CapsuleShape3D_1"]
    radius = {:.4}
    height = {:.4}

    [node name="{}" type="StaticBody3D"]
    script = ExtResource("1_script")
    initial_state = "{}"
    "#, rel_path, prop_name, r_val, h_val, actor.name, autoplay_anim);

            if is_solid {
                tscn_content.push_str(&format!(
    r#"
    [node name="CollisionShape3D" type="CollisionShape3D" parent="."]
    transform = Transform3D(1, 0, 0, 0, 1, 0, 0, 0, 1, 0, {:.4}, 0)
    shape = SubResource("CapsuleShape3D_1")
    "#, h_val / 2.0));
            }

            // Add 4 sprites for volumetric effect
            for i in 0..4 {
                let rot = (i as f32) * 45.0;
                tscn_content.push_str(&format!(
    r#"
    [node name="Sprite3D_{}" type="AnimatedSprite3D" parent="."]
    transform = Transform3D({:.4}, 0, {:.4}, 0, 1, 0, {:.4}, 0, {:.4}, 0, {:.4}, 0)
    shaded = true
    texture_filter = 0
    sprite_frames = ExtResource("2_sprites")
    autoplay = "{}"
    "#, 
                    i, 
                    (rot.to_radians()).cos(), (rot.to_radians()).sin(),
                    -(rot.to_radians()).sin(), (rot.to_radians()).cos(),
                    h_val / 2.0,
                    autoplay_anim
                ));
            }

            fs::write(&tscn_path, tscn_content)?;
            Ok(())
        })
    }

    pub fn generate_item_resources(
        actor: &ActorDefinition,
        actor_root: &Path,
        rel_base: &str,
        label_sprites: &HashMap<String, Vec<(String, i32)>>,
    ) -> std::io::Result<()> {
        with_atomic_gen(actor_root, |target_dir| {
            let item_name = actor.name.to_lowercase();
            let rel_path = rel_base.trim_end_matches('/');

            // 1. Generate SpriteFrames
            let sprite_frames_path = target_dir.join(format!("{}_spriteframes.tres", item_name));
            let mut sf_content = String::from("[gd_resource type=\"SpriteFrames\" format=3]\n\n");
            let mut ext_resources = Vec::new();
            let mut animations = Vec::new();
            let mut id_counter = 1;

            let mut keys: Vec<_> = label_sprites.keys().collect();
            keys.sort();

            let mut first_sprite_path = String::new();

            for anim_name in keys {
                let sprites = label_sprites.get(anim_name).unwrap();
                if sprites.is_empty() { continue; }
                
                let mut frames = Vec::new();
                for (sprite_rel_path, duration) in sprites {
                    if first_sprite_path.is_empty() {
                        first_sprite_path = sprite_rel_path.clone();
                    }
                    let id = format!("{}_ext", id_counter);
                    ext_resources.push(format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"{}\"]", sprite_rel_path, id));
                    let dur = if *duration <= 0 { 1.0 } else { *duration as f32 };
                    frames.push(format!("{{\n\"duration\": {},\n\"texture\": ExtResource(\"{}\")\n}}", dur, id));
                    id_counter += 1;
                }
                
                animations.push(format!(
    r#"{{
    "frames": [{}],
    "loop": true,
    "name": &"{}",
    "speed": 5.0
    }}"#, frames.join(", "), anim_name.to_lowercase()));
            }

            sf_content.push_str(&ext_resources.join("\n"));
            sf_content.push_str("\n\n[resource]\nanimations = [");
            sf_content.push_str(&animations.join(", "));
            sf_content.push_str("]\n");
            fs::write(&sprite_frames_path, sf_content)?;

            let autoplay_anim = if label_sprites.contains_key("idle") {
                "idle".to_string()
            } else if label_sprites.contains_key("ground") {
                "ground".to_string()
            } else if !label_sprites.is_empty() {
                label_sprites.keys().next().unwrap().to_lowercase()
            } else {
                "default".to_string()
            };

            // 2. Generate InventoryItemData (Resource)
            let item_res_path = target_dir.join(format!("{}_item.tres", item_name));
            let mut item_res_content = format!(
    r#"[gd_resource type="Resource" script_class="InventoryItemData" load_steps=3 format=3]

    [ext_resource type="Script" path="res://items/inventory_item_data.gd" id="1_script"]
    [ext_resource type="SpriteFrames" path="{}/{}_spriteframes.tres" id="2_sprites"]
    "# , rel_path, item_name);

            if !first_sprite_path.is_empty() {
                 item_res_content.push_str(&format!("[ext_resource type=\"Texture2D\" path=\"{}\" id=\"3_icon\"]\n", first_sprite_path));
            }

            item_res_content.push_str(&format!(
    r#"
    [resource]
    script = ExtResource("1_script")
    item_name = "{}"
    sprite_frames = ExtResource("2_sprites")
    "# , actor.name));

            if !first_sprite_path.is_empty() {
                item_res_content.push_str("icon = ExtResource(\"3_icon\")\n");
            }

            fs::write(&item_res_path, item_res_content)?;

            // 3. Generate Pickup Scene
            let pickup_path = target_dir.join(format!("{}_pickup.tscn", item_name));
            let pickup_content = format!(
    r#"[gd_scene load_steps=5 format=3]

    [ext_resource type="Script" path="res://items/pickup.gd" id="1_pickup"]
    [ext_resource type="Resource" path="{}/{}_item.tres" id="2_data"]
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
    autoplay = "{}"
    "#, rel_path, item_name, rel_path, item_name, autoplay_anim);
            
            fs::write(&pickup_path, pickup_content)?;
            Ok(())
        })
    }
}
