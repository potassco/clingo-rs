extern crate clingo;

use std::env;
use clingo::*;

fn print_model(model: &mut Model, label: &str, show: &ShowType) {
    print!("{}:", label);

    // retrieve the symbols in the model
    let atoms = model
        .symbols(show)
        .expect("Failed to retrieve symbols in the model.");

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!();
}

fn solve(ctl: &mut Control) {
    // get a solve handle
    let handle = ctl.solve(&SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        if let Ok(model) = handle.model() {
            // get model type
            let model_type = model.model_type().unwrap();

            let mut type_string = match model_type {
                ModelType::StableModel => "Stable model",
                ModelType::BraveConsequences => "Brave consequences",
                ModelType::CautiousConsequences => "Cautious consequences",
            };

            // get running number of model
            let number = model.number().unwrap();

            println!("{}: {}", type_string, number);

            print_model(model, "  shown", &ShowType::SHOWN);
            print_model(model, "  atoms", &ShowType::ATOMS);
            print_model(model, "  terms", &ShowType::TERMS);
            print_model(model, " ~atoms", &(ShowType::COMPLEMENT | ShowType::ATOMS));
        } else {
            // stop if there are no more models
            break;
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
    let mut ctl = Control::new(options).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    ctl.add("base", parameters, "1 {a; b} 1. #show c : b. #show a/0.")
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", &[]);
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // solve
    solve(&mut ctl);
}
