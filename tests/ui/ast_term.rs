use clingo::*;

fn main() {
    let string = String::from("test");
    let sym = Symbol::create_id(&string, true).unwrap();
    let term = ast::Term::from(sym);
    drop(string);
    drop(sym);
    let _end = term;
}