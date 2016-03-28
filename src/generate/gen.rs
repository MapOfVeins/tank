use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use syntax::ast::Ast;
use syntax::ast::AstType;
use generate::emit::Emitter;

const EXT: &'static str = ".html";
const INDENTATION_COUNT: usize = 2;

#[derive(Clone)]
struct Scope {
    indentation: usize,
    val: String
}

pub struct Gen {
    emitter: Emitter,
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
        let e = Emitter::new(buf_writer);
        let els = Vec::new();

        Gen {
            emitter: e,
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
        self.emitter.space(indentation);

        self.emitter.left_angle_bracket();
        self.emitter.emit(&ast.val);

        self
    }

    fn gen_attr_list(&mut self, ast: &Box<Ast>) -> &Gen {
        if !ast.children.is_empty() {
            self.emitter.space(1);
            let attributes = ast.children.clone();

            // If we're here, we know we have x number of attribute pairs. We will
            // always have an even number of elements in the attributes vec, otherwise
            // there would have been a parse error. We split the vec into chunks of
            // two and emit them as pairs of identifiers.
            //
            // We define a counter to count the number of pairs we emit. At the end
            // of each pair, we must insert a space to separate the next pair from it.
            // However, at the end of the last pair, we don't want to write a space
            // because the next character inserted will be the closing tag '>'. we
            // therefore only write a space if the counter is the same size as the
            // size of the attributes list.
            let mut counter = 0;

            for attr_pair in attributes.chunks(2) {
                let ref attr_key = attr_pair[0];
                let ref attr_val = attr_pair[1];

                if attr_key.ast_type != AstType::Ident {
                    panic!("tank: Wrong ast type found, expected {:?}, found {:?}",
                           AstType::Ident,
                           attr_key.ast_type);
                }

                if attr_val.ast_type != AstType::Ident {
                    panic!("tank: Wrong ast type found, expected {:?}, found {:?}",
                           AstType::Ident,
                           attr_val.ast_type);
                }

                self.emitter.emit(&attr_key.val);
                self.emitter.equals();
                self.emitter.string(&attr_val.val);

                // We only write a space here if we are not at the end of the attr list.
                // This space separates the attribute pairs.
                counter = counter + 2;
                if counter != attributes.len() {
                    self.emitter.space(1);
                }
            }
        }

        self.emitter.right_angle_bracket();
        self.emitter.newline();

        self
    }

    fn gen_el_contents(&mut self, ast: &Box<Ast>) -> &Gen {
        let indentation = self.el_stack.len() * INDENTATION_COUNT;
        self.emitter.space(indentation + INDENTATION_COUNT);

        self.emitter.emit(&ast.val);
        self.emitter.newline();

        // TODO: Clone the stack to avoid borrow :/
        let mut stack = self.el_stack.clone();

        // Close all the nested elements so far.
        let mut scope: Option<Scope> = stack.pop();
        while scope.is_some() {
            let name = scope.unwrap();
            self.emitter.space(name.indentation);
            self.emitter.close_element(&name.val);
            scope = stack.pop();
        }

        self
    }

    fn gen_empty(&mut self) -> &Gen {
        self.emitter.newline();
        self
    }
}
