// Enum used for returning the Tokens
#[derive(Debug, PartialEq)]
pub enum Token {
    KeywordInt,
    KeywordVoid,
    KeywordReturn,
    Identifier(String),
    Constant(i64),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(_))
    }
}

impl Token {
    pub fn is_constant(&self) -> bool {
        matches!(self, Token::Constant(_))
    }
    
}