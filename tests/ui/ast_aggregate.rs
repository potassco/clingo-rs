use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term2: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term3: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term4: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();

    let atom1 = ast::symbolic_atom(term1).unwrap();
    let lit = ast::basic_literal_from_symbolic_atom(&loc, ast::Sign::NoSign, atom1).unwrap();

    let atom2 = ast::symbolic_atom(term2).unwrap();
    let lit2: ast::Literal = ast::basic_literal_from_symbolic_atom(&loc, ast::Sign::NoSign, atom2)
        .unwrap()
        .into();

    let condition = vec![lit2];
    let cond = ast::conditional_literal(&loc, lit, &condition).unwrap();
    let elements = vec![cond];

    let guard_l = ast::guard(ast::ComparisonOperator::GreaterThan, term3).unwrap();
    let guard_r = ast::guard(ast::ComparisonOperator::GreaterThan, term4).unwrap();
    let agg = ast::aggregate(&loc, Some(guard_l), &elements, Some(guard_r)).unwrap();

    drop(elements);
    let _end = agg;
}
