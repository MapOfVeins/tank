#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    Colon,
    Equals,
    Minus,
    Ampersand,
    Arrow,
    Plus,
    Ident,
    Number,
    EqualsEquals,
    Gt,
    Lt,
    GtEquals,
    LtEquals,
    NotEquals,
    Eof
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub tok_type: TokenType,
    pub val: String,
    pub is_reserved: bool
}

impl Token {
    pub fn new(token_type: TokenType) -> Token {
        Token {
             tok_type: token_type,
             val: "".to_string(),
             is_reserved: false
        }
    }

    pub fn new_from_value(token_type: TokenType, value: &str) -> Token {
        Token {
             tok_type: token_type,
             val: value.to_owned(),
             is_reserved: false
        }
    }

    pub fn set_reserved(&mut self, r: bool) -> &mut Token {
        self.is_reserved = r;

        self
    }
}
