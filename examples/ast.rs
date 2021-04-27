use clingo::{
    ast, control, Control, Location, Model, Part, ShowType, SolveMode, Symbol, TruthValue,
};
use std::env;

pub struct OnStatementData<'a, 'b> {
    atom: &'b ast::Ast,
    builder: &'a mut ast::ProgramBuilder<'a>,
}

impl<'a, 'b> ast::StatementHandler for OnStatementData<'a, 'b> {
    // adds atom enable to all rule bodies
    fn on_statement(&mut self, stm: &mut ast::Ast) -> bool {
        // pass through all statements that are not rules

        match stm.get_type() {
            Ok(ast::AstType::Rule) => {
                let body = stm.ast_array(ast::AstAttribute::Body);
                let mut extended_body = std::vec::Vec::with_capacity(body.size().unwrap() + 1);
                for e in body.iter() {
                    extended_body.push(e.clone());
                }
                // create literal enable
                let loc = Location::default();
                let lit: clingo::ast::Ast =
                    ast::Literal(&loc, ast::Sign::NoSign, &self.atom).unwrap();
                extended_body.push(lit);

                // initialize the rule
                let head = stm.get_attribute_ast(&ast::AstAttribute::Head).unwrap();
                let rule = ast::Rule(&loc, head, &extended_body).unwrap();

                // add the rewritten rule to the program builder
                self.builder
                    .add(&rule)
                    .expect("Failed to add Ast to ProgramBuilder.");
            }
            _ => {
                self.builder
                    .add(stm)
                    .expect("Failed to add Ast to ProgramBuilder.");
            }
        }
        true
    }
}

fn print_model(model: &Model) {
    // retrieve the symbols in the model
    let atoms = model
        .symbols(ShowType::SHOWN)
        .expect("Failed to retrieve symbols in the model.");

    print!("Model:");

    for symbol in atoms {
        print!(" {}", symbol);
    }
    println!();
}

fn solve(ctl: Control) -> Control {
    // get a solve handle
    let mut handle = ctl
        .solve(SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // print the model
            Ok(Some(model)) => print_model(model),
            // stop if there are no more models
            Ok(None) => break,
            Err(e) => panic!("Error: {}", e),
        }
    }

    // close the solve handle
    handle
        .get()
        .expect("Failed to get result from solve handle.");
    handle.close().expect("Failed to close solve handle.")
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    let mut ctl = control(options).expect("Failed creating Control.");
    let mut builder = ast::ProgramBuilder::from(&mut ctl).unwrap();

    let loc = Location::default();
    let id = ast::Function(&loc, "enable", &[], false).unwrap();
    let atom = ast::SymbolicAtom(id).unwrap();

    // add the external statement: #external enable. [false]
    let sym = Symbol::create_id("false", true).unwrap();
    let external_type = ast::SymbolicTerm(&loc, &sym).unwrap();
    let mut ext = ast::External(&loc, &atom, &[], &external_type).unwrap();

    builder
        .add(&mut ext)
        .expect("Failed to add statement to ProgramBuilder.");

    let mut stm_handler = OnStatementData {
        atom: &atom,
        builder: &mut builder,
    };

    // get the AST of the program
    ast::parse_string_with_statement_handler("a :- not b. b :- not a.", &mut stm_handler)
        .expect("Failed to parse logic program.");

    // finish building a program
    let builder = ast::ProgramBuilder::from(&mut ctl).unwrap();
    builder.end().expect("Failed to finish building a program.");

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // get the program literal corresponding to the external atom
    let atoms = ctl.symbolic_atoms().unwrap();
    let mut atm_it = atoms.iter().unwrap();

    let sym = Symbol::create_id("enable", true).unwrap();
    let item = atm_it.find(|e| e.symbol().unwrap() == sym).unwrap();
    let atm = item.literal().unwrap();

    // solve with external enable = false
    println!("Solving with enable = false...");
    ctl = solve(ctl);

    // solve with external enable = true
    println!("Solving with enable = true...");
    ctl.assign_external(atm, TruthValue::True)
        .expect("Failed to assign #external enable true.");
    ctl = solve(ctl);

    // solve with external enable = false
    println!("Solving with enable = false...");
    ctl.assign_external(atm, TruthValue::False)
        .expect("Failed to assign #external enable false.");
    let _ = solve(ctl);
}
