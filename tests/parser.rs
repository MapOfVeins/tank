extern crate tank;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

use tank::syntax::parser::Parser;
use tank::syntax::symbol_table::SymbolTable;
use tank::syntax::ast::AstType;

const DIR: &'static str = "tests/parser_input/";

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
#[should_panic(expected = "tank: End of input reached, nothing to parse!")]
fn test_parse_empty_file() {
    let filename = DIR.to_owned() + "empty_file.tank";
    let mut parser = setup_parser(filename);

    parser.parse();
}

#[test]
#[should_panic]
fn test_parse_if_expr_no_left_brace() {
    let filename = DIR.to_owned() + "if_no_left_brace.tank";
    let mut parser = setup_parser(filename);

    parser.parse();
}

#[test]
#[should_panic]
fn test_parse_if_expr_no_right_brace() {
    let filename = DIR.to_owned() + "if_no_right_brace.tank";
    let mut parser = setup_parser(filename);

    parser.parse();
}

#[test]
fn test_parse_if_valid_expr() {
    let filename = DIR.to_owned() + "if_valid_expr.tank";
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

#[test]
#[should_panic]
#[ignore]
fn test_parse_element_no_left_paren() {
    let filename = DIR.to_owned() + "el_no_left_paren.tank";
    let mut parser = setup_parser(filename);

    parser.parse();
}

#[test]
#[should_panic]
#[ignore]
fn test_parse_element_no_right_paren() {
    let filename = DIR.to_owned() + "el_no_right_paren.tank";
    let mut parser = setup_parser(filename);

    parser.parse();
}

#[test]
fn test_parse_element_no_attribute_list() {
    let filename = DIR.to_owned() + "el_no_attr_list.tank";
    let mut parser = setup_parser(filename);

    parser.parse();

    let ast = parser.root;
    assert_eq!(ast.children.len(), 2);

    let el_ast = &ast.children[0];
    assert_eq!(el_ast.ast_type, AstType::Element);
    assert_eq!(el_ast.children.len(), 3);

    let el_name_ast = &el_ast.children[0];
    assert_eq!(el_name_ast.ast_type, AstType::ElementName);
    assert_eq!(el_name_ast.val, "div".to_owned());

    let attr_list_ast = &el_ast.children[1];
    assert_eq!(attr_list_ast.ast_type, AstType::AttrList);
    assert_eq!(attr_list_ast.children.len(), 0);

    let el_contents_ast = &el_ast.children[2];
    assert_eq!(el_contents_ast.ast_type, AstType::Ident);
    assert_eq!(el_contents_ast.val, "divContents".to_owned());
}

#[test]
fn test_parse_nested_elements() {
    let filename = DIR.to_owned() + "el_nested.tank";
    let mut parser = setup_parser(filename);

    parser.parse();

    let ast = parser.root;
    let el_ast = &ast.children[0];
    assert_eq!(el_ast.ast_type, AstType::Element);
    assert_eq!(el_ast.children.len(), 3);

    let first_element = &el_ast.children[0];
    assert_eq!(first_element.ast_type, AstType::ElementName);
    assert_eq!(first_element.val, "div".to_owned());

    let element_contents = &el_ast.children[2];
    assert_eq!(element_contents.ast_type, AstType::Element);
    assert_eq!(element_contents.children.len(), 3);

    let nested_element = &element_contents.children[0];
    assert_eq!(nested_element.ast_type, AstType::ElementName);
    assert_eq!(nested_element.val, "p".to_owned());

    let nested_element_contents = &element_contents.children[2];
    assert_eq!(nested_element_contents.ast_type, AstType::Ident);
    assert_eq!(nested_element_contents.val, "pContents".to_owned());
}

#[test]
fn test_parse_element_with_attribute_list() {

}
