extern crate clingo;

use std::env;
use clingo::*;


fn print_model(model: &mut ClingoModel, label: &str, show: clingo_show_type_bitset_t) {

    print!("{}:", label);

    // retrieve the symbols in the model
    let atoms = model.symbols(show).expect(
        "Failed to retrieve symbols in the model.",
    );

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!("");
}

fn solve(ctl: &mut ClingoControl) {

    let solve_mode = clingo_solve_mode_yield as clingo_solve_mode_bitset_t;
    let assumptions = vec![];
    let solve_event_callback = None;
    let data = std::ptr::null_mut();

    // get a solve handle
    let handle = ctl.solve(solve_mode, assumptions, solve_event_callback, data)
        .expect("Failed retrieving solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        if let Ok(model) = handle.model() {

            // get model type
            let model_type = model.model_type().unwrap();

            let mut type_string = "";
            match model_type {
                0 => type_string = "Stable model",
                1 => type_string = "Brave consequences",
                2 => type_string = "Cautious consequences",
                _ => {}
            };

            // get running number of model
            let number = model.number().unwrap();

            println!("{}: {}", type_string, number);

            print_model(
                model,
                "  shown",
                clingo_show_type_shown as clingo_show_type_bitset_t,
            );
            print_model(
                model,
                "  atoms",
                clingo_show_type_atoms as clingo_show_type_bitset_t,
            );
            print_model(
                model,
                "  terms",
                clingo_show_type_terms as clingo_show_type_bitset_t,
            );
            print_model(
                model,
                " ~atoms",
                (clingo_show_type_complement as clingo_show_type_bitset_t +
                     clingo_show_type_atoms as clingo_show_type_bitset_t),
            );
        } else {
            // stop if there are no more models
            break;
        }
    }

    // close the solve handle
    handle.get().expect(
        "Failed to get result from solve handle.",
    );
    handle.close().expect("Failed to close solve handle.");
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
    ctl.add("base", parameters, "1 {a; b} 1. #show c : b. #show a/0.")
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    ctl.ground(parts, ground_callback, ground_callback_data)
        .expect("Failed to ground a logic program.");

    // solve
    solve(ctl);
}
