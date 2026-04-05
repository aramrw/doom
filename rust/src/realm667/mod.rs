pub mod actor;
pub mod lexer;
pub mod parser;
pub mod asset_handler;
pub mod resource_gen;

use godot::prelude::*;
use godot::classes::notify::NodeNotification;
use std::io::Read;
use zip::ZipArchive;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use crate::realm667::parser::Parser;
use crate::realm667::asset_handler::AssetHandler;
use crate::realm667::resource_gen::ResourceGenerator;
use crate::realm667::actor::{ActorDefinition, ActorCategory};

#[derive(GodotClass)]
#[class(tool, base=Node)]
pub struct Realm667Importer {
    #[export]
    pk3_path: GString,
    
    #[export]
    output_directory: GString,

    #[export]
    trigger_import: bool,

    base: Base<Node>,
}

#[godot_api]
impl INode for Realm667Importer {
    fn init(base: Base<Node>) -> Self {
        Self { 
            pk3_path: GString::from(""),
            output_directory: GString::from("res://"),
            trigger_import: false,
            base 
        }
    }

    fn on_notification(&mut self, what: NodeNotification) {
        if what == NodeNotification::PROCESS {
             if self.trigger_import {
                self.trigger_import = false;
                let path = self.pk3_path.to_string();
                let output = self.output_directory.to_string();
                self.import_pk3(path, output);
             }
        }
    }
}

#[godot_api]
impl Realm667Importer {
    #[func]
    pub fn import_pk3(&self, path: String, output_base: String) {
        if path.is_empty() {
            godot_warn!("Realm667Importer: PK3 path is empty!");
            return;
        }

        godot_print!("Realm667Importer: Starting one-click import from {}", path);

        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                godot_error!("Failed to open PK3 file at {}: {}", path, e);
                return;
            }
        };

        let mut archive = match ZipArchive::new(file) {
            Ok(a) => a,
            Err(e) => {
                godot_error!("Failed to read ZIP archive: {}", e);
                return;
            }
        };

        let mut decorate_content = String::new();
        let mut sndinfo_content = String::new();

        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            let name = zip_file.name().to_lowercase();
            if name == "decorate" || name == "decorate.txt" || name.ends_with("/decorate.txt") {
                let _ = zip_file.read_to_string(&mut decorate_content);
            }
            if name == "sndinfo" || name == "sndinfo.txt" || name.ends_with("/sndinfo.txt") {
                let _ = zip_file.read_to_string(&mut sndinfo_content);
            }
        }

        let mut sounds_map = HashMap::new();
        if !sndinfo_content.is_empty() {
            let mut snd_parser = Parser::new(&sndinfo_content);
            sounds_map = snd_parser.parse_sndinfo();
        }

        let mut parser = Parser::new(&decorate_content);
        let actors = parser.parse_actors();
        let base_out = Path::new(&output_base);

        for actor in actors {
            let category = actor.determine_category();
            if category == ActorCategory::Unknown { continue; }
            
            godot_print!("Processing Actor: {} ({:?})", actor.name, category);
            
            let category_dir = match category {
                ActorCategory::Weapon => "weapons",
                ActorCategory::Enemy => "enemies",
                ActorCategory::NPC => "npcs",
                ActorCategory::Item => "items",
                _ => "others",
            };

            let actor_root = base_out.join(category_dir).join(actor.name.to_lowercase());
            self.extract_actor_assets(&actor, &mut archive, &actor_root, &sounds_map);
            
            if category == ActorCategory::Weapon {
                ResourceGenerator::generate_weapon_resources(&actor, &actor_root);
                godot_print!("Realm667Importer: Generated complete wpdata.tres for {}", actor.name);
            }
        }
        
        godot_print!("Realm667Importer: Import finished! Check your {} directory.", output_base);
    }

    fn extract_actor_assets(&self, actor: &ActorDefinition, archive: &mut ZipArchive<File>, actor_root: &Path, sounds_map: &HashMap<String, String>) {
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

        let sound_props = ["SeeSound", "AttackSound", "PainSound", "DeathSound", "ActiveSound", "SelectSound"];
        let sounds_dir = actor_root.join("sounds");

        for prop in sound_props {
            if let Some(alias) = actor.properties.get(prop) {
                if let Some(file_path) = sounds_map.get(&alias.to_uppercase()) {
                    self.extract_sound_file(file_path, archive, &sounds_dir);
                }
            }
        }
    }

    fn extract_sprites_for_frame(&self, frame: &crate::realm667::actor::StateFrame, archive: &mut ZipArchive<File>, dest_dir: &Path) {
        for f_char in frame.frames.chars() {
            let sprite_name = format!("{}{}", frame.sprite_prefix, f_char).to_uppercase();
            for i in 0..archive.len() {
                let name = {
                    let zip_file = archive.by_index(i).unwrap();
                    zip_file.name().to_uppercase()
                };
                if name.contains(&sprite_name) && (name.contains("SPRITES/") || !name.contains("/")) {
                     AssetHandler::extract_with_extension_fix(archive, i, dest_dir);
                }
            }
        }
    }

    fn extract_sound_file(&self, sound_path: &str, archive: &mut ZipArchive<File>, dest_dir: &Path) {
        let search_name = sound_path.to_uppercase().replace('\\', "/");
        for i in 0..archive.len() {
            let name = {
                let zip_file = archive.by_index(i).unwrap();
                zip_file.name().to_uppercase().replace('\\', "/")
            };
            if name.contains(&search_name) {
                AssetHandler::extract_with_extension_fix(archive, i, dest_dir);
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
