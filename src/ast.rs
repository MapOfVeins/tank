#[derive(Debug)]
pub enum AstType {
    Template
}

#[derive(Debug)]
pub struct Ast {
    pub ast_type: AstType,
    pub val: String,
    pub children: Vec<Box<Ast>>
}

impl Ast {
    pub fn new(t: AstType) -> Ast {
        let mut c = Vec::new();
        Ast {
            ast_type: t,
            val: "".to_string(),
            children: c
        }
    }
}
