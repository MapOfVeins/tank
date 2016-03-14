#[derive(Debug)]
pub enum AstType {
    Template,
    Element,
    Ident,
    Number,
    AttrList,
    ElContent,
    AssignExpr
}

#[derive(Debug)]
pub struct Ast {
    pub ast_type: AstType,
    pub val: String,
    pub children: Vec<Box<Ast>>,
    pub var_type: Option<String>
}

impl Ast {
    pub fn new(t: AstType) -> Ast {
        let c = Vec::new();
        Ast {
            ast_type: t,
            val: "".to_string(),
            children: c,
            var_type: None
        }
    }

    /// "Overloaded" new function, used to create a new ast node
    /// but with a defined value instead of using an empty string,
    /// as in the above new() method.
    pub fn new_with_val(t: AstType, v: String) -> Ast {
        let c = Vec::new();
        Ast {
            ast_type: t,
            val: v,
            children: c,
            var_type: None
        }
    }
}
