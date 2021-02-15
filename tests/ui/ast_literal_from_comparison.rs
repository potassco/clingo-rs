use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let term2 = ast::Term::from(sym);
    let mut comp = ast::Comparison::gt(term1, term2);
    let lit = ast::Literal::from_comparison(ast::Sign::NoSign, &comp);
    
    comp = ast::Comparison::gt(term1, term2);
    let _end = (lit,comp);
}