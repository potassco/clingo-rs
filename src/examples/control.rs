extern crate clingo;

use std::env;
use clingo::*;


fn print_model(model: &mut ClingoModel) -> Result<(), &'static str>{

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
    Ok(())
}

fn solve(ctl: &mut ClingoControl) -> Result<(), &'static str> {

    let solve_mode = clingo_solve_mode::clingo_solve_mode_yield as clingo_solve_mode_bitset_t;
    let assumptions = vec![];
    let solve_event_callback = None;
    let data = std::ptr::null_mut();

    // get a solve handle
    let handle = ctl.solve(solve_mode, assumptions, solve_event_callback, data)
        .expect("Failed retrieving solve handle");

    // loop over all models
    loop {
        handle.resume()?;
        match handle.model() {
            // stop if there are no more models
            None => break,
            // print the model
            Some(model) => print_model(model)?,
        }
    }

    // close the solve handle
    let _result = handle.get()?;
    handle.close()?;
    Ok(())
}

fn run() -> Result<(), &'static str> {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(options, logger, logger_data, 20)?;

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    ctl.add("base", parameters, "a :- not b. b :- not a.")?;

    print!("");

    // ground the base part
    let name = String::from("base");
    let part = ClingoPart::new_part(&name, &[]);
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    ctl.ground(parts, ground_callback, ground_callback_data)?;

    // solve
    solve(ctl)?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        // print error
        println!("{}", e);
    }
}
