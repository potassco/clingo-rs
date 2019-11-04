use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let term2 = ast::Term::from(sym);
    let args = vec![term1,term2];
    let pool = ast::Pool::new(&args);

    drop(args);
    println!("{:?}",pool);
}