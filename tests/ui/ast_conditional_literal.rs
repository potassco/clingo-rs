use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term2: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();

    let atom1 = ast::symbolic_atom(term1).unwrap();
    let lit = ast::basic_literal_from_symbolic_atom(&loc, ast::Sign::NoSign, atom1).unwrap();

    let atom2 = ast::symbolic_atom(term2).unwrap();
    let lit2: ast::Literal = ast::basic_literal_from_symbolic_atom(&loc, ast::Sign::NoSign, atom2)
        .unwrap()
        .into();

    let condition = vec![lit2];
    let cond = ast::conditional_literal(&loc, lit, &condition).unwrap();

    drop(condition);
    let _end = cond;
}
