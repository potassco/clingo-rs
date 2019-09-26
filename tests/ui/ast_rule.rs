use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let lit = ast::Literal::from_term(ast::Sign::None, &term);
    let hlit = ast::HeadLiteral::from(&lit);
    let blit = ast::BodyLiteral::from_literal(ast::Sign::None, &lit);
    let body = vec![blit];
    let rule = ast::Rule::new(hlit, &body);
    drop(blit);
    drop(body);
    drop(lit);
    let _end = rule;
}