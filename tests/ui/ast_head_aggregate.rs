use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::Term::from(sym);
    let mut guard = ast::AggregateGuard::gt(term);
    let elements = vec![];
    let hagg = ast::HeadAggregate::new(ast::AggregateFunction::Count, &elements, &guard, &guard);
    guard =  ast::AggregateGuard::gt(term);
    drop(elements);
    let _end = (guard, hagg);
}