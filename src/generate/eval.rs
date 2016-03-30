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

    // TODO: support more complex expressions in if statements
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

#[cfg(test)]
mod tests {
    use super::*;
    use syntax::ast::Ast;
    use syntax::ast::AstType;
    use syntax::symbol_table::SymbolTable;

    const IDENT_NAME: &'static str = "ident";
    const IDENT_VAL: &'static str = "10";

    fn setup() -> Evaluator {
        let mut table = SymbolTable::new();
        let mut ident = Ast::new(AstType::AssignExpr);

        let mut var = Ast::new_from_value(AstType::Ident, IDENT_NAME);
        var.var_type = Some(String::from("int"));

        let var_val = Ast::new_from_value(AstType::Number, IDENT_VAL);

        ident.children.push(Box::new(var));
        ident.children.push(Box::new(var_val));

        table.insert(&Box::new(ident));

        Evaluator::new(table)
    }

    #[test]
    fn test_eval_gt_false() {
        let mut eval = setup();

        let mut expr_ast = Ast::new(AstType::Gt);
        let ident = Ast::new_from_value(AstType::Ident, IDENT_NAME);
        let value = Ast::new_from_value(AstType::Number, "11");

        expr_ast.children.push(Box::new(ident));
        expr_ast.children.push(Box::new(value));

        assert_eq!(eval.gt(&Box::new(expr_ast)), false);
    }

    #[test]
    fn test_eval_gt_when_true() {
        let mut eval = setup();

        let mut expr_ast = Ast::new(AstType::Gt);
        let ident = Ast::new_from_value(AstType::Ident, IDENT_NAME);
        let value = Ast::new_from_value(AstType::Number, "9");

        expr_ast.children.push(Box::new(ident));
        expr_ast.children.push(Box::new(value));

        assert_eq!(eval.gt(&Box::new(expr_ast)), true);
    }

    #[test]
    fn test_eval_gt_equals() {
        let mut eval = setup();

        let mut expr_ast = Ast::new(AstType::GtEquals);
        let ident = Ast::new_from_value(AstType::Ident, IDENT_NAME);
        let value = Ast::new_from_value(AstType::Number, "11");

        expr_ast.children.push(Box::new(ident));
        expr_ast.children.push(Box::new(value));

        assert_eq!(eval.gt_equals(&Box::new(expr_ast)), false);
    }

    #[test]
    fn test_eval_gt_equals_when_equal() {
        let mut eval = setup();

        let mut expr_ast = Ast::new(AstType::GtEquals);
        let ident = Ast::new_from_value(AstType::Ident, IDENT_NAME);
        let value = Ast::new_from_value(AstType::Number, "10");

        expr_ast.children.push(Box::new(ident));
        expr_ast.children.push(Box::new(value));

        assert_eq!(eval.gt_equals(&Box::new(expr_ast)), true);
    }

    #[test]
    fn test_eval_lt() {
        let mut eval = setup();

        let mut expr_ast = Ast::new(AstType::Lt);
        let ident = Ast::new_from_value(AstType::Ident, IDENT_NAME);
        let value = Ast::new_from_value(AstType::Number, "11");

        expr_ast.children.push(Box::new(ident));
        expr_ast.children.push(Box::new(value));

        assert_eq!(eval.lt(&Box::new(expr_ast)), true);
    }

    #[test]
    fn test_eval_lt_equals() {
        let mut eval = setup();

        let mut expr_ast = Ast::new(AstType::LtEquals);
        let ident = Ast::new_from_value(AstType::Ident, IDENT_NAME);
        let value = Ast::new_from_value(AstType::Number, "9");

        expr_ast.children.push(Box::new(ident));
        expr_ast.children.push(Box::new(value));

        assert_eq!(eval.lt_equals(&Box::new(expr_ast)), false);
    }

    #[test]
    fn test_eval_equals_equals() {
        let mut eval = setup();

        let mut expr_ast = Ast::new(AstType::EqualsEquals);
        let ident = Ast::new_from_value(AstType::Ident, IDENT_NAME);
        let value = Ast::new_from_value(AstType::Number, "11");

        expr_ast.children.push(Box::new(ident));
        expr_ast.children.push(Box::new(value));

        assert_eq!(eval.equals_equals(&Box::new(expr_ast)), false);
    }

    #[test]
    fn test_eval_not_equals() {
        let mut eval = setup();

        let mut expr_ast = Ast::new(AstType::Gt);
        let ident = Ast::new_from_value(AstType::Ident, IDENT_NAME);
        let value = Ast::new_from_value(AstType::Number, "11");

        expr_ast.children.push(Box::new(ident));
        expr_ast.children.push(Box::new(value));

        assert_eq!(eval.not_equals(&Box::new(expr_ast)), true);
    }
}
