use std::fs::OpenOptions;
use std::io::{BufWriter, Write, Read};
use std::error::Error;
use compile::compiler::Compiler;
use syntax::ast::{Ast, AstType};
use syntax::symbol_table::SymbolTable;
use error::error_traits::Diagnostic;
use error::gen_err::GenDiagnostic;
use generate::emit::Emitter;
use generate::eval::Evaluator;

const EXT: &'static str = ".html";
const TANK_EXT: &'static str = ".tank";
const INDENTATION_COUNT: usize = 2;

#[derive(Clone, Debug)]
struct Scope {
    /// Number of spaces to write before generation
    indentation: usize,
    /// Symbol name of this scope
    val: String
}

pub struct Gen {
    /// Emitter struct handle file writing operations
    emitter: Emitter,
    /// Evaluates expressions to determine if code needs to be generated
    eval: Evaluator,
    /// Stack of elements used to determine scope
    el_stack: Vec<Scope>,
    /// Error and warning information
    pub diagnostic: GenDiagnostic
}

impl Gen {
    pub fn new(filename: &String, symbol_table: SymbolTable) -> Gen {
        let mut options = OpenOptions::new();
        options.write(true);
        options.create(true);
        options.append(true);
        options.truncate(true);

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
            el_stack: m_el_stack,
            diagnostic: GenDiagnostic::new()
        }
    }

    /// Generate the contents of an HTML template from the given ast. The contents are written
    /// to the file provided when creating the generator.  This function will panic if the ast
    /// does not contain a template, or if the ast is empty.
    pub fn output(&mut self, template: &Ast) {
        if template.ast_type != AstType::Template {
            let err_str = format!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                                  template.ast_type,
                                  AstType::Template);

            self.diagnostic.new_err(&err_str);
        }

        if template.children.is_empty() {
            self.diagnostic.new_err(&"tank: Empty ast found, nothing to generate.");
        }

        for ast in &template.children {
            // Clear out the element stack, in case the last un-nested element is left over.
            self.el_stack.clear();
            self.expr_or_element(&ast);
        }
    }

    /// Determines if we are currently generating an element or an expression. Simply
    /// calls the approriate gen function based on the type of the ast.
    fn expr_or_element(&mut self, ast: &Box<Ast>) -> &Gen {
        match ast.ast_type {
            AstType::Element => self.gen_element(ast),
            AstType::IfExpr => self.gen_if(ast),
            AstType::ForExpr => self.gen_for(ast),
            AstType::Include => self.gen_include(ast),
            _ => self.gen_empty()
        };

        self
    }

    /// Given an Element ast, generate the html contents from it and write to file.
    /// Expects the given ast to be of type Element, and contain at least 1 child.
    /// This functions will then be recursively called if the contents of this element
    /// contain another element.
    fn gen_element(&mut self, ast: &Box<Ast>) -> &Gen {
        if ast.ast_type != AstType::Element {
            let err_str = format!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                                  ast.ast_type,
                                  AstType::Element);

            self.diagnostic.new_err(&err_str);
        }

        if ast.children.len() == 0 {
            self.diagnostic.fatal("Invalid element found, no children present in ast");
        }

        let first_child = &ast.children[0];

        // Nothing to generate if we are doing an assignment. Variable value is
        // already in symbol table.
        if first_child.ast_type == AstType::AssignExpr {
            return self;
        }

        // We expect the first child to be ElementName with element name,
        // second child is attribute list, third child is another element,
        // containing either the contents or a nested element.
        // We should be guaranteed to have at least 3 ast types in the children vector.
        if ast.children.len() < 3 {
            self.diagnostic.fatal("Invalid Element ast found, not enough children present");
        }

        self.gen_el_name(&ast.children[0]);
        self.gen_attr_list(&ast.children[1]);

        match ast.children[2].ast_type {
            AstType::Element => self.gen_element(&ast.children[2]),
            AstType::Contents | AstType::VariableValue => self.gen_el_contents(&ast.children[2]),
            AstType::Include => self.gen_include(&ast.children[2]),
            AstType::Eof => self.gen_empty(),
            _ => {
                self.diagnostic.fatal("Unexpected ast type found");
                self.gen_empty()
            }
        };

        self
    }

    /// Evaluate and in statement and then generate the result. Evaluation of the
    /// provided ast is performed and if the if-statement conditions are not met,
    /// we skip the generation phase so that the contents of the if-statement
    /// are never written to file.
    fn gen_if(&mut self, ast: &Box<Ast>) -> &Gen {
        if ast.ast_type != AstType::IfExpr {
            let err_str = format!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                                  ast.ast_type,
                                  AstType::IfExpr);

            self.diagnostic.new_err(&err_str);
        }

        // Expect the first child to be the comparison operator in the if expression,
        // which itself should have two children containing the left hand term and
        // right hand term of the expression.
        //
        // Following this, we expect another element or expression which is contained
        // inside the if block.
        if ast.children.len() == 0 {
            self.diagnostic.fatal("Invalid ast found, no children for if expression");
        }

        let expr = &ast.children[0];

        if expr.children.len() < 2 {
            self.diagnostic.fatal("Invalid expression found, not enough children in if expression");
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

    fn gen_for(&mut self, ast: &Box<Ast>) -> &Gen {
        if ast.ast_type != AstType::ForExpr {
            let err_str = format!("tank: Invalid ast provided to generator. Found {:?}, expected {:?}",
                                  ast.ast_type,
                                  AstType::ForExpr);

            self.diagnostic.new_err(&err_str);
        }

        // Expect there to be at least 2 children, each identifier in the
        // "for" "in" declaration.  Technically the for block can be empty, but normally
        // we would also expect a third child which is the contents of the block.
        if ast.children.len() < 2 {
            self.diagnostic.fatal("Invalid ast found, not enough children found in for expression");
        }

        let second_ident = &ast.children[1];

        // Ensure that the second ident is in symbol table. We should have access to this
        // variable from an inputted file, since we can't yet declare array types inside
        // tank files.
        match self.eval.symbol_table.get(second_ident.val.clone()) {
            Some(_) => (),
            None => {
                let err_str = format!("tank: Error - variable {} is undefined.",
                                      second_ident.val);
                self.diagnostic.new_err(&err_str);
            }
        };

        if ast.children.get(2).is_none() {
            return self;
        }

        let containing_element = &ast.children[2];
        // TODO: Insert elements for each loop in this for-in block. Will require
        // passing in values from the data source and assigning them in the symbol table.
        self.expr_or_element(containing_element);

        self
    }

    /// Try to open the file with a .html extension. If this file exists,
    /// we assume that we have html already so we can read it into a string
    /// and insert it.
    ///
    /// If the file doesn't exist, then we need to try and open the corresponding
    /// tank template, compile it, and then open the html file and write the contents
    /// to this file.
    /// If we can't find the .tank file, then we panic.
    fn gen_include(&mut self, ast: &Box<Ast>) -> &Gen {

        let mut is_compile = false;
        let filename = ast.val.to_owned();
        let html_filename = filename.to_owned() + EXT;

        let mut options = OpenOptions::new();
        options.read(true);

        let mut file = match options.open(&html_filename) {
            Ok(file) => file,
            Err(..) => {
                let tank_filename = filename + TANK_EXT;
                let tank_file = match options.open(&tank_filename) {
                    Ok(tank_file) => tank_file,
                    Err(error) => panic!("tank: Unable to open file {}: {}",
                                         tank_filename,
                                         Error::description(&error))
                };
                is_compile = true;

                tank_file
            }
        };

        if is_compile {
            // Create a new compiler struct and use it to compile
            // the referenced .tank file.
            let tank_filename = ast.val.to_owned() + TANK_EXT;
            let mut compiler = Compiler::new(&mut file, &tank_filename);

            compiler.compile();
        } else {
            // read html file to string and then insert its contents into this file.
            let mut inserted_html = String::new();

            match file.read_to_string(&mut inserted_html) {
                Err(error) => panic!("Failed to read file: {}", Error::description(&error)),
                Ok(_) => ()
            }

            // Generate the html from the referenced file and clear the element stack.
            self.emitter.emit(&inserted_html);
            self.clear_element_stack();
        }

        self
    }

    /// Write the name of an element to file, as well as pushes the name on to the
    /// element stack. The stack is used to keep track of nested elements.
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

    /// Write all the attribute name-value pairs to file, as well as additional
    /// characters before the contents of the element. This function will panic
    /// if non-identifier types are found in the attribute list.
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
                    let err_str = format!("tank: Wrong ast type found, expected {:?}, found {:?}",
                                          AstType::Ident,
                                          attr_key.ast_type);
                    self.diagnostic.fatal(&err_str);
                }

                if attr_val.ast_type != AstType::Ident {
                    let err_str = format!("tank: Wrong ast type found, expected {:?}, found {:?}",
                                          AstType::Ident,
                                          attr_val.ast_type);
                    self.diagnostic.fatal(&err_str);
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

    /// Write the contents of an element to file, and also write all applicable closing
    /// tags. This is done by popping the values in the scope stack until the stack is
    /// empty.
    fn gen_el_contents(&mut self, ast: &Box<Ast>) -> &Gen {
        let mut contents_str = String::new();

        for child in ast.children.to_owned() {
            match child.ast_type {
                AstType::Ident => {
                    contents_str = contents_str + " " + &child.val;
                },
                AstType::VariableValue => {
                    let true_val = self.get_var_val(&child.val);
                    contents_str = contents_str + " " + &true_val;
                },
                _ => {
                    let err_str = format!("tank: Unexpected ast type {:?} found in element contents",
                                          child.ast_type);
                    self.diagnostic.fatal(&err_str);
                }
            };
        }

        let indentation = self.el_stack.len() * INDENTATION_COUNT;
        self.emitter.space(indentation + INDENTATION_COUNT);

        self.emitter.emit(&contents_str.trim_left());
        self.emitter.newline();

        self.clear_element_stack();

        self
    }

    /// Retrieve the value of a variable from the symbol table, and panic
    /// if the var_name passed in does not exist in the symbol table.
    fn get_var_val(&mut self, var_name: &String) -> String {
        match self.eval.symbol_table.get(var_name.to_owned()) {
            Some(symbol) => symbol.val.to_owned(),
            None => {
                let err_str = format!("tank: Invalid variable '{}' referenced", var_name);
                self.diagnostic.new_err(&err_str);
                String::new()
            }
        }
    }

    /// Clears all the nested element scopes in the element stack. This function will
    /// write the closing element tag of each scope found in the stack.
    fn clear_element_stack(&mut self) -> &Gen {
        let mut scope: Option<Scope> = self.el_stack.pop();

        while scope.is_some() {
            let name = scope.unwrap();
            self.emitter.space(name.indentation);
            self.emitter.close_element(&name.val);
            scope = self.el_stack.pop();
        }

        self
    }

    fn gen_empty(&mut self) -> &Gen {
        self
    }
}
