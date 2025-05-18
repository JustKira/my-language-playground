use std::error::Error;
use std::string::String;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number, // Note: `String` as a variant name is fine here,
    // it doesn't conflict with the `std::string::String` type.

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof, // End Of File
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    StringValue(String),
    IntValue(i64),
    FloatValue(f64),
    BoolValue(bool),
    IdentifierValue(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{}, {}, {:?}", self.token_type, self.lexeme, self.literal)
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, Box<dyn Error>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        Ok(self.tokens.clone())
    }

    pub fn scan_token(self: &mut Self) -> Result<(), Box<dyn Error>> {
        let c = self.advance();

        match c {
            Ok(c) => match c {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),

                _ => todo!(),
            },
            Err(e) => {
                return Err(e);
            }
        }

        todo!()
    }

    pub fn advance(self: &mut Self) -> Result<char, Box<dyn Error>> {
        let c = self
            .source
            .chars()
            .nth(self.current)
            .ok_or("Unexpected EOF while reading")?;

        Ok(c)
    }

    pub fn add_token(self: &mut Self, token_type: TokenType, literal: Option<Literal>) {
        todo!()
    }

    fn is_at_end(self: &Self) -> bool {
        self.current >= self.source.len()
    }
}
