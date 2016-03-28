use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use syntax::ast::Ast;
use syntax::ast::AstType;
use syntax::symbol_table::SymbolTable;
use generate::emit::Emitter;
use generate::eval::Evaluator;

const EXT: &'static str = ".html";
const INDENTATION_COUNT: usize = 2;

#[derive(Clone, Debug)]
struct Scope {
    indentation: usize,
    val: String
}

pub struct Gen {
    emitter: Emitter,
    eval: Evaluator,
    el_stack: Vec<Scope>
}

impl Gen {
    pub fn new(filename: &String, symbol_table: SymbolTable) -> Gen {
        let mut options = OpenOptions::new();
        options.write(true);
        options.create(true);
        options.append(true);

        let file = match options.open(filename.to_owned() + EXT) {
            Ok(file) => file,
            Err(..) => panic!("tank: unable to open file {}", filename)
        };

        let buf_writer = BufWriter::new(file);
        let m_emitter = Emitter::new(buf_writer);
        let m_el_stack = Vec::new();

        let m_eval = Evaluator::new(symbol_table);

        Gen {
            emitter: m_emitter,
            eval: m_eval,
            el_stack: m_el_stack
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
            // Clear out the element stack, in case the last un-nested element is left over.
            self.el_stack.clear();
            self.expr_or_element(&ast);
        }
    }

    fn expr_or_element(&mut self, ast: &Box<Ast>) -> &Gen {
        match ast.ast_type {
            AstType::Element => self.gen_element(ast),
            AstType::IfExpr => self.gen_if(ast),
            _ => self.gen_empty()
        };

        self
    }

    fn gen_element(&mut self, ast: &Box<Ast>) -> &Gen {
        if ast.ast_type != AstType::Element {
            panic!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                   ast.ast_type,
                   AstType::Element);
        }

        if ast.children.len() == 0 {
            panic!("tank: Invalid element found, no children present in ast");
        }

        let first_child = &ast.children[0];

        // Nothing to generate if we are doing an assignment. Variable value is
        // already in symbol table.
        // TODO: type check here maybe?
        if first_child.ast_type == AstType::AssignExpr {
            return self;
        }

        // We expect the first child to be ElementName with element name,
        // second child is attribute list, third child is another element,
        // containing either the contents or a nested element.
        // We should be guaranteed to have at least 3 ast types in the children vector.
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

    fn gen_if(&mut self, ast: &Box<Ast>) -> &Gen {
        if ast.ast_type != AstType::IfExpr {
            panic!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                   ast.ast_type,
                   AstType::IfExpr);
        }

        // Expect the first child to be the comparison operator in the if expression,
        // which itself should have two children containing the left hand term and
        // right hand term of the expression.
        //
        // Following this, we expect another element or expression which is contained
        // inside the if block.
        if ast.children.len() == 0 {
            panic!("tank: Invalid ast found, no children for if expression");
        }

        let expr = &ast.children[0];

        if expr.children.len() < 2 {
            panic!("tank: Invalid expression ast found, not enough children in if expression");
        }

        let is_gen = match expr.ast_type {
            AstType::Gt => self.eval.gt(expr),
            AstType::GtEquals => self.eval.gt_equals(expr),
            AstType::Lt => self.eval.lt(expr),
            AstType::LtEquals => self.eval.lt_equals(expr),
            AstType::EqualsEquals => self.eval.equals_equals(expr),
            AstType::NotEquals => self.eval.not_equals(expr),
            _ => false
        };

        if is_gen {
            let element = &ast.children[1];
            self.expr_or_element(element);
        }

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
        self
    }
}
