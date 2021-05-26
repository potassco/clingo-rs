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

struct MyEFH;

impl FunctionHandler for MyEFH {
    fn on_external_function(
        &mut self,
        _location: &Location,
        name: &str,
        arguments: &[Symbol],
    ) -> Result<Vec<Symbol>, ExternalError> {
        if name == "c" && arguments.len() == 0 {
            Ok(vec![Symbol::create_number(42), Symbol::create_number(43)])
        } else {
            return Err(ExternalError {
                msg: "function not found",
            });
        }
    }
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let mut ctl = control(options).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    ctl.add("base", &[], "p(@c()). p(d). p(e).")
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
    let mut ctl = ctl.register_function_handler(MyEFH);
    ctl.ground(&parts).unwrap_or_else(|e| {
        panic!("Failed to ground a logic program. {:?}", e);
    });

    // solve
    let mut handle = ctl
        .solve(SolveMode::YIELD, &[])
        .expect("Failed to retrieve solve handle.");

    print_model(handle.model().unwrap().unwrap());

    handle.close().expect("Failed to close solve handle.");
}
