pub mod actor;
pub mod asset_handler;
pub mod lexer;
pub mod parser;
pub mod resource_gen;
pub mod parser_tests;

use crate::realm667::actor::{ActorCategory, ActorDefinition};
use crate::realm667::asset_handler::AssetHandler;
use crate::realm667::parser::Parser;
use crate::realm667::resource_gen::ResourceGenerator;
use godot::classes::ProjectSettings;
use godot::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[derive(GodotClass)]
#[class(tool, base=Node)]
pub struct Realm667Importer {
    #[export]
    #[var(hint = FILE_PATH, hint_string = "*.pk3,*.zip")]
    pk3_path: GString,

    #[export]
    trigger_import: bool,

    base: Base<Node>,
}

#[godot_api]
impl INode for Realm667Importer {
    fn init(base: Base<Node>) -> Self {
        Self {
            pk3_path: GString::from(""),
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

        let mut parser = Parser::new(&script_content);
        let actors = parser.parse_actors();

        if actors.is_empty() {
            godot_warn!("Realm667Importer: No actors found in scripts.");
            return;
        }

        // Primary actor for the top-level folder name
        let primary_actor = actors
            .iter()
            .find(|a| {
                matches!(
                    a.determine_category(),
                    ActorCategory::Weapon | ActorCategory::Enemy
                )
            })
            .unwrap_or(&actors[0]);

        let primary_name = primary_actor.name.to_lowercase();
        let parent_dir = pk3_path_obj.parent().unwrap_or(Path::new(""));

        // Output directory is parent/primary_name
        let abs_output_base = parent_dir.join(&primary_name);
        let _ = fs::create_dir_all(&abs_output_base);

        let parent_res_path = if let Some(pos) = path.rfind('/') {
            path[..pos].to_string()
        } else {
            "res://".to_string()
        };
        let res_output_base = format!("{}/{}", parent_res_path.trim_end_matches('/'), primary_name);

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
                let mut snd_parser = Parser::new(&content);
                sounds_map = snd_parser.parse_sndinfo();
                break;
            }
        }

        // 4. Organize each actor into "godot_data"
        for actor in actors {
            let category = actor.determine_category();
            if category == ActorCategory::Unknown {
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
                &mut archive,
                &godot_data_root,
                &godot_data_res,
                &sounds_map,
            );

            if category == ActorCategory::Weapon {
                godot_print!("Realm667Importer: Weapon states for {}: {:?}", actor.name, actor.states.keys());
                if let Some(fire_states) = actor.states.get("Fire") {
                    for (i, frame) in fire_states.iter().enumerate() {
                        godot_print!("  Frame {}: prefix={}, action={:?}", i, frame.sprite_prefix, frame.action);
                    }
                }
                ResourceGenerator::generate_weapon_resources(
                    &actor,
                    &godot_data_root,
                    &godot_data_res,
                    &label_sprites,
                );
                godot_print!("Realm667Importer: Created Godot data for weapon {}", actor.name);
            } else if category == ActorCategory::Enemy {
                ResourceGenerator::generate_enemy_resources(
                    &actor,
                    &godot_data_root,
                    &godot_data_res,
                    &label_sprites,
                );
                godot_print!("Realm667Importer: Created Godot data for enemy {}", actor.name);
            } else if category == ActorCategory::Projectile {
                ResourceGenerator::generate_projectile_resources(
                    &actor,
                    &godot_data_root,
                    &godot_data_res,
                    &label_sprites,
                );
                godot_print!("Realm667Importer: Created Godot data for projectile {}", actor.name);
            }
        }

        godot_print!("Realm667Importer: IMPORT FINISHED SUCCESSFULLY");
        godot_print!("--------------------------------------------------");
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
                if name.ends_with(&format!("/{}", normalized_path)) {
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
        archive: &mut ZipArchive<File>,
        godot_data_root: &Path,
        godot_data_res: &str,
        sounds_map: &HashMap<String, String>,
    ) -> HashMap<String, Vec<String>> {
        let mut label_to_folder = HashMap::new();
        let category = actor.determine_category();

        if category == ActorCategory::Weapon {
            label_to_folder.insert("Ready", "idle");
            label_to_folder.insert("Fire", "shoot");
            label_to_folder.insert("Reload", "reload");
            label_to_folder.insert("Pain", "pain");
            label_to_folder.insert("Death", "death");
        } else if category == ActorCategory::Enemy {
            label_to_folder.insert("Spawn", "walk");
            label_to_folder.insert("See", "walk");
            label_to_folder.insert("Missile", "attack");
            label_to_folder.insert("Melee", "attack");
            label_to_folder.insert("Pain", "pain");
            label_to_folder.insert("Death", "death");
            label_to_folder.insert("XDeath", "xdeath");
        } else if category == ActorCategory::Projectile {
            label_to_folder.insert("Spawn", "spawn");
            label_to_folder.insert("Death", "death");
        } else if category == ActorCategory::Item || category == ActorCategory::Ammo {
            label_to_folder.insert("Spawn", "idle");
        }

        let mut result_map = HashMap::new();
        let actor_name = actor.name.to_lowercase();

        // Process states in order to preserve frame sequence
        let mut sorted_labels: Vec<_> = actor.states.keys().collect();
        sorted_labels.sort(); // Consistent order

        for label in sorted_labels {
            if let Some(folder_name) = label_to_folder.get(label.as_str()) {
                let frames = &actor.states[label];
                let state_dir = godot_data_root
                    .join("sprites")
                    .join(&actor_name)
                    .join(folder_name);
                let mut sprite_paths = Vec::new();

                for frame in frames {
                    let extracted = self.extract_sprites_for_frame(frame, archive, &state_dir);
                    for p in extracted {
                        let rel_res = format!(
                            "{}/sprites/{}/{}/{}",
                            godot_data_res,
                            actor_name,
                            folder_name,
                            p.file_name().unwrap().to_str().unwrap()
                        );
                        sprite_paths.push(rel_res);
                    }
                }

                // Aggregate frames for labels that map to the same folder (e.g. Ready -> idle)
                result_map
                    .entry(folder_name.to_string())
                    .or_insert_with(Vec::new)
                    .extend(sprite_paths);
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
        let sounds_dir = godot_data_root.join("sounds").join(&actor_name);
        for (prop, subfolder) in sound_props {
            if let Some(alias) = actor.properties.get(prop) {
                if let Some(file_path) = sounds_map.get(&alias.to_string_lossy().to_uppercase()) {
                    let dest_dir = if category == ActorCategory::Enemy {
                        sounds_dir.join(subfolder)
                    } else {
                        sounds_dir.clone()
                    };
                    self.extract_sound_file(file_path, archive, &dest_dir);
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
        for f_char in frame.frames.chars() {
            // Standard Doom naming: PREFIX + FRAME + ROTATION (e.g., POSSA1, POSSA2A8)
            let sprite_search_prefix = format!("{}{}", frame.sprite_prefix, f_char).to_uppercase();

            for i in 0..archive.len() {
                let entry_name = archive
                    .by_index(i)
                    .unwrap()
                    .name()
                    .to_lowercase()
                    .replace('\\', "/");
                let file_name = entry_name.split('/').last().unwrap_or("").to_uppercase();

                // Skip non-image files and brightmaps
                if !file_name.ends_with(".PNG")
                    && !file_name.ends_with(".JPG")
                    && !file_name.ends_with(".TGA")
                {
                    continue;
                }
                if file_name.starts_with("BM") || file_name.starts_with("BR") {
                    continue;
                }

                // Match sprite prefix and frame char
                if file_name.starts_with(&sprite_search_prefix) {
                    // Check if it's a valid Doom sprite name (e.g. Q2BLA0.PNG)
                    // Length must be at least prefix+frame+rotation (e.g. 5 + 1 + 1 = 7)
                    let base_name = file_name.split('.').next().unwrap_or("");
                    if base_name.len() >= sprite_search_prefix.len() {
                        if let Some(path) =
                            AssetHandler::extract_with_extension_fix(archive, i, dest_dir)
                        {
                            extracted_paths.push(path);
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
                let _ = AssetHandler::extract_with_extension_fix(archive, i, dest_dir);
            }
        }
    }

    #[func]
    pub fn fix_extensions(&self, dir_path: String) {
        let path = Path::new(&dir_path);
        AssetHandler::fix_extensions_in_dir(path);
    }
}
