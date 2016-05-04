use std::default::Default;
use error::error_traits::Diagnostic;

#[derive(Default)]
pub struct ParseDiagnostic {
    errors: Vec<String>,
    warnings: Vec<String>
}

impl ParseDiagnostic {
    pub fn new() -> ParseDiagnostic {
        Default::default()
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
            println!("{}", err);
        }

        for warn in &self.warnings {
            println!("{}", warn);
        }

        // An extra line here makes the messages a bit more readable before exiting.
        print!("\n");
    }
}
