#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i64),
    StringLiteral(String),
    Plus,
    Minus,
    Star,
    Slash,
    Identifier(String),
    Assign,
    Print,
    Semicolon,
    LParen,
    RParen,
    EOF,
    Join,
    Split,
    Count,
    Comma,
    True,
    False,
    Eq,      // ==
    NotEq,   // !=
    Lt,      // <
    Gt,      // >
    LtEq,    // <=
    GtEq,    // >=
    And,     // &&
    Or,      // ||
    Not,     // !
    LBracket,
    RBracket,
    If,
    Else,
    ElseIf,
    LBrace,
    RBrace,
    While,
    For,
}

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.current_char = if self.position >= self.input.len() {
            None
        } else {
            Some(self.input.chars().nth(self.position).unwrap())
        };
        self.position += 1;
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        match self.current_char {
            Some('+') => {
                self.read_char();
                Ok(Token::Plus)
            }
            Some('-') => {
                self.read_char();
                Ok(Token::Minus)
            }
            Some('*') => {
                self.read_char();
                Ok(Token::Star)
            }
            Some('/') => {
                self.read_char();
                if self.current_char == Some('/') {
                    self.skip_comment();
                    self.next_token()
                } else {
                    Ok(Token::Slash)
                }
            }
            Some('=') => {
                if self.peek() == Some('=') {
                    self.read_char();
                    self.read_char();
                    Ok(Token::Eq)
                } else {
                    self.read_char();
                    Ok(Token::Assign)
                }
            }
            Some('!') => {
                if self.peek() == Some('=') {
                    self.read_char();
                    self.read_char();
                    Ok(Token::NotEq)
                } else {
                    self.read_char();
                    Ok(Token::Not)
                }
            }
            Some('<') => {
                if self.peek() == Some('=') {
                    self.read_char();
                    self.read_char();
                    Ok(Token::LtEq)
                } else {
                    self.read_char();
                    Ok(Token::Lt)
                }
            }
            Some('>') => {
                if self.peek() == Some('=') {
                    self.read_char();
                    self.read_char();
                    Ok(Token::GtEq)
                } else {
                    self.read_char();
                    Ok(Token::Gt)
                }
            }
            Some('&') => {
                if self.peek() == Some('&') {
                    self.read_char();
                    self.read_char();
                    Ok(Token::And)
                } else {
                    Err("Expected '&&'".to_string())
                }
            }
            Some('|') => {
                if self.peek() == Some('|') {
                    self.read_char();
                    self.read_char();
                    Ok(Token::Or)
                } else {
                    Err("Expected '||'".to_string())
                }
            }
            Some(';') => {
                self.read_char();
                Ok(Token::Semicolon)
            }
            Some('(') => {
                self.read_char();
                Ok(Token::LParen)
            }
            Some(')') => {
                self.read_char();
                Ok(Token::RParen)
            }
            Some('"') => self.read_string().map(Token::StringLiteral),
            Some(c) if c.is_digit(10) => self.read_number().map(Token::Number),
            Some(c) if c.is_alphabetic() => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "print" => Ok(Token::Print),
                    "join" => Ok(Token::Join),
                    "split" => Ok(Token::Split),
                    "count" => Ok(Token::Count),
                    "true" => Ok(Token::True),
                    "false" => Ok(Token::False),
                    "if" => Ok(Token::If),
                    "else" => Ok(Token::Else),
                    "elseif" => Ok(Token::ElseIf),
                    "while" => Ok(Token::While),
                    "for" => Ok(Token::For),
                    _ => Ok(Token::Identifier(ident)),
                }
            }
            Some(',') => {
                self.read_char();
                Ok(Token::Comma)
            }
            Some('[') => {
                self.read_char();
                Ok(Token::LBracket)
            }
            Some(']') => {
                self.read_char();
                Ok(Token::RBracket)
            }
            Some('{') => {
                self.read_char();
                Ok(Token::LBrace)
            }
            Some('}') => {
                self.read_char();
                Ok(Token::RBrace)
            }
            None => Ok(Token::EOF),
            _ => Err(format!("Unknown character: {}", self.current_char.unwrap())),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_some() && self.current_char.unwrap().is_whitespace() {
            self.read_char();
        }
    }

    fn skip_comment(&mut self) {
        while self.current_char.is_some() && self.current_char.unwrap() != '\n' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Result<i64, String> {
        let start = self.position - 1;
        while self.current_char.is_some() && self.current_char.unwrap().is_digit(10) {
            self.read_char();
        }
        self.input[start..self.position - 1].parse().map_err(|e: std::num::ParseIntError| e.to_string())
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position - 1;
        while self.current_char.is_some() && self.current_char.unwrap().is_alphabetic() {
            self.read_char();
        }
        self.input[start..self.position - 1].to_string()
    }

    fn read_string(&mut self) -> Result<String, String> {
        self.read_char(); // Skip the opening quote
        let start = self.position - 1;
        while self.current_char.is_some() && self.current_char.unwrap() != '"' {
            self.read_char();
        }
        if self.current_char.is_none() {
            return Err("Unterminated string literal".to_string());
        }
        let result = self.input[start..self.position - 1].to_string();
        self.read_char(); // Skip the closing quote
        Ok(result)
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
}