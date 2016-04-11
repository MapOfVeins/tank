use std::collections::HashMap;

use syntax::ast::Ast;
use syntax::ast::AstType;

const GLOBAL_SCOPE: &'static str = "global";
const FOR_SCOPE: &'static str = "for";

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub sym_type: String,
    pub val: String,
    pub scope: String
}

#[derive(Clone)]
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
            val: value,
            scope: GLOBAL_SCOPE.to_owned()
        };

        self.table.insert(ident, sym);

        self
    }

    pub fn insert_for_id(&mut self, ast: &Ast) -> &mut SymbolTable {
        if ast.ast_type != AstType::Ident {
            panic!("tank: Invalid ast type {:?} found", ast.ast_type);
        }

        let ident = ast.clone().val;

        match self.table.get(&ident) {
            Some(sym) => panic!("tank: Redeclared symbol {} found", sym.val),
            _ => ()
        };

        let ident_type = ast.var_type.clone().unwrap_or_else(|| {
            panic!("tank: Variable declared without a type");
        });

        let sym = Symbol {
            name: ident.clone(),
            sym_type: ident_type,
            val: ident.clone(),
            scope: FOR_SCOPE.to_owned()
        };

        self.table.insert(ident, sym);

        self
    }

    /// Wrapper function for getting a Symbol struct from the symbol table. Used for
    /// convenience and so that the symbol_table struct field is left private.
    pub fn get(&mut self, key: String) -> Option<&Symbol> {
        self.table.get(&key)
    }
}
