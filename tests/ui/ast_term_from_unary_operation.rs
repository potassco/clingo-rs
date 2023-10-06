// ast_term_from_unary_operation
use clingo::ast::*;
use clingo::*;

fn main() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let minus = UnaryOperator::Minus;
    let op = unary_operation(&loc, minus, term).unwrap();
    let term2: Term = op.into();
    drop(op);
    let _end = term2;
}
