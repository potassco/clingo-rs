extern crate clingo;

use std::env;
use clingo::*;

pub struct OnStatementData<'a> {
    atom: ClingoAstTerm,
    builder: &'a mut ClingoProgramBuilder,
}

struct MyAstHandler;
impl<'a> ClingoAstStatementHandler<OnStatementData<'a>> for MyAstHandler {
    // adds atom enable to all rule bodies
    fn on_statement(stm: &ClingoAstStatement, data: &mut OnStatementData) -> bool {

        // pass through all statements that are not rules
        if stm.get_type() != Ok(ClingoAstStatementType::Rule) {
            data.builder.add(stm).expect(
                "Failed to add statement to ProgramBuilder.",
            );
            return true;
        }

        // copy the current rule body
        let body = unsafe { stm.rule() }.body();
        let mut extended_body = std::vec::Vec::with_capacity(body.len() + 1);
        for e in body {
            extended_body.push(e.clone());
        }

        // create atom enable
        let lit = ClingoAstLiteral::new(
            data.atom.location(),
            ClingoAstSign::None,
            ClingoAstLiteralType::Symbolic,
            &data.atom,
        );
        // add atom enable to the rule body
        let y: ClingoAstBodyLiteral = ClingoAstBodyLiteral::new(
            data.atom.location(),
            ClingoAstSign::None,
            ClingoAstBodyLiteralType::Literal,
            &lit,
        );
        extended_body.push(y);

        // initialize the rule
        let head = unsafe { stm.rule() }.head();
        let rule = ClingoAstRule::new(head, &extended_body);

        // initialize the statement
        let stm2 = ClingoAstStatement::new_rule(stm.location(), &rule);

        // add the rewritten statement to the program
        data.builder.add(&stm2).expect(
            "Failed to add statement to ProgramBuilder.",
        );
        true
    }
}

fn print_model(model: &mut ClingoModel) {

    // retrieve the symbols in the model
    let atoms = model
        .symbols(ClingoShowType::Shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model.");

    print!("Model:");

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!();
}

fn solve(ctl: &mut ClingoControl) {

    let solve_mode = ClingoSolveMode::Yield;
    let assumptions = vec![];

    // get a solve handle
    let handle = ctl.solve(solve_mode, &assumptions).expect(
        "Failed retrieving solve handle.",
    );

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
    handle.get().expect(
        "Failed to get result from solve handle.",
    );
    handle.close().expect("Failed to close solve handle.");
}

fn get_modified_rules(builder: &mut ClingoProgramBuilder, sym: ClingoSymbol) {

        // begin building a program
        builder.begin().expect("Failed building logic program.");

        // initialize the location
        let location = create_clingo_location(0, 0, 0, 0, "<rewrite>", "<rewrite>");

        // initilize atom to add
        let atom = ClingoAstTerm::new_symbol(location, sym);

        let mut data = OnStatementData {
            atom: atom,
            builder: builder,
        };

        // get the AST of the program
        parse_program_with_event_handler("a :- not b. b :- not a.", &MyAstHandler, &mut data)
            .expect("Failed to parse logic program.");

        // add the external statement: #external enable.
        let ext = ClingoAstExternal::new(atom, &[]);

        let stm = ClingoAstStatement::new_external(location, ClingoAstStatementType::External, &ext);
        data.builder.add(&stm).expect(
            "Failed to add statement to ProgramBuilder.",
        );

        // finish building a program
        data.builder.end().expect(
            "Failed to finish building a program.",
        );
}


fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    let mut ctl = ClingoControl::new(options, 20).expect("Failed creating ClingoControl.");

    let sym = create_id("enable", true).unwrap();
    
    let sym2 = sym.clone();

    // get the program builder
    { let mut builder = ctl.program_builder().unwrap();
      get_modified_rules(&mut builder, sym);
    }
    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    ctl.ground(parts).expect(
        "Failed to ground a logic program.",
    );

    // solve with external enable = false
    println!("Solving with enable = false...");
    solve(&mut ctl);

    // solve with external enable = true
    println!("Solving with enable = true...");
    ctl.assign_external(&sym2, ClingoTruthValue::True).expect(
        "Failed to assign #external enable true.",
    );
    solve(&mut ctl);

    // solve with external enable = false
    println!("Solving with enable = false...");
    ctl.assign_external(&sym2, ClingoTruthValue::False).expect(
        "Failed to assign #external enable false.",
    );
    solve(&mut ctl);   
}
