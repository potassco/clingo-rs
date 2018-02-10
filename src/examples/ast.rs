extern crate clingo;

use std::env;
use clingo::*;

pub struct OnStatementData<'a> {
    atom: AstTerm,
    builder: &'a mut ProgramBuilder,
}

struct MyAstHandler;
impl<'a> AstStatementHandler<OnStatementData<'a>> for MyAstHandler {
    // adds atom enable to all rule bodies
    fn on_statement(stm: &AstStatement, data: &mut OnStatementData) -> bool {
        // pass through all statements that are not rules
        if stm.get_type() != Ok(AstStatementType::Rule) {
            data.builder
                .add(stm)
                .expect("Failed to add statement to ProgramBuilder.");
            return true;
        }

        // copy the current rule body
        let body = unsafe { stm.rule() }.body();
        let mut extended_body = std::vec::Vec::with_capacity(body.len() + 1);
        for e in body {
            extended_body.push(e.clone());
        }

        // create atom enable
        let lit = AstLiteral::new(
            data.atom.location(),
            AstSign::None,
            AstLiteralType::Symbolic,
            &data.atom,
        );
        // add atom enable to the rule body
        let y: AstBodyLiteral = AstBodyLiteral::new(
            data.atom.location(),
            AstSign::None,
            AstBodyLiteralType::Literal,
            &lit,
        );
        extended_body.push(y);

        // initialize the rule
        let head = unsafe { stm.rule() }.head();
        let rule = AstRule::new(head, &extended_body);

        // initialize the statement
        let stm2 = AstStatement::new_rule(stm.location(), &rule);

        // add the rewritten statement to the program
        data.builder
            .add(&stm2)
            .expect("Failed to add statement to ProgramBuilder.");
        true
    }
}

fn print_model(model: &mut Model) {
    // retrieve the symbols in the model
    let atoms = model
        .symbols(ShowType::SHOWN)
        .expect("Failed to retrieve symbols in the model.");

    print!("Model:");

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!();
}

fn solve(ctl: &mut Control) {
    // get a solve handle
    let handle = ctl.solve(SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // print the model
            Ok(model) => print_model(model),
            // stop if there are no more models
            Err(_) => break,
        }
    }

    // close the solve handle
    handle
        .get()
        .expect("Failed to get result from solve handle.");
    handle.close().expect("Failed to close solve handle.");
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    let mut ctl = Control::new(options, 20).expect("Failed creating Control.");

    let sym = create_id("enable", true).unwrap();
    let sym2 = sym.clone();

    {
        // get the program builder
        let builder = ctl.program_builder().unwrap();

        // begin building a program
        builder.begin().expect("Failed building logic program.");

        // initialize the location
        let location = create_location(0, 0, 0, 0, "<rewrite>", "<rewrite>");

        // initilize atom to add
        let atom = AstTerm::new_symbol(location, sym);

        let mut data = OnStatementData {
            atom: atom,
            builder: builder,
        };

        // get the AST of the program
        parse_program("a :- not b. b :- not a.", &MyAstHandler, &mut data)
            .expect("Failed to parse logic program.");

        // add the external statement: #external enable.
        let ext = AstExternal::new(atom, &[]);

        let stm = AstStatement::new_external(location, AstStatementType::External, &ext);
        data.builder
            .add(&stm)
            .expect("Failed to add statement to ProgramBuilder.");

        // finish building a program
        data.builder
            .end()
            .expect("Failed to finish building a program.");
    }

    // ground the base part
    let part = Part::new("base", &[]);
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // solve with external enable = false
    println!("Solving with enable = false...");
    solve(&mut ctl);

    // solve with external enable = true
    println!("Solving with enable = true...");
    ctl.assign_external(&sym2, TruthValue::True)
        .expect("Failed to assign #external enable true.");
    solve(&mut ctl);

    // solve with external enable = false
    println!("Solving with enable = false...");
    ctl.assign_external(&sym2, TruthValue::False)
        .expect("Failed to assign #external enable false.");
    solve(&mut ctl);
}
