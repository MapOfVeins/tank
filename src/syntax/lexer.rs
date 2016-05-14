use syntax::token::{Token, TokenType};
use syntax::reserved::Reserved;

// EOF isn't technically a char, but we can use this as a stand in when unwrapping things.
const EOF: char = '\0';

#[derive(Debug)]
pub struct Lexer {
    /// Contents of input file
    input: String,
    /// Numbering of current char in the input string
    char_count: usize,
    /// Line number of input
    line_num: usize,
    /// Char count of current line
    line_char_num: usize,
    /// Reserved word struct for checking if identifiers are valid
    pub reserved: Reserved,
    /// Last token to be consumed
    pub curr_tok: Option<Token>,
    /// Last char seen by the lexer
    pub curr_char: Option<char>,


}

impl Lexer {
    pub fn new(file_contents: String) -> Lexer {
        let r = Reserved::new();
        let c = file_contents.chars().nth(0);

        Lexer {
            input: file_contents,
            char_count: 1,
            line_num: 1,
            line_char_num: 1,
            reserved: r,
            curr_tok: None,
            curr_char: c,
        }
    }

    /// Lex the input at the current point, represented by the char_count field.
    /// Once lexing is complete, sets the curr_tok field to the correct token.
    ///
    /// After this function is called, the curr_tok field should never be None.
    /// If we cannot find a proper token during lexing, we will return the token
    /// for EOF.
    pub fn lex(&mut self) -> &mut Lexer {
        if self.curr_char.is_none() {
            self.curr_tok = Some(Token::new_from_empty());
            return self;
        }

        // curr_char is guaranteed to be Some here.
        let mut ch = self.curr_char.unwrap();

        if ch == EOF {
            self.curr_tok = Some(Token::new_from_empty());
            return self;
        }

        while ch.is_whitespace() {
            if ch == '\n' {
                self.line_num = self.line_num + 1;
                self.line_char_num = 1;
            }

            self.get_char();
            if self.curr_char.is_none() {
                self.curr_tok = Some(Token::new_from_empty());
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
            '+' => self.curr_tok = self.get_token(TokenType::Plus),
            '&' => self.curr_tok = self.get_token(TokenType::Ampersand),
            '%' => self.curr_tok = self.get_token(TokenType::Percent),
            '=' => self.curr_tok = self.lex_operator_equals(),
            '!' => self.curr_tok = self.lex_operator_equals(),
            '>' => self.curr_tok = self.lex_operator_cmp(),
            '<' => self.curr_tok = self.lex_operator_cmp(),
            '-' => self.curr_tok = self.get_minus_or_arrow(),
            _   => self.curr_tok = self.lex_word_or_number()
        }

        self
    }

    /// Returns a token built from the current char, without consuming
    /// those characters. Used by the parser to determine some context
    /// about element declarations and their contents.
    ///
    /// Currently, only checks if the current char is '(', so we can
    /// decide the difference between an element name and its contents.
    pub fn peek_tok(&self) -> Token {
        let tok = match self.curr_char.unwrap_or(EOF) {
            '(' => Token::new(TokenType::LeftParen, self.line_char_num, self.line_num),
            _ => Token::new_from_empty()
        };

        tok
    }

    /// Returns the next available char from the file contents. If no
    /// char is available (ie. at end of input, or when the char_count
    /// field is greater than the number of chars in the file), then
    /// None is returned.
    fn get_char(&mut self) -> &mut Lexer {
        //TODO: O(n)
        match self.input.chars().nth(self.char_count) {
            Some(c) => self.curr_char = Some(c),
            None => self.curr_char = None
        }

        self.char_count = self.char_count + 1;
        self.line_char_num = self.line_char_num + 1;

        self
    }

    /// Retrieve a new token based on the current character, or None
    /// if no curr_char exists. This function is only used for tokens
    /// that have single character values.
    ///
    /// After a token is created, we advance the current char pointer.
    fn get_token(&mut self, token_type: TokenType) -> Option<Token> {
        let tok = match self.curr_char {
            Some(c) => Some(Token::new_from_value(token_type,
                                                  &c.to_string(),
                                                  self.line_char_num,
                                                  self.line_num)),
            None => Some(Token::new_from_empty())
        };

        self.get_char();

        tok
    }

    /// Called when a '-' characted is encountered. Checks for a following
    /// '>' character, then returns a token for either a minus sign or an arrow.
    fn get_minus_or_arrow(&mut self) -> Option<Token> {
        let ch = self.peek(0).unwrap_or(EOF);
        let tok;

        if ch == '>' {
            // Consume the '-' char here, the '>' is consumed below.
            self.get_char();
            tok = Some(Token::new_from_value(TokenType::Arrow,
                                             &"->",
                                             self.line_char_num - 1,
                                             self.line_num));
        } else {
            tok = Some(Token::new_from_value(TokenType::Minus, &"-",
                                             self.line_char_num,
                                             self.line_num));
        }

        self.get_char();

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
        let mut ch = self.curr_char.unwrap_or(EOF);
        let mut ident = ch.to_string();
        let tok;

        if ch.is_alphabetic() {
            self.get_char();
            ch = self.curr_char.unwrap_or(EOF);

            while self.alphanumeric_or_valid_punc(ch) {
                let append = self.curr_char.unwrap_or(EOF);
                if !self.is_valid_char_in_ident(append) {
                    break;
                }

                if append.is_alphanumeric() {
                    ident = ident + &append.to_string();
                }

                self.get_char();
                ch = append;
            }

            let mut some_tok = Token::new_from_value(TokenType::Ident,
                                                     &ident,
                                                     self.line_char_num - (ident.len() + 1),
                                                     self.line_num);

            // Match on reserved words
            // TODO: way better reserved word handling is needed here.
            // The is_reserved field is not useful.
            match self.reserved.words.get(&ident) {
                Some(_) => some_tok.set_reserved(true),
                None => some_tok.set_reserved(false)
            };

            tok = Some(some_tok);

        } else if ch.is_digit(10) {
            self.get_char();

            while ch.is_digit(10) {
                let append = self.curr_char.unwrap_or(EOF);
                if !append.is_whitespace() && append != EOF {
                    ident = ident + &append.to_string();
                }

                self.get_char();
                ch = append;
            }

            tok = Some(Token::new_from_value(TokenType::Number,
                                             &ident,
                                             self.line_char_num - (ident.len() + 1),
                                             self.line_num));
        } else {
            tok = Some(Token::new_from_empty());
        }

        tok
    }

    /// When we see an '=' or '!' character, we check the next character to determine
    /// what token to return. If the next token is an '=', then we have a two-character
    /// operator to use (either NotEquals or EqualsEquals). Otherwise, we can use a single
    /// '=' character.
    fn lex_operator_equals(&mut self) -> Option<Token> {
        let ch = self.peek(0).unwrap_or(EOF);
        let tok;

        match self.curr_char.unwrap() {
            '=' => {
                if ch == '=' {
                    self.get_char();
                    tok = Some(Token::new_from_value(TokenType::EqualsEquals,
                                                     &"==",
                                                     self.line_char_num - 1,
                                                     self.line_num));
                } else {
                    tok = Some(Token::new_from_value(TokenType::Equals,
                                                     &"=",
                                                     self.line_char_num,
                                                     self.line_num));
                }
            },
            '!' => {
                if ch == '=' {
                    self.get_char();
                    tok = Some(Token::new_from_value(TokenType::NotEquals,
                                                     &"!=",
                                                     self.line_char_num - 1,
                                                     self.line_num));
                } else {
                    // TODO: ! operator not supported yet
                    tok = Some(Token::new_from_empty());
                }
            },
            _ => tok = Some(Token::new_from_empty())
        }

        self.get_char();

        tok
    }

    /// Determines if we have a two-character operator with the '>' and '<' characters. Checks
    /// the following character to see if we need to use GreaterEquals or LessEquals.
    fn lex_operator_cmp(&mut self) -> Option<Token> {
        let ch = self.peek(0).unwrap_or(EOF);
        let tok;

        match self.curr_char.unwrap() {
            '>' => {
                if ch == '=' {
                    self.get_char();
                    tok = Some(Token::new_from_value(TokenType::GtEquals,
                                                     &">=",
                                                     self.line_char_num - 1,
                                                     self.line_num));
                } else {
                    tok = Some(Token::new_from_value(TokenType::Gt,
                                                     &">",
                                                     self.line_char_num,
                                                     self.line_num));
                }
            },
            '<' => {
                if ch == '=' {
                    self.get_char();
                    tok = Some(Token::new_from_value(TokenType::LtEquals,
                                                     &"<=",
                                                     self.line_char_num - 1,
                                                     self.line_num));
                } else {
                    tok = Some(Token::new_from_value(TokenType::Lt,
                                                     &"<",
                                                     self.line_char_num,
                                                     self.line_num));
                }
            },
            _ => tok = Some(Token::new_from_empty())
        }

        self.get_char();

        tok
    }

    /// Get the next char to be lexed. Does not consume the char,
    /// that will be left to the get_char method. Allows for an offset value
    /// to be passed in, indicating how far to look ahead.
    fn peek(&self, offset: usize) -> Option<char> {
        let next_ch = match self.input.chars().nth(self.char_count + offset) {
            Some(c) => Some(c),
            None => None
        };

        next_ch
    }

    /// Checks if an identifier contains an illegal character or not.
    fn is_valid_char_in_ident(&self, ch: char) -> bool {
        match ch {
            ':' | '(' | ')' => false,
            _ => true
        }
    }

    /// Determine if a char is a valid char in an identifier or in
    /// the contents of an element.
    fn alphanumeric_or_valid_punc(&self, ch: char) -> bool {
        if ch.is_whitespace() || ch == EOF {
            return false;
        }

        if ch.is_alphanumeric() {
            return true;
        }

        self.is_valid_char_in_ident(ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syntax::token::Token;
    use syntax::token::TokenType;

    #[test]
    fn test_lex_single_char() {
        let mut left_brace_lex = Lexer::new("{".to_string());
        left_brace_lex.lex();

        let curr_tok = left_brace_lex.curr_tok.unwrap();
        let expected = Token::new_from_value(TokenType::LeftBrace, &"{", 1, 1);

        assert_eq!(curr_tok, expected);
    }

    #[test]
    fn test_lex_arrow() {
        let mut arrow_lex = Lexer::new("->".to_string());
        arrow_lex.lex();

        let curr_tok = arrow_lex.curr_tok.unwrap();
        let expected = Token::new_from_value(TokenType::Arrow, &"->", 1, 1);

        assert_eq!(curr_tok, expected);
    }

    #[test]
    fn test_lex_minus() {
        let mut minus_lex = Lexer::new("-".to_string());
        minus_lex.lex();

        let curr_tok_minus = minus_lex.curr_tok.unwrap();
        let expected_minus = Token::new_from_value(TokenType::Minus, &"-", 1, 1);

        assert_eq!(curr_tok_minus, expected_minus);
    }

    #[test]
    fn test_lex_ident_not_reserved() {
        let mut ident_lex = Lexer::new("testIdentifier".to_string());
        ident_lex.lex();

        let curr_tok = ident_lex.curr_tok.unwrap();
        let expected = Token::new_from_value(TokenType::Ident, &"testIdentifier", 1, 1);

        assert_eq!(curr_tok, expected);
    }

    #[test]
    fn test_lex_ident_reserved() {
        let mut ident_lex = Lexer::new("int".to_string());
        ident_lex.lex();

        let curr_tok = ident_lex.curr_tok.unwrap();
        let mut expected = Token::new_from_value(TokenType::Ident, &"int", 1, 1);
        expected.set_reserved(true);

        assert_eq!(curr_tok, expected);
        assert!(curr_tok.is_reserved);
    }

    #[test]
    fn test_lex_number() {
        let mut num_lex = Lexer::new("8080".to_string());
        num_lex.lex();

        let curr_tok = num_lex.curr_tok.unwrap();
        let expected = Token::new_from_value(TokenType::Number, &"8080", 1, 1);

        assert_eq!(curr_tok, expected);
    }

    #[test]
    fn test_lex_empty() {
        let mut empty_lex = Lexer::new("".to_string());
        empty_lex.lex();

        let curr_tok = empty_lex.curr_tok.unwrap();
        let expected = Token::new_from_empty();

        assert_eq!(curr_tok, expected);
    }
}
