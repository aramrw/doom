#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(i32),
    StringLiteral(String),
    BraceOpen,
    BraceClose,
    Colon,
    Comma,
    SemiColon,
    Quote,
    Operator(String),
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        if self.pos >= self.input.len() {
            return Token::Eof;
        }

        let ch = self.input[self.pos];
        
        // Handle comments
        if ch == '/' {
            if self.pos + 1 < self.input.len() && self.input[self.pos+1] == '/' {
                self.skip_line_comment();
                return self.next_token();
            }
            if self.pos + 1 < self.input.len() && self.input[self.pos+1] == '*' {
                self.skip_block_comment();
                return self.next_token();
            }
        }

        match ch {
            '{' => { self.pos += 1; Token::BraceOpen }
            '}' => { self.pos += 1; Token::BraceClose }
            ':' => { self.pos += 1; Token::Colon }
            ',' => { self.pos += 1; Token::Comma }
            ';' => { self.pos += 1; Token::SemiColon }
            '"' => self.read_string(),
            _ if ch.is_alphabetic() || ch == '_' || ch == '+' || ch == '-' || ch == '$' || ch == '.' => self.read_identifier(),
            _ if ch.is_digit(10) => self.read_number(),
            _ => {
                self.pos += 1;
                Token::Operator(ch.to_string())
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn skip_line_comment(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos] != '\n' {
            self.pos += 1;
        }
    }

    fn skip_block_comment(&mut self) {
        self.pos += 2; // skip /*
        while self.pos + 1 < self.input.len() && !(self.input[self.pos] == '*' && self.input[self.pos+1] == '/') {
            self.pos += 1;
        }
        self.pos += 2; // skip */
    }

    fn read_string(&mut self) -> Token {
        self.pos += 1; // skip first "
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            self.pos += 1;
        }
        let s = self.input[start..self.pos].iter().collect();
        self.pos += 1; // skip closing "
        Token::StringLiteral(s)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_' || self.input[self.pos] == '+' || self.input[self.pos] == '-' || self.input[self.pos] == '$' || self.input[self.pos] == '.') {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        Token::Identifier(s)
    }

    fn read_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_digit(10) {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        Token::Number(s.parse().unwrap_or(0))
    }
}
