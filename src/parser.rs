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

        if self.curr_val == "let" {
            el_ast.children.push(self.assign_expr());
            // We expect to hit another element here
            self.get_next_tok();
            el_ast.children.push(self.element());
        } else if self.lexer.is_next_paren() {
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

    fn number(&mut self) -> Box<Ast> {
        if self.curr_type != TokenType::Number {
            panic!("tank: Parse error - Expected number");
        }

        let num_ast = Ast::new_with_val(AstType::Number, self.curr_val.clone());

        Box::new(num_ast)
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

    // Expects current token to be "let" when called.
    fn assign_expr(&mut self) -> Box<Ast> {
        // Advance to var name
        self.get_next_tok();

        if self.curr_type != TokenType::Ident {
            panic!("tank: Parse error - Expected an identifier");
        }

        let mut assign_ast = Ast::new_with_val(AstType::AssignExpr, self.curr_val.clone());

        // Advance to type declaration
        self.get_next_tok();

        if self.curr_type != TokenType::Colon {
            panic!("tank: Parse error - Expected ':'");
        }

        // Advance to type name
        self.get_next_tok();

        if self.curr_type != TokenType::Ident {
            panic!("tank: Parse error - Expected an identifier");
        }

        // Check if type is valid (ie. exists)
        match self.lexer.reserved.words.get(&self.curr_val) {
            Some(_) => (),
            None => panic!("tank: Invalid type provided for variable declaration")
        }

        assign_ast.var_type = Some(self.curr_val.clone());

        // advance to equals symbol
        self.get_next_tok();

        if self.curr_type != TokenType::Equals {
            panic!("tank: Invalid assignment expression");
        }

        // advance to assignment value
        self.get_next_tok();

        // TODO: normal expression parsing in assignment statements
        match self.curr_type {
            TokenType::Ident => assign_ast.children.push(self.ident()),
            TokenType::Number => assign_ast.children.push(self.number()),
            _ => panic!("tank: invalid variable assignment value provided")
        }

        Box::new(assign_ast)
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
