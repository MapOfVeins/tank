#[derive(Debug)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Arrow,
    Colon,
    Type,
    Let,
    Equals,
    Identifier,
    Number,
    If,
    For,
    In,
    Eof
}

#[derive(Debug)]
pub struct Token {
    tok_type: TokenType,
    val: String
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token {
            tok_type: token_type,
            val: value
        }
    }
}
