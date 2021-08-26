use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let lit = ast::Literal::from_term(ast::Sign::NoSign, &term1);
    let term2 = ast::Term::from(sym);
    let lit2 = ast::Literal::from_term(ast::Sign::NoSign, &term2);
    let args = vec![lit2];
    let cond = ast::ConditionalLiteral::new(&lit, &args);
    let args2 = vec![cond];
    let dis = ast::Disjunction::new(&args2);
    drop(args2);
    let _end = dis;
}