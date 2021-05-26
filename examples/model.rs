use clingo::*;
use std::env;

fn print_model(model: &Model, label: &str, show: ShowType) {
    print!("{}:", label);

    // retrieve the symbols in the model
    let atoms = model
        .symbols(show)
        .expect("Failed to retrieve symbols in the model.");

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
            Ok(Some(model)) => {
                // get model type
                let model_type = model.model_type().unwrap();

                let type_string = match model_type {
                    ModelType::StableModel => "Stable model",
                    ModelType::BraveConsequences => "Brave consequences",
                    ModelType::CautiousConsequences => "Cautious consequences",
                };

                // get running number of model
                let number = model.number().unwrap();

                println!("{}: {}", type_string, number);

                print_model(model, "  shown", ShowType::SHOWN);
                print_model(model, "  atoms", ShowType::ATOMS);
                print_model(model, "  terms", ShowType::TERMS);
                print_model(model, " ~atoms", ShowType::COMPLEMENT | ShowType::ATOMS);
            }
            Ok(None) => {
                // stop if there are no more models
                break;
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }

    // close the solve handle
    handle.close().expect("Failed to close solve handle.");
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let mut ctl = control(options).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    ctl.add("base", &[], "1 {a; b} 1. #show c : b. #show a/0.")
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // solve
    solve(ctl);
}
