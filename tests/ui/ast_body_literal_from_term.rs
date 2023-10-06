// ast_body_literal_from_term
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let atom1 = ast::symbolic_atom(term1).unwrap();
    let lit = ast::basic_literal_from_symbolic_atom(&loc, ast::Sign::NoSign, atom1).unwrap();

    let blit: ast::BodyLiteral = lit.into();

    drop(lit);
    let _end = blit;
}
