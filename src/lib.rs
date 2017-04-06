#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// mod clingo {
include!("bindings.rs");
// }
// use clingo;
use std::ffi::CStr;

pub fn safe_clingo_version() -> (i32, i32, i32) {
    let mut m1 = 0;
    let ma = &mut m1 as *mut c_int;

    let mut m2 = 0;
    let mi = &mut m2 as *mut c_int;

    let mut m3 = 0;
    let re = &mut m3 as *mut c_int;

    unsafe { clingo_version(ma, mi, re) };

    let major = unsafe { *ma };
    let minor = unsafe { *mi };
    let revision = unsafe { *re };
    (major, minor, revision)
}


// pub fn safe_clingo_control_new2(arguments: std::env::Args,
//                                logger: clingo_logger_t,
//                                logger_data: *mut c_void,
//                                message_limit: ::std::os::raw::c_uint
//                               )
//                               -> std::option::Option< &mut clingo_control_t> {
//
//     let mut control = std::ptr::null_mut() as *mut clingo_control_t;
//     let arguments_size = arguments.len()-1;
//     // create a vector of zero terminated strings
//     let args = arguments.map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
//     // convert the strings to raw pointers
//     let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
//
//     unsafe {
//              println!("x1{:?}",&control);
//              println!("x2{:?}",control);
//              let ret = clingo_control_new(c_args.as_ptr(),
//                                           arguments_size,
//                                           logger,
//                                           logger_data,
//                                           message_limit,
//                                           &mut control);
//
//              println!("y1{:?}",&control);
//              println!("y2{:?}",control);
//
//              if ret == 0 { None }
//              else { Some(control) }
//    }
// }

pub fn safe_clingo_control_new(arguments: std::env::Args,
                               logger: clingo_logger_t,
                               logger_data: *mut c_void,
                               message_limit: ::std::os::raw::c_uint)
                               -> std::option::Option<*mut clingo_control_t> {

    let arguments_size = arguments.len() - 1;
    // create a vector of zero terminated strings
    let args = arguments.map(|arg| CString::new(arg).unwrap()).collect::<Vec<CString>>();
    // drop first element
    let (_, tail) = args.split_first().unwrap();
    // convert the strings to raw pointers
    let c_args = tail.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

    let mut control = std::ptr::null_mut() as *mut clingo_control_t;

    unsafe {
        let ret = clingo_control_new(c_args.as_ptr(),
                                     arguments_size,
                                     logger,
                                     logger_data,
                                     message_limit,
                                     &mut control);
        if ret == 0 {
            None
        } else {
            Some(control)
        }
    }
}

pub fn safe_clingo_control_add(control: *mut clingo_control_t,
                               name: &str,
                               parameters: Vec<&str>,
                               program: &str)
                               -> bool {

    let mname = CString::new(name).unwrap();
    let mprogram = CString::new(program).unwrap();
    let parameters_size = parameters.len();
    // create a vector of zero terminated strings
    let args =
        parameters.into_iter().map(|arg| CString::new(arg).unwrap()).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

    unsafe {
        clingo_control_add(control,
                           mname.as_ptr(),
                           c_args.as_ptr(),
                           parameters_size,
                           mprogram.as_ptr()) == 1
    }
}

pub struct safe_clingo_part {
    pub params: clingo_symbol_t,
    pub size: size_t,
    pub name: CString,
}
fn from_clingo_part(spart: &safe_clingo_part) -> clingo_part {
    clingo_part {
        name: spart.name.as_ptr(),
        params: &spart.params,
        size: spart.size,
    }
}

pub fn safe_clingo_control_ground(control: *mut clingo_control_t,
                                  sparts: Vec<safe_clingo_part>,
                                  ground_callback: clingo_ground_callback_t,
                                  ground_callback_data: *mut c_void)
                                  -> bool {

    let parts = sparts.iter().map(|arg| from_clingo_part(arg)).collect::<Vec<clingo_part>>();
    let parts_size = parts.len();

    unsafe {
        clingo_control_ground(control,
                              parts.as_ptr(),
                              parts_size,
                              ground_callback,
                              ground_callback_data) == 1
    }
}
pub fn safe_clingo_control_solve(control: *mut clingo_control_t,
                                 model_callback: clingo_model_callback_t,
                                 model_callback_data: *mut c_void,
                                 assumptions: Vec<clingo_symbolic_literal_t>,
                                 result: *mut clingo_solve_result_bitset_t)
                                 -> bool {
    let assumptions_size = assumptions.len();
    unsafe {
        clingo_control_solve(control,
                             model_callback,
                             model_callback_data,
                             assumptions.as_ptr(),
                             assumptions_size,
                             result) == 1
    }
}
pub fn safe_clingo_control_free(control: *mut *mut clingo_control_t) {
    unsafe { clingo_control_free(*control) }
}
pub fn safe_clingo_error_code() -> clingo_error_t {
    unsafe { clingo_error_code() }
}
pub fn safe_clingo_error_message() -> &'static str {
    unsafe {
        let c_buf: *const c_char = clingo_error_message();
        println!("h1{:?}", c_buf);
        if c_buf.is_null() {
            return "";
        } else {
            let c_str = CStr::from_ptr(c_buf);
            println!("h2{:?}", c_str);
            let blub = c_str.to_str();
            println!("hh{:?}", blub);
            return blub.unwrap();
        }
    }
}

pub fn safe_clingo_model_symbols(model: *mut clingo_model_t,
                                 show: clingo_show_type)
                                 -> std::result::Result<Vec<clingo_symbol_t>, u8> {

    let ishow = show as u32;
    let mut size: usize = 0;
    let size_p = &mut size as *mut usize;
    unsafe {
        let err1 = clingo_model_symbols_size(model, ishow, size_p);
        if err1 == 0 {
            Err(err1)
        } else {
            let mut a1 = Vec::<clingo_symbol_t>::with_capacity(size);
            let slice = a1.as_mut_slice();
            let symbols = slice.as_ptr() as *mut clingo_symbol_t;
            let err2 = clingo_model_symbols(model, ishow, symbols, size);
            if err2 == 0 {
                Err(err2)
            } else {
                let res = Vec::from_raw_parts(symbols, size, size);
                Ok(res)
            }
        }
    }
}

pub fn safe_clingo_symbol_to_string(symbol: &clingo_symbol_t) -> std::result::Result<CString, u8> {

    let mut size: usize = 0;
    let size_p = &mut size as *mut usize;
    unsafe {
        let sym = *symbol as clingo_symbol_t;
        let err1 = clingo_symbol_to_string_size(sym, size_p);
        if err1 == 0 {
            Err(err1)
        } else {
            let a1 = vec![1; size];
            let string = CString::from_vec_unchecked(a1);

            let err2 = clingo_symbol_to_string(sym, string.as_ptr() as *mut i8, size);
            if err2 == 0 {
                Err(err2)
            } else {
                Ok(string)
            }
        }
    }
}

pub fn safe_clingo_control_symbolic_atoms
    (control: *mut clingo_control_t)
     -> std::result::Result<*mut clingo_symbolic_atoms_t, u8> {
    unsafe {
        let mut ato = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
        let err = clingo_control_symbolic_atoms(control, &mut ato);
        if err == 0 {
            Err(err)
        } else {
            Ok(ato)
        }
    }
}

pub fn safe_clingo_symbolic_atoms_begin
    (atoms: *mut clingo_symbolic_atoms_t,
     signature: *const clingo_signature_t)
     -> std::result::Result<clingo_symbolic_atom_iterator_t, u8> {
    unsafe {
        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        let err = clingo_symbolic_atoms_begin(atoms, signature, &mut iterator);
        if err == 0 {
            Err(err)
        } else {
            Ok(iterator)
        }
    }
}

pub fn safe_clingo_symbolic_atoms_end
    (atoms: *mut clingo_symbolic_atoms_t)
     -> std::result::Result<clingo_symbolic_atom_iterator_t, u8> {
    unsafe {
        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        let err = clingo_symbolic_atoms_end(atoms, &mut iterator);
        if err == 0 {
            Err(err)
        } else {
            Ok(iterator)
        }
    }
}

pub fn safe_clingo_symbolic_atoms_iterator_is_equal_to(atoms: *mut clingo_symbolic_atoms_t,
                                                       a: clingo_symbolic_atom_iterator_t,
                                                       b: clingo_symbolic_atom_iterator_t)
                                                       -> std::result::Result<bool, u8> {
    unsafe {
        let mut equal = 0;
        let err = clingo_symbolic_atoms_iterator_is_equal_to(atoms, a, b, &mut equal);
        if err == 0 {
            Err(err)
        } else {
            Ok(equal == 1)
        }
    }
}

pub fn safe_clingo_symbolic_atoms_symbol(atoms: *mut clingo_symbolic_atoms_t,
                                         iterator: clingo_symbolic_atom_iterator_t)
                                         -> std::result::Result<clingo_symbol_t, u8> {
    unsafe {
        let mut symbol = 0 as clingo_symbol_t;
        let err = clingo_symbolic_atoms_symbol(atoms, iterator, &mut symbol);
        if err == 0 {
            Err(err)
        } else {
            Ok(symbol)
        }
    }
}

pub fn safe_clingo_symbolic_atoms_is_fact(atoms: *mut clingo_symbolic_atoms_t,
                                          iterator: clingo_symbolic_atom_iterator_t)
                                          -> std::result::Result<bool, u8> {
    unsafe {
        let mut fact = 0;
        let err = clingo_symbolic_atoms_is_fact(atoms, iterator, &mut fact);
        if err == 0 {
            Err(err)
        } else {
            Ok(fact == 1)
        }
    }
}

pub fn safe_clingo_symbolic_atoms_is_external(atoms: *mut clingo_symbolic_atoms_t,
                                              iterator: clingo_symbolic_atom_iterator_t)
                                              -> std::result::Result<bool, u8> {
    unsafe {
        let mut external = 0;
        let err = clingo_symbolic_atoms_is_external(atoms, iterator, &mut external);
        if err == 0 {
            Err(err)
        } else {
            Ok(external == 1)
        }
    }
}

pub fn safe_clingo_symbolic_atoms_next
    (atoms: *mut clingo_symbolic_atoms_t,
     iterator: clingo_symbolic_atom_iterator_t)
     -> std::result::Result<clingo_symbolic_atom_iterator_t, u8> {
    unsafe {
        let mut next = 0 as clingo_symbolic_atom_iterator_t;
        let err = clingo_symbolic_atoms_next(atoms, iterator, &mut next);
        if err == 0 {
            Err(err)
        } else {
            Ok(next)
        }
    }
}

pub fn safe_clingo_symbol_create_number(number: c_int) -> clingo_symbol_t {
    unsafe {
        let mut symbol = 0 as clingo_symbol_t;
        clingo_symbol_create_number(number, &mut symbol);
        symbol
    }
}

pub fn safe_clingo_symbol_create_id(name: &str,
                                    positive: bool)
                                    -> std::result::Result<clingo_symbol_t, u8> {
    unsafe {
        let mut symbol = 0 as clingo_symbol_t;
        if positive {
            let err = clingo_symbol_create_id(CString::new(name).unwrap().as_ptr(), 1, &mut symbol);
            if err == 0 {
                Err(err)
            } else {
                Ok(symbol)
            }
        } else {
            let err = clingo_symbol_create_id(CString::new(name).unwrap().as_ptr(), 0, &mut symbol);
            if err == 0 {
                Err(err)
            } else {
                Ok(symbol)
            }
        }
    }
}

pub fn safe_clingo_symbol_create_function(name: &str,
                                          arguments: &[&clingo_symbol_t],
                                          arguments_size: size_t,
                                          positive: bool)
                                          -> std::result::Result<clingo_symbol_t, u8> {
    unsafe {
        let mut symbol = 0 as clingo_symbol_t;
        if positive {
            let err = clingo_symbol_create_function(CString::new(name).unwrap().as_ptr(),
                                                    *(arguments.as_ptr()),
                                                    arguments_size,
                                                    1,
                                                    &mut symbol);
            if err == 0 {
                Err(err)
            } else {
                Ok(symbol)
            }
        } else {
            let err = clingo_symbol_create_function(CString::new(name).unwrap().as_ptr(),
                                                    *(arguments.as_ptr()),
                                                    arguments_size,
                                                    0,
                                                    &mut symbol);
            if err == 0 {
                Err(err)
            } else {
                Ok(symbol)
            }
        }
    }
}

pub fn safe_clingo_symbol_hash(symbol: &clingo_symbol_t) -> size_t {
    unsafe { clingo_symbol_hash(*symbol) }
}


pub fn safe_clingo_symbol_arguments(symbol: &clingo_symbol_t)
                                    -> std::result::Result<Vec<&clingo_symbol_t>, u8> {
    unsafe {
        let mut a_ptr = std::ptr::null() as *const clingo_symbol_t;
        let mut size: usize = 0;
        let err = clingo_symbol_arguments(*symbol, &mut a_ptr, &mut size);
        if err == 0 {
            Err(err)
        } else {

            let mut a1 = Vec::<&clingo_symbol_t>::with_capacity(size);
            for i in 0..size {
                let symbol_ref = a_ptr.offset(i as isize)
                    .as_ref()
                    .unwrap();
                a1.push(symbol_ref);
            }
            Ok(a1)
        }
    }
}

pub fn safe_clingo_symbol_is_equal_to(a: &clingo_symbol_t, b: &clingo_symbol_t) -> bool {
    unsafe { clingo_symbol_is_equal_to(*a, *b) == 1 }
}

pub fn safe_clingo_symbol_is_less_than(a: &clingo_symbol_t, b: &clingo_symbol_t) -> bool {
    unsafe { clingo_symbol_is_less_than(*a, *b) == 1 }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_test() {
        let (ma, mi, re) = safe_clingo_version();
        assert!(ma == 5);
        assert!(mi == 0);
        assert!(re == 0);
    }
}
