extern crate clingo;
extern crate rand;

use std::env;
use rand::distributions::{IndependentSample, Range};
use std::sync::atomic::{AtomicBool, Ordering};
use clingo::*;


extern "C" fn on_event(
    etype: clingo_solve_event_type_t,
    event: *mut ::std::os::raw::c_void,
    data: *mut ::std::os::raw::c_void,
    goon: *mut bool,
) -> bool {
    if etype == clingo_solve_event_type_finish as u32 {
        let atomic_bool = unsafe { (data as *mut AtomicBool).as_ref() }.unwrap();
        atomic_bool.store(false, Ordering::Relaxed);
    }
    true
}

fn main() {

    // collect clingo options from the command line
    let options = env::args().skip(1).collect();

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let ctl = ClingoControl::new(options, logger, logger_data, 20)
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
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    ctl.ground(parts, ground_callback, ground_callback_data)
        .expect("Failed to ground a logic program.");

    
    // create a solve handle with an attached vent handler
    let assumptions = vec![];
    let solve_event_callback: ClingoSolveEventCallback = Some(on_event);
    let mut running = AtomicBool::new(true);
    let running_ref = &mut running as *mut std::sync::atomic::AtomicBool;
    let handle = ctl.solve(
        (clingo_solve_mode_async as clingo_solve_mode_bitset_t) +
            (clingo_solve_mode_yield as clingo_solve_mode_bitset_t),
        assumptions,
        solve_event_callback,
        running_ref as *mut std::os::raw::c_void,
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
