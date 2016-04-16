extern crate tank;

use std::env;
use std::path::Path;
use std::fs::File;
use std::error::Error;

use tank::compile::compiler::Compiler;

fn main() {
    let file_name = env::args().nth(1).unwrap_or_else(|| {
        panic!("tank: Expected a file or directory name as the first arg.");
    });

    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("Failed to open {}: {}", display, Error::description(&error)),
        Ok(file) => file
    };

    let mut compiler = Compiler::new(&mut file, &file_name);

    compiler.compile();
}
