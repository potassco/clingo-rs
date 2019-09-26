use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let ext = ast::External::new(term, &[]);
    drop(term);
    let _end = ext;
}