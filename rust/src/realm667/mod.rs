pub mod actor;
pub mod asset_handler;
pub mod nom_parser;
pub mod resource_gen;
pub mod parser_tests;
pub mod hell_apprentice_test;
pub mod cultist_integration_test;

use crate::realm667::actor::{ActorCategory, ActorDefinition};
use crate::realm667::asset_handler::AssetHandler;
use crate::realm667::nom_parser::{parse_document, parse_sndinfo};
use crate::realm667::resource_gen::ResourceGenerator;
use godot::classes::{ProjectSettings, EditorInterface, Engine};
use godot::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[derive(GodotConvert, Export, Default, Var, PartialEq, Eq, Clone, Copy, Debug)]
#[godot(via = i32)]
pub enum ImportMode {
    #[default]
    Automatic = 0,
    Weapon = 1,
    Enemy = 2,
    Prop = 3,
    Projectile = 4,
    Item = 5,
}

#[derive(GodotClass)]
#[class(tool, base=Node)]
pub struct Realm667Importer {
    #[export]
    #[var(hint = FILE_PATH, hint_string = "*.pk3,*.zip")]
    pk3_path: GString,

    #[export]
    import_mode: ImportMode,

    #[export]
    trigger_import: bool,

    base: Base<Node>,
}

#[godot_api]
impl INode for Realm667Importer {
    fn init(base: Base<Node>) -> Self {
        Self {
            pk3_path: GString::from(""),
            import_mode: ImportMode::Automatic,
            trigger_import: false,
            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_process(true);
        godot_print!("Realm667Importer: Node Ready.");
    }

    fn process(&mut self, _delta: f64) {
        if self.trigger_import {
            self.trigger_import = false;
            let path = self.pk3_path.to_string();
            self.import_pk3(path);
        }
    }
}

#[godot_api]
impl Realm667Importer {
    #[func]
    pub fn import_pk3(&self, path: String) {
        if path.is_empty() {
            godot_warn!("Realm667Importer: PK3 path is empty!");
            return;
        }

        let abs_pk3_path_gs = ProjectSettings::singleton().globalize_path(&path);
        let abs_pk3_path_str = abs_pk3_path_gs.to_string();
        let pk3_path_obj = Path::new(&abs_pk3_path_str);

        let file = match File::open(&abs_pk3_path_str) {
            Ok(f) => f,
            Err(e) => {
                godot_error!("Realm667Importer: Failed to open PK3: {}", e);
                return;
            }
        };

        let mut archive = match ZipArchive::new(file) {
            Ok(a) => a,
            Err(e) => {
                godot_error!("Realm667Importer: Failed to read ZIP: {}", e);
                return;
            }
        };

        // 1. Pre-parse scripts
        let mut script_content = String::new();
        let main_scripts = ["zscript.txt", "zscript", "decorate.txt", "decorate"];
        for main_name in main_scripts {
            let mut visited = HashSet::new();
            if let Some(content) = self.read_zip_file_recursive(&mut archive, main_name, &mut visited) {
                script_content = content;
                break;
            }
        }

        if script_content.is_empty() {
            godot_warn!("Realm667Importer: No DECORATE/ZSCRIPT found.");
            return;
        }

        let mut actors = match parse_document(&script_content) {
            Ok(a) => a,
            Err(e) => {
                godot_error!("Realm667Importer: Parse error: {}", e);
                return;
            }
        };

        if actors.is_empty() {
            godot_warn!("Realm667Importer: No actors found in scripts.");
            return;
        }

        // 1.5 Resolve Inheritance
        let actor_map: HashMap<String, usize> = actors.iter().enumerate()
            .map(|(i, a)| (a.name.to_lowercase(), i))
            .collect();

        let mut actors_to_update = Vec::new();
        for i in 0..actors.len() {
            if let Some(ref parent_name) = actors[i].parent {
                if let Some(&parent_idx) = actor_map.get(&parent_name.to_lowercase()) {
                    actors_to_update.push((i, parent_idx));
                }
            }
        }

        for (child_idx, parent_idx) in actors_to_update {
            if child_idx == parent_idx { continue; }
            
            let (child, parent) = if child_idx < parent_idx {
                let (left, right) = actors.split_at_mut(parent_idx);
                (&mut left[child_idx], &right[0])
            } else {
                let (left, right) = actors.split_at_mut(child_idx);
                (&mut right[0], &left[parent_idx])
            };

            // Copy properties if not present in child
            for (k, v) in &parent.properties {
                child.properties.entry(k.clone()).or_insert_with(|| v.clone());
            }
            // Copy flags if not present in child
            for flag in &parent.flags {
                if !child.flags.contains(flag) {
                    child.flags.push(flag.clone());
                }
            }
            // Copy states if not present in child
            for (k, v) in &parent.states {
                child.states.entry(k.clone()).or_insert_with(|| v.clone());
            }
        }

        // Primary actor for the top-level folder name
        let _primary_actor = if self.import_mode == ImportMode::Automatic {            actors
                .iter()
                .find(|a: &&ActorDefinition| {
                    matches!(
                        a.determine_category(),
                        ActorCategory::Weapon | ActorCategory::Enemy
                    )
                })
                .unwrap_or(&actors[0])
        } else {
            // If mode is forced, use the first actor of that type if possible
            let target_category = match self.import_mode {
                ImportMode::Weapon => ActorCategory::Weapon,
                ImportMode::Enemy => ActorCategory::Enemy,
                ImportMode::Prop => ActorCategory::Prop,
                ImportMode::Projectile => ActorCategory::Projectile,
                ImportMode::Item => ActorCategory::Item,
                _ => ActorCategory::Unknown,
            };
            
            actors.iter()
                .find(|a| a.determine_category() == target_category)
                .unwrap_or(&actors[0])
        };

        let mod_folder_name = pk3_path_obj.file_stem().unwrap().to_string_lossy().replace(' ', "_").to_lowercase();
        let parent_dir = pk3_path_obj.parent().unwrap_or(Path::new(""));

        // Output directory is parent/mod_folder_name
        let abs_output_base = parent_dir.join(&mod_folder_name);
        let _ = fs::create_dir_all(&abs_output_base);

        let mut parent_res_path = if let Some(pos) = path.rfind('/') {
            path[..pos].to_string()
        } else {
            "res://".to_string()
        };
        if !parent_res_path.starts_with("res://") {
            parent_res_path = format!("res://{}", parent_res_path.trim_start_matches('/'));
        }
        let res_output_base = format!("{}/{}", parent_res_path.trim_end_matches('/'), mod_folder_name);

        godot_print!("--------------------------------------------------");
        godot_print!("Realm667Importer: STARTING CLEAN IMPORT");
        godot_print!("Source: {}", abs_pk3_path_str);
        godot_print!("Mod Folder: {}", abs_output_base.display());

        // 2. Extract RAW files to Mod Folder
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            if zip_file.is_dir() {
                continue;
            }
            let name = zip_file.name().replace('\\', "/");
            let dest_path = abs_output_base.join(&name);
            if let Some(parent) = dest_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let mut content = Vec::new();
            let _ = zip_file.read_to_end(&mut content);
            let mut out_file = match File::create(&dest_path) {
                Ok(f) => f,
                Err(_) => continue,
            };
            let _ = out_file.write_all(&content);
        }

        // 3. Process SNDINFO
        let mut sounds_map = HashMap::new();
        let sndinfo_names = ["sndinfo.txt", "sndinfo"];
        for snd_name in sndinfo_names {
            let mut visited = HashSet::new();
            if let Some(content) = self.read_zip_file_recursive(&mut archive, snd_name, &mut visited) {
                sounds_map = parse_sndinfo(&content);
                break;
            }
        }

        // 4. Organize each actor into "godot_data"
        for actor in actors {
            let actor: ActorDefinition = actor;

            // Respect forced import mode if not Automatic
            let category = match self.import_mode {
                ImportMode::Weapon => {
                    let det = actor.determine_category();
                    if det == ActorCategory::Projectile {
                        ActorCategory::Projectile
                    } else {
                        ActorCategory::Weapon
                    }
                }
                ImportMode::Enemy => {
                    let det = actor.determine_category();
                    if det == ActorCategory::Projectile {
                        ActorCategory::Projectile
                    } else {
                        ActorCategory::Enemy
                    }
                }
                ImportMode::Prop => ActorCategory::Prop,
                ImportMode::Projectile => ActorCategory::Projectile,
                ImportMode::Item => ActorCategory::Item,
                ImportMode::Automatic => actor.determine_category(),
            };

            if category == ActorCategory::Unknown {
                godot_print!("Realm667Importer: Skipping unknown actor: {}", actor.name);
                continue;
            }

            godot_print!(
                "Realm667Importer: Processing {:?}: {}",
                category,
                actor.name
            );

            // One godot_data folder for the whole mod
            let godot_data_root = abs_output_base.join("godot_data");
            let godot_data_res = format!("{}/godot_data", res_output_base);

            // Extract and categorize sprites for this actor
            let label_sprites = self.extract_actor_assets_clean(
                &actor,
                category,
                &mut archive,
                &godot_data_root,
                &godot_data_res,
                &sounds_map,
                &mod_folder_name,
            );

            if category == ActorCategory::Weapon {
                godot_print!("Realm667Importer: Weapon states for {}: {:?}", actor.name, actor.states.keys());
                if let Some(fire_states) = actor.states.get("Fire") {
                    for (i, frame) in fire_states.iter().enumerate() {
                        let frame: &crate::realm667::actor::StateFrame = frame;
                        godot_print!("  Frame {}: prefix={}, actions={:?}", i, frame.sprite_prefix, frame.actions);
                    }
                }
                let _ = ResourceGenerator::generate_weapon_resources(
                    &actor,
                    &godot_data_root,
                    &godot_data_res,
                    &label_sprites,
                    &sounds_map,
                    &mod_folder_name,
                );
                godot_print!("Realm667Importer: Created Godot data for weapon {}", actor.name);
            } else if category == ActorCategory::Enemy {
                if let Err(e) = ResourceGenerator::generate_enemy_resources(
                    &actor,
                    &godot_data_root,
                    &godot_data_res,
                    &label_sprites,
                    &sounds_map,
                    &mod_folder_name,
                ) {
                    godot_error!("Failed to generate enemy resources: {}", e);
                    return;
                }
                godot_print!("Realm667Importer: Created Godot data for enemy {}", actor.name);
            } else if category == ActorCategory::Prop {
                let _ = ResourceGenerator::generate_prop_resources(
                    &actor,
                    &godot_data_root,
                    &godot_data_res,
                    &label_sprites,
                );
                godot_print!("Realm667Importer: Created Godot data for prop {}", actor.name);
            } else if category == ActorCategory::Item || category == ActorCategory::Ammo {
                let _ = ResourceGenerator::generate_item_resources(
                    &actor,
                    &godot_data_root,
                    &godot_data_res,
                    &label_sprites,
                );
                godot_print!("Realm667Importer: Created Godot data for item {}", actor.name);
            }
        }

        godot_print!("Realm667Importer: IMPORT FINISHED SUCCESSFULLY");
        godot_print!("--------------------------------------------------");

        if Engine::singleton().is_editor_hint() {
            let editor = EditorInterface::singleton();
            if let Some(mut fs) = editor.get_resource_filesystem() {
                fs.scan();
            }
        }
    }

    fn read_zip_file_recursive(
        &self,
        archive: &mut ZipArchive<File>,
        file_path: &str,
        visited: &mut HashSet<String>,
    ) -> Option<String> {
        let normalized_path = file_path.to_lowercase().replace('\\', "/");
        if visited.contains(&normalized_path) {
            godot_warn!("Realm667Importer: Circular #include detected: {}", file_path);
            return None;
        }
        visited.insert(normalized_path.clone());

        let mut target_index = None;
        // Try exact match first
        for i in 0..archive.len() {
            let name = archive
                .by_index(i)
                .unwrap()
                .name()
                .to_lowercase()
                .replace('\\', "/");
            if name == normalized_path {
                target_index = Some(i);
                break;
            }
        }

        // If not found, try searching for it (original behavior as fallback)
        if target_index.is_none() {
            for i in 0..archive.len() {
                let name = archive
                    .by_index(i)
                    .unwrap()
                    .name()
                    .to_lowercase()
                    .replace('\\', "/");
                
                // Check if it ends with the name (normal fallback) 
                // OR if it matches a pattern like 0000_NAME.LMP
                if name.ends_with(&format!("/{}", normalized_path)) || 
                   name == normalized_path ||
                   name.ends_with(&format!("_{}", normalized_path)) ||
                   name.ends_with(&format!("_{}.lmp", normalized_path)) {
                    target_index = Some(i);
                    break;
                }
            }
        }

        if let Some(idx) = target_index {
            let actual_name = archive.by_index(idx).unwrap().name().replace('\\', "/");
            let current_dir = Path::new(&actual_name).parent().unwrap_or(Path::new(""));

            let mut content = String::new();
            {
                let mut zip_file = archive.by_index(idx).unwrap();
                let _ = zip_file.read_to_string(&mut content);
            }

            let mut full_content = String::new();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.to_lowercase().starts_with("#include") {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let include_path = parts[1].replace('"', "").replace('\'', "");

                        // Resolve relative path
                        let resolved_path = if include_path.starts_with('/') {
                            include_path.trim_start_matches('/').to_string()
                        } else {
                            let mut path = current_dir.to_path_buf();
                            path.push(&include_path);
                            path.to_string_lossy().replace('\\', "/")
                        };

                        if let Some(sub_content) =
                            self.read_zip_file_recursive(archive, &resolved_path, visited)
                        {
                            full_content.push_str(&sub_content);
                            full_content.push('\n');
                        }
                    }
                } else {
                    full_content.push_str(line);
                    full_content.push('\n');
                }
            }
            return Some(full_content);
        }
        None
    }

    fn extract_actor_assets_clean(
        &self,
        actor: &ActorDefinition,
        category: ActorCategory,
        archive: &mut ZipArchive<File>,
        godot_data_root: &Path,
        godot_data_res: &str,
        sounds_map: &HashMap<String, String>,
        mod_name: &str,
    ) -> HashMap<String, Vec<(String, i32)>> {
        let mut label_to_folder = HashMap::new();

        // Helper to populate mappings
        let mut add_mapping = |key: &str, folder: &str| {
            label_to_folder.insert(key.to_lowercase(), folder.to_string());
        };

        if category == ActorCategory::Weapon {
            add_mapping("Ready", "idle");
            add_mapping("Idle", "idle");
            add_mapping("Select", "idle");
            add_mapping("Fire", "shoot");
            add_mapping("Fire2", "shoot");
            add_mapping("Hold", "shoot");
            add_mapping("AltFire", "shoot");
            add_mapping("Pain", "pain");
            add_mapping("Death", "death");
            add_mapping("Spawn", "ground");
        } else if category == ActorCategory::Enemy {
            add_mapping("Spawn", "idle");
            add_mapping("Idle", "idle");
            add_mapping("See", "walk");
            add_mapping("Walk", "walk");
            add_mapping("Missile", "attack");
            add_mapping("Melee", "attack");
            add_mapping("Pain", "pain");
            add_mapping("Death", "death");
            add_mapping("XDeath", "xdeath");
            add_mapping("Raise", "raise");
        } else if category == ActorCategory::Projectile {
            add_mapping("Spawn", "spawn");
            add_mapping("Fly", "spawn");
            add_mapping("Idle", "spawn");
            add_mapping("Fade", "spawn");
            add_mapping("Death", "death");
            add_mapping("Crash", "death");
            add_mapping("XDeath", "death");
        } else if category == ActorCategory::Item || category == ActorCategory::Ammo {
            add_mapping("Spawn", "idle");
            add_mapping("Idle", "idle");
        }

        let mut result_map = HashMap::new();
        let actor_name = actor.name.to_lowercase();
        for (label, frames) in &actor.states {
            let label_lower = label.to_lowercase();
            let folder = label_to_folder.get(&label_lower)
                .cloned()
                .unwrap_or_else(|| label_lower.clone());

            let sprites_root = godot_data_root
                .join("sprites")
                .join(mod_name)
                .join(&actor_name)
                .join(&folder);

            let mut state_frames = Vec::new();
            for frame in frames {
                let extracted = self.extract_sprites_for_frame(frame, archive, &sprites_root);
                for p in extracted {
                    let rel_res = format!(
                        "{}/sprites/{}/{}/{}/{}",
                        godot_data_res,
                        mod_name,
                        actor_name,
                        folder,
                        p.file_name().unwrap().to_str().unwrap()
                    );
                    state_frames.push((rel_res, frame.duration));
                }
            }
            if !state_frames.is_empty() {
                result_map.insert(label.clone(), state_frames);
            }
        }

        let sound_props = [
            ("SeeSound", "taunt"),
            ("AttackSound", "taunt"),
            ("PainSound", "hurt"),
            ("DeathSound", "death"),
            ("ActiveSound", "taunt"),
            ("SelectSound", "taunt"),
        ];
        let sounds_dir = godot_data_root.join("sounds").join(mod_name);
        for (prop, subfolder) in sound_props {
            if let Some(alias) = actor.properties.get(prop) {
                let alias_str = alias.to_string_lossy().to_uppercase();
                if let Some(file_path) = sounds_map.get(&alias_str) {
                    let desired_stem = alias_str.split('/').last().unwrap().to_lowercase();
                    let dest_dir = if category == ActorCategory::Enemy {
                        sounds_dir.join(subfolder)
                    } else {
                        sounds_dir.clone()
                    };
                    self.extract_sound_file(file_path, Some(&desired_stem), archive, &dest_dir);
                }
            }
        }

        // Extract sounds referenced in actions
        for frames in actor.states.values() {
            for frame in frames {
                for action in &frame.actions {
                    let lower_name = action.name.to_lowercase();
                    if lower_name == "a_playsound" || lower_name == "a_startsound" || lower_name == "a_playweaponsound" || lower_name == "a_custommeleeattack" {
                        if let Some(alias) = action.args.get(if lower_name == "a_custommeleeattack" { 1 } else { 0 }) {
                            let alias_str = alias.to_string_lossy().to_uppercase();
                            if let Some(file_path) = sounds_map.get(&alias_str) {
                                let desired_stem = alias_str.split('/').last().unwrap().to_lowercase();
                                self.extract_sound_file(file_path, Some(&desired_stem), archive, &sounds_dir);
                            }
                        }
                    }
                }
            }
        }
        result_map
    }

fn extract_sprites_for_frame(
    &self,
    frame: &crate::realm667::actor::StateFrame,
    archive: &mut ZipArchive<File>,
    dest_dir: &Path,
) -> Vec<PathBuf> {
    let mut extracted_paths = Vec::new();
    godot_print!("Extracting for prefix: {} frames: {}", frame.sprite_prefix, frame.frames);

    for f_char in frame.frames.chars() {
        let mut seen_filenames = HashSet::new();
        let f_char_upper = f_char.to_uppercase().next().unwrap();
        let prefix_upper = frame.sprite_prefix.to_uppercase();

        for i in 0..archive.len() {
            let entry_name = archive
                .by_index(i)
                .unwrap()
                .name()
                .to_lowercase()
                .replace('\\', "/");
            let raw_file_name = entry_name.split('/').last().unwrap_or("").to_uppercase();
            let mut file_name = raw_file_name.clone();
            if let Some(pos) = raw_file_name.find('_') {
                 if raw_file_name[..pos].chars().all(|c| c.is_ascii_digit()) {
                     file_name = raw_file_name[pos+1..].to_string();
                 }
            }

            if file_name.starts_with("BM") || file_name.starts_with("BR") {
                continue;
            }

            let is_match = if file_name.starts_with(&prefix_upper) {
                if file_name.len() > 4 {
                    let char_at_4 = file_name.chars().nth(4).unwrap();
                    let char_at_6 = file_name.chars().nth(6);

                    char_at_4 == f_char_upper || (char_at_6.is_some() && char_at_6.unwrap() == f_char_upper)
                } else {
                    false
                }
            } else if let Some(pos) = file_name.find('_') {
                let sub = &file_name[pos+1..];
                if sub.starts_with(&prefix_upper) {
                    if sub.len() > 4 {
                        let char_at_4 = sub.chars().nth(4).unwrap();
                        let char_at_6 = sub.chars().nth(6);
                        char_at_4 == f_char_upper || (char_at_6.is_some() && char_at_6.unwrap() == f_char_upper)
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if is_match {
                let base_name = if let Some(pos) = file_name.find('.') {
                    &file_name[..pos]
                } else {
                    &file_name
                };

                let actual_base = if let Some(pos) = base_name.find('_') {
                    &base_name[pos+1..]
                } else {
                    base_name
                };

                if actual_base.starts_with(&prefix_upper) {
                    if !seen_filenames.contains(&file_name) {
                        if let Some(path) =
                            AssetHandler::extract_with_extension_fix(archive, i, dest_dir)
                        {
                            extracted_paths.push(path);
                            seen_filenames.insert(file_name);
                        }
                    }
                }
            }
        }
    }
    extracted_paths
}

fn extract_sound_file(
    &self,
    sound_path: &str,
    desired_stem: Option<&str>,
    archive: &mut ZipArchive<File>,
    dest_dir: &Path,
) {
    let search_name = sound_path.to_uppercase().replace('\\', "/");
    for i in 0..archive.len() {
        let name = archive
            .by_index(i)
            .unwrap()
            .name()
            .to_uppercase()
            .replace('\\', "/");
        if name.contains(&search_name) {
            if let Some(stem) = desired_stem {
                let mut content = Vec::new();
                {
                    let mut zip_file = archive.by_index(i).unwrap();
                    let _ = zip_file.read_to_end(&mut content);
                }
                let fixed_name = format!("{}.ogg", stem);
                let dest_path = dest_dir.join(fixed_name);
                let _ = fs::create_dir_all(dest_dir);
                if let Ok(mut out_file) = File::create(dest_path) {
                    let _ = out_file.write_all(&content);
                }
            } else {
                let _ = AssetHandler::extract_with_extension_fix(archive, i, dest_dir);
            }
        }
    }
}

#[func]
pub fn fix_extensions(&self, dir_path: String) {
    let path = Path::new(&dir_path);
    AssetHandler::fix_extensions_in_dir(path);
}
}
