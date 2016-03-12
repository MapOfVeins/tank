mod token;
mod lexer;
mod reserved;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

use lexer::Lexer;

fn main() {
    let file_name = env::args().nth(1).unwrap();

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
    
    let lexer = Lexer::new(str);
}
