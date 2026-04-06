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
                        actor.states = self.parse_states();
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
                        .map(|v| v.to_string_lossy())
                        .collect::<Vec<_>>()
                        .join(", ");
                    actor.properties.insert(id, crate::realm667::actor::GZValue::String(joined));
                }
            }

            if self.cur_token == Token::SemiColon {
                self.next_token();
            }
        } else {
            self.next_token();
        }
    }

    fn parse_gz_value(&mut self) -> Option<crate::realm667::actor::GZValue> {
        match self.cur_token.clone() {
            Token::NumberStr(n) => {
                let val = if n.contains('.') {
                    crate::realm667::actor::GZValue::Float(n.parse().unwrap_or(0.0))
                } else {
                    crate::realm667::actor::GZValue::Integer(n.parse().unwrap_or(0))
                };
                self.next_token();
                Some(val)
            }
            Token::StringLiteral(s) => {
                self.next_token();
                Some(crate::realm667::actor::GZValue::String(s))
            }
            Token::Identifier(id) => {
                self.next_token();
                Some(crate::realm667::actor::GZValue::Identifier(id))
            }
            Token::Minus => {
                self.next_token();
                if let Token::NumberStr(n) = self.cur_token.clone() {
                    let neg_n = format!("-{}", n);
                    let val = if neg_n.contains('.') {
                        crate::realm667::actor::GZValue::Float(neg_n.parse().unwrap_or(0.0))
                    } else {
                        crate::realm667::actor::GZValue::Integer(neg_n.parse().unwrap_or(0))
                    };
                    self.next_token();
                    Some(val)
                } else {
                    Some(crate::realm667::actor::GZValue::Identifier("-".to_string()))
                }
            }
            _ => None,
        }
    }

    fn parse_states(&mut self) -> HashMap<String, Vec<StateFrame>> {
        let mut states = HashMap::new();
        self.next_token(); // skip "states"
        if self.cur_token != Token::BraceOpen {
            return states;
        }
        self.next_token();

        let mut current_labels = Vec::new();

        while self.cur_token != Token::BraceClose && self.cur_token != Token::Eof {
            match self.cur_token.clone() {
                Token::Identifier(id) => {
                    if self.peek_token == Token::Colon {
                        current_labels.push(id);
                        self.next_token(); // skip id
                        self.next_token(); // skip colon

                        // Collect multiple labels if they are consecutive
                        while let Token::Identifier(next_id) = self.cur_token.clone() {
                            if self.peek_token == Token::Colon {
                                current_labels.push(next_id);
                                self.next_token();
                                self.next_token();
                            } else {
                                break;
                            }
                        }
                    } else {
                        let lower = id.to_lowercase();
                        if lower == "loop" || lower == "wait" || lower == "stop" || lower == "fail" {
                            // These are state flow control keywords.
                            // We could store them as special frames or just skip.
                            // For now, let's just clear labels as they mark the end of a block.
                            self.next_token();
                            current_labels.clear();
                        } else if lower == "goto" {
                            self.next_token();
                            // Skip goto target (usually Identifier or Identifier + Colon + Identifier)
                            while self.cur_token != Token::SemiColon && self.cur_token != Token::BraceClose && !matches!(self.cur_token, Token::Identifier(_)) {
                                self.next_token();
                            }
                            if let Token::Identifier(_) = self.cur_token {
                                self.next_token();
                            }
                            current_labels.clear();
                        } else {
                            // This looks like a state line: Sprite Frames Duration [Bright] [Action]
                            let frames = self.parse_state_line();
                            for label in &current_labels {
                                states.entry(label.clone()).or_insert_with(Vec::new).extend(frames.clone());
                            }
                            
                            // If the next token is a label, we should NOT clear current_labels yet?
                            // Actually, in DECORATE, a label applies to all following lines until another label or flow control.
                            // But usually, we only want to associate the label with the START of the sequence.
                            // For our purposes (importer), associating it with the first line is usually enough.
                            // If we want to be thorough, we'd keep current_labels until a flow control keyword.
                            
                            // Check if next is a label
                            if let Token::Identifier(_) = self.cur_token {
                                if self.peek_token == Token::Colon {
                                    current_labels.clear();
                                }
                            }
                        }
                    }
                }
                Token::SemiColon => self.next_token(),
                _ => self.next_token(),
            }
        }
        if self.cur_token == Token::BraceClose {
            self.next_token();
        }
        states
    }

    fn parse_state_line(&mut self) -> Vec<StateFrame> {
        let mut frames_out = Vec::new();
        
        // Sprite Prefix (e.g., "CULT")
        let prefix = if let Token::Identifier(p) = self.cur_token.clone() {
            p
        } else {
            // Not a valid state line
            return frames_out;
        };
        self.next_token();

        // Frames (e.g., "ABCD")
        let frame_chars = if let Token::Identifier(f) = self.cur_token.clone() {
            f
        } else {
            // Might be a sprite with only 1 frame that looks like a keyword? 
            // Or just invalid.
            return frames_out;
        };
        self.next_token();

        // Duration (tics)
        let duration = match self.cur_token.clone() {
            Token::NumberStr(d) => {
                let val = d.parse().unwrap_or(0);
                self.next_token();
                val
            }
            Token::Identifier(id) if id == "-1" => {
                self.next_token();
                -1
            }
            Token::Minus => {
                self.next_token();
                if let Token::NumberStr(d) = self.cur_token.clone() {
                    self.next_token();
                    -(d.parse::<i32>().unwrap_or(0))
                } else {
                    0
                }
            }
            _ => 0,
        };

        // Optional keywords/Action
        let mut is_bright = false;
        let mut action = None;

        while self.cur_token != Token::SemiColon && self.cur_token != Token::BraceClose && self.cur_token != Token::Eof {
            match self.cur_token.clone() {
                Token::Identifier(id) => {
                    let lower = id.to_lowercase();
                    if lower == "bright" {
                        is_bright = true;
                        self.next_token();
                    } else if lower == "offset" {
                        // Skip offset(x, y)
                        self.next_token();
                        if self.cur_token == Token::ParenthesisOpen {
                            self.next_token();
                            while self.cur_token != Token::ParenthesisClose && self.cur_token != Token::Eof {
                                self.next_token();
                            }
                            self.next_token();
                        }
                    } else if lower == "canraise" || lower == "light" {
                        // Skip other keywords
                        self.next_token();
                        if self.cur_token == Token::ParenthesisOpen {
                            self.next_token();
                            while self.cur_token != Token::ParenthesisClose && self.cur_token != Token::Eof {
                                self.next_token();
                            }
                            self.next_token();
                        }
                    } else {
                        // Likely an action function
                        let mut is_action = id.starts_with("A_");
                        if self.peek_token == Token::ParenthesisOpen || self.peek_token == Token::BraceOpen {
                            is_action = true;
                        }

                        if is_action {
                            let action_name = id;
                            self.next_token();
                            let mut args = Vec::new();

                            if self.cur_token == Token::ParenthesisOpen {
                                self.next_token();
                                while self.cur_token != Token::ParenthesisClose && self.cur_token != Token::Eof {
                                    if let Some(val) = self.parse_gz_value() {
                                        args.push(val);
                                    }
                                    if self.cur_token == Token::Comma {
                                        self.next_token();
                                    } else if self.cur_token != Token::ParenthesisClose {
                                        self.next_token();
                                    }
                                }
                                if self.cur_token == Token::ParenthesisClose {
                                    self.next_token();
                                }
                            } else if self.cur_token == Token::BraceOpen {
                                // ZScript anonymous function block - skip for now
                                let mut brace_count = 1;
                                self.next_token();
                                while brace_count > 0 && self.cur_token != Token::Eof {
                                    if self.cur_token == Token::BraceOpen { brace_count += 1; }
                                    else if self.cur_token == Token::BraceClose { brace_count -= 1; }
                                    self.next_token();
                                }
                            }

                            action = Some(crate::realm667::actor::GZFunctionCall {
                                name: action_name,
                                args,
                            });
                            // Usually an action is the last thing on a line (before semicolon)
                        } else {
                            // Unknown identifier, break to avoid infinite loop
                            break;
                        }
                    }
                }
                Token::BraceOpen => {
                    // Anonymous function block
                    let mut brace_count = 1;
                    self.next_token();
                    while brace_count > 0 && self.cur_token != Token::Eof {
                        if self.cur_token == Token::BraceOpen { brace_count += 1; }
                        else if self.cur_token == Token::BraceClose { brace_count -= 1; }
                        self.next_token();
                    }
                }
                _ => break,
            }
        }

        // For now, we'll store all frames in one StateFrame if they share everything else.
        // This matches the current StateFrame struct.
        frames_out.push(StateFrame {
            sprite_prefix: prefix,
            frames: frame_chars,
            duration,
            action,
            is_bright,
        });

        frames_out
    }
}
