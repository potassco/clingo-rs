

extern crate clingo;
extern crate rand;

use std::env;
use std::ffi::CString;
use clingo::*;
use rand::distributions::{IndependentSample, Range};

use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};


fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

unsafe extern "C" fn on_event(etype: clingo_solve_event_type_t,
                              event: *mut ::std::os::raw::c_void,
                              data: *mut ::std::os::raw::c_void,
                              goon: *mut bool)
                              -> bool {
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

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = new_clingo_control(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err = ctl.add("base",
                      parameters,
                      "#const n = 17.1 { p(X); q(X) } 1 :- X = 1..n.:- not n+1 { p(1..n); \
                       q(1..n) }.");
    if !err {
        return error_main();
    }

    // ground the base part
    let part = ClingoPart {
        name: CString::new("base").unwrap(),
        params: &[],
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err = ctl.ground(parts, ground_callback, ground_callback_data);
    if !err {
        return error_main();
    }

//     let mut running = ATOMIC_BOOL_INIT;
    let running = std::ptr::null_mut();

    // create a solve handle with an attached vent handler
    let assumptions = vec![];
    let solve_event_callback: clingo_solve_event_callback_t = Some(on_event);
    let mut handle =
        ctl.solve((clingo_solve_mode::clingo_solve_mode_async as clingo_solve_mode_bitset_t) +
                   (clingo_solve_mode::clingo_solve_mode_yield as clingo_solve_mode_bitset_t),
                   assumptions,
                   solve_event_callback,
                   running)
            .expect("Failed to retrieve solve handle");

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
       if x*x + y*y <= 1. {
           in_circle += 1;
       }        
    }

    println!("pi = {}", 4 * in_circle * samples);

    // get the solve result
    let _result = handle.get().expect("Failed to get solve result");

    // close the handle
    handle.close();
}
