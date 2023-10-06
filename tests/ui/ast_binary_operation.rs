// ast_binary_operation
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term2: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let op = ast::binary_operation(&loc, ast::BinaryOperator::Xor,term1, term2);
    drop(term1);
    drop(term2);
    let _end = op;
}