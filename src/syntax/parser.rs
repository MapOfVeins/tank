use syntax::lexer::Lexer;
use syntax::token::Token;
use syntax::token::TokenType;
use syntax::ast::Ast;
use syntax::ast::AstType;
use syntax::symbol_table::SymbolTable;

pub struct Parser {
    lexer: Lexer,
    pub symbol_table: SymbolTable,
    curr_val: String,
    curr_type: TokenType,
    pub root: Ast
}

impl Parser {
    pub fn new(template: String, symbol_table: SymbolTable) -> Parser {
        let mut m_lexer = Lexer::new(template);
        m_lexer.lex();
        let tok = m_lexer.curr_tok
            .take()
            .unwrap_or(Token::new(TokenType::Eof));

        let m_val = tok.val;
        let m_type = tok.tok_type;
        let m_root = Ast::new(AstType::Template);

        Parser {
            lexer: m_lexer,
            symbol_table: symbol_table,
            curr_val: m_val,
            curr_type: m_type,
            root: m_root
        }
    }

    /// Initiate recursive parsing process. Ast will take the from of Template -> [Element]
    /// here. Template is the top level ast, and should contain any elements that are not
    /// nested in other elements. The parsing process will continually call the lex() method
    /// from the struct's lexer object until EOF is reached.
    pub fn parse(&mut self) -> &mut Parser {
        if self.curr_type == TokenType::Eof {
            panic!("tank: End of input reached, nothing to parse!");
        }

        let el = self.element();
        self.root.children.insert(0, el);

        self
    }

    /// Parse and add an Element ast type to the tree. This method is
    /// recursive in all cases, and will call itself until no input remains.
    /// An element ast in tank can contain an html element, a variable assignment,
    /// an if statement or a for-in statement. In the case that we have no elements
    /// left to parse, we will append an EOF to the ast indicating the end of input.
    fn element(&mut self) -> Box<Ast> {
        let mut el_ast = Ast::new(AstType::Element);
        match self.curr_type {
            TokenType::Ident => {
                match self.curr_val.as_ref() {
                    "if" => {
                        // Consume "if"
                        self.get_next_tok();
                        el_ast = Ast::new(AstType::IfExpr);
                        el_ast.children.push(self.expr());

                        // Consume "{"
                        self.expect(TokenType::LeftBrace);

                        el_ast.children.push(self.element());

                        // Consume "}"
                        self.expect(TokenType::RightBrace);

                        let next = self.element();
                        self.root.children.insert(0, next);
                    },
                    "for" => {
                        // Consume "for"
                        self.get_next_tok();
                        el_ast = Ast::new(AstType::ForExpr);
                        el_ast.children.push(self.term());

                        if self.curr_val != "in" {
                            panic!("tank: Parse error - Expected 'in' at for loop");
                        } else {
                            self.get_next_tok();
                        }

                        el_ast.children.push(self.term());
                        el_ast.children.push(self.element());
                    },
                    "let" => {
                        // Consume "let"
                        self.get_next_tok();
                        let assign_el = self.expr();

                        // Add this variable to the symbol table, and panic
                        // if we already tried to declare it before.
                        self.symbol_table.insert(&assign_el);

                        el_ast.children.push(assign_el);
                        let next = self.element();
                        self.root.children.insert(0, next);
                    },
                    _ => {
                        el_ast.children.push(self.term());

                        if self.curr_type == TokenType::LeftParen {
                            el_ast.children.push(self.attr_list());
                        }

                        // Look ahead and see if we have another element
                        if self.peek() == TokenType::LeftParen {
                            el_ast.children.push(self.element());
                        } else {
                            el_ast.children.push(self.term());
                        }

                        let next = self.element();
                        self.root.children.insert(0, next);
                    }
                };
            },
            TokenType::LeftBrace => {
                //  Consume "{"
                self.get_next_tok();

                el_ast.children.push(self.element());

                // Consume "}"
                self.expect(TokenType::RightBrace);
            },
            _ => {
                el_ast = Ast::new(AstType::Eof);
            }
        }

        Box::new(el_ast)
    }

    /// Parse an attribute list for an html element. An attribute list can contain any number
    /// of desired html attributes, which do not need to be separated by commas (a space is fine).
    /// This method will consume all required punctuation as well.
    fn attr_list(&mut self) -> Box<Ast> {
        let mut attr_ast = Ast::new(AstType::AttrList);

        self.expect(TokenType::LeftParen);

        while self.curr_type != TokenType::RightParen {
            attr_ast.children.push(self.term());

            self.expect(TokenType::Colon);

            attr_ast.children.push(self.term());
        }

        self.expect(TokenType::RightParen);

        self.expect(TokenType::Arrow);

        Box::new(attr_ast)
    }

    /// Parse an intial test inside an expression.
    fn expr(&mut self) -> Box<Ast> {
        let mut test_ast = self.op();
        let curr_ast_type = match self.curr_type {
            TokenType::Gt => AstType::Gt,
            TokenType::Lt => AstType::Lt,
            TokenType::GtEquals => AstType::GtEquals,
            TokenType::LtEquals => AstType::LtEquals,
            TokenType::NotEquals => AstType::NotEquals,
            TokenType::EqualsEquals => AstType::EqualsEquals,
            TokenType::Colon => {
                self.get_next_tok();
                test_ast.var_type = Some(self.curr_val.clone());
                // Consume the type.
                self.get_next_tok();

                AstType::AssignExpr
            },
            TokenType::Equals => {
                self.expect(TokenType::Ident);
                AstType::Empty
            },
            _ => test_ast.ast_type.clone()
        };

        let test_ast_next = test_ast;
        test_ast = Box::new(Ast::new(curr_ast_type));
        self.get_next_tok();

        test_ast.children.push(test_ast_next);
        test_ast.children.push(self.op());

        test_ast
    }

    /// Parse an operation inside an expression.
    fn op(&mut self) -> Box<Ast> {
        let mut op_ast = self.term();

        while self.curr_type == TokenType::Plus || self.curr_type == TokenType::Minus {
            let op_ast_next = op_ast;

            //TODO: Currently, only supporting plus and minus
            let curr_ast_type = match self.curr_type {
                TokenType::Plus => AstType::Plus,
                TokenType::Minus => AstType::Minus,
                _ => AstType::Empty
            };

            op_ast = Box::new(Ast::new(curr_ast_type));
            self.get_next_tok();
            op_ast.children.push(op_ast_next);
            op_ast.children.push(self.term());
        }

        op_ast
    }

    /// Method will parse a term in an expression. This can be a constant identifier
    /// or number, or could also contain another expression.
    fn term(&mut self) -> Box<Ast> {
        let term_ast;
        match self.curr_type {
            TokenType::Ident => {
                // If we find a left paren next, we are declaring an element.
                let m_type = match self.peek() {
                    TokenType::LeftParen => AstType::ElementName,
                    _ => AstType::Ident
                };
                term_ast = Box::new(Ast::new_with_val(m_type, self.curr_val.clone()));
                self.get_next_tok();
            },
            TokenType::Number => {
                term_ast = Box::new(Ast::new_with_val(AstType::Number, self.curr_val.clone()));
                self.get_next_tok();
            },
            TokenType::Eof => {
                term_ast = Box::new(Ast::new(AstType::Eof));
            },
            _ => {
                term_ast = self.expr();
            }
        }

        term_ast
    }

    /// Match the current token to an expected one. If the current token does not equal
    /// the expected one, the parser will panic. Otherwise, we will advance to the next
    /// token and update the parser internals.
    fn expect(&mut self, token: TokenType) {
        if self.curr_type == token {
            self.get_next_tok();
        } else {
            panic!("tank: Parse error - Expected {:?}, found {:?}", token, self.curr_type);
        }
    }

    /// Retrieve the next available token for parsing. This token is retrieved from the lexer's
    /// lex() method. If the next token from the lexer is None, then we return a token
    /// indicating EOF. We then update the internal value and type fields of the Parser
    /// struct.
    fn get_next_tok(&mut self) -> &mut Parser {
        self.lexer.lex();

        let tok = self.lexer.curr_tok
            .take()
            .unwrap_or(Token::new(TokenType::Eof));

        self.curr_val = tok.val;
        self.curr_type = tok.tok_type;

        self
    }

    /// Check the current token but do not consume it.
    fn peek(&self) -> TokenType {
        self.lexer.peek_tok().tok_type
    }
}
