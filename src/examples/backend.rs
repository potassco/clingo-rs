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
    println!();
}

fn solve(ctl: &mut ClingoControl) {
    let solve_mode = ClingoSolveMode::Yield;
    let assumptions = vec![];

    // get a solve handle
    let handle = ctl.solve(solve_mode, &assumptions)
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
    handle
        .get()
        .expect("Failed to get result from solve handle.");
    handle.close().expect("Failed to close solve handle.");
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let mut ctl = ClingoControl::new(options, 20).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    ctl.add("base", parameters, "{a; b; c}.")
        .expect("Failed to add a logic program");

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let atom_strings = ["a", "b", "c"];

    // get the ids of atoms a, b, and c
    let mut atom_ids = Vec::new();
    {
        // get symbolic atoms
        let atoms = ctl.symbolic_atoms().unwrap();

        for atom in &atom_strings {
            let symbol = create_id(atom, true).unwrap();
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
        backend
            .rule(false, &head, &body)
            .expect("Failed to add a rule to the program.");

        // add rule: :- not d, c.
        let head = vec![];
        let body = vec![ClingoLiteral::UNSAFE_from(atom_d).negate(), atom_ids[2]];

        backend
            .rule(false, &head, &body)
            .expect("Failed to add a rule to the program.");
    }

    // solve
    solve(&mut ctl);
}
