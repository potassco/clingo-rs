extern crate clingo;

use std::env;
use clingo::*;

pub struct OnStatementData<'a> {
    atom: ClingoAstTerm,
    builder: &'a mut ClingoProgramBuilder,
}

// adds atom enable to all rule bodies
extern "C" fn on_statement(
    stm_: *const clingo_ast_statement_t,
    data: *mut std::os::raw::c_void,
) -> bool {

    let stm = unsafe {
        (std::mem::transmute::<*const clingo_ast_statement_t, *const ClingoAstStatement>(stm_))
            .as_ref()
    }.unwrap();
    let on_statement_data = unsafe { (data as *mut OnStatementData).as_mut() }.unwrap();

    // pass through all statements that are not rules
    if stm.get_type() != clingo_ast_statement_type_rule {
        on_statement_data.builder.add(stm).expect(
            "Failed to add statement to ProgramBuilder.",
        );
        return true;
    }

    // copy the current rule body
    let body = unsafe { stm.rule() }.body();
    let mut extended_body = std::vec::Vec::with_capacity(body.len() + 1);
    for e in body {
        extended_body.push(*e);
    }

    // create atom enable
    let lit = ClingoAstLiteral::new(
        on_statement_data.atom.location(),
        clingo_ast_sign_none,
        clingo_ast_literal_type_symbolic,
        &on_statement_data.atom,
    );

    // add atom enable to the rule body
    let y: ClingoAstBodyLiteral = ClingoAstBodyLiteral::new(
        on_statement_data.atom.location(),
        clingo_ast_sign_none,
        clingo_ast_body_literal_type_literal,
        &lit,
    );
    extended_body.push(y);

    // initialize the rule
    let rule = ClingoAstRule::new(unsafe { stm.rule() }.head(), &extended_body);

    // initialize the statement
    let stm2 = ClingoAstStatement::new_rule(stm.location(), &rule);

    // add the rewritten statement to the program
    on_statement_data.builder.add(&stm2).expect(
        "Failed to add statement to ProgramBuilder.",
    );
    true
}

fn print_model(model: &mut ClingoModel) {

    // retrieve the symbols in the model
    let atoms = model
        .symbols(clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model.");

    print!("Model:");

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!("");
}

fn solve(ctl: &mut ClingoControl) {
    let solve_mode = clingo_solve_mode_yield as clingo_solve_mode_bitset_t;
    let assumptions = vec![];

    // get a solve handle
    let handle = ctl.solve(solve_mode, assumptions)
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
    handle.get().expect(
        "Failed to get result from solve handle.",
    );
    handle.close().expect("Failed to close solve handle.");
}

fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    let logger = None;
    let logger_data = std::ptr::null_mut();
    let ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating ClingoControl.");

    let sym = ClingoSymbol::create_id("enable", true).unwrap();

    {
        // get the program builder
        let builder = ctl.program_builder().unwrap();

        // initialize the location
        let location = ClingoLocation::new(0, 0, 0, 0, "<rewrite>", "<rewrite>");

        // initilize atom to add
        let atom = ClingoAstTerm::new_symbol(location, sym);

        let mut data = OnStatementData {
            atom: atom,
            builder: builder,
        };

        // begin building a program
        data.builder.begin().expect(
            "Failed building logic program.",
        );

        // get the AST of the program
        let logger = None;
        let logger_data = std::ptr::null_mut();
        let callback: ClingoAstCallback = Some(on_statement);
        let data_ptr = &mut data as *mut OnStatementData;
        parse_program(
            "a :- not b. b :- not a.",
            callback,
            data_ptr as *mut ::std::os::raw::c_void,
            logger,
            logger_data,
            20,
        ).expect("Failed to parse logic program.");

        // add the external statement: #external enable.
        let ext = ClingoAstExternal::new(data.atom, &[]);

        let location2 = ClingoLocation::new(0, 0, 0, 0, "<rewrite>", "<rewrite>");

        let stm =
            ClingoAstStatement::new_external(location2, clingo_ast_statement_type_external, &ext);
        data.builder.add(&stm).expect(
            "Failed to add statement to ProgramBuilder.",
        );

        // finish building a program
        data.builder.end().expect(
            "Failed to finish building a program.",
        );
    }

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    ctl.ground(parts)
        .expect("Failed to ground a logic program.");

    // solve with external enable = false
    println!("Solving with enable = false...");
    solve(ctl);

    // solve with external enable = true
    println!("Solving with enable = true...");
    ctl.assign_external(sym, clingo_truth_value_true).expect(
        "Failed to assign #external enable true.",
    );
    solve(ctl);

    // solve with external enable = false
    println!("Solving with enable = false...");
    ctl.assign_external(sym, clingo_truth_value_false).expect(
        "Failed to assign #external enable false.",
    );
    solve(ctl);
}
