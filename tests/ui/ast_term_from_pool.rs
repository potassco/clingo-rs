use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let term2 = ast::Term::from(sym);
    let mut args = vec![term1,term2];
    let pool = ast::Pool::new(&mut args);
    let term3 = ast::Term::from(&pool);
    drop(pool);
    println!("{:?}",term3);
}