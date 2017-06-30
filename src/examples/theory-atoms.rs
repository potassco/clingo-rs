extern crate clingo;

use std::env;
use clingo::*;


fn error_main() {
    let error_message = clingo::error_message();
    println!("Error {}: {}", clingo::error_code(), error_message);
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
        print!(" {}", atom.to_string().unwrap());
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
        handle.resume().expect("Failed resume on solve handle.");
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

fn get_theory_atom_literal(ctl: &mut ClingoControl) -> std::option::Option<ClingoLiteral> {

    // get the theory atoms container
    let atoms = ctl.theory_atoms().unwrap();

    // print the number of grounded theory atoms
    let size = atoms.size().unwrap();
    println!("number of grounded theory atoms: {}", size);

    // verify that theory atom b has a guard
    for atom in UNSAFEClingoTheoryAtomsIterator::from(atoms) {

        // get the term associated with the theory atom
        let term = atoms.atom_term(atom as ClingoId).unwrap();

        // get the name associated with the theory atom
        let name = atoms.term_name(term).unwrap();
        // we got theory atom b/1 here
        if name == "b" {
            let guard = atoms.atom_has_guard(atom as ClingoId).unwrap();
            if guard {
                println!("theory atom b/1 has a guard: true");
            } else {
                println!("theory atom b/1 has a guard: false");
            }
            // get the literal associated with the theory atom
            return Some(atoms.atom_literal(atom as ClingoId).unwrap());
        }
    }
    None
}

fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    if let Err(e) = ctl.add(
        "base",
        parameters,
        "#theory t { term   { + : 1, binary, left };&a/0 : term, any;&b/1 : term, \
                       {=}, term, any}.x :- &a { 1+2 }.y :- &b(3) { } = 17.",
    )
    {
        println!("{}",e);
        return;
    }

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    if let Err(e) = ctl.ground(parts, ground_callback, ground_callback_data) {
        println!("{}", e);
        return;
    }

    // use the backend to assume that the theory atom is true
    // (note that only symbolic literals can be passed as assumptions to a solve call;
    // the backend accepts any aspif literal)
    if let Some(lit) = get_theory_atom_literal(ctl) {
        // get the backend
        let backend = ctl.backend().unwrap();
        // add the assumption
        let err = backend.assume(&[lit]);
        if !err {
            return error_main();
        }
    }

    // solve
    let _solve_result = solve(ctl);
}
