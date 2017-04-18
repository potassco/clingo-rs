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
    let mut ctl = new_clingo_control(env::args(), logger, logger_data, 20)
        .expect("Failed creating clingo_control");

    // add a logic program to the base part
    let parameters: Vec<&str> = Vec::new();
    let err1 = ctl.add("base", parameters, "a. {b}. #external c.");
    if err1 == 0 {
        return error_main();
    }

    // ground the base part
    let part = safe_clingo_part {
        name: CString::new("base").unwrap(),      
        params: &[],
    };
    let parts = vec![part];
    let ground_callback: clingo_ground_callback_t = None;
    let ground_callback_data: *mut c_void = std::ptr::null_mut();
    let err2 = ctl.ground(parts, ground_callback, ground_callback_data);
    if err2 == 0 {
        return error_main();
    }

    // get symbolic atoms
    let atoms = ctl.symbolic_atoms().unwrap();

    println!("Symbolic atoms:");

    // get begin and end iterator
    let sig_ptr = std::ptr::null();
    let mut it_a = atoms.begin(sig_ptr).unwrap();
    let ie_a = atoms.end().unwrap();

    loop {
        let equal = atoms.iterator_is_equal_to(it_a, ie_a).unwrap();
        if equal {
            break;
        }
        let symbol = atoms.symbol(it_a).unwrap();
        let atom_string = safe_clingo_symbol_to_string(symbol).unwrap();
        print!("  {}", atom_string.to_str().unwrap());


        let fact = atoms.is_fact(it_a).unwrap();
        if fact {
            print!(", fact");
        }
        let external = atoms.is_external(it_a).unwrap();
        if external {
            print!(", external");
        }
        println!("");

        it_a = atoms.next(it_a).unwrap();
    }
}
