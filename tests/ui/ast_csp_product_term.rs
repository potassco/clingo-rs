use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let mut term = ast::Term::from(sym);
    let lit = ast::CspProductTerm::new(term, &term);
    
    term = ast::Term::from(sym);
    let _end = (lit,term);
}