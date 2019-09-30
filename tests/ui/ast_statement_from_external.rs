use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let ext = ast::External::new(term, &[]);
    let stmt = ext.ast_statement();
    drop(ext);
    let _end = stmt;
}