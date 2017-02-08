#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");


use std::env;
use std::iter;
use std::ffi::CStr;
use std::slice;

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
//     let mut ctl = std::ptr::null_mut() as *mut clingo_control_t;
//     let control = &mut ctl as *mut *mut clingo_control_t;
//     let arguments_size = arguments.len()-1;
//     // create a vector of zero terminated strings
//     let args = arguments.map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
//     // convert the strings to raw pointers
//     let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
//     
//     unsafe {
//              println!("x1{:?}",control);
//              println!("x2{:?}",*control);
//              let ret = clingo_control_new(c_args.as_ptr(),
//                                           arguments_size,
//                                           logger,
//                                           logger_data,
//                                           message_limit,
//                                           control);
//                                           
//              println!("y1{:?}",control);
//              println!("y2{:?}",*control);
//              
//              if ret == 0 { None }
//              else { Some(*control) }
//    }
// }                              

pub fn safe_clingo_control_new(arguments: std::env::Args,
                               logger: clingo_logger_t,
                               logger_data: *mut c_void,
                               message_limit: ::std::os::raw::c_uint
                              )
                              -> std::option::Option< *mut clingo_control_t> {

    let mut ctl = std::ptr::null_mut() as *mut clingo_control_t;
    let control = &mut ctl as *mut *mut clingo_control_t;
    let arguments_size = arguments.len()-1;
    // create a vector of zero terminated strings
    let args = arguments.map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    
    unsafe {
             let ret = clingo_control_new(c_args.as_ptr(),
                                          arguments_size,
                                          logger,
                                          logger_data,
                                          message_limit,
                                          control);
             if ret == 0 { None }
             else { Some(*control) }
   }
}

pub fn safe_clingo_control_add(control: *mut clingo_control_t,
                               name: &str,
                               parameters: Vec<&str>,
                               program: &str
                              )
                              -> std::option::Option< *mut clingo_control_t> { 

    let mname = CString::new(name).unwrap();
    let mprogram = CString::new(program).unwrap();
    let parameters_size = parameters.len();
    // create a vector of zero terminated strings
    let args = parameters.into_iter().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    
    unsafe {
             let ret = clingo_control_add(control,
                              mname.as_ptr(),
                              c_args.as_ptr(),
                              parameters_size,
                              mprogram.as_ptr());                           
             if ret == 0 { None }
             else { Some(control) }
    }
}

pub struct safe_clingo_part {
    pub params: clingo_symbol_t,
    pub size: size_t,
    pub name: CString,
}
fn from_clingo_part(spart: &safe_clingo_part) -> clingo_part {
  clingo_part {name : spart.name.as_ptr(), params : &spart.params, size : spart.size }
}

pub fn safe_clingo_control_ground(control: *mut clingo_control_t,
                                  sparts: Vec<safe_clingo_part>,
                                  ground_callback: clingo_ground_callback_t,
                                  ground_callback_data: *mut c_void
                                 )
                                 -> bool {
  
    let parts = sparts.iter().map(|arg| from_clingo_part(arg)).collect::<Vec<clingo_part>>();
    let parts_size = parts.len();    
    
    unsafe { clingo_control_ground(control, parts.as_ptr(), parts_size, ground_callback, ground_callback_data)==1 }
}
pub fn safe_clingo_control_solve(control: *mut clingo_control_t,
                                 model_callback: clingo_model_callback_t,
                                 model_callback_data: *mut c_void,
                                 assumptions: Vec<clingo_symbolic_literal_t>,
                                 result: *mut clingo_solve_result_bitset_t
                                )
                                -> bool {
    let assumptions_size = assumptions.len();
    unsafe { clingo_control_solve(control,
                                  model_callback,
                                  model_callback_data,
                                  assumptions.as_ptr(),
                                  assumptions_size,
                                  result)==1  
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
             let c_buf : *const c_char = clingo_error_message();
             println!("h1{:?}",c_buf);
             if c_buf.is_null() { return "";}
             else {
                let c_str = CStr::from_ptr(c_buf);
                println!("h2{:?}",c_str);
                let blub = c_str.to_str();
                println!("hh{:?}",blub);
                return blub.unwrap();
             }
    }
}

pub fn safe_clingo_model_symbols(model: *mut clingo_model_t,
                                 show: clingo_show_type,
                                )
                                -> std::result::Result<Vec<clingo_symbol_t>, u8> {
                               
    let ishow = show as u32;
    let mut size : usize = 0;
    let size_p = &mut size as *mut usize;
    unsafe { 
             let err1 = clingo_model_symbols_size(model, ishow, size_p);
             if err1 == 0 { return Err(err1); }
             else {
                let mut a1  = Vec::<clingo_symbol_t>::with_capacity(size);              
                let slice   = a1.as_mut_slice();
                let symbols = slice.as_ptr() as *mut clingo_symbol_t;
                let err2    = clingo_model_symbols(model, ishow, symbols, size);
                if err2  == 0 { return Err(err2); }
                else {
                  let res = Vec::from_raw_parts(symbols, size, size);
                  return Ok(res);
}   }         } }

pub fn safe_clingo_symbol_to_string(symbol: &clingo_symbol_t) -> std::result::Result<CString, u8> {
                                   
    let mut size : usize = 0;
    let size_p = &mut size as *mut usize;
    unsafe {    
      let sym = *symbol as clingo_symbol_t;
      let err1 = clingo_symbol_to_string_size(sym, size_p);
      let mut a1  = Vec::<u8>::new();//with_capacity(size);
      for i in 1..size {
            a1.push(1);
      }
      let mut string = CString::from_vec_unchecked(a1);

      let err2 = clingo_symbol_to_string(sym,
                                         string.as_ptr() as *mut i8,
                                         size);
      return Ok(string);
    }
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
