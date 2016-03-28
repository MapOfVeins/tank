use std::collections::HashMap;

use ast::Ast;
use ast::AstType;

struct Symbol {
    name: String,
    sym_type: String,
    val: String
}

pub struct SymbolTable {
    table: HashMap<String, Symbol>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let symbols = HashMap::new();

        SymbolTable {
            table: symbols
        }
    }

    /// Expects an ast with a type of 'AssignExpr'. We then check the children
    /// and their types, and then if that identifier already exists. If it does,
    /// we will panic. If not, we construct a new symbol struct and put it in the
    /// symbol table.
    pub fn insert(&mut self, ast: &Box<Ast>) -> &mut SymbolTable {
        if ast.ast_type != AstType::AssignExpr {
            panic!("tank: Invalid ast type found in symbol table");
        }

        if ast.children.len() < 2 {
            panic!("tank: Invalid ast passed to symbol table");
        }

        // Expect the first child to the be the identifier name and the type, and
        // the second child will be the value of the variable.
        let ident = ast.children[0].clone().val;
        let value = ast.children[1].clone().val;

        let ident_type = ast.children[0].clone().var_type.unwrap_or_else(|| {
            panic!("tank: Variable declared without a type");
        });

        match self.table.get(&ident) {
            Some(sym) => panic!("tank: Redeclared symbol {} found", sym.val),
            _ => ()
        };

        let sym = Symbol {
            name: ident.clone(),
            sym_type: ident_type,
            val: value
        };

        self.table.insert(ident, sym);

        self
    }
}
