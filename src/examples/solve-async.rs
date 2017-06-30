extern crate clingo;
extern crate rand;

use std::env;
use rand::distributions::{IndependentSample, Range};
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
use clingo::*;


fn error_main() {
    let error_message = clingo::error_message();
    println!("Error {}: {}", clingo::error_code(), error_message);
}

extern "C" fn on_event(
    etype: clingo_solve_event_type_t,
    event: *mut ::std::os::raw::c_void,
    data: *mut ::std::os::raw::c_void,
    goon: *mut bool,
) -> bool {
    //   (void)type;
    //   (void)event;
    //   (void)goon; // this is true by default
    //   if (type == clingo_solve_event_type_finish) {
    //       atomic_flag *running = (atomic_flag*)data;
    //       atomic_flag_clear(running);
    //   }
    return true;
}

fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = ClingoControl::new(options, logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    if let Err(e) = ctl.add(
        "base",
        parameters,
        "#const n = 17.1 { p(X); q(X) } 1 :- X = 1..n.:- not n+1 { p(1..n); \
                       q(1..n) }.",
    )
    {
        println!("{}",e);
        return;
    }

    // ground the base part
    let part = ClingoPart::new_part("base", &[]);
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    if let Err(e) = ctl.ground(parts, ground_callback, ground_callback_data) {
        println!("{}", e);
        return;
    }

    //     let mut running = ATOMIC_BOOL_INIT;
    let running = std::ptr::null_mut();

    // create a solve handle with an attached vent handler
    let assumptions = vec![];
    let solve_event_callback: ClingoSolveEventCallback = Some(on_event);
    let mut handle = ctl.solve(
        (clingo_solve_mode::clingo_solve_mode_async as clingo_solve_mode_bitset_t) +
            (clingo_solve_mode::clingo_solve_mode_yield as
                 clingo_solve_mode_bitset_t),
        assumptions,
        solve_event_callback,
        running,
    ).expect("Failed to retrieve solve handle");

    // let's approximate pi
    let mut samples = 0;
    let mut in_circle = 0;

    let between = Range::new(-1f64, 1.);
    let mut rng = rand::thread_rng();
    //         while (atomic_flag_test_and_set(&running)) {
    while samples < 10000000 {
        samples = samples + 1;
        let x = between.ind_sample(&mut rng);
        let y = between.ind_sample(&mut rng);
        if x * x + y * y <= 1. {
            in_circle += 1;
        }
    }

    println!("pi = {}", 4 * in_circle * samples);

    // get the solve result
    let _result = handle.get().expect("Failed to get solve result");

    // close the handle
    handle.close();
}
