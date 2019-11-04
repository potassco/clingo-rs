use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let op = ast::AggregateGuard::gt(term);
    drop(term);
    let _end = op;
}