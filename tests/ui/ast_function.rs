use clingo::*;

fn main() {
    let name = String::from("name");
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let term2 = ast::Term::from(sym);
    let args = vec![term1,term2];
    let fun = ast::Function::new(&name, &args).unwrap();
    drop(args);
    println!("{:?}",fun);
}