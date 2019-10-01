use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let mut term1 = ast::Term::from(sym);
    let mut term2 = ast::Term::from(sym);
    let mut lit1 = ast::Literal::from_term(ast::Sign::None, &term1);
    let mut lit2 = ast::Literal::from_term(ast::Sign::None, &term2);
    let hlit = ast::HeadLiteral::from(&lit1);
    let blit = ast::BodyLiteral::from_literal(ast::Sign::None, &lit2);
    let body = vec![blit];
    let mut rule = ast::Rule::new(hlit, &body);
    let stmt = rule.ast_statement();
    
    term1 = ast::Term::from(sym);
    term2 = ast::Term::from(sym);
    lit1 = ast::Literal::from_term(ast::Sign::None, &term1);
    lit2 = ast::Literal::from_term(ast::Sign::None, &term2);
    rule = ast::Rule::new(hlit, &body);
    drop(body);
    let _end = (lit1, lit2, rule, stmt);
}