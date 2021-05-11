use clingo::ast::*;
use clingo::{Location, Symbol};

fn main() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let hlit: Head = lit.clone().into();
    let blit: BodyLiteral = lit.into();
    let body = vec![blit];
    let rule = rule(&loc, hlit, &body);
    drop(body);
    let _end = rule;
}
