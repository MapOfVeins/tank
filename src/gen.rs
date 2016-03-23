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

        if self.template.children.is_empty() {
            panic!("tank: Empty ast found, nothing to generate.");
        }

        let child_ast = self.template.children.iter();

        for ast in child_ast {
            self.gen_element(&ast);
        }
    }

    fn gen_element(&mut self, ast: &Box<Ast>) -> &Gen {
        if self.template.ast_type != AstType::Element {
            panic!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                   ast.ast_type,
                   AstType::Element);
        }

        // Expect first child to be ElementName with element name, second child is attribute list
        // third child is another element, containing either the contents or a
        // nested element. We should be guaranteed to have at least 3 ast types in the children vector.
        if ast.children.len() != 3 {
            panic!("tank: Invalid element ast found, not enough children present");
        }

        self.gen_el_name(&ast.children[0]);
        self.gen_attr_list(&ast.children[1]);

        match ast.children[3].ast_type {
            AstType::Element => self.gen_element(&ast.children[2]),
            AstType::Ident => self.gen_el_contents(&ast.children[2]),
            AstType::Eof => self.gen_empty(),
            _ => panic!("tank: Unexpected ast type found")
        };

        self
    }

    fn gen_el_name(&mut self, ast: &Box<Ast>) -> &Gen {
        self.emit(ast.val.clone());
        self
    }

    fn gen_attr_list(&mut self, ast: &Box<Ast>) -> &Gen {
        self
    }

    fn gen_el_contents(&mut self, ast: &Box<Ast>) -> &Gen {
        //self.emit(ast.val.clone());
        self
    }

    fn gen_empty(&self) -> &Gen {
        self
    }

    fn emit(&mut self, output: String) {
        match write!(self.writer, "{}", output) {
            Err(error) => panic!("tank: Failed to write -  {}", Error::description(&error)),
            Ok(_) => ()
        };
    }
}
