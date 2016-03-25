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

const INDENTATION_COUNT: usize = 2;

#[derive(Clone)]
struct Scope {
    indentation: usize,
    val: String
}

pub struct Gen {
    writer: BufWriter<File>,
    el_stack: Vec<Scope>
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
        let indentation = self.el_stack.len() * INDENTATION_COUNT;

        let el_scope = Scope {
            indentation: indentation,
            val: ast.val.clone()
        };

        self.el_stack.push(el_scope);
        self.emit_space(indentation);

        self.emit(&OPEN_SEPARATOR.to_string());
        self.emit(&ast.val);

        self
    }

    fn gen_attr_list(&mut self, ast: &Box<Ast>) -> &Gen {
        if !ast.children.is_empty() {
            self.emit_space(1);
            let attributes = ast.children.clone();

            // If we're here, we know we have x number of attribute pairs. We will
            // always have an even number of elements in the attributes vec, otherwise
            // there would have been a parse error. We split the vec into chunks of
            // two and emit them as pairs of identifiers.
            let mut counter = 0;

            for attr_pair in attributes.chunks(2) {
                let ref attr_key = attr_pair[0];
                let ref attr_val = attr_pair[1];

                self.emit(&attr_key.val);
                self.emit(&"=".to_string());
                self.emit_string(&attr_val.val);

                // We only write a space here if we are not at the end of the attr list.
                // This space separates the attributes.
                counter = counter + 2;
                if counter != attributes.len() {
                    self.emit_space(1);
                }
            }
        }

        self.emit(&CLOSE_SEPARATOR.to_string());
        self.emit_newline();

        self
    }

    fn gen_el_contents(&mut self, ast: &Box<Ast>) -> &Gen {
        let indentation = self.el_stack.len() * INDENTATION_COUNT;
        self.emit_space(indentation + INDENTATION_COUNT);

        self.emit(&ast.val);
        self.emit_newline();

        // TODO: Clone the stack to avoid borrow :/
        let mut stack = self.el_stack.clone();

        // Close all the nested elements so far.
        let mut scope: Option<Scope> = stack.pop();
        while scope.is_some() {
            let name = scope.unwrap();
            self.emit_space(name.indentation);
            self.emit_closing_tag(&name.val);
            scope = stack.pop();
        }

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
        tag = tag + "\n";

        self.emit(&tag);
    }

    fn emit_space(&mut self, count: usize) {
        if count == 0 {
            return;
        }

        let mut spaces = String::from("");
        let mut i = 0;
        while i < count {
            spaces = spaces + " ";
            i = i + 1;
        }

        self.emit(&spaces);
    }

    fn emit_string(&mut self, str_val: &String) {
        let mut val = "\"".to_string();
        val = val + str_val;
        val = val + "\"";

        self.emit(&val);
    }

    fn emit_newline(&mut self) {
        self.emit(&"\n".to_string());
    }
}
