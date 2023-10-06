// ast_comparison
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let guard = ast::guard(ast::ComparisonOperator::GreaterThan, term1).unwrap();
    let term2: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let comp = ast::comparison(term2, &[guard]);

    drop(term1);
    drop(term2);
    let _end = comp;
}
