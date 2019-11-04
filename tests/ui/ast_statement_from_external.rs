use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let mut ext = ast::External::new(term, &[]);
    let stmt = ext.ast_statement();
    
    ext = ast::External::new(term, &[]);
    let _end = (stmt, ext);
}