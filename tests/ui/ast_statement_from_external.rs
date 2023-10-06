use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let atom1 = ast::symbolic_atom(term1).unwrap();
    let body = [];
    let ext = ast::external(&loc, atom1, &body, ExternalType::Free).unwrap();
    let stmt : ast::Statement = ext.into();

    drop(ext);
    let _end = stmt;
}