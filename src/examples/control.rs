extern crate clingo;

use std::env;
use clingo::*;


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
    println!("");
}

fn solve(ctl: &mut ClingoControl) {

    let solve_mode = ClingoSolveMode::Yield;
    let assumptions = vec![];

    // get a solve handle
    let handle = ctl.solve(solve_mode, assumptions).expect(
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

fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating ClingoControl.");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    ctl.add("base", parameters, "a :- not b. b :- not a.")
        .expect("Failed to add a logic program.");;

    print!("");

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    ctl.ground(parts).expect(
        "Failed to ground a logic program.",
    );

    // solve
    solve(&mut ctl);
}
