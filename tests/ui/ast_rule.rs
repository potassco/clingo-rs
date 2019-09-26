use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let lit = ast::Literal::from_term(ast::Sign::None, &term);
    let hlit = ast::HeadLiteral::from(&lit);
    let rule = ast::Rule::new(hlit, &[]);
    drop(hlit);
    let _end = rule;
}