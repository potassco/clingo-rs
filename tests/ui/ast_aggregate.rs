use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let term2 = ast::Term::from(sym);
    let term3 = ast::Term::from(sym);
    let mut lit = ast::Literal::from_term(ast::Sign::NoSign, &term1);
    let lit2 = ast::Literal::from_term(ast::Sign::NoSign, &term2);
    let condition = vec![lit2];
    let cond = ast::ConditionalLiteral::new(&lit, &condition);
    let elements = vec![cond];
    let mut guard = ast::AggregateGuard::gt(term3);
    let agg = ast::Aggregate::new( &elements, Some(&guard), Some(&guard));
    
    guard =  ast::AggregateGuard::gt(term1);
    lit = ast::Literal::from_term(ast::Sign::NoSign, &term1);
    drop(elements);
    let _end = (guard, lit, agg);
}
