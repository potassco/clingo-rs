// ast_head_aggregate
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term2: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let guard_l = ast::guard(ast::ComparisonOperator::GreaterThan, term1).unwrap();
    let guard_r = ast::guard(ast::ComparisonOperator::GreaterThan, term2).unwrap();
    let elements = vec![];
    let hagg = ast::head_aggregate(
        &loc,
        Some(guard_l),
        ast::AggregateFunction::Count,
        &elements,
        Some(guard_r),
    );
    drop(elements);
    let _end = hagg;
}
