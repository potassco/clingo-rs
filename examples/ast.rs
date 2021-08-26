use clingo::ast::Location;
use clingo::{
    ast, control, Control, ExternalType, Model, Part, ShowType, SolveMode, Symbol, TruthValue,
};
use std::env;

pub struct OnStatementData<'a, 'b> {
    atom: &'b ast::SymbolicAtom<'b>,
    builder: &'a mut ast::ProgramBuilder<'a>,
}

impl<'a, 'b> ast::StatementHandler for OnStatementData<'a, 'b> {
    // adds atom enable to all rule bodies
    fn on_statement(&mut self, stm: &ast::Statement) -> bool {
        let stm_clone = stm.clone();
        // pass through all statements that are not rules
        match stm_clone.is_a().unwrap() {
            ast::StatementIsA::Rule(stm) => {
                let body = stm.body();
                let mut extended_body = std::vec::Vec::with_capacity(body.size().unwrap() + 1);
                for e in body {
                    extended_body.push(e.clone());
                }
                // create literal enable
                let loc = Location::default();
                let atom_copy = self.atom.clone();
                let basic_lit =
                    ast::basic_literal_from_symbolic_atom(&loc, ast::Sign::NoSign, atom_copy)
                        .unwrap();
                let lit: ast::Literal = basic_lit.into();
                extended_body.push(lit.into());

                // initialize the rule
                let head = stm.head();
                let rule = ast::rule(&loc, head, &extended_body).unwrap();

                // add the rewritten rule to the program builder
                self.builder
                    .add(&rule.into())
                    .expect("Failed to add Rule to ProgramBuilder.");
            }
            _ => {
                self.builder
                    .add(stm)
                    .expect("Failed to add Statement to ProgramBuilder.");
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
    let id = ast::function(&loc, "enable", &[], false).unwrap();

    let atom = ast::symbolic_atom(id).unwrap();

    // add the external statement: #external enable. [false]
    let ext = ast::external(&loc, atom.clone(), &[], ExternalType::False).unwrap();

    builder
        .add(&mut ext.into())
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
    let part = Part::new("base", vec![]).unwrap();
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
