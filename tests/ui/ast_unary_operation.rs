use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let op = ast::UnaryOperation::minus(term);
    drop(term);
    let _end = op;
}