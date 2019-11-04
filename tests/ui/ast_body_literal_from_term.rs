use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let mut lit = ast::Literal::from_term(ast::Sign::None, &term);
    let blit = ast::BodyLiteral::from_literal(ast::Sign::None, &lit);
    lit = ast::Literal::from_term(ast::Sign::None, &term);
    let _end = (lit, blit);
}