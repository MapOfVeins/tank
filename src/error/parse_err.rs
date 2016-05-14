use error::error_traits::Diagnostic;
use syntax::token::Token;

pub struct ParseDiagnostic {
    errors: Vec<String>,
    warnings: Vec<String>,
    pub curr_tok: Token
}

impl ParseDiagnostic {
    pub fn new() -> ParseDiagnostic {
        ParseDiagnostic {
            errors: Vec::new(),
            warnings: Vec::new(),
            curr_tok: Token::new_from_empty()
        }
    }
}

impl Diagnostic for ParseDiagnostic {
    fn is_err(&self) -> bool {
        self.errors.len() != 0
    }

    fn is_warn(&self) -> bool {
        self.warnings.len() != 0
    }

    fn has_diag(&self) -> bool {
        self.is_err() || self.is_warn()
    }

    fn new_err(&mut self, err_message: &str) {
        self.errors.push(err_message.to_owned());
    }

    fn print_diag(&self) {
        for err in &self.errors {
            let err_str = format!("tank: Parse error at line {}, pos {} - {}",
                                  self.curr_tok.line_num,
                                  self.curr_tok.char_pos,
                                  err);
            println!("{}", err_str);
        }

        for warn in &self.warnings {
            println!("{}", warn);
        }

        // An extra line here makes the messages a bit more readable before exiting.
        print!("\n");
    }
}
