use std::env;
extern crate clingo;
use clingo::*;
extern crate libc;
use std::ffi::CString;


fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

fn main() {

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = SafeClingoControl::new(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err1 = ctl.add("base", parameters, "a :- not b. b :- not a.");
    if err1 == 0 {
        return error_main();
    }

    println!("");

    // ground the base part
    let part = safe_clingo_part {
        params: 0,
        size: 0,
        name: CString::new("base").unwrap(),
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err2 = ctl.ground(parts, ground_callback, ground_callback_data);
    if err2 == 0 {
        return error_main();
    }

    // solve using a model callback
    let it = ctl.solve_iteratively(std::ptr::null_mut(), 0).unwrap();
    loop {

        let model = safe_clingo_solve_iteratively_next(it).unwrap();

        // stop if the search space has been exhausted or the requested number of models found
        if model.is_null() {
            break;
        }

        // retrieve the symbols in the model
        let atoms = safe_clingo_model_symbols(model, clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
            .expect("Failed to retrieve symbols in the model");

        println!("Model:");
        for atom in &atoms {
            // retrieve and print the symbol's string
            let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
            println!(" {}", atom_string.to_str().unwrap());
        }
    }
}
