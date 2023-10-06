// ast_term_from_pool
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term2: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let args = vec![term1, term2];
    let mut pool = ast::pool(&loc, &args).unwrap();
    let term3: ast::Term = pool.into();

    pool = ast::pool(&loc, &args).unwrap();
    drop(args);
    let _end = (pool, term3);
}
