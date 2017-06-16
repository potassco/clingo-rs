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

fn get_theory_atom_literal(ctl: &mut ClingoControl) -> std::option::Option<clingo_literal_t> {
    // get the theory atoms container
    let atoms = ctl.theory_atoms().unwrap();

    // print the number of grounded theory atoms
    let size = atoms.size().unwrap();
    println!("number of grounded theory atoms: {}", size);

    // verify that theory atom b has a guard
    for atom in 0..size {

        // get the term associated with the theory atom
        let term = atoms.atom_term(atom as clingo_id_t).unwrap();

        // get the name associated with the theory atom
        let name = atoms.term_name(term).unwrap();
        // we got theory atom b/1 here
        if name == "b" {
            let guard = atoms.atom_has_guard(atom as clingo_id_t).unwrap();
            if guard {
                println!("theory atom b/1 has a guard: true");
            } else {
                println!("theory atom b/1 has a guard: false");
            }
            // get the literal associated with the theory atom
            return Some(atoms.atom_literal(atom as clingo_id_t).unwrap());
        }
    }
    None
}

fn main() {

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err = ctl.add(
        "base",
        parameters,
        "#theory t { term   { + : 1, binary, left };&a/0 : term, any;&b/1 : term, \
                       {=}, term, any}.x :- &a { 1+2 }.y :- &b(3) { } = 17.",
    );
    if !err {
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
    let err = ctl.ground(parts, ground_callback, ground_callback_data);
    if !err {
        return error_main();
    }

    let lit = get_theory_atom_literal(ctl).unwrap();
    // use the backend to assume that the theory atom is true
    // (note that only symbolic literals can be passed as assumptions to a solve call;
    // the backend accepts any aspif literal)
    if lit != 0 {
        // get the backend
        let backend = ctl.backend().unwrap();
        // add the assumption
        let err = backend.assume(&lit, 1);
        if !err {
            return error_main();
        }
    }

    // solve
    let _solve_result = solve(ctl);
}
