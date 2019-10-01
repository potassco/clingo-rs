use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let mut lit = ast::Literal::from_term(ast::Sign::None, &term);
    let hlit = ast::HeadLiteral::from(&lit);
    let mut blit = ast::BodyLiteral::from_literal(ast::Sign::None, &lit);
    let body = vec![blit];
    let rule = ast::Rule::new(hlit, &body);
    blit = ast::BodyLiteral::from_literal(ast::Sign::None, &lit);
    drop(body);
    lit = ast::Literal::from_term(ast::Sign::None, &term);
    let _end = (rule, lit, blit);
}