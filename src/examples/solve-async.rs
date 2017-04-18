use std::env;
extern crate clingo;
use clingo::*;
extern crate libc;
use libc::c_void;
use std::ffi::CString;

use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};


unsafe extern "C" fn on_model(model: *mut clingo_model_t, data: *mut c_void, goon: *mut u8) -> u8 {

    // retrieve the symbols in the model
    let atoms = (*model).symbols(clingo_show_type::clingo_show_type_shown as clingo_show_type_bitset_t)
        .expect("Failed to retrieve symbols in the model");

    println!("Model:");
    for atom in atoms {
        // retrieve and print the symbol's string
        let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
        println!(" {}", atom_string.to_str().unwrap());
    }
    return 1;
}

extern "C" fn on_finish(result: clingo_solve_result_bitset_t ,atomic_flag: *mut c_void) -> u8 {
//   (void)result;
//   atomic_flag_clear(running);
  return 1;
}

fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

fn main() {

    // create a control object and pass command line arguments
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let mut ctl = new_clingo_control(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err1 = ctl.add("base", parameters, "#const n = 17.\
                                            1 { p(X); q(X) } 1 :- X = 1..n.\
                                            :- not n+1 { p(1..n); q(1..n) }.");
    if err1 == 0 {
        return error_main();
    }
    
    // ground the base part
    let part = safe_clingo_part {
        params: 0,
        size: 0,
        name: CString::new("base").unwrap(),
    };
    let parts = vec![part];
    let ground_callback = None;
    let ground_callback_data = std::ptr::null_mut();
    let err2 = ctl.ground(parts, ground_callback, ground_callback_data);
    if err2 == 0 {
        return error_main();
    }
    
//   atomic_flag running = ATOMIC_FLAG_INIT;
//   atomic_flag_test_and_set(&running);
//     let mut running = ATOMIC_BOOL_INIT;
    let running = std::ptr::null_mut();
//     *running = ATOMIC_BOOL_INIT;

    // solve using a model callback
    let solve_callback: clingo_model_callback_t = Some(on_model);
    let solve_callback_data = std::ptr::null_mut();
    let finish_callback: clingo_finish_callback_t = Some(on_finish);
    let assumptions = vec![];

    let async = ctl.solve_async(solve_callback, solve_callback_data, finish_callback, running, assumptions).unwrap();

    // let's approximate pi
//      while (atomic_flag_test_and_set(&running)) {
//     ++samples;
//     x = rand();
//     y = rand();
//     if (x * x + y * y <= (uint64_t)RAND_MAX * RAND_MAX) { incircle+= 1; }
//   }

     println!("pi = {}", 4.0*incircle//*samples*/);

     // get the result (and make sure the search is running because calling the finish handler is still part of the search)
     let _solve_ret = async.get().unwrap();

}

