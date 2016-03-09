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

impl Default for TokenType {
    fn default() -> TokenType {
        TokenType::Eof
    }
}

#[derive(Default, Debug)]
pub struct Token {
    tok_type: TokenType,
    val: String
}

impl Token {
    pub fn new() -> Token {
        Default::default()
    }
}
