#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Colon,
    Equals,
    Minus,
    Arrow,
    Ident,
    Number,
    Eof
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub val: String,
    pub is_reserved: bool
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token {
             tok_type: token_type,
             val: value,
             is_reserved: false
        }
    }

    pub fn set_reserved(&mut self, r: bool) -> &mut Token {
        self.is_reserved = r;

        self
    }
}
