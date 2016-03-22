use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::error::Error;

use ast::Ast;
use ast::AstType;

const EXT: &'static str = ".html";

pub struct Gen {
    template: Ast,
    writer: BufWriter<File>
}

impl Gen {
    pub fn new(templ: Ast, filename: &String) -> Gen {
        let mut options = OpenOptions::new();
        options.write(true);
        options.create(true);
        options.append(true);

        let file = match options.open(filename.to_string() + EXT) {
            Ok(file) => file,
            Err(..) => panic!("tank: unable to open file")
        };

        let buf_writer = BufWriter::new(file);

        Gen {
            template: templ,
            writer: buf_writer
        }
    }

    pub fn output(&mut self) -> &mut Gen {
        self.emit("hello tank");

        self
    }

    fn emit(&mut self, output: &'static str) {
        match write!(self.writer, "{}", output) {
            Err(error) => panic!("tank: Failed to write -  {}", Error::description(&error)),
            Ok(_) => ()
        };
    }
}
