mod token;
mod lexer;
mod reserved;

use std::env;
use lexer::Lexer;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let lexer = Lexer::new(file_name);
}
