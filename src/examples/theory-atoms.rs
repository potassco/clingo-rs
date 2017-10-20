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
    let _result = handle.get().expect(
        "Failed to get result from solve handle.",
    );
    handle.close().expect("Failed to close solve handle.");
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

        if name == "b" {
            // we got theory atom b/1 here
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
    let ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating clingo_control.");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    ctl.add(
        "base",
        parameters,
        "#theory t {\
          term   { + : 1, binary, left };\
          &a/0 : term, any;\
          &b/1 : term, {=}, term, any\
         }.\
         x :- &a { 1+2 }.\
         y :- &b(3) { } = 17.",
    ).expect("Failed to add a logic program.");

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    ctl.ground(parts, ground_callback, ground_callback_data)
        .expect("Failed to ground a logic program.");

    // use the backend to assume that the theory atom is true
    // (note that only symbolic literals can be passed as assumptions to a solve call;
    // the backend accepts any aspif literal)
    if let Some(lit) = get_theory_atom_literal(ctl) {
        // get the backend
        let backend = ctl.backend().unwrap();
        // add the assumption
        backend.assume(&[lit]).expect("Failed to add assumption.");
    }

    // solve
    solve(ctl);
}
