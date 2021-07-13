use clingo::*;
use std::env;

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

fn get_theory_atom_literal(ctl: &mut Control) -> Option<SolverLiteral> {
    // get the theory atoms container
    let atoms = ctl.theory_atoms().unwrap();

    // print the number of grounded theory atoms
    let size = atoms.size().unwrap();
    println!("number of grounded theory atoms: {}", size);

    // verify that theory atom b has a guard
    for atom in atoms.iter() {
        // get the term associated with the theory atom
        let term = atoms.atom_term(atom).unwrap();

        // get the name associated with the theory atom
        let name = atoms.term_name(term).unwrap();

        if name == "b" {
            // we got theory atom b/1 here
            let guard = atoms.atom_has_guard(atom).unwrap();
            if guard {
                println!("theory atom b/1 has a guard: true");
            } else {
                println!("theory atom b/1 has a guard: false");
            }
            // get the literal associated with the theory atom
            return atoms.atom_literal(atom).ok();
        }
    }
    None
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let mut ctl = control(options).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    ctl.add(
        "base",
        &[],
        "#theory t {\
         term   { + : 1, binary, left };\
         &a/0 : term, any;\
         &b/1 : term, {=}, term, any\
         }.\
         x :- &a { 1+2 }.\
         y :- &b(3) { } = 17.",
    )
    .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // use the backend to assume that the theory atom is true
    // (note that only symbolic literals can be passed as assumptions to a solve call;
    // the backend accepts any aspif literal)
    if let Some(lit) = get_theory_atom_literal(&mut ctl) {
        // get the backend
        let mut backend = ctl.backend().unwrap();
        // add the assumption
        backend.assume(&[lit]).expect("Failed to add assumption.");
    }

    // solve
    solve(ctl);
}
