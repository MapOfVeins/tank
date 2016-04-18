use std::fs::File;
use std::error::Error;
use std::io::Read;

use syntax::parser::Parser;
use syntax::symbol_table::SymbolTable;
use generate::gen::Gen;

pub struct Compiler {
    parser: Parser,
    filename: String
}

impl Compiler {
    pub fn new(m_file: &mut File, filename: &String) -> Compiler {
        // TODO: Read file by lines instead of into a string?
        let mut file_contents = String::new();

        match m_file.read_to_string(&mut file_contents) {
            Err(error) => panic!("Failed to read {}: {}",
                                 &filename,
                                 Error::description(&error)),
            Ok(_) => ()
        }

        let sym_tab = SymbolTable::new();
        let parser = Parser::new(file_contents, sym_tab);

        Compiler {
            parser: parser,
            filename: filename.to_owned()
        }
    }

    /// Given a file and a parser created by the new functions,
    /// this function compiles a .tank file and writes the output
    /// to the corresponding .html file.
    pub fn compile(&mut self) -> &Compiler {
        println!("tank: Compiling '{}'...", &self.filename);
        self.parser.parse();

        let ast = &self.parser.root;
        let sym = self.parser.symbol_table.clone();

        let mut gen = Gen::new(&self.filename, sym);
        gen.output(ast.clone());

        self
    }
}
