use token::Token;

use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    curr_tok: Token,
    curr_char: char
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
            curr_tok: Token::new(),
            curr_char: ' '
        }
    }

    //pub fn lex(&mut self) -> &mut Lexer {
//
//        self
//    }
}
