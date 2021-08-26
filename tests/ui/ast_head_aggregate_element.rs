use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let mut term1 = ast::Term::from(sym);
    let mut term2 = ast::Term::from(sym);
    let tuple = vec![term1];
    let lit = ast::Literal::from_term(ast::Sign::NoSign, &term1);
    let lit2 = ast::Literal::from_term(ast::Sign::NoSign, &term2);
    let condition = vec![lit2];
    let cond = ast::ConditionalLiteral::new(&lit, &condition);
    let helem = ast::HeadAggregateElement::new(&tuple,cond);
    
    term1 =  ast::Term::from(sym);
    term2 =  ast::Term::from(sym);
    lit = ast::Literal::from_term(ast::Sign::NoSign, &term1);
    drop(tuple);
    let _end = (term1, term2, lit, helem);
}