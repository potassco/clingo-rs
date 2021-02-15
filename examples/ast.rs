use clingo::*;
use std::env;

pub struct OnStatementData<'a, 'b> {
    atom: &'b ast::Term<'b>,
    control: &'a mut Control,
}

impl<'a, 'b> StatementHandler for OnStatementData<'a, 'b> {
    // adds atom enable to all rule bodies
    fn on_statement(&mut self, stm: &ast::Statement) -> bool {
        // pass through all statements that are not rules
        let mut builder = ast::ProgramBuilder::from(self.control).unwrap();

        match stm.statement_type() {
            ast::StatementType::Rule(rule) => {
                let body = rule.body();
                let mut extended_body = std::vec::Vec::with_capacity(body.len() + 1);
                for e in body {
                    extended_body.push(e.clone());
                }

                // create atom enable
                let lit = ast::Literal::from_term(ast::Sign::NoSign, &self.atom);
                // add atom enable to the rule body
                let blit = ast::BodyLiteral::from_literal(ast::Sign::NoSign, &lit);
                extended_body.push(blit);

                // initialize the rule
                let head = rule.head();
                let rule = ast::Rule::new(*head, &extended_body);

                // initialize the statement
                let stm2 = rule.ast_statement();

                // add the rewritten statement to the program
                builder
                    .add(&stm2)
                    .expect("Failed to add statement to ProgramBuilder.");
                true
            }
            _ => {
                builder
                    .add(stm)
                    .expect("Failed to add statement to ProgramBuilder.");
                true
            }
        }
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

    let mut ctl = Control::new(options).expect("Failed creating Control.");

    let sym = Symbol::create_id("enable", true).unwrap();

    {
        // initilize atom to add and the program builder
        let mut data = OnStatementData {
            atom: &ast::Term::from(sym),
            control: &mut ctl,
        };

        // get the AST of the program
        parse_program("a :- not b. b :- not a.", &mut data)
            .expect("Failed to parse logic program.");

        // add the external statement: #external enable. [false]
        let ext = ast::External::new(ast::Term::from(sym), &[]);
        let mut builder = ast::ProgramBuilder::from(&mut ctl).unwrap();
        let stm = ext.ast_statement();
        builder
            .add(&stm)
            .expect("Failed to add statement to ProgramBuilder.");

        // finish building a program
        builder.end().expect("Failed to finish building a program.");
    }

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // get the program literal corresponding to the external atom
    let atoms = ctl.symbolic_atoms().unwrap();
    let mut atm_it = atoms.iter().unwrap();
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
