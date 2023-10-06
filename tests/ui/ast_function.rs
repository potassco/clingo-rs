// ast_function
use clingo::*;

fn main() {
    let loc = ast::Location::default();
    let name = String::from("name");
    let sym = Symbol::create_id("test", true).unwrap();
    let term1: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let term2: ast::Term = ast::symbolic_term(&loc, &sym).unwrap().into();
    let args = vec![term1, term2];
    let fun = ast::function(&loc, &name, &args, false).unwrap();
    drop(args);
    println!("{:?}", fun);
}
