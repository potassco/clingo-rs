use clingo::ast::*;
use clingo::Symbol;

fn main() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let blit: BodyLiteral = lit.clone().into();
    let body = vec![blit];
    let rule = rule(&loc, lit, &body);
    drop(body);
    let _end = rule;
}
