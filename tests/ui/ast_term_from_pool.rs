use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let term2 = ast::Term::from(sym);
    let args = vec![term1,term2];
    let mut pool = ast::Pool::new(&args);
    let term3 = ast::Term::from(&pool);

    pool = ast::Pool::new(&args);
    drop(args);
    let _end= (pool, term3);
}