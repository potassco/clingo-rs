extern crate clingo;

use std::env;
use clingo::*;


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
    let solve_event_callback = None;
    let data = std::ptr::null_mut();

    // get a solve handle
    let handle = ctl.solve(solve_mode, assumptions, solve_event_callback, data)
        .expect("Failed retrieving solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // stop if there are no more models
            Err(_) => break,
            // print the model
            Ok(model) => print_model(model),
        }
    }

    // close the solve handle
    let _result = handle.get();
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

    // get the configuration object and its root key
    {
        let conf = ctl.configuration().unwrap();
        let root_key = conf.root().unwrap();

        // configure to enumerate all models
        let mut sub_key = conf.map_at(root_key, "solve.models").unwrap();
        conf.value_set(sub_key, "0").expect(
            "Failed to set solve.models to 0.",
        );
        sub_key = conf.map_at(root_key, "solver").unwrap();

        // configure the first solver to use the berkmin heuristic
        sub_key = conf.array_at(sub_key, 0).unwrap();
        sub_key = conf.map_at(sub_key, "heuristic").unwrap();
        conf.value_set(sub_key, "berkmin").expect(
            "Failed to set heuristic to berkmin.",
        );
    }
    // note that the solver entry can be used both as an array and a map
    // if used as a map, this simply sets the configuration of the first solver and
    // is equivalent to the code above

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    ctl.add("base", parameters, "a :- not b. b :- not a.")
        .expect("Failed to add a logic program.");

    print!("");

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    ctl.ground(parts, ground_callback, ground_callback_data)
        .expect("Failed to ground a logic program.");

    // solve
    let _solve_result = solve(ctl);
}
