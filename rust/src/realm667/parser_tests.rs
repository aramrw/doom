#[cfg(test)]
mod tests {
    use crate::realm667::actor::GZValue;
    use crate::realm667::nom_parser::parse_document;

    #[test]
    fn test_parser_actor_properties() {
        let input = "
            actor MyActor : Parent 123 {
                Default {
                    Damage 5.6
                    Speed -10.2
                    Health -5
                    Tag \"Cool Actor\"
                }
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        assert_eq!(actor.name, "MyActor");
        assert_eq!(actor.ed_number, Some(123));
        assert_eq!(actor.parent, Some("Parent".to_string()));
        
        assert_eq!(actor.properties.get("Damage"), Some(&GZValue::Float(5.6)));
        assert_eq!(actor.properties.get("Speed"), Some(&GZValue::Float(-10.2)));
        assert_eq!(actor.properties.get("Health"), Some(&GZValue::Integer(-5)));
        assert_eq!(actor.properties.get("Tag"), Some(&GZValue::String("Cool Actor".to_string())));
    }

    #[test]
    fn test_parser_states_multiple_labels() {
        let input = "
            actor MyActor {
                States {
                    Spawn:
                    Idle:
                        PLAY A 10
                        Loop
                }
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        
        assert!(actor.states.contains_key("Spawn"));
        assert!(actor.states.contains_key("Idle"));
        
        let spawn_states = actor.states.get("Spawn").unwrap();
        assert_eq!(spawn_states.len(), 1);
        assert_eq!(spawn_states[0].sprite_prefix, "PLAY");
        
        let idle_states = actor.states.get("Idle").unwrap();
        assert_eq!(idle_states.len(), 1);
        assert_eq!(idle_states[0].sprite_prefix, "PLAY");
    }

    #[test]
    fn test_parser_state_function_call() {
        let input = "
            actor MyActor {
                States {
                    Spawn:
                        PLAY A 1 A_JumpIfHealthLower(10, \"LowHealth\")
                        Loop
                }
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        let spawn_states = actor.states.get("Spawn").unwrap();
        assert_eq!(spawn_states.len(), 1);
        let state = &spawn_states[0];
        
        assert_eq!(state.actions.len(), 1);
        let action = &state.actions[0];
        assert_eq!(action.name, "A_JumpIfHealthLower");
        assert_eq!(action.args.len(), 2);
        assert_eq!(action.args[0], GZValue::Integer(10));
        assert_eq!(action.args[1], GZValue::String("LowHealth".to_string()));
    }

    #[test]
    fn test_parser_class_keyword() {
        let input = "
            class MyZScriptActor : Actor replaces DoomPlayer {
                Default {
                    Health 200;
                }
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        assert_eq!(actor.name, "MyZScriptActor");
        assert_eq!(actor.parent, Some("Actor".to_string()));
        assert_eq!(actor.properties.get("Health"), Some(&GZValue::Integer(200)));
    }

    #[test]
    fn test_parser_zscript_anonymous_block() {
        let input = "
            class MyActor : Actor {
                States {
                    Fire:
                        Q2BL F 4 {
                            A_Gunflash();
                            A_FireProjectile(\"SMCBlast\",0.1,0,4.0,5.0);
                        }
                        Q2BL B 3;
                        Goto Ready;
                }
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        let fire_states = actor.states.get("Fire").unwrap();
        
        // The first state frame should have an action (A_FireProjectile or similar)
        let first_frame = &fire_states[0];
        assert_eq!(first_frame.sprite_prefix, "Q2BL");
        assert_eq!(first_frame.frames, "F");
        assert_eq!(first_frame.duration, 4);
        
        assert_eq!(first_frame.actions.len(), 2);
        assert_eq!(first_frame.actions[0].name, "A_Gunflash");
        assert_eq!(first_frame.actions[1].name, "A_FireProjectile"); 
    }

    #[test]
    fn test_parser_states_no_braces() {
        let input = "
            actor MyActor {
                States
                    Spawn:
                        PLAY A 10
                        Loop
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        assert!(actor.states.contains_key("Spawn"));
    }

    #[test]
    fn test_parser_flags_with_whitespace() {
        let input = "
            Class SmithGhost1 : Actor
            {
              Default
              {
                Radius 40;
                Height 70;
                Speed 1;
                Damage 0;
                RenderStyle \"Translucent\";
                Alpha 0.5;
                +NORADIUSDMG
                +BOSS
                +FIRERESIST
                +NOTARGET
                +MISSILEMORE
                PROJECTILE;
              }
              states
              {
              Spawn:
                SMT1 O 35;
              Fade:
                SMT1 O 2 A_FadeOut(0.10);
                Loop;
              }
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        assert_eq!(actor.name, "SmithGhost1");
        assert!(actor.flags.contains(&"NORADIUSDMG".to_string()));
        assert!(actor.flags.contains(&"BOSS".to_string()));
        assert!(actor.flags.contains(&"PROJECTILE".to_string()));
    }

    #[test]
    fn test_parser_zscript_methods() {
        let input = "
            class MyActor : Actor {
                void MyMethod() {
                    A_Log(\"Hello\");
                }
                States {
                    Spawn:
                        PLAY A 10
                        Loop
                }
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        assert!(actor.states.contains_key("Spawn"));
    }

    #[test]
    fn test_parser_complex_expression() {
        let input = "
            actor MyActor {
                States {
                    Spawn:
                        SMT1 L 10 A_CustomMeleeAttack(15*random(1,8), \"monster/hamhit\")
                        Goto See
                }
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        let spawn_states = actor.states.get("Spawn").unwrap();
        let state = &spawn_states[0];
        
        assert_eq!(state.actions.len(), 1);
        let action = &state.actions[0];
        assert_eq!(action.name, "A_CustomMeleeAttack");
        assert_eq!(action.args.len(), 2);
        assert_eq!(action.args[0].to_string_lossy(), "15*random(1,8)");
    }

    #[test]
    fn test_parser_enemy_properties() {
        let input = "
            actor MyEnemy : Actor 999 {
                Health 100
                Speed 8
                PainChance 50
                Radius 20
                Height 56
                +NOGRAVITY
                States {
                    Spawn:
                        PLAY A 10
                        Loop
                    See:
                        PLAY B 5
                        Loop
                }
            }
        ";
        let actors = parse_document(input).unwrap();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        assert_eq!(actor.name, "MyEnemy");
        assert_eq!(actor.ed_number, Some(999));
        assert_eq!(actor.properties.get("Health"), Some(&GZValue::Integer(100)));
        assert_eq!(actor.properties.get("Speed"), Some(&GZValue::Integer(8)));
        assert_eq!(actor.properties.get("PainChance"), Some(&GZValue::Integer(50)));
        assert!(actor.flags.contains(&"NOGRAVITY".to_string()));
        assert!(actor.states.contains_key("Spawn"));
        assert!(actor.states.contains_key("See"));
    }
}
