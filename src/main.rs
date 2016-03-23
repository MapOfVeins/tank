mod token;
mod lexer;
mod reserved;
mod parser;
mod ast;
mod gen;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

use parser::Parser;
use gen::Gen;

fn main() {
    let file_name = env::args().nth(1).unwrap_or_else(|| {
        panic!("tank: Expected a file name as the first arg.");
    });

    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("Failed to open {}: {}", display, Error::description(&error)),
        Ok(file) => file
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(error) => panic!("Failed to read {}: {}", display, Error::description(&error)),
        Ok(_) => ()
    }

    let mut parser = Parser::new(file_contents);
    let ast = parser.parse();

    let mut gen = Gen::new(&file_name);
    gen.output(ast);
}
