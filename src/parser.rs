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
        let start_tok = self.token.unwrap_or_else(|| {
           panic!("tank: End of input reached, nothing to parse!");
        });

        if start_tok.tok_type ==  TokenType::Eof {
            panic!("tank: End of input reached, nothing to parse!");
        }
        
        let mut root_ast = Ast::new(AstType::Template);
        root_ast.children.push(self.element());

        root_ast
    }

    fn element(&mut self) -> Box<Ast> {
        let mut el_ast: Ast;
        let curr_tok = self.token.unwrap();

        match curr_tok.tok_type {
            TokenType::Ident => el_ast.children.push(self.decide_reserved_or_ident()),
            _ => panic!("tank: Illegal character found, expected identifier")
        }

        Box::new(el_ast);
    }

    fn decide_reserved_or_ident(&mut self) -> Box<Ast> {
        let curr_tok = self.token.unwrap();
        let ident_ast = Ast::new_with_val(AstType::Ident, curr_tok.val);

        if curr_tok.is_reserved {
            let num = match self.lexer.reserved.words.get(&curr_tok.val) {
                Some(num) => num,
                None => panic!("tank: Unrecognized reserved word found.")
            };

            match *num {
                0 => ident_ast.children.push(self.if_expr()),
                1 => ident_ast.children.push(self.assign_expr()),
                2 => ident_ast.children.push(self.for_expr()),
                _ => panic!("tank: Unrecognized reserved word found.")
            }
        } else {
            // TODO: element content can have spaces, need to loop on that
            // to get all content in ast nodes.
            if curr_tok.tok_type == TokenType::LeftParen {
                ident_ast.children.push(self.attr_list());
            }
        }

        Box::new(ident_ast)
    }

    fn attr_list(&mut self) -> Box<Ast> {
        self.get_next_tok();
        let attr_ast = Ast::new(AstType::AttrList);
        let mut curr_tok = self.token.unwrap();

        if curr_tok.tok_type == TokenType::LeftParen {
            self.get_next_tok();
        } else {
            panic!("tank: Parse error - Expected '('")
        }

        while curr_tok.tok_type != TokenType::RightParen {
            curr_tok = self.token.unwrap();
            attr_ast.children.push(self.ident());
            
            self.get_next_tok();
            curr_tok = self.token.unwrap();

            if curr_tok.tok_type == TokenType::Colon {
                self.get_next_tok();
            } else {
                panic!("tank: Parse error - Expected ':'")
            }

            attr_ast.children.push(self.ident());
            self.get_next_tok();
            curr_tok = self.token.unwrap();
        }

        if curr_tok.tok_type == TokenType::RightParen {
            self.get_next_tok();
        } else {
            panic!("tank: Parse error - Expected ')'");
        }

        curr_tok = self.token.unwrap();
        if curr_tok.tok_type == TokenType::Arrow {
            self.get_next_tok();
        } else {
            panic!("tank: Parse error - Expected '->'");
        }

        attr_ast.children.push(self.element());
        
        Box::new(attr_ast)
    }

    // Expects the current token to be the identifier
    fn ident(&mut self) -> Box<Ast> {
        let curr_tok = self.token.unwrap();

        if curr_tok.tok_type != TokenType::Ident {
            panic!("tank: Parse error - Expected identifier");
        }
        
        Box::new(Ast::new_with_val(AstType::Ident, curr_tok.val))
    }

    fn if_expr(&mut self) -> Box<Ast> {
        
    }

    fn for_expr(&mut self) -> Box<Ast> {
        
    }

    fn assign_expr(&mut self) -> Box<Ast> {
        
    }

    fn get_next_tok(&mut self) -> &mut Parser {
        self.lexer.lex();

        let curr_tok = self.lexer.curr_tok.take();
        self.token = curr_tok;

        self
    }
}
