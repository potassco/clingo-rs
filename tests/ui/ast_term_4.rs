use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let term2 = ast::Term::from(sym);
    let mut args = vec![term1,term2];
    let fun = ast::Function::new("name", &mut args).unwrap();
    let term3 = ast::Term::from(&fun);
    let term4 = ast::Term::from(sym);
    args.push(term4);
    drop(args);
    drop(fun);
    let _end = term3;
}