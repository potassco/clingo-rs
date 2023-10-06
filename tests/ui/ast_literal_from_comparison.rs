// ast_literal_from_comparison
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term2: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let guard = ast::guard(ast::ComparisonOperator::LessEqual, term1).unwrap();
    let guards = [guard];
    let comp = ast::comparison(term2, &guards).unwrap();
    let lit = ast::basic_literal_from_comparison(&loc, ast::Sign::NoSign, comp).unwrap();

    let _end = (lit, comp);
}
