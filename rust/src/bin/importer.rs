use std::env;
use std::fs::File;
use zip::ZipArchive;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

use rsdoom::realm667::nom_parser::parse_document;
use rsdoom::realm667::resource_gen::ResourceGenerator;
use rsdoom::realm667::actor::ActorDefinition;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: cargo run --bin importer <zip_path> <output_dir> <mod_name>");
        return;
    }
    let zip_path = &args[1];
    let out_dir = Path::new(&args[2]);
    let mod_name = &args[3];
    
    let file = File::open(zip_path).expect("Failed to open ZIP");
    let mut archive = ZipArchive::new(file).expect("Failed to read ZIP");
    
    let mut script_content = String::new();
    let mut found = false;
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap();
        let name = zip_file.name().to_uppercase();
        if name.contains("ZSCRIPT") || name.contains("DECORATE") {
            zip_file.read_to_string(&mut script_content).unwrap();
            found = true;
            println!("Found script: {}", zip_file.name());
            break;
        }
    }
    
    if !found {
        println!("Could not find DECORATE or ZSCRIPT in the provided archive.");
        return;
    }
    
    match parse_document(&script_content) {
        Ok(actors) => {
            println!("Successfully parsed {} actors.", actors.len());
            for actor in actors {
                println!("Generating resources for: {}", actor.name);
                let _ = ResourceGenerator::generate_enemy_resources(
                   &actor, 
                   out_dir, 
                   "res://enemies/wicked/godot_data", 
                   &HashMap::new(), // label_sprites 
                   &HashMap::new(), // sounds_map
                   "wicked"         // mod_name
                );            }
        },
        Err(e) => {
            println!("Error parsing document: {:?}", e);
        }
    }
}
