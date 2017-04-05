use std::env;
extern crate clingo;
use clingo::*;
extern crate libc;
use libc::c_void;
use std::ffi::CString;


fn error_main() {
    let error_message = safe_clingo_error_message();
    println!("{}", error_message);
    safe_clingo_error_code();
}

fn main() {

    // create a control object and pass command line arguments
    let logger: clingo_logger_t = None;
    let logger_data: *mut c_void = std::ptr::null_mut();
    let ctl = safe_clingo_control_new(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    if !safe_clingo_control_add(ctl, "base", parameters, "a. {b}. #external c.") {
        return error_main();
    }

    // ground the base part
    let part = safe_clingo_part {
        params: 0,
        size: 0,
        name: CString::new("base").unwrap(),
    };
    let parts = vec![part];
    let ground_callback: clingo_ground_callback_t = None;
    let ground_callback_data: *mut c_void = std::ptr::null_mut();

    if !safe_clingo_control_ground(ctl, parts, ground_callback, ground_callback_data) {
        return error_main();
    }

    // get symbolic atoms
    let ato = safe_clingo_control_symbolic_atoms(ctl).unwrap();

    println!("Symbolic atoms:");

    // get begin and end iterator
    let sig_ptr = std::ptr::null();
    let mut it_a = safe_clingo_symbolic_atoms_begin(ato, sig_ptr).unwrap();
    let ie_a = safe_clingo_symbolic_atoms_end(ato).unwrap();

    let mut equal = safe_clingo_symbolic_atoms_iterator_is_equal_to(ato, it_a, ie_a).unwrap();
    while !equal {

        let symbol = safe_clingo_symbolic_atoms_symbol(ato, it_a).unwrap();
        let atom_string = safe_clingo_symbol_to_string(&symbol).unwrap();
        print!("  {}", atom_string.to_str().unwrap());


        let fact = safe_clingo_symbolic_atoms_is_fact(ato, it_a).unwrap();
        if fact {
            print!(", fact");
        }
        let external = safe_clingo_symbolic_atoms_is_external(ato, it_a).unwrap();
        if external {
            print!(", external");
        }
        println!("");
        it_a = safe_clingo_symbolic_atoms_next(ato, it_a).unwrap();

        equal = safe_clingo_symbolic_atoms_iterator_is_equal_to(ato, it_a, ie_a).unwrap();
    }
}
