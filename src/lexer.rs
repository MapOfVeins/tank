use token::Token;
use token::TokenType;

use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    curr_tok: Option<Token>,
    curr_char: Option<char>,
    char_count: usize
}

impl Lexer {
    pub fn new(file_name: String) -> Lexer {
        let path = Path::new(&file_name);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(error) => panic!("Failed to open {}: {}", display, Error::description(&error)),
            Ok(file) => file
        };
        
        let mut str = String::new();
        match file.read_to_string(&mut str) {
            Err(error) => panic!("Failed to read {}: {}", display, Error::description(&error)),
            Ok(_) => ()
        }

        Lexer {
            input: str,
            curr_tok: None,
            curr_char: None,
            char_count: 0
        }
    }

    pub fn lex(&mut self) -> &mut Lexer {
        let ch = self.curr_char.unwrap();
        while ch.is_whitespace() {
            self.get_char();
        }

        // TODO: exhaustive matching here
        match ch {
            '{' => self.curr_tok = self.get_token(TokenType::LeftBrace),
            '}' => self.curr_tok = self.get_token(TokenType::RightBrace),
            '(' => self.curr_tok = self.get_token(TokenType::LeftParen),
            ')' => self.curr_tok = self.get_token(TokenType::RightParen),
            ':' => self.curr_tok = self.get_token(TokenType::Colon),
            '=' => self.curr_tok = self.get_token(TokenType::Equals),
            _   => self.curr_tok = Some(Token::new(TokenType::Eof, String::new()))
        }

        self
    }

    fn get_char(&mut self) -> &mut Lexer {
        match self.input.chars().nth(self.char_count) {
            Some(c) => self.curr_char = Some(c),
            None => self.curr_char = None
        }

        self.char_count = self.char_count + 1;

        self
    }

    fn get_token(&mut self, token_type: TokenType) -> Option<Token> {
        let tok = match self.curr_char {
            Some(c) => Some(Token::new(token_type, c.to_string())),
            None => Some(Token::new(TokenType::Eof, String::new()))
        };

        self.get_char();

        tok
    }
}
