// ast_term_from_binary_operation
use clingo::ast::symbolic_term;
use clingo::ast::Location;
use clingo::*;
fn main() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = symbolic_term(&loc, &sym).unwrap();
    let term2 = symbolic_term(&loc, &sym).unwrap();
    let xor = ast::BinaryOperator::Xor;
    let op = ast::binary_operation(&loc, xor, term1, term2).unwrap();
    let term3: ast::Term = op.into();
    drop(op);
    let _end = term3;
}
