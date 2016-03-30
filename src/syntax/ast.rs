#[derive(Debug, PartialEq, Clone)]
pub enum AstType {
    Template,
    Element,
    Ident,
    ElementName,
    Number,
    AttrList,
    IfExpr,
    ForExpr,
    AssignExpr,
    Plus,
    Minus,
    EqualsEquals,
    Gt,
    Lt,
    GtEquals,
    LtEquals,
    NotEquals,
    Empty,
    Eof
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub ast_type: AstType,
    pub val: String,
    pub var_type: Option<String>,
    pub children: Vec<Box<Ast>>
}

impl Ast {
    pub fn new(t: AstType) -> Ast {
        let c = Vec::new();
        Ast {
            ast_type: t,
            val: "".to_owned(),
            var_type: None,
            children: c
        }
    }

    /// "Overloaded" new function, used to create a new ast node
    /// but with a defined value instead of using an empty string,
    /// as in the above new() method.
    pub fn new_from_value(t: AstType, v: &str) -> Ast {
        let c = Vec::new();
        Ast {
            ast_type: t,
            val: v.to_owned(),
            var_type: None,
            children: c
        }
    }
}
