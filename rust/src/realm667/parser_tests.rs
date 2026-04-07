#[cfg(test)]
mod tests {
    use crate::realm667::lexer::{Lexer, Token};
    use crate::realm667::actor::{GZValue, ActorDefinition};
    use crate::realm667::parser::Parser;

    #[test]
    fn test_lexer_tokens() {
        let input = "actor MyActor // comment\n{ }";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token(), Token::Identifier("actor".to_string()));
        assert_eq!(lexer.next_token(), Token::Identifier("MyActor".to_string()));
        assert_eq!(lexer.next_token(), Token::Comment(" comment".to_string()));
        assert_eq!(lexer.next_token(), Token::BraceOpen);
        assert_eq!(lexer.next_token(), Token::BraceClose);
    }

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
        let mut parser = Parser::new(input);
        let actors = parser.parse_actors();
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
        let mut parser = Parser::new(input);
        let actors = parser.parse_actors();
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
        let mut parser = Parser::new(input);
        let actors = parser.parse_actors();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        let spawn_states = actor.states.get("Spawn").unwrap();
        assert_eq!(spawn_states.len(), 1);
        let state = &spawn_states[0];
        
        let action = state.action.as_ref().unwrap();
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
        let mut parser = Parser::new(input);
        let actors = parser.parse_actors();
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
        let mut parser = Parser::new(input);
        let actors = parser.parse_actors();
        assert_eq!(actors.len(), 1);
        let actor = &actors[0];
        let fire_states = actor.states.get("Fire").unwrap();
        
        // The first state frame should have an action (A_FireProjectile or similar)
        let first_frame = &fire_states[0];
        assert_eq!(first_frame.sprite_prefix, "Q2BL");
        assert_eq!(first_frame.frames, "F");
        assert_eq!(first_frame.duration, 4);
        
        // Even if we don't parse everything in the block, we should at least 
        // find ONE relevant action if it's there, or at least not crash.
        // Ideally we want A_FireProjectile.
        assert!(first_frame.action.is_some());
        let action = first_frame.action.as_ref().unwrap();
        // Since the block has two, it might pick one or we might need to handle multiple.
        // Currently GZFunctionCall is Option, not Vec. We might need to change it to Vec
        // if we want to support multiple actions per frame (ZScript allows this).
        assert_eq!(action.name, "A_FireProjectile"); 
    }
}
