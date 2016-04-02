extern crate tank;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

use tank::syntax::symbol_table::SymbolTable;
use tank::syntax::ast::AstType;
use tank::syntax::ast::Ast;
use tank::generate::gen::Gen;

const OUT_FILENAME: &'static str = "tests/gen_test_output";

fn setup_gen(out_filename: &String) -> Gen {
    let symbol_table = SymbolTable::new();

    Gen::new(out_filename, symbol_table)
}

fn open_gen_output_file() -> String {
    let full_filename = OUT_FILENAME.to_owned() + ".html";

    let path = Path::new(&full_filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("Failed to open {}: {}", display, Error::description(&error)),
        Ok(file) => file
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(error) => panic!("Failed to read {}: {}", display, Error::description(&error)),
        Ok(_) => ()
    };

    file_contents
}

#[test]
#[should_panic(expected = "tank: Invalid ast provided to generator.")]
fn test_output_invalid_ast_type() {
    let mut gen = setup_gen(&OUT_FILENAME.to_owned());
    let invalid_ast = Ast::new(AstType::Eof);

    gen.output(invalid_ast);
}

#[test]
#[should_panic(expected = "tank: Empty ast found")]
fn test_output_no_children_in_ast() {
    let mut gen = setup_gen(&OUT_FILENAME.to_owned());
    let invalid_ast = Ast::new(AstType::Template);

    gen.output(invalid_ast);
}

#[test]
#[should_panic(expected = "tank: Invalid element found, no children present in ast")]
fn test_output_invalid_ast_no_element_children() {
    let mut gen = setup_gen(&OUT_FILENAME.to_owned());
    let mut invalid_ast = Ast::new(AstType::Template);
    let first_child = Box::new(Ast::new(AstType::Element));

    invalid_ast.children.push(first_child);

    gen.output(invalid_ast);
}

#[test]
#[should_panic(expected = "tank: Invalid Element ast found, not enough children present")]
fn test_output_invalid_ast_element_not_enough_children() {
    let mut gen = setup_gen(&OUT_FILENAME.to_owned());
    let mut invalid_ast = Ast::new(AstType::Template);
    let mut first_child = Box::new(Ast::new(AstType::Element));

    first_child.children.push(Box::new(Ast::new(AstType::Element)));
    invalid_ast.children.push(first_child);

    gen.output(invalid_ast);
}

#[test]
fn test_output_should_write_nothing_for_assignment() {
    let mut gen = setup_gen(&OUT_FILENAME.to_owned());
    let mut valid_ast = Ast::new(AstType::Template);
    let mut first_child = Box::new(Ast::new(AstType::Element));

    first_child.children.push(Box::new(Ast::new(AstType::AssignExpr)));
    valid_ast.children.push(first_child);

    gen.output(valid_ast);

    let file_contents = open_gen_output_file();

    assert!(file_contents.is_empty());
}

#[test]
#[should_panic(expected = "tank: Invalid ast found, no children for if expression")]
fn test_output_invalid_if_expr() {
    let mut gen = setup_gen(&OUT_FILENAME.to_owned());
    let mut valid_ast = Ast::new(AstType::Template);
    let first_child = Box::new(Ast::new(AstType::IfExpr));

    valid_ast.children.push(first_child);

    gen.output(valid_ast);
}

#[test]
#[should_panic(expected = "tank: Invalid expression ast found")]
fn test_output_invalid_if_expr_not_enough_children() {
    let mut gen = setup_gen(&OUT_FILENAME.to_owned());
    let mut valid_ast = Ast::new(AstType::Template);
    let mut first_child = Box::new(Ast::new(AstType::IfExpr));

    first_child.children.push(Box::new(Ast::new(AstType::AssignExpr)));
    valid_ast.children.push(first_child);

    gen.output(valid_ast);
}
