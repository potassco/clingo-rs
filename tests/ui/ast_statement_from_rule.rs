// ast_statement_from_rule
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = ast::symbolic_term(&loc, &sym).unwrap();
    let atom = ast::symbolic_atom(term).unwrap();
    let lit = ast::basic_literal_from_symbolic_atom(&loc, ast::Sign::NoSign, atom).unwrap();
    let hlit: ast::Head = lit.clone().into();
    let blit: ast::BodyLiteral = lit.into();
    let body = vec![blit];
    let rule = ast::rule(&loc, hlit, &body).unwrap();
    let stmt: ast::Statement = rule.into();

    drop(rule);
    let _end = stmt;
}
