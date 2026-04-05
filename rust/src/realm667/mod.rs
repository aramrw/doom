pub mod actor;
pub mod asset_handler;
pub mod lexer;
pub mod parser;
pub mod resource_gen;

use crate::realm667::actor::{ActorCategory, ActorDefinition};
use crate::realm667::asset_handler::AssetHandler;
use crate::realm667::parser::Parser;
use crate::realm667::resource_gen::ResourceGenerator;
use godot::classes::ProjectSettings;
use godot::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
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
        
        let parent_dir = pk3_path_obj.parent().unwrap_or(Path::new(""));
        let stem = pk3_path_obj.file_stem().unwrap_or_default().to_str().unwrap_or("extracted");
        
        // Output directory is parent/stem
        let abs_output_base = parent_dir.join(stem);
        
        // Derive Godot res:// path for the output base
        let res_output_base = path.strip_suffix(".pk3").unwrap_or(&path).strip_suffix(".PK3").unwrap_or(&path).to_string();

        godot_print!("--------------------------------------------------");
        godot_print!("Realm667Importer: STARTING IMPORT");
        godot_print!("Source: {}", abs_pk3_path_str);
        godot_print!("Output: {}", abs_output_base.display());

        let file = match File::open(&abs_pk3_path_str) {
            Ok(f) => f,
            Err(e) => {
                godot_error!(
                    "Realm667Importer: Failed to open PK3 file at {}: {}",
                    abs_pk3_path_str,
                    e
                );
                return;
            }
        };

        let mut archive = match ZipArchive::new(file) {
            Ok(a) => a,
            Err(e) => {
                godot_error!("Realm667Importer: Failed to read ZIP archive: {}", e);
                return;
            }
        };

        let mut script_content = String::new();
        let mut sndinfo_content = String::new();

        // Try to find main script (DECORATE or ZSCRIPT)
        let main_scripts = ["decorate.txt", "decorate", "zscript.txt", "zscript"];
        let mut found_main = false;

        for main_name in main_scripts {
            if let Some(content) = self.read_zip_file_recursive(&mut archive, main_name) {
                script_content = content;
                godot_print!("Realm667Importer: Loaded {} (including sub-files)", main_name);
                found_main = true;
                break;
            }
        }

        if !found_main {
            godot_warn!("Realm667Importer: No DECORATE or ZSCRIPT file found in PK3.");
            return;
        }

        // Try to find SNDINFO
        let sndinfo_names = ["sndinfo.txt", "sndinfo"];
        for snd_name in sndinfo_names {
            if let Some(content) = self.read_zip_file_recursive(&mut archive, snd_name) {
                sndinfo_content = content;
                godot_print!("Realm667Importer: Loaded {} (including sub-files)", snd_name);
                break;
            }
        }

        let mut sounds_map = HashMap::new();
        if !sndinfo_content.is_empty() {
            let mut snd_parser = Parser::new(&sndinfo_content);
            sounds_map = snd_parser.parse_sndinfo();
            godot_print!(
                "Realm667Importer: Parsed {} sound aliases",
                sounds_map.len()
            );
        }

        let mut parser = Parser::new(&script_content);
        let actors = parser.parse_actors();
        godot_print!(
            "Realm667Importer: Parsed {} actors from scripts",
            actors.len()
        );

        let base_out = abs_output_base.as_path();

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

            // Removed category_dir nesting. Now it's OutputDir/ActorName/
            let actor_root = base_out.join(actor.name.to_lowercase());
            let actor_res_path = format!("{}/{}", res_output_base.trim_end_matches('/'), actor.name.to_lowercase());
            
            self.extract_actor_assets(&actor, &mut archive, &actor_root, &sounds_map);

            if category == ActorCategory::Weapon {
                ResourceGenerator::generate_weapon_resources(&actor, &actor_root, &actor_res_path);
                godot_print!("Realm667Importer: Created resources for {}", actor.name);
            }
        }

        godot_print!("Realm667Importer: IMPORT FINISHED SUCCESSFULLY");
        godot_print!("--------------------------------------------------");
    }

    fn read_zip_file_recursive(&self, archive: &mut ZipArchive<File>, file_name: &str) -> Option<String> {
        let mut full_content = String::new();
        
        // Find file by name (case-insensitive)
        let mut target_index = None;
        let lower_name = file_name.to_lowercase().replace('\\', "/");
        
        for i in 0..archive.len() {
            let name = archive.by_index(i).unwrap().name().to_lowercase().replace('\\', "/");
            if name == lower_name || name.ends_with(&format!("/{}", lower_name)) {
                target_index = Some(i);
                break;
            }
        }

        if let Some(idx) = target_index {
            let mut content = String::new();
            {
                let mut zip_file = archive.by_index(idx).unwrap();
                let _ = zip_file.read_to_string(&mut content);
            }

            // Resolve #include
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.to_lowercase().starts_with("#include") {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let mut include_path = parts[1].replace('"', "").replace('\'', "");
                        if let Some(sub_content) = self.read_zip_file_recursive(archive, &include_path) {
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

    fn extract_actor_assets(
        &self,
        actor: &ActorDefinition,
        archive: &mut ZipArchive<File>,
        actor_root: &Path,
        sounds_map: &HashMap<String, String>,
    ) {
        let mut label_to_folder = HashMap::new();
        label_to_folder.insert("Ready", "idle");
        label_to_folder.insert("Spawn", "idle");
        label_to_folder.insert("Fire", "shoot");
        label_to_folder.insert("Missile", "shoot");
        label_to_folder.insert("Melee", "shoot");
        label_to_folder.insert("Reload", "reload");
        label_to_folder.insert("Pain", "pain");
        label_to_folder.insert("Death", "death");

        for (label, frames) in &actor.states {
            if let Some(folder_name) = label_to_folder.get(label.as_str()) {
                let state_dir = actor_root.join("sprites").join(folder_name);
                for frame in frames {
                    self.extract_sprites_for_frame(frame, archive, &state_dir);
                }
            }
        }

        let sound_props = [
            "SeeSound",
            "AttackSound",
            "PainSound",
            "DeathSound",
            "ActiveSound",
            "SelectSound",
        ];
        let sounds_dir = actor_root.join("sounds");

        for prop in sound_props {
            if let Some(alias) = actor.properties.get(prop) {
                if let Some(file_path) = sounds_map.get(&alias.to_uppercase()) {
                    self.extract_sound_file(file_path, archive, &sounds_dir);
                }
            }
        }
    }

    fn extract_sprites_for_frame(
        &self,
        frame: &crate::realm667::actor::StateFrame,
        archive: &mut ZipArchive<File>,
        dest_dir: &Path,
    ) {
        for f_char in frame.frames.chars() {
            let sprite_name = format!("{}{}", frame.sprite_prefix, f_char).to_uppercase();
            for i in 0..archive.len() {
                let name = {
                    let zip_file = archive.by_index(i).unwrap();
                    zip_file.name().to_uppercase()
                };
                if name.contains(&sprite_name) && (name.contains("SPRITES/") || !name.contains("/"))
                {
                    if let Some(path) =
                        AssetHandler::extract_with_extension_fix(archive, i, dest_dir)
                    {
                        godot_print!("  - Sprite: {:?}", path.file_name().unwrap());
                    }
                }
            }
        }
    }

    fn extract_sound_file(
        &self,
        sound_path: &str,
        archive: &mut ZipArchive<File>,
        dest_dir: &Path,
    ) {
        let search_name = sound_path.to_uppercase().replace('\\', "/");
        for i in 0..archive.len() {
            let name = {
                let zip_file = archive.by_index(i).unwrap();
                zip_file.name().to_uppercase().replace('\\', "/")
            };
            if name.contains(&search_name) {
                if let Some(path) = AssetHandler::extract_with_extension_fix(archive, i, dest_dir) {
                    godot_print!("  - Sound: {:?}", path.file_name().unwrap());
                }
            }
        }
    }

    #[func]
    pub fn fix_extensions(&self, dir_path: String) {
        godot_print!("Realm667Importer: Fixing extensions in {}", dir_path);
        let path = Path::new(&dir_path);
        AssetHandler::fix_extensions_in_dir(path);
    }
}
