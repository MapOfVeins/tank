use lexer::Lexer;
use token::Token;
use token::TokenType;
use ast::Ast;
use ast::AstType;

pub struct Parser {
    lexer: Lexer,
    pub token: Option<Token>,
    curr_val: String,
    curr_type: TokenType
}

impl Parser {
    pub fn new(template: String) -> Parser {
        let mut l = Lexer::new(template);
        l.lex();
        let tok = l.curr_tok
            .take()
            .unwrap_or(Token::new(TokenType::Eof, "".to_string()));

        let tv = tok.val;
        let tt = tok.tok_type;

        Parser {
            lexer: l,
            curr_val: tv,
            curr_type: tt,
            token: None
        }
    }

    pub fn parse(&mut self) -> Ast {
        if self.curr_type == TokenType::Eof {
            panic!("tank: End of input reached, nothing to parse!");
        }
        
        let mut root_ast = Ast::new(AstType::Template);
        root_ast.children.push(self.element());

        root_ast
    }

    fn element(&mut self) -> Box<Ast> {
        let mut el_ast = Ast::new(AstType::Element);

        match self.curr_type {
            TokenType::Ident => el_ast.children.push(self.ident()),
            _ => panic!("tank: Illegal character found, expected identifier")
        }

        self.get_next_tok();
        self.attr_list();

        Box::new(el_ast)
    }

    // Expects the current token to be the identifier
    fn ident(&mut self) -> Box<Ast> {
        if self.curr_type != TokenType::Ident {
            panic!("tank: Parse error - Expected identifier");
        }
        
        let ident_ast = Ast::new_with_val(AstType::Ident, self.curr_val.clone());

        Box::new(ident_ast)
    }

    fn attr_list(&mut self) -> Box<Ast> {
        let mut attr_ast = Ast::new(AstType::AttrList);

        if self.curr_type == TokenType::LeftParen {
            self.get_next_tok();
        } else {
            panic!("tank: Parse error - Expected '('")
        }

        while self.curr_type != TokenType::RightParen {
            attr_ast.children.push(self.ident());
            
            self.get_next_tok();

            if self.curr_type == TokenType::Colon {
                self.get_next_tok();
            } else {
                panic!("tank: Parse error - Expected ':'")
            }

            attr_ast.children.push(self.ident());
            self.get_next_tok();
        }

        if self.curr_type == TokenType::RightParen {
            self.get_next_tok();
        } else {
            panic!("tank: Parse error - Expected ')'");
        }
        
        println!("{:?}", self.curr_type);

        if self.curr_type == TokenType::Arrow {
            self.get_next_tok();
        } else {
            panic!("tank: Parse error - Expected '->'");
        }

        println!("{:?}", self.curr_type);

        attr_ast.children.push(self.element());
        
        Box::new(attr_ast)
    }

    fn if_expr(&mut self) -> Box<Ast> {
        Box::new(Ast::new(AstType::Ident))
    }

    fn for_expr(&mut self) -> Box<Ast> {
        Box::new(Ast::new(AstType::Ident))
    }

    fn assign_expr(&mut self) -> Box<Ast> {
        Box::new(Ast::new(AstType::Ident))
    }

    fn get_next_tok(&mut self) -> &mut Parser {
        self.lexer.lex();

        let tok = self.lexer.curr_tok
            .take()
            .unwrap_or(Token::new(TokenType::Eof, "".to_string()));

        self.curr_val = tok.val;
        self.curr_type = tok.tok_type;

        self
    }
}
