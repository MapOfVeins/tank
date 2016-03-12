use token::Token;
use token::TokenType;
use reserved::Reserved;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    pub reserved: Reserved,
    pub curr_tok: Option<Token>,
    pub curr_char: Option<char>,
    char_count: usize
}

impl Lexer {
    pub fn new(file_contents: String) -> Lexer {
        let r = Reserved::new();

        Lexer {
            input: file_contents,
            reserved: r,
            curr_tok: None,
            curr_char: None,
            char_count: 0
        }
    }

    /// Lex the input at the current point, represented by the char_count field.
    /// Once lexing is complete, sets the curr_tok field to the correct token.
    ///
    /// After this function is called, the curr_tok field should never be None.
    /// If we cannot find a proper token during lexing, we will return the token
    /// for EOF.
    pub fn lex(&mut self) -> &mut Lexer {
        self.get_char();

        if self.curr_char.is_none() {
            self.curr_tok = Some(Token::new(TokenType::Eof, "".to_string()));
            return self;
        }

        // curr_char is guaranteed to be Some here.
        let mut ch = self.curr_char.unwrap();

        // TODO: preserve whitespace for proper generation?
        while ch.is_whitespace() {
            self.get_char();
            if self.curr_char.is_none() {
                self.curr_tok = Some(Token::new(TokenType::Eof, "".to_string()));
                return self;
            }
        
            ch = self.curr_char.unwrap();
        }

        match ch {
            '{' => self.curr_tok = self.get_token(TokenType::LeftBrace),
            '}' => self.curr_tok = self.get_token(TokenType::RightBrace),
            '(' => self.curr_tok = self.get_token(TokenType::LeftParen),
            ')' => self.curr_tok = self.get_token(TokenType::RightParen),
            ':' => self.curr_tok = self.get_token(TokenType::Colon),
            '=' => self.curr_tok = self.get_token(TokenType::Equals),
            '-' => self.curr_tok = self.get_minus_or_arrow(),
            _   => self.curr_tok = self.lex_word_or_number()
        }

        self
    }

    /// Returns the next available char from the file contents. If no
    /// char is available (ie. at end of input, or when the char_count
    /// field is greater than the number of chars in the file), then
    /// None is returned.
    fn get_char(&mut self) -> &mut Lexer {
        // O(n)
        match self.input.chars().nth(self.char_count) {
            Some(c) => self.curr_char = Some(c),
            None => self.curr_char = None
        }

        self.char_count = self.char_count + 1;

        self
    }

    /// Retrieve a new token based on the current character, or None
    /// if no curr_char exists. This function is only used for tokens
    /// that have single character values.
    ///
    /// After a token is created, we advance the current char pointer.
    fn get_token(&mut self, token_type: TokenType) -> Option<Token> {
        let tok = match self.curr_char {
            Some(c) => Some(Token::new(token_type, c.to_string())),
            None => Some(Token::new(TokenType::Eof, String::new()))
        };

        self.get_char();

        tok
    }

    /// Called when a '-' characted is encountered. Checks for a following
    /// '>' character, then returns a token for either a minus sign or an arrow.
    fn get_minus_or_arrow(&mut self) -> Option<Token> {
        let tok = match self.peek() {
            Some('>') => Some(Token::new(TokenType::Arrow, "->".to_string())),
            _ => Some(Token::new(TokenType::Minus, "-".to_string())),
        };

        tok
    }

    /// When we encounter a potential identifier, we continue lexing here
    /// to build the full string or number. The char pointer will be advanced
    /// to the end of the word.
    ///
    /// Also checks for reserved words if we have a valid identifier. Sets the
    /// proper token field to true if the word is reserved, then we can deal with
    /// it during parsing.
    fn lex_word_or_number(&mut self) -> Option<Token> {
        let mut ch = self.curr_char.unwrap_or(' ');
        let mut ident = ch.to_string();
        let mut tok = None;

        if ch.is_alphabetic() {
            self.get_char();

            while ch.is_alphanumeric() {
                let append = self.curr_char.unwrap_or(' ');
                if !append.is_whitespace() {
                    ident = ident + &append.to_string();
                }

                self.get_char();
                ch = append;
            }

            let i = ident.clone();
            let mut some_tok = Token::new(TokenType::Ident, i);

            // Match on reserved words
            // TODO: separate checking for types here?
            match self.reserved.words.get(&ident) {
                Some(num) => some_tok.set_reserved(true),
                None => some_tok.set_reserved(false)
            };

            tok = Some(some_tok);

        } else if ch.is_digit(10) {
            self.get_char();

            while ch.is_digit(10) {
                let append = self.curr_char.unwrap_or(' ');
                if !append.is_whitespace() {
                    ident = ident + &append.to_string();
                }

                self.get_char();
                ch = append;
            }

            tok = Some(Token::new(TokenType::Number, ident));
        } else {
            tok = Some(Token::new(TokenType::Eof, "".to_string()));
        }

        tok
    }

    /// Get the next char to be lexed. Does not consume the char,
    /// that will be left to the get_char method.
    fn peek(&self) -> Option<char> {
        let next_ch = match self.input.chars().nth(self.char_count) {
            Some(c) => Some(c),
            None => None
        };

        next_ch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token;
    use token::TokenType;

    #[test]
    fn test_lex_single_char() {
        let mut left_brace_lex = Lexer::new("{".to_string());
        left_brace_lex.lex();

        let curr_tok = left_brace_lex.curr_tok.unwrap();
        let expected = Token::new(TokenType::LeftBrace, "{".to_string());

        assert_eq!(curr_tok, expected);
    }

    #[test]
    fn test_lex_arrow() {
        let mut arrow_lex = Lexer::new("->".to_string());
        arrow_lex.lex();

        let curr_tok = arrow_lex.curr_tok.unwrap();
        let expected = Token::new(TokenType::Arrow, "->".to_string());

        assert_eq!(curr_tok, expected);
    }

    #[test]
    fn test_lex_minus() {
        let mut minus_lex = Lexer::new("-".to_string());
        minus_lex.lex();

        let curr_tok_minus = minus_lex.curr_tok.unwrap();
        let expected_minus = Token::new(TokenType::Minus, "-".to_string());

        assert_eq!(curr_tok_minus, expected_minus);
    }

    #[test]
    fn test_lex_ident_not_reserved() {
        let mut ident_lex = Lexer::new("testIdentifier".to_string());
        ident_lex.lex();

        let curr_tok = ident_lex.curr_tok.unwrap();
        let expected = Token::new(TokenType::Ident, "testIdentifier".to_string());

        assert_eq!(curr_tok, expected);
    }

    #[test]
    fn test_lex_ident_reserved() {
        let mut ident_lex = Lexer::new("int".to_string());
        ident_lex.lex();

        let curr_tok = ident_lex.curr_tok.unwrap();
        let mut expected = Token::new(TokenType::Ident, "int".to_string());
        expected.set_reserved(true);

        assert_eq!(curr_tok, expected);
        assert!(curr_tok.is_reserved);
    }

    #[test]
    fn test_lex_number() {
        let mut num_lex = Lexer::new("8080".to_string());
        num_lex.lex();

        let curr_tok = num_lex.curr_tok.unwrap();
        let expected = Token::new(TokenType::Number, "8080".to_string());

        assert_eq!(curr_tok, expected);
    }

    #[test]
    fn test_lex_empty() {
        let mut empty_lex = Lexer::new("".to_string());
        empty_lex.lex();

        let curr_tok = empty_lex.curr_tok.unwrap();
        let expected = Token::new(TokenType::Eof, "".to_string());

        assert_eq!(curr_tok, expected);
    }
}
