use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::error::Error;

use ast::Ast;
use ast::AstType;

const EXT: &'static str = ".html";
const OPEN_SEPARATOR: &'static str = "<";
const CLOSE_SEPARATOR: &'static str = ">";
const CLOSING_TAG: &'static str = "</";

pub struct Gen {
    writer: BufWriter<File>,
    el_stack: Vec<String>
}

impl Gen {
    pub fn new(filename: &String) -> Gen {
        let mut options = OpenOptions::new();
        options.write(true);
        options.create(true);
        options.append(true);

        let file = match options.open(filename.to_string() + EXT) {
            Ok(file) => file,
            Err(..) => panic!("tank: unable to open file {}", filename)
        };

        let buf_writer = BufWriter::new(file);
        let els = Vec::new();

        Gen {
            writer: buf_writer,
            el_stack: els
        }
    }

    pub fn output(&mut self, template: Ast) {
        if template.ast_type != AstType::Template {
            panic!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                   template.ast_type,
                   AstType::Template);
        }

        if template.children.is_empty() {
            panic!("tank: Empty ast found, nothing to generate.");
        }

        for ast in template.children {
            self.gen_element(&ast);
        }
    }

    fn gen_element(&mut self, ast: &Box<Ast>) -> &Gen {
        if ast.ast_type != AstType::Element {
            panic!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                   ast.ast_type,
                   AstType::Element);
        }

        // Expect first child to be ElementName with element name, second child is attribute list
        // third child is another element, containing either the contents or a
        // nested element. We should be guaranteed to have at least 3 ast types in the children vector.
        if ast.children.len() < 3 {
            panic!("tank: Invalid Element ast found, not enough children present");
        }

        self.gen_el_name(&ast.children[0]);
        self.gen_attr_list(&ast.children[1]);

        match ast.children[2].ast_type {
            AstType::Element => self.gen_element(&ast.children[2]),
            AstType::Ident => self.gen_el_contents(&ast.children[2]),
            AstType::Eof => self.gen_empty(),
            _ => panic!("tank: Unexpected ast type found")
        };

        self
    }

    fn gen_el_name(&mut self, ast: &Box<Ast>) -> &Gen {
        self.el_stack.push(ast.val.clone());

        self.emit(&OPEN_SEPARATOR.to_string());
        self.emit(&ast.val);

        self
    }

    fn gen_attr_list(&mut self, ast: &Box<Ast>) -> &Gen {
        if ast.children.is_empty() {
            self.emit(&CLOSE_SEPARATOR.to_string());
            self.emit_space();
        } else {
            self.emit_space();

        }

        self
    }

    fn gen_el_contents(&mut self, ast: &Box<Ast>) -> &Gen {
        self.emit(&ast.val);
        self.emit_space();

        // TODO: Error handling here
        let el_name = self.el_stack.pop().unwrap();

        self.emit_closing_tag(&el_name);

        self
    }

    fn gen_empty(&mut self) -> &Gen {
        self.emit_newline();
        self
    }

    fn emit(&mut self, output: &String) {
        match write!(self.writer, "{}", output) {
            Err(error) => panic!("tank: Failed to write -  {}", Error::description(&error)),
            Ok(_) => ()
        };
    }

    fn emit_closing_tag(&mut self, tag_value: &String) {
        let mut tag = String::from(CLOSING_TAG);
        tag = tag + tag_value;
        tag = tag + CLOSE_SEPARATOR;

        match write!(self.writer, "{}", tag) {
            Err(error) => panic!("tank: Failed to write -  {}", Error::description(&error)),
            Ok(_) => ()
        };
    }

    fn emit_space(&mut self) {
        match write!(self.writer, "{}", " ") {
            Err(error) => panic!("tank: Failed to write -  {}", Error::description(&error)),
            Ok(_) => ()
        };
    }

    fn emit_newline(&mut self) {
        match write!(self.writer, "{}", "\n") {
            Err(error) => panic!("tank: Failed to write -  {}", Error::description(&error)),
            Ok(_) => ()
        };
    }
}
