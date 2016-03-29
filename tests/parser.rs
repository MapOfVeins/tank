extern crate tank;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

use tank::syntax::parser::Parser;
use tank::syntax::symbol_table::SymbolTable;
use tank::syntax::ast::AstType;

const DIR: &'static str = "tests/parser_input/";

const EMPTY_FILE: &'static str = "empty_file.tank";
const IF_NO_LEFT_BRACE: &'static str = "if_no_left_brace.tank";
const IF_NO_RIGHT_BRACE: &'static str = "if_no_right_brace.tank";
const IF_VALID_EXPR: &'static str = "if_valid_expr.tank";

fn setup_parser(filename: String) -> Parser {
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("Failed to open {}: {}", display, Error::description(&error)),
        Ok(file) => file
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(error) => panic!("Failed to read {}: {}", display, Error::description(&error)),
        Ok(_) => ()
    }

    let symbol_table = SymbolTable::new();

    Parser::new(file_contents, symbol_table)
}

#[test]
#[should_panic]
fn test_parse_empty_file() {
    let filename = DIR.to_owned() + EMPTY_FILE;
    let mut parser = setup_parser(filename);

    parser.parse();
}

#[test]
#[should_panic]
fn test_parse_if_expr_no_left_brace() {
    let filename = DIR.to_owned() + IF_NO_LEFT_BRACE;
    let mut parser = setup_parser(filename);

    parser.parse();
}

#[test]
#[should_panic]
fn test_parse_if_expr_no_right_brace() {
    let filename = DIR.to_owned() + IF_NO_RIGHT_BRACE;
    let mut parser = setup_parser(filename);

    parser.parse();
}

#[test]
fn test_parse_if_valid_expr() {
    let filename = DIR.to_owned() + IF_VALID_EXPR;
    let mut parser = setup_parser(filename);

    parser.parse();

    // Assert that the ast root is of the correcr form.
    let ast = parser.root;
    assert_eq!(ast.ast_type, AstType::Template);
    assert_eq!(ast.children.len(), 2);

    // Assert that the first child is the if expression.
    let if_ast = &ast.children[0];
    assert_eq!(if_ast.ast_type, AstType::IfExpr);
    assert_eq!(if_ast.children.len(), 2);

    // Assert that the ast for the operator contains two terms.
    let if_expr_ast = &if_ast.children[0];
    assert_eq!(if_expr_ast.children.len(), 2);

    // Asert that the terms are equal to those found in the test file.
    let first_term = &if_expr_ast.children[0];
    let second_term = &if_expr_ast.children[1];
    assert_eq!(first_term.ast_type, AstType::Ident);
    assert_eq!(first_term.val, "x".to_owned());
    assert_eq!(second_term.ast_type, AstType::Number);
    assert_eq!(second_term.val, "10".to_owned());
}
