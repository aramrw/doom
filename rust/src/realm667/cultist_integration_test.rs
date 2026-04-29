use super::*;
use crate::realm667::nom_parser::parse_document;
use std::fs;
use std::fs::File;
use zip::ZipArchive;

#[test]
fn test_cultist_pk3_import_logic() {
    let pk3_path = "Cultist.pk3";
    
    // Verify PK3 exists
    assert!(std::path::Path::new(pk3_path).exists(), "Cultist.pk3 should exist for the test. Current dir: {:?}", std::env::current_dir().unwrap());
    
    let file = File::open(pk3_path).expect("Failed to open PK3");
    let mut archive = ZipArchive::new(file).expect("Failed to read ZIP");
    
    // Read the script to get actor definitions
    // We assume the script is named ZSCRIPT or DECORATE inside
    let mut script_content = String::new();
    let mut found = false;
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i).unwrap();
        if zip_file.name() == "DECORATE" || zip_file.name() == "decorate" {
            zip_file.read_to_string(&mut script_content).unwrap();
            found = true;
            break;
        }
    }
    assert!(found, "DECORATE file not found in PK3");
    
    let actors = parse_document(&script_content).expect("Failed to parse script");
    assert!(!actors.is_empty(), "No actors found in script");
    
    // Verify we found a likely actor
    let actor = actors.iter().find(|a| a.name.to_lowercase().contains("cultist"));
    assert!(actor.is_some(), "Cultist actor not found in DECORATE");
}

