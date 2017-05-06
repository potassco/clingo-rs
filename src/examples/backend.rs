extern crate clingo;

use std::env;
use std::ffi::CString;
use clingo::*;


fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

fn print_model(model: &mut ClingoModel) {

    // retrieve the symbols in the model
    let atoms = model
        .symbols(clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model");

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

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = new_clingo_control(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    if !ctl.add("base", parameters, "{a; b; c}.") {
        return error_main();
    }

    // ground the base part
    let part = ClingoPart {
        name: CString::new("base").unwrap(),
        params: &[],
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    if !ctl.ground(parts, ground_callback, ground_callback_data) {
        return error_main();
    }

    let atom_strings = ["a", "b", "c"];
    // get the ids of atoms a, b, and c
    let mut atom_ids = Vec::new();
    {
        // get symbolic atoms
        let atoms = ctl.symbolic_atoms().unwrap();

        for atom in atom_strings.iter() {
            let symbol = safe_clingo_symbol_create_id(atom, true).unwrap();
            let atom_it = atoms.find(symbol).unwrap();

            // get the atom's id
            let lit = atoms.literal(atom_it).unwrap();
            atom_ids.push(lit);
        }
    }

    {
        // get the backend
        let backend = ctl.backend().unwrap();

        // add an additional atom (called d below)
        let atom_d = backend.add_atom().unwrap();

        // add rule: d :- a, b.
        let head = vec![atom_d];
        let body = vec![atom_ids[0], atom_ids[1]];
        if !backend.rule(false, &head, &body) {
            return error_main();
        }

        // add rule: :- not d, c.
        let head = vec![];
        let body = vec![-(atom_d as clingo_literal_t), atom_ids[2]];

        if !backend.rule(false, &head, &body) {
            return error_main();
        }
    }

    // solve
    solve(ctl);
}
