use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let lit = ast::Literal::from_term(ast::Sign::None, &term);
    let hlit = ast::HeadLiteral::from(&lit);
    drop(lit);
    let _end = hlit;
}