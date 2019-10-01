use clingo::*;

fn main() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = ast::Term::from(sym);
    let term2 = ast::Term::from(sym);
    let args = vec![term1,term2];
    let mut fun = ast::Function::new("name", &args).unwrap();
    let term3 = ast::Term::from(&fun);

    let mut fun2 = ast::Function::new("name2", &args).unwrap();
    let term4 = ast::Term::external_function(&fun2);

    fun = ast::Function::new("name", &args).unwrap();
    fun2 = ast::Function::new("name", & args).unwrap();
    drop(args);
    let _end = (term3, term4, fun, fun2);
}