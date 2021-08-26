use clingo::*;
use std::env;

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let mut ctl = control(options).expect("Failed creating Control.");

    // add a logic program to the base part
    ctl.add("base", &[], "a. {b}. #external c.")
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // get symbolic atoms
    let atoms = ctl.symbolic_atoms().unwrap();

    println!("Symbolic atoms:");

    let mut atoms_iterator = atoms.iter().unwrap();
    while let Some(item) = atoms_iterator.next() {
        let symbol = item.symbol().unwrap();
        print!("  {}", symbol);

        if item.is_fact().unwrap() {
            print!(", fact");
        }

        if item.is_external().unwrap() {
            print!(", external");
        }
        println!();
    }
}
