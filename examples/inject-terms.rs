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
        _location: &ast::Location,
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

struct CtrlCtx {
    non: defaults::Non,
    function_handler: MyEFH,
}
impl ControlCtx for CtrlCtx {
    type L = defaults::Non;
    type P = defaults::Non;
    type O = defaults::Non;
    type F = MyEFH;

    fn logger(&mut self) -> (&mut Self::L, u32) {
        (&mut self.non, 0)
    }
    fn propagator(&mut self) -> (&mut Self::P, bool) {
        (&mut self.non, false)
    }
    fn observer(&mut self) -> (&mut Self::O, bool) {
        (&mut self.non, false)
    }
    fn function_handler(&mut self) -> &mut Self::F {
        &mut self.function_handler
    }
}

fn main() {
    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    let ctrl_ctx = CtrlCtx {
        non: defaults::Non,
        function_handler: MyEFH,
    };
    // create a control object and pass command line arguments
    let mut ctl = control_with_context(options, ctrl_ctx).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    ctl.add("base", &[], "p(@c()). p(d). p(e).")
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", vec![]).unwrap();
    let parts = vec![part];
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
