// ast_term_from_function
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term2: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let args = vec![term1, term2];
    let fun = ast::function(&loc, "name", &args, false).unwrap();
    let term3: ast::Term = fun.into();

    let fun2 = ast::function(&loc, "name2", &args, true).unwrap();
    let term4: ast::Term = fun2.into();

    drop(args);
    let _end = (term3, term4);
}
