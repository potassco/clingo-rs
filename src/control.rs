use std::env;

extern crate clingo;
use clingo::*;
extern crate libc;
use libc::c_void;
use libc::c_char;


//callback function
extern fn on_model(model: *mut clingo_model_t, data: *mut c_void, goon: *mut u8) -> u8 {
  println!("Im the solve callback on_model");

  // retrieve the symbols in the model
  let atoms = safe_clingo_model_symbols(model, clingo_show_type::clingo_show_type_shown)
      .expect("Failed to retrieve symbols in the model");

  println!("Model:");
  for atom in &atoms {
    // retrieve the symbol's string
    let atom_string = safe_clingo_symbol_to_string(atom).unwrap();
    println!(" {}", atom_string.to_str().unwrap());
  }
  return 1;
}

// pub type clingo_logger_t =
//     ::std::option::Option<unsafe extern "C" fn(code: clingo_warning_t,
//                                                  message: *const c_char,
//                                                  data: *mut c_void
// )>;
//replace with safe version
// pub type safe_clingo_logger_t =
//     ::std::option::Option<fn(code: clingo_warning_t,
//                                    message: &str,
//                                    data: &safe_logger_data
// )>;

extern fn my_logger_callback(code: clingo_warning_t, message: *const c_char, data: *mut c_void) {
    println!("Im the logger callback function");
}

extern fn my_ground_callback(location: clingo_location_t,
                             name: *const c_char,
                             arguments: *const clingo_symbol_t,
                             arguments_size: size_t,
                             data: *mut c_void,
                             symbol_callback: clingo_symbol_callback_t,
                             symbol_callback_data: *mut c_void
                            )
                            -> u8 {
    println!("Im the ground callback function");
    return 1;
}


use std::ffi::CString;
fn main() {

  // create a control object and pass command line arguments
  let logger: clingo_logger_t = None;
  let logger_data: *mut c_void = std::ptr::null_mut();
  let control = safe_clingo_control_new(env::args(), logger, logger_data, 20).unwrap();
//   if ctlref2==None { return error_main(); }

  // add a logic program to the base part
  let parameters : Vec<&str> = Vec::new();

  let ctlref3 = safe_clingo_control_add(control, "base", parameters, "a :- not b. b :- not a." );

  // ground the base part
  let part = safe_clingo_part {params : 0, size : 0, name : CString::new("base").unwrap() };
  let parts = vec![part];
  
  let ground_callback : clingo_ground_callback_t = Some(my_ground_callback);
  let ground_callback_data : *mut c_void = std::ptr::null_mut();

  if !safe_clingo_control_ground(control, parts, ground_callback, ground_callback_data) { return error_main(); }
  
  // solve using a model callback
  let mut sret = 0 as ::std::os::raw::c_uint;
  let solve_ret : *mut clingo_solve_result_bitset_t = &mut sret;
  let solve_callback : clingo_model_callback_t = Some(on_model); 
  let solve_callback_data : *mut c_void = std::ptr::null_mut();
  let assumptions = vec![];

  if !safe_clingo_control_solve(control, solve_callback, solve_callback_data, assumptions, solve_ret) { return error_main(); }
  
//   safe_clingo_control_free(control);

}

fn error_main() {
  let error_message = safe_clingo_error_message();
  println!("{}", error_message);
//   safe_clingo_error_code();
}

