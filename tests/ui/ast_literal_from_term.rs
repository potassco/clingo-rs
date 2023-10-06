// ast_literal_from_term
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let atom = ast::symbolic_atom(term).unwrap();
    let lit = ast::basic_literal_from_symbolic_atom(&loc, ast::Sign::NoSign, atom).unwrap();

    let term2: ast::Term = term;
    let _end = (lit, term2);
}
