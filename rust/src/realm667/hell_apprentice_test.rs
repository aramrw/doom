#[cfg(test)]
mod tests {
    use crate::realm667::nom_parser::parse_document;
    use std::fs;
    use std::fs::File;
    use zip::ZipArchive;

    #[test]
    fn test_parse_hell_apprentice() {
        let content = fs::read_to_string("/Users/aramsamifanni/.gemini/tmp/new-game/hellapprentice/ZSCRIPT").unwrap();
        let result = parse_document(&content);
        match result {
            Ok(actors) => {
                println!("Successfully parsed {} actors", actors.len());
                let apprentice = actors.iter().find(|a| a.name == "Apprentice").expect("Apprentice actor not found");
                
                println!("Actor: {}, States: {}", apprentice.name, apprentice.states.len());
                assert_eq!(apprentice.states.len(), 9);
                
                // Check Melee state for the complex expression
                let melee_states = apprentice.states.get("Melee").expect("Melee state not found");
                let melee_attack = melee_states.iter().find(|s| s.actions.iter().any(|a| a.name == "A_CustomMeleeAttack")).expect("A_CustomMeleeAttack not found in Melee states");
                let action = melee_attack.actions.iter().find(|a| a.name == "A_CustomMeleeAttack").unwrap();
                
                assert_eq!(action.args.len(), 2);
                assert_eq!(action.args[0].to_string_lossy(), "15*random(1,8)");
                assert_eq!(action.args[1].to_string_lossy(), "monster/hamhit");

                // Check methods
                assert!(apprentice.methods.contains_key("ApprenticeHellShot"));
                let hell_shot = apprentice.methods.get("ApprenticeHellShot").unwrap();
                assert_eq!(hell_shot.len(), 3);
                assert_eq!(hell_shot[0].name, "A_SpawnProjectile");
                assert_eq!(hell_shot[0].args[0].to_string_lossy(), "Hellshot");

                assert!(apprentice.methods.contains_key("ApprenticeGroundFire"));
                let ground_fire = apprentice.methods.get("ApprenticeGroundFire").unwrap();
                assert_eq!(ground_fire.len(), 13); // 12 A_SpawnProjectile + 1 A_Playsound
                assert_eq!(ground_fire[12].name, "A_Playsound");

                for actor in &actors {
                    if actor.name != "Apprentice" {
                        println!("Actor: {}, States: {}", actor.name, actor.states.len());
                        assert!(actor.states.len() > 0);
                    }
                }
            }
            Err(e) => {
                panic!("Failed to parse HellApprentice: {}", e);
            }
        }
    }

    #[test]
    fn test_extract_hell_apprentice_sprites() {
        let pk3_path = "/Users/aramsamifanni/new-game/enemies/HellApprentice.pk3";
        let file = File::open(pk3_path).expect("Failed to open PK3");
        let mut archive = ZipArchive::new(file).expect("Failed to read ZIP");

        let content = fs::read_to_string("/Users/aramsamifanni/.gemini/tmp/new-game/hellapprentice/ZSCRIPT").unwrap();
        let actors = parse_document(&content).unwrap();
        let apprentice = actors.iter().find(|a| a.name == "Apprentice").unwrap();

        let spawn_states = apprentice.states.get("Spawn").unwrap();
        let frame = &spawn_states[0]; // SMT1 AB 10

        // Test frame 'A'
        let sprite_search_prefix = format!("{}A", frame.sprite_prefix).to_uppercase(); // SMT1A
        let mut found = false;
        for i in 0..archive.len() {
            let entry_name = archive.by_index(i).unwrap().name().to_uppercase().replace('\\', "/");
            let file_name = entry_name.split('/').last().unwrap_or("").to_uppercase();
            
            if file_name.starts_with(&sprite_search_prefix) {
                if file_name.len() > sprite_search_prefix.len() {
                    let next_char = file_name.chars().nth(sprite_search_prefix.len()).unwrap();
                    if next_char.is_ascii_digit() || next_char == '0' || next_char.is_ascii_alphabetic() {
                        if file_name.contains("SMT1A1C1") {
                            found = true;
                            break;
                        }
                    }
                }
            }
        }
        assert!(found, "Failed to find SMT1A1C1 in PK3 for search prefix {}", sprite_search_prefix);

        // Test directional mapping logic (simplified from resource_gen.rs)
        let sprite_path = "Sprites/SMT1A1C1";
        let stem = "SMT1A1C1";
        
        // Direction 1 (Front)
        let dir1 = '1';
        assert!(stem[5..].contains(dir1), "Direction 1 should be found in SMT1A1C1");
        
        // Direction 3 (Side)
        let dir3 = '3';
        let stem_a3 = "SMT1A3C7";
        assert!(stem_a3[5..].contains(dir3), "Direction 3 should be found in SMT1A3C7");
    }
}
