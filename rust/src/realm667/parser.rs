use crate::realm667::lexer::{Lexer, Token};
use crate::realm667::actor::{ActorDefinition, StateFrame, GZValue, GZFunctionCall};
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
        let mut last_metadata = HashMap::new();

        while self.cur_token != Token::Eof {
            match self.cur_token.clone() {
                Token::Comment(ref content) => {
                    let trimmed = content.trim();
                    if trimmed.starts_with("$category") {
                        let parts: Vec<&str> = trimmed.splitn(2, '"').collect();
                        if parts.len() >= 2 {
                            let category = parts[1].trim_end_matches('"');
                            last_metadata.insert("category".to_string(), category.to_string());
                        } else {
                            // Try without quotes
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() >= 2 {
                                last_metadata.insert("category".to_string(), parts[1].to_string());
                            }
                        }
                    }
                    self.next_token();
                }
                Token::Identifier(ref id) => {
                    let lower = id.to_lowercase();
                    if lower == "actor" || lower == "class" {
                        if let Some(mut actor) = self.parse_actor() {
                            // Merge metadata collected before and during actor block
                            for (k, v) in last_metadata.drain() {
                                actor.metadata.insert(k, v);
                            }
                            actors.push(actor);
                        }
                    } else {
                        self.next_token();
                    }
                }
                _ => self.next_token(),
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

        // Inheritance, ED Number, or replaces
        loop {
            match self.cur_token.clone() {
                Token::Colon => {
                    self.next_token();
                    if let Token::Identifier(parent) = self.cur_token.clone() {
                        actor.parent = Some(parent);
                        self.next_token();
                    }
                }
                Token::Identifier(ref id) if id.to_lowercase() == "replaces" => {
                    self.next_token();
                    if let Token::Identifier(_) = self.cur_token {
                        self.next_token();
                    }
                }
                Token::NumberStr(ref num) => {
                    actor.ed_number = num.parse().ok();
                    self.next_token();
                }
                Token::Identifier(ref id) => {
                    let lower = id.to_lowercase();
                    if lower == "native" || lower == "static" || lower == "internal" || lower == "abstract" {
                        self.next_token();
                    } else {
                        break;
                    }
                }
                _ => break,
            }
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
                        self.parse_states_into(&mut actor);
                    } else if lower == "default" {
                        self.next_token(); // skip "default"
                        if self.cur_token == Token::BraceOpen {
                            self.next_token();
                            while self.cur_token != Token::BraceClose && self.cur_token != Token::Eof {
                                self.parse_actor_property(&mut actor);
                            }
                            if self.cur_token == Token::BraceClose {
                                self.next_token();
                            }
                        }
                    } else if id.starts_with('+') {
                        actor.flags.push(id[1..].to_string());
                        self.next_token();
                    } else if id.starts_with('-') {
                        // Just skip negative flags
                        self.next_token();
                    } else {
                        // In DECORATE, properties/flags are directly in the actor block
                        self.parse_actor_property(&mut actor);
                    }
                }
                Token::Comment(content) => {
                    let trimmed = content.trim();
                    if trimmed.starts_with("$category") {
                        let parts: Vec<&str> = trimmed.splitn(2, '"').collect();
                        if parts.len() >= 2 {
                            let category = parts[1].trim_end_matches('"');
                            actor.metadata.insert("category".to_string(), category.to_string());
                        } else {
                            // Try without quotes
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() >= 2 {
                                actor.metadata.insert("category".to_string(), parts[1].to_string());
                            }
                        }
                    }
                    self.next_token();
                }
                _ => self.next_token(),
            }
        }
        
        if self.cur_token == Token::BraceClose {
            self.next_token();
        }

        Some(actor)
    }

    fn parse_actor_property(&mut self, actor: &mut ActorDefinition) {
        if let Token::Identifier(id) = self.cur_token.clone() {
            if id.starts_with('+') {
                actor.flags.push(id[1..].to_string());
                self.next_token();
                return;
            } else if id.starts_with('-') {
                self.next_token();
                return;
            }

            self.next_token();
            
            // Optional colon in ZScript properties
            if self.cur_token == Token::Colon {
                self.next_token();
            }

            // Parse property values
            let mut values = Vec::new();
            loop {
                if let Some(val) = self.parse_gz_value() {
                    values.push(val);
                }

                if self.cur_token == Token::Comma {
                    self.next_token();
                } else {
                    break;
                }
            }

            if !values.is_empty() {
                if values.len() == 1 {
                    actor.properties.insert(id, values[0].clone());
                } else {
                    let joined = values.iter()
                        .map(|v: &GZValue| v.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(", ");
                    actor.properties.insert(id, GZValue::String(joined));
                }
            } else {
                // No values - standalone flag like "Projectile;"
                actor.properties.insert(id, GZValue::Identifier("true".to_string()));
            }

            if self.cur_token == Token::SemiColon {
                self.next_token();
            }
        } else {
            self.next_token();
        }
    }

    fn parse_gz_value(&mut self) -> Option<GZValue> {
        match self.cur_token.clone() {
            Token::NumberStr(n) => {
                let val = if n.contains('.') {
                    GZValue::Float(n.parse().unwrap_or(0.0))
                } else {
                    GZValue::Integer(n.parse().unwrap_or(0))
                };
                self.next_token();
                Some(val)
            }
            Token::StringLiteral(s) => {
                self.next_token();
                Some(GZValue::String(s))
            }
            Token::Identifier(id) => {
                self.next_token();
                Some(GZValue::Identifier(id))
            }
            Token::Minus => {
                self.next_token();
                if let Token::NumberStr(n) = self.cur_token.clone() {
                    let neg_n = format!("-{}", n);
                    let val = if neg_n.contains('.') {
                        GZValue::Float(neg_n.parse().unwrap_or(0.0))
                    } else {
                        GZValue::Integer(neg_n.parse().unwrap_or(0))
                    };
                    self.next_token();
                    Some(val)
                } else {
                    Some(GZValue::Identifier("-".to_string()))
                }
            }
            _ => None,
        }
    }

    fn parse_states_into(&mut self, actor: &mut ActorDefinition) {
        self.next_token(); // skip "states"
        if self.cur_token != Token::BraceOpen {
            return;
        }
        self.next_token();

        let mut current_labels = Vec::new();
        while self.cur_token != Token::BraceClose && self.cur_token != Token::Eof {
            match self.cur_token.clone() {
                Token::Identifier(id) => {
                    self.next_token();
                    if self.cur_token == Token::Colon {
                        self.next_token();
                        current_labels.push(id);
                    } else {
                        // Frame data
                        let frames_str = if self.peek_token != Token::Colon && self.peek_token != Token::BraceClose {
                            if let Token::Identifier(f) = self.cur_token.clone() {
                                self.next_token();
                                f
                            } else {
                                "".to_string()
                            }
                        } else {
                            "".to_string()
                        };

                        let duration = if let Token::NumberStr(n) = self.cur_token.clone() {
                            self.next_token();
                            n.parse().unwrap_or(1)
                        } else if self.cur_token == Token::Minus {
                             self.next_token();
                             if let Token::NumberStr(n) = self.cur_token.clone() {
                                 self.next_token();
                                 -n.parse::<i32>().unwrap_or(1)
                             } else {
                                 1
                             }
                        } else {
                            1
                        };

                        let duration = if duration == 0 { 1 } else { duration };

                        let mut action = None;
                        let mut is_bright = false;

                        // Check for keywords or block
                        while self.cur_token != Token::SemiColon && self.cur_token != Token::BraceClose && self.cur_token != Token::Eof {
                            match self.cur_token.clone() {
                                Token::Identifier(ref k) if k.to_lowercase() == "bright" => {
                                    is_bright = true;
                                    self.next_token();
                                }
                                Token::Identifier(ref k) if k.to_lowercase().starts_with("a_") => {
                                    action = self.parse_function_call();
                                }
                                Token::BraceOpen => {
                                    // Skip anonymous blocks for now
                                    self.skip_balanced_braces();
                                }
                                _ => break, // Stop on unknown tokens (could be Loop, Stop, or a new label)
                            }
                        }

                        if self.cur_token == Token::SemiColon {
                            self.next_token();
                        }

                        let frame = StateFrame {
                            sprite_prefix: id,
                            frames: frames_str,
                            duration,
                            action,
                            is_bright,
                        };

                        for label in &current_labels {
                            actor.states.entry(label.clone()).or_insert_with(Vec::new).push(frame.clone());
                        }
                    }
                }
                Token::SemiColon => self.next_token(),
                Token::Comment(content) => {
                    let trimmed = content.trim();
                    if trimmed.starts_with("$category") {
                        let parts: Vec<&str> = trimmed.splitn(2, '"').collect();
                        if parts.len() >= 2 {
                            let category = parts[1].trim_end_matches('"');
                            actor.metadata.insert("category".to_string(), category.to_string());
                        } else {
                            // Try without quotes
                            let parts: Vec<&str> = trimmed.split_whitespace().collect();
                            if parts.len() >= 2 {
                                actor.metadata.insert("category".to_string(), parts[1].to_string());
                            }
                        }
                    }
                    self.next_token();
                }
                _ => self.next_token(),
            }
        }
        if self.cur_token == Token::BraceClose {
            self.next_token();
        }
    }

    fn parse_function_call(&mut self) -> Option<GZFunctionCall> {
        if let Token::Identifier(name) = self.cur_token.clone() {
            self.next_token();
            if self.cur_token == Token::ParenthesisOpen {
                self.next_token();
                let mut args = Vec::new();
                while self.cur_token != Token::ParenthesisClose && self.cur_token != Token::Eof {
                    if let Some(val) = self.parse_gz_value() {
                        args.push(val);
                    }
                    if self.cur_token == Token::Comma {
                        self.next_token();
                    }
                }
                if self.cur_token == Token::ParenthesisClose {
                    self.next_token();
                }
                return Some(GZFunctionCall { name, args });
            }
        }
        None
    }

    fn skip_balanced_braces(&mut self) {
        let mut depth = 0;
        loop {
            match self.cur_token {
                Token::BraceOpen => {
                    depth += 1;
                    self.next_token();
                }
                Token::BraceClose => {
                    depth -= 1;
                    self.next_token();
                    if depth <= 0 { break; }
                }
                Token::Eof => break,
                _ => self.next_token(),
            }
        }
    }
}
