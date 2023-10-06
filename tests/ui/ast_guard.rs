// ast_guard
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let op = ast::guard(ast::ComparisonOperator::LessEqual,term);
    drop(term);
    let _end = op;
}