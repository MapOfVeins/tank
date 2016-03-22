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
            Err(..) => panic!("tank: unable to open file {}", filename)
        };

        let buf_writer = BufWriter::new(file);

        Gen {
            template: templ,
            writer: buf_writer
        }
    }

    pub fn output(&mut self) {
        if self.template.ast_type != AstType::Template {
            panic!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                   self.template.ast_type,
                   AstType::Template);
        }

        let child_ast = self.template.children.iter();

        for ast in child_ast {
            self.gen_element(ast);
        }
    }

    fn gen_element(&self, ast: &Box<Ast>) -> &Gen {
        // Expect first child to be Ident with element name, second child is attribute list
        // third child is another element, containing either the contents or a
        // nested element

        self
    }

    fn emit(&mut self, output: &'static str) {
        match write!(self.writer, "{}", output) {
            Err(error) => panic!("tank: Failed to write -  {}", Error::description(&error)),
            Ok(_) => ()
        };
    }
}
