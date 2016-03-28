extern crate tank;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

use tank::syntax::symbol_table::SymbolTable;
use tank::syntax::parser::Parser;
use tank::generate::gen::Gen;

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

    // TODO: Read file by lines instead of into a string
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(error) => panic!("Failed to read {}: {}", display, Error::description(&error)),
        Ok(_) => ()
    }

    let sym_tab = SymbolTable::new();
    let mut parser = Parser::new(file_contents, sym_tab);
    parser.parse();

    let ast = parser.root;

    let mut gen = Gen::new(&file_name);
    gen.output(ast);
}
