pub struct Scanner {
    source: String,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Literal {
    StringValue(String),
    IntValue(i64),
    FloatValue(f64),
    BoolValue(bool),
    IdentifierValue(String),
    None,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
        }
    }

    pub fn scan_tokens(self: &Self) -> Vec<Token> {
        todo!()
    }
}
