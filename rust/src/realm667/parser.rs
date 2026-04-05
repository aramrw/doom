use crate::realm667::lexer::{Lexer, Token};
use crate::realm667::actor::{ActorDefinition, StateFrame};
use std::collections::HashMap;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            cur_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_actors(&mut self) -> Vec<ActorDefinition> {
        let mut actors = Vec::new();
        while self.cur_token != Token::Eof {
            if let Token::Identifier(ref id) = self.cur_token {
                let lower = id.to_lowercase();
                if lower == "actor" || lower == "class" {
                    if let Some(actor) = self.parse_actor() {
                        actors.push(actor);
                    }
                } else {
                    self.next_token();
                }
            } else if let Token::Operator(op) = self.cur_token.clone() {
                if op == "#" {
                    // Skip #include lines for now in the general parser,
                    // as they should be pre-processed or handled differently.
                    self.next_token();
                } else {
                    self.next_token();
                }
            } else {
                self.next_token();
            }
        }
        actors
    }

    pub fn parse_sndinfo(&mut self) -> HashMap<String, String> {
        let mut sounds = HashMap::new();
        while self.cur_token != Token::Eof {
            if let Token::Identifier(alias) = self.cur_token.clone() {
                self.next_token();
                if let Token::Identifier(path) = self.cur_token.clone() {
                    sounds.insert(alias.to_uppercase(), path);
                    self.next_token();
                } else if let Token::StringLiteral(path) = self.cur_token.clone() {
                    sounds.insert(alias.to_uppercase(), path);
                    self.next_token();
                }
            } else {
                self.next_token();
            }
        }
        sounds
    }

    fn parse_actor(&mut self) -> Option<ActorDefinition> {
        let mut actor = ActorDefinition::default();
        self.next_token(); // skip "actor" or "class"

        // Name
        if let Token::Identifier(name) = self.cur_token.clone() {
            actor.name = name;
            self.next_token();
        }

        // Inheritance or ED Number
        if let Token::Colon = self.cur_token {
            self.next_token();
            if let Token::Identifier(parent) = self.cur_token.clone() {
                actor.parent = Some(parent);
                self.next_token();
            }
        }

        if let Token::Number(num) = self.cur_token {
            actor.ed_number = Some(num);
            self.next_token();
        }

        // ZScript sometimes has "native" or other keywords here
        while let Token::Identifier(_) = self.cur_token {
            self.next_token();
        }

        if self.cur_token != Token::BraceOpen {
            return None;
        }
        self.next_token();

        while self.cur_token != Token::BraceClose && self.cur_token != Token::Eof {
            match self.cur_token.clone() {
                Token::Identifier(id) => {
                    let lower = id.to_lowercase();
                    if lower == "states" {
                        actor.states = self.parse_states();
                    } else if lower == "default" {
                        self.next_token(); // skip "default"
                        if self.cur_token == Token::BraceOpen {
                            self.next_token();
                            while self.cur_token != Token::BraceClose && self.cur_token != Token::Eof {
                                self.parse_actor_property(&mut actor);
                            }
                            self.next_token(); // skip brace close
                        }
                    } else if id.starts_with('+') {
                        actor.flags.push(id[1..].to_string());
                        self.next_token();
                    } else if id.starts_with('-') {
                        self.next_token();
                    } else {
                        self.parse_actor_property(&mut actor);
                    }
                }
                _ => self.next_token(),
            }
        }

        Some(actor)
    }

    fn parse_actor_property(&mut self, actor: &mut ActorDefinition) {
        if let Token::Identifier(id) = self.cur_token.clone() {
            if id.starts_with('+') {
                actor.flags.push(id[1..].to_string());
                self.next_token();
                return;
            }
            if id.starts_with('-') {
                self.next_token();
                return;
            }
            
            self.next_token();
            let mut value = String::new();
            while self.cur_token != Token::SemiColon && !matches!(self.cur_token, Token::Identifier(_)) && self.cur_token != Token::BraceClose {
                match &self.cur_token {
                    Token::Number(n) => value.push_str(&n.to_string()),
                    Token::Identifier(s) => value.push_str(s),
                    Token::StringLiteral(s) => value.push_str(s),
                    Token::Comma => value.push(','),
                    _ => {}
                }
                value.push(' ');
                self.next_token();
            }
            if self.cur_token == Token::SemiColon {
                self.next_token();
            }
            actor.properties.insert(id, value.trim().to_string());
        } else {
            self.next_token();
        }
    }

    fn parse_states(&mut self) -> HashMap<String, Vec<StateFrame>> {
        let mut states = HashMap::new();
        self.next_token(); // skip "states"
        if self.cur_token != Token::BraceOpen {
            return states;
        }
        self.next_token();

        let mut current_label = String::new();

        while self.cur_token != Token::BraceClose && self.cur_token != Token::Eof {
            match self.cur_token.clone() {
                Token::Identifier(id) => {
                    if self.peek_token == Token::Colon {
                        current_label = id;
                        self.next_token(); // skip id
                        self.next_token(); // skip colon
                    } else {
                        // This is a state line
                        let frames = self.parse_state_line();
                        states.entry(current_label.clone()).or_insert_with(Vec::new).extend(frames);
                    }
                }
                _ => self.next_token(),
            }
        }
        self.next_token(); // skip closing brace
        states
    }

    fn parse_state_line(&mut self) -> Vec<StateFrame> {
        let mut frames_out = Vec::new();
        
        // Sprite Prefix (e.g., "CULT")
        let prefix = if let Token::Identifier(p) = self.cur_token.clone() {
            p
        } else {
            self.next_token();
            return frames_out;
        };
        self.next_token();

        // Frames (e.g., "ABCD")
        let frames = if let Token::Identifier(f) = self.cur_token.clone() {
            f
        } else {
            return frames_out;
        };
        self.next_token();

        // Duration (tics)
        let duration = if let Token::Number(d) = self.cur_token {
            d
        } else {
            0
        };
        self.next_token();

        // Optional Action (e.g., "A_Look")
        let mut action = None;
        if let Token::Identifier(a) = self.cur_token.clone() {
            if a.starts_with("A_") {
                action = Some(a);
                self.next_token();
                // Handle optional parameters in parentheses
                if self.cur_token == Token::Operator("(".to_string()) {
                    while self.cur_token != Token::Operator(")".to_string()) && self.cur_token != Token::Eof {
                        self.next_token();
                    }
                    self.next_token(); // skip )
                }
            }
        }

        frames_out.push(StateFrame {
            sprite_prefix: prefix,
            frames,
            duration,
            action,
            is_bright: false, // Bright detection can be added later
        });

        frames_out
    }
}
