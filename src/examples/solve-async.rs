extern crate clingo;
extern crate rand;

use std::env;
use rand::distributions::{IndependentSample, Range};
use std::sync::atomic::{AtomicBool, Ordering};
use clingo::*;

struct MySEHandler;
impl ClingoSolveEventHandler<AtomicBool> for MySEHandler {
    fn on_solve_event(
        type_: ClingoSolveEventType,
        data: &mut AtomicBool,
        _goon: &mut bool,
    ) -> bool {
        if type_ == ClingoSolveEventType::finish {
            data.store(false, Ordering::Relaxed);
        }
        true
    }
}

fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating ClingoControl.");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    ctl.add(
        "base",
        parameters,
        "#const n = 17.\
         1 { p(X); q(X) } 1 :- X = 1..n.\
         :- not n+1 { p(1..n); \
         q(1..n) }.",
    ).expect("Failed to add a logic program.");

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    ctl.ground(parts).expect(
        "Failed to ground a logic program.",
    );

    let solve_event_handler = MySEHandler;
    let mut running = AtomicBool::new(true);

    // create a solve handle with an attached event handler
    let handle = ctl.solve_with_event_handler(
        (clingo_solve_mode::mode_async as clingo_solve_mode_bitset_t) +
            (clingo_solve_mode::mode_yield as clingo_solve_mode_bitset_t),
        vec![],
        solve_event_handler,
        &mut running,
    ).expect("Failed to retrieve solve handle.");

    // let's approximate pi
    let mut samples = 0.;
    let mut in_circle = 0.;
    let between = Range::new(-1f64, 1.);
    let mut rng = rand::thread_rng();

    while running.load(Ordering::Relaxed) {
        samples += 1.;
        let x = between.ind_sample(&mut rng);
        let y = between.ind_sample(&mut rng);
        if x * x + y * y <= 1. {
            in_circle += 1.;
        }
    }
    println!("pi = {}", 4. * in_circle / samples);

    // get the solve result
    handle.get().expect(
        "Failed to get result from solve handle.",
    );

    // close the handle
    handle.close().expect("Failed to close solve handle.");
}
