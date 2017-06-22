extern crate clingo;

use std::env;
use clingo::*;


fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

fn print_model(model: &mut ClingoModel) {

    // retrieve the symbols in the model
    let atoms = model
        .symbols(
            clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t,
        )
        .expect("Failed to retrieve symbols in the model");

    print!("Model:");

    for atom in atoms {
        // retrieve and print the symbol's string
        let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
        print!(" {}", atom_string.to_str().unwrap());
    }
    println!("");
}

fn solve(ctl: &mut ClingoControl) {

    let solve_mode = clingo_solve_mode::clingo_solve_mode_yield as clingo_solve_mode_bitset_t;
    let assumptions = vec![];
    let solve_event_callback = None;
    let data = std::ptr::null_mut();

    // get a solve handle
    let handle = ctl.solve(solve_mode, assumptions, solve_event_callback, data)
        .expect("Failed retrieving solve handle");

    // loop over all models
    loop {
        if !handle.resume() {
            return error_main();
        }
        match handle.model() {
            // stop if there are no more models
            None => break,
            // print the model
            Some(model) => print_model(model),
        }
    }

    // close the solve handle
    let _result = handle.get().expect("Failed to get solve handle");
    handle.close();
}

fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // get the configuration object and its root key
    {
        let conf = ctl.configuration().unwrap();
        let root_key = conf.configuration_root().unwrap();

        // configure to enumerate all models
        let mut sub_key = conf.configuration_map_at(root_key, "solve.models").unwrap();
        let err = conf.configuration_value_set(sub_key, "0");
        if !err {
            return error_main();
        }
        sub_key = conf.configuration_map_at(root_key, "solver").unwrap();

        // configure the first solver to use the berkmin heuristic
        sub_key = conf.configuration_array_at(sub_key, 0).unwrap();
        sub_key = conf.configuration_map_at(sub_key, "heuristic").unwrap();
        let err = conf.configuration_value_set(sub_key, "berkmin");
        if !err {
            return error_main();
        }
    }
    // note that the solver entry can be used both as an array and a map
    // if used as a map, this simply sets the configuration of the first solver and
    // is equivalent to the code above

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err = ctl.add("base", parameters, "a :- not b. b :- not a.");
    if !err {
        return error_main();
    }

    print!("");

    // ground the base part
    let part = new_part("base",&[]);
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err = ctl.ground(parts, ground_callback, ground_callback_data);
    if !err {
        return error_main();
    }

    // solve
    let _solve_result = solve(ctl);
}
