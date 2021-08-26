use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let mut lit = ast::Literal::from_term(ast::Sign::NoSign, &term);
    let hlit = ast::HeadLiteral::from(&lit);
    lit = ast::Literal::from_term(ast::Sign::NoSign, &term);
    let _end = (lit, hlit);
}