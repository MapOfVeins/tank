/// Trait for providing some simple error and warning messages for compiler errors.
/// This is used by the generator and the parser for some simple diagnostic
/// information when errors occur.
pub trait Diagnostic {
    fn is_err(&self) -> bool;
    fn is_warn(&self) -> bool;
    fn has_diag(&self) -> bool;
    fn new_err(&mut self, err_message: &str);
    fn print_diag(&self);
}
