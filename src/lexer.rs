use token::Token;

#[derive(Default)]
pub struct Lexer {
    input: String,
    curr_tok: Token,
    curr_char: char
}

impl Lexer {
    pub fn new() -> Lexer {
        Default::default()
    }

    pub fn lex(&mut self) -> &mut Lexer {

        self
    }
}
