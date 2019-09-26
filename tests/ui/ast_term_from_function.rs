use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let term2 = ast::Term::from(sym);
    let mut args = vec![term1,term2];
    let fun = ast::Function::new("name", &mut args).unwrap();
    let term3 = ast::Term::from(&fun);
    drop(fun);
    println!("{:?}",term3);
    let fun2 = ast::Function::new("name2", &mut args).unwrap();
    let term4 = ast::Term::external_function(&fun2);
    drop(fun2);
    println!("{:?}",term4);

}