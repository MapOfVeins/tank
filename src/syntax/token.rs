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
    Percent,
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
    /// The type of the token, correspoing to the TokenType enum
    pub tok_type: TokenType,
    /// String value of the token (possibly an identifier/number)
    pub val: String,
    /// True if the token is a reserved word, false otherwise
    pub is_reserved: bool,
    /// Character position of the token on the current line.
    /// If the token is multiple characters, this is the index of the first charaster.
    pub char_pos: usize,
    /// Line number of the token
    pub line_num: usize
}

impl Token {
    pub fn new(token_type: TokenType, pos: usize, line: usize) -> Token {
        Token {
            tok_type: token_type,
            val: "".to_string(),
            is_reserved: false,
            char_pos: pos,
            line_num: line
        }
    }

    pub fn new_from_value(token_type: TokenType, value: &str, pos: usize, line: usize) -> Token {
        Token {
            tok_type: token_type,
            val: value.to_owned(),
            is_reserved: false,
            char_pos: pos,
            line_num: line
        }
    }

    pub fn new_from_empty() -> Token {
        Token {
            tok_type: TokenType::Eof,
            val: "".to_string(),
            is_reserved: false,
            char_pos: 0,
            line_num: 0
        }
    }

    pub fn set_reserved(&mut self, r: bool) -> &mut Token {
        self.is_reserved = r;

        self
    }
}
