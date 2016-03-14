use lexer::Lexer;
use token::Token;
use token::TokenType;
use ast::Ast;
use ast::AstType;

pub struct Parser {
    lexer: Lexer,
    curr_val: String,
    curr_type: TokenType
}

impl Parser {
    pub fn new(template: String) -> Parser {
        let mut l = Lexer::new(template);
        l.lex();
        let tok = l.curr_tok
            .take()
            .unwrap_or(Token::new(TokenType::Eof));

        let tv = tok.val;
        let tt = tok.tok_type;

        Parser {
            lexer: l,
            curr_val: tv,
            curr_type: tt,
        }
    }

    pub fn parse(&mut self) -> Ast {
        if self.curr_type == TokenType::Eof {
            panic!("tank: End of input reached, nothing to parse!");
        }
        
        let mut root_ast = Ast::new(AstType::Template);
        root_ast.children.push(self.element());

        println!("{:?}", root_ast);

        root_ast
    }

    fn element(&mut self) -> Box<Ast> {
        if self.curr_type != TokenType::Ident {
            panic!("tank: Illegal character found, expected identifier")            
        }
        
        let mut el_ast = Ast::new(AstType::Element);

        if self.lexer.is_next_paren() {
            el_ast.children.push(self.ident());
            self.get_next_tok();
            el_ast.children.push(self.attr_list());
            // Here, we expect the element contents or the next element
            el_ast.children.push(self.element());
        } else {
            let mut content = String::new();
            while self.curr_type == TokenType::Ident {
                content = content + &self.curr_val;
                self.get_next_tok();
            }
            
            // We re-assign el_ast to be content now, instead
            // of pushing this content into the children of a
            // new element.
            el_ast = self.el_content(content);
        }

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

    fn el_content(&mut self, content_val: String) -> Ast {        
        let content_ast = Ast::new_with_val(AstType::ElContent, content_val);

        content_ast
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
        
        if self.curr_type == TokenType::Arrow {
            self.get_next_tok();
        } else {
            panic!("tank: Parse error - Expected '->'");
        }

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
            .unwrap_or(Token::new(TokenType::Eof));

        self.curr_val = tok.val;
        self.curr_type = tok.tok_type;

        self
    }
}
