extern crate clingo;

use std::env;
use clingo::*;


fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let logger: clingo_logger_t = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating ClingoControl.");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    ctl.add("base", parameters, "a. {b}. #external c.").expect(
        "Failed to add a logic program.",
    );

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    ctl.ground(parts).expect(
        "Failed to ground a logic program.",
    );

    // get symbolic atoms
    let atoms = ctl.symbolic_atoms().unwrap();

    println!("Symbolic atoms:");

    // get begin and end iterator
    let mut it_a = atoms.begin(None).unwrap();
    let ie_a = atoms.end().unwrap();

    loop {
        if atoms.iterator_is_equal_to(it_a, ie_a).unwrap() {
            break;
        }
        let symbol = atoms.symbol(it_a).unwrap();
        print!("  {}", symbol.to_string().unwrap());

        if atoms.is_fact(it_a).unwrap() {
            print!(", fact");
        }

        if atoms.is_external(it_a).unwrap() {
            print!(", external");
        }
        println!("");

        it_a = atoms.next(it_a).unwrap();
    }
}
