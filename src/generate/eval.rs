use syntax::ast::Ast;
use syntax::symbol_table::SymbolTable;

pub struct Evaluator {
    symbol_table: SymbolTable
}

impl Evaluator {
    pub fn new(m_symbol_table: SymbolTable) -> Evaluator {
        Evaluator {
            symbol_table: m_symbol_table
        }
    }

    pub fn gt(&mut self, ast: &Box<Ast>) -> bool {
        let val_pair = self.unwrap_ast(ast);

        val_pair.0 > val_pair.1
    }

    pub fn gt_equals(&mut self, ast: &Box<Ast>) -> bool {
        let val_pair = self.unwrap_ast(ast);

        val_pair.0 >= val_pair.1
    }

    pub fn lt(&mut self, ast: &Box<Ast>) -> bool {
        let val_pair = self.unwrap_ast(ast);

        val_pair.0 < val_pair.1
    }

    pub fn lt_equals(&mut self, ast: &Box<Ast>) -> bool {
        let val_pair = self.unwrap_ast(ast);

        val_pair.0 <= val_pair.1
    }

    pub fn equals_equals(&mut self, ast: &Box<Ast>) -> bool {
        let val_pair = self.unwrap_ast(ast);

        val_pair.0 == val_pair.1
    }

    pub fn not_equals(&mut self, ast: &Box<Ast>) -> bool {
        let val_pair = self.unwrap_ast(ast);

        val_pair.0 != val_pair.1
    }

    fn unwrap_ast(&mut self, ast: &Box<Ast>) -> (i64, i64) {
        self.validate_ast(ast);

        let first_term = &ast.children[0];
        let second_term = &ast.children[1];

        let symbol = self.symbol_table.get(first_term.val.to_owned()).unwrap_or_else(|| {
           panic!("tank: Invalid expression found, could not find identifier {}",
                  first_term.val);
        });

        let first_val: i64 = symbol.val
            .clone()
            .parse()
            .ok()
            .expect("tank: Expected an integer");

        let second_val: i64 = second_term.val
            .clone()
            .parse()
            .ok()
            .expect("tank: Expected an integer");

        (first_val, second_val)
    }

    fn validate_ast(&self, ast: &Box<Ast>) {
        if ast.children.len() < 2 {
            panic!("tank: Invalid expression ast found, not enough children");
        }
    }
}
