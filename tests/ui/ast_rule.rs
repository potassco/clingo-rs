use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let mut lit = ast::Literal::from_term(ast::Sign::NoSign, &term);
    let hlit = ast::HeadLiteral::from(&lit);
    let mut blit = ast::BodyLiteral::from_literal(ast::Sign::NoSign, &lit);
    let body = vec![blit];
    let rule = ast::Rule::new(hlit, &body);
    blit = ast::BodyLiteral::from_literal(ast::Sign::NoSign, &lit);
    drop(body);
    lit = ast::Literal::from_term(ast::Sign::NoSign, &term);
    let _end = (rule, lit, blit);
}