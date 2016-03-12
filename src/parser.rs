use lexer::Lexer;
use token::Token;
use token::TokenType;
use ast::Ast;
use ast::AstType;

pub struct Parser {
    lexer: Lexer,
    pub token: Option<Token>
}

impl Parser {
    pub fn new(template: String) -> Parser {
        let mut l = Lexer::new(template);
        l.lex();
        let tok = l.curr_tok.take();

        Parser {
            lexer: l,
            token: tok
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut root_ast = Ast::new(AstType::Template);
        root_ast.children.push(self.element());

        root_ast
    }

    fn element(&mut self) -> Box<Ast> {
        
    }
}
