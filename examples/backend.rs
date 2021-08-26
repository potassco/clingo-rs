use clingo::*;
use std::{env, vec};

fn print_model(model: &Model) {
    // retrieve the symbols in the model
    let atoms = model
        .symbols(ShowType::SHOWN)
        .expect("Failed to retrieve symbols in the model.");

    print!("Model:");

    for symbol in atoms {
        print!(" {}", symbol);
    }
    println!();
}

fn solve(ctl: Control) {
    // get a solve handle
    let mut handle = ctl
        .solve(SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            // print the model
            Ok(Some(model)) => print_model(model),
            // stop if there are no more models
            Ok(None) => break,
            Err(e) => panic!("Error: {}", e),
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
    let mut ctl = control(options).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    ctl.add("base", &[], "{a; b; c}.")
        .expect("Failed to add a logic program");

    // ground the base part
    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let atom_strings = ["a", "b", "c"];

    // get the ids of atoms a, b, and c
    let mut atom_ids = vec![];
    {
        // get iterator of the symbolic atoms
        let mut atoms_iterator = ctl.symbolic_atoms().unwrap().iter().unwrap();

        for atom in &atom_strings {
            let symbol = Symbol::create_id(atom, true).unwrap();
            let item = atoms_iterator
                .find(|e| e.symbol().unwrap() == symbol)
                .unwrap();

            // get the atom's id
            let lit = item.literal().unwrap();
            atom_ids.push(lit);
        }
    }

    {
        // get the backend
        let mut backend = ctl.backend().unwrap();

        // add an additional atom (called d below)
        let atom_d = backend.add_atom(None).unwrap();

        // add rule: d :- a, b.
        let head = vec![atom_d];
        let body = vec![atom_ids[0], atom_ids[1]];
        backend
            .rule(false, &head, &body)
            .expect("Failed to add a rule to the program.");

        // add rule: :- not d, c.
        let head = vec![];
        let body = vec![SolverLiteral::from(atom_d).negate(), atom_ids[2]];

        backend
            .rule(false, &head, &body)
            .expect("Failed to add a rule to the program.");
    }

    // solve
    solve(ctl);
}
