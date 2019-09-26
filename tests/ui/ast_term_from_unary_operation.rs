use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let op = ast::UnaryOperation::minus(term);
    let term2 = ast::Term::from(&op);
    drop(op);
    let _end = term2;
}