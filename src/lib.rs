
extern crate libc;
extern crate clingo_sys;

use libc::c_void;
use libc::c_int;
use libc::c_char;

use clingo_sys::*;

use std::ffi::CStr;
use std::ffi::CString;

pub use clingo_sys::{clingo_show_type_bitset_t,clingo_show_type, uint64_t, clingo_logger_t, clingo_literal_t, clingo_id_t};
use clingo_sys::{clingo_ground_callback_t, clingo_model_callback_t};
pub type ClingoModelCallback =
    ::std::option::Option<unsafe extern "C" fn(model: &mut ClingoModel,
                                               data: *mut c_void,
                                               goon: *mut u8)
                                                 -> u8>;
pub type ClingoGroundCallback =
    ::std::option::Option<unsafe extern "C" fn(location: clingo_location_t,
                                               name: *const c_char,
                                               arguments: *const clingo_symbol_t,
                                               arguments_size: size_t,
                                               data: *mut c_void,
                                               symbol_callback: clingo_symbol_callback_t,
                                               symbol_callback_data: *mut c_void)
                              -> u8>;

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

pub struct ClingoPart<'a> {
    pub name: CString,
    pub params: &'a [clingo_symbol_t],
}

fn from_clingo_part(spart: &ClingoPart) -> clingo_part {
    clingo_part {
        name: spart.name.as_ptr(),
        params: spart.params.as_ptr(),
        size: spart.params.len(),
    }
}
pub fn safe_clingo_error_code() -> clingo_error_t {
    unsafe { clingo_error_code() }
}
pub fn safe_clingo_error_message() -> &'static str {
    unsafe {
        let c_buf: *const c_char = clingo_error_message();
        if c_buf.is_null() {
            return "";
        } else {
            let c_str = CStr::from_ptr(c_buf);
            return c_str.to_str().unwrap();
        }
    }
}

pub fn safe_clingo_set_error(code: clingo_error_t, message: &str) {
    unsafe {
        let m2 = CString::new(message).unwrap().as_ptr();
        clingo_set_error(code, m2);
    }
}
pub fn safe_clingo_symbol_to_string(symbol: clingo_symbol_t) -> std::result::Result<CString, u8> {

    let mut size: usize = 0;
    let size_p = &mut size as *mut usize;
    unsafe {
        let err1 = clingo_symbol_to_string_size(symbol, size_p);
        if err1 == 0 {
            Err(err1)
        } else {
            let a1 = vec![1; size];
            let string = CString::from_vec_unchecked(a1);

            let err2 = clingo_symbol_to_string(symbol, string.as_ptr() as *mut c_char, size);
            if err2 == 0 {
                Err(err2)
            } else {
                Ok(string)
            }
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
                                          arguments: &[clingo_symbol_t],
                                          positive: bool)
                                          -> std::result::Result<clingo_symbol_t, u8> {
    unsafe {
        let mut symbol = 0 as clingo_symbol_t;
        if positive {
            let err = clingo_symbol_create_function(CString::new(name).unwrap().as_ptr(),
                                                    arguments.as_ptr(),
                                                    arguments.len(),
                                                    1,
                                                    &mut symbol);
            if err == 0 {
                Err(err)
            } else {
                Ok(symbol)
            }
        } else {
            let err = clingo_symbol_create_function(CString::new(name).unwrap().as_ptr(),
                                                    arguments.as_ptr(),
                                                    arguments.len(),
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
pub fn safe_clingo_symbol_number(symbol: clingo_symbol_t) -> Option<size_t> {
    unsafe {
        let mut number = 0;
        let err = clingo_symbol_number(symbol, &mut number);
        if err == 0 {
            None
        } else {
            Some(number as size_t)
        }
    }
}
pub fn safe_clingo_symbol_hash(symbol: clingo_symbol_t) -> size_t {
    unsafe { clingo_symbol_hash(symbol) }
}
pub fn safe_clingo_symbol_arguments(symbol: clingo_symbol_t)
                                    -> std::result::Result<Vec<clingo_symbol_t>, u8> {
    unsafe {
        let mut a_ptr = std::ptr::null() as *const clingo_symbol_t;
        let mut size: usize = 0;
        let err = clingo_symbol_arguments(symbol, &mut a_ptr, &mut size);
        if err == 0 {
            Err(err)
        } else {

            let mut a1 = Vec::<clingo_symbol_t>::with_capacity(size);
            for _ in 0..size {
                let nsymbol = *a_ptr;
                a1.push(nsymbol);
            }
            Ok(a1)
        }
    }
}
pub fn safe_clingo_symbol_is_equal_to(a: clingo_symbol_t, b: clingo_symbol_t) -> bool {
    unsafe { clingo_symbol_is_equal_to(a, b) == 1 }
}
pub fn safe_clingo_symbol_is_less_than(a: clingo_symbol_t, b: clingo_symbol_t) -> bool {
    unsafe { clingo_symbol_is_less_than(a, b) == 1 }
}
pub fn new_clingo_control<'a>(arguments: std::env::Args,
                              logger: clingo_logger_t,
                              logger_data: *mut c_void,
                              message_limit: ::std::os::raw::c_uint)
                              -> std::result::Result<&'a mut ClingoControl, u8> {
    let arguments_size = arguments.len() - 1;
    // create a vector of zero terminated strings
    let args = arguments.map(|arg| CString::new(arg).unwrap()).collect::<Vec<CString>>();
    // drop first element
    let (_, tail) = args.split_first().unwrap();
    // convert the strings to raw pointers
    let c_args = tail.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

    let mut ctl = std::ptr::null_mut() as *mut clingo_control_t;

    unsafe {
        let err = clingo_control_new(c_args.as_ptr(),
                                     arguments_size,
                                     logger,
                                     logger_data,
                                     message_limit,
                                     &mut ctl);
        if err == 0 {
            Err(err)
        } else {
            Ok(&mut *(ctl as *mut ClingoControl))
        }
    }
}

pub struct ClingoControl(clingo_control_t);
impl Drop for ClingoControl {
    fn drop(&mut self) {
        unsafe {
            let control = &mut self.0;
            clingo_control_free(control)
        }

    }
}
impl ClingoControl {
    //     pub fn clingo_control_load(control: *mut ClingoControl, file: *const c_char) -> u8;
    pub fn add(&mut self, name: &str, parameters: Vec<&str>, program: &str) -> u8 {

        let mname = CString::new(name).unwrap();
        let mprogram = CString::new(program).unwrap();
        let parameters_size = parameters.len();
        // create a vector of zero terminated strings
        let args =
            parameters.into_iter().map(|arg| CString::new(arg).unwrap()).collect::<Vec<CString>>();
        // convert the strings to raw pointers
        let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

        unsafe {
            let control = &mut self.0;
            // let mut ClingoControl(control) = self;
            clingo_control_add(control,
                               mname.as_ptr(),
                               c_args.as_ptr(),
                               parameters_size,
                               mprogram.as_ptr())
        }
    }
    pub fn ground(&mut self,
                  sparts: Vec<ClingoPart>,
                  ground_callback: clingo_ground_callback_t,
                  ground_callback_data: *mut c_void)
                  -> u8 {

        let parts = sparts.iter().map(|arg| from_clingo_part(arg)).collect::<Vec<clingo_part>>();
        let parts_size = parts.len();

        unsafe {
            let control = &mut self.0;
            // let mut ClingoControl(control) = self;
            clingo_control_ground(control,
                                  parts.as_ptr(),
                                  parts_size,
                                  ground_callback,
                                  ground_callback_data)
        }
    }
    pub fn solve(&mut self,
                 model_callback: ClingoModelCallback,
                 model_callback_data: *mut c_void,
                 assumptions: Vec<clingo_symbolic_literal_t>)
                 -> std::result::Result<clingo_solve_result_bitset_t, u8> {
        let mut solve_result = 0 as clingo_solve_result_bitset_t;
        unsafe {
            match model_callback {
            Some(mc) => { 
                    // try casting the type of the function pointer
                    let hc = std::mem::transmute::<unsafe extern "C" fn(&mut ClingoModel, *mut libc::c_void, *mut u8) -> u8, unsafe extern "C" fn(*mut clingo_model_t, *mut libc::c_void, *mut u8) -> u8>(mc);
                    let ClingoControl(ref mut control) = *self;
                    let err = clingo_control_solve(control,
                                                Some(hc),
                                                model_callback_data,
                                                assumptions.as_ptr(),
                                                assumptions.len(),
                                                &mut solve_result);
                    if err == 0 {
                        Err(err)
                    } else {
                        Ok(solve_result)
                    }
                }
            _ => { 
                    let ClingoControl(ref mut control) = *self;
                    let err = clingo_control_solve(control,
                                                None,
                                                model_callback_data,
                                                assumptions.as_ptr(),
                                                assumptions.len(),
                                                &mut solve_result);
                    if err == 0 {
                        Err(err)
                    } else {
                        Ok(solve_result)
                    }
                }
            }
        }
    }
    pub fn solve_iteratively(&mut self,
                             assumptions: Vec<clingo_symbolic_literal_t>)
                             -> std::result::Result<&mut ClingoSolveIteratively, u8> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoControl(control) = self;
            let mut handle = std::ptr::null_mut() as *mut clingo_solve_iteratively;
            let err = clingo_control_solve_iteratively(control,
                                                       assumptions.as_ptr(),
                                                       assumptions.len(),
                                                       &mut handle);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *(handle as *mut ClingoSolveIteratively))
            }
        }
    }
    pub fn solve_async(&mut self,
                       model_callback: clingo_model_callback_t,
                       model_callback_data: *mut c_void,
                       finish_callback: clingo_finish_callback_t,
                       finish_callback_data: *mut c_void,
                       assumptions: Vec<clingo_symbolic_literal_t>)
                       -> std::result::Result<*mut clingo_solve_async_t, u8> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoControl(control) = self;
            let mut handle = std::ptr::null_mut() as *mut clingo_solve_async_t;
            
            let err = clingo_control_solve_async(control,
                                                 model_callback,
                                                 model_callback_data,
                                                 finish_callback,
                                                 finish_callback_data,
                                                 assumptions.as_ptr(),
                                                 assumptions.len(),
                                                 &mut handle);
            if err == 0 {
                Err(err)
            } else {
                Ok(handle)
            }
        }
    }

    //     pub fn clingo_control_cleanup(control: *mut ClingoControl) -> u8;
    //     pub fn clingo_control_assign_external(control: *mut ClingoControl,
    //                                           atom: clingo_symbol_t,
    //                                           value: clingo_truth_value_t)
    //                                           -> u8;
    //     pub fn clingo_control_release_external(control: *mut ClingoControl,
    //                                            atom: clingo_symbol_t)
    //                                            -> u8;
    //     pub fn clingo_control_register_propagator(control: *mut ClingoControl,
    //                                               propagator: clingo_propagator_t,
    //                                               data: *mut c_void,
    //                                               sequential: u8)
    //                                               -> u8;
    pub fn statistics(&mut self) -> std::result::Result<&mut ClingoStatistics, u8> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoControl(control) = self;
            let mut stat = std::ptr::null_mut() as *mut clingo_statistics_t;
            let err = clingo_control_statistics(control, &mut stat);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *(stat as *mut ClingoStatistics))
            }
        }
    }
    //     pub fn clingo_control_interrupt(control: *mut ClingoControl);
    pub fn configuration(&mut self) -> std::result::Result<&mut ClingoConfiguration, u8> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoControl(control) = self;
            let mut conf = std::ptr::null_mut() as *mut clingo_configuration_t;
            let err = clingo_control_configuration(control, &mut conf);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *(conf as *mut ClingoConfiguration))
            }
        }
    }
    //     pub fn clingo_control_use_enumeration_assumption(control: *mut ClingoControl,
    //                                                      enable: u8)
    //                                                     -> u8;
    //     pub fn clingo_control_get_const(control: *mut ClingoControl,
    //                                     name: *const c_char,
    //                                     symbol: *mut clingo_symbol_t)
    //                                    -> u8;
    //     pub fn clingo_control_has_const(control: *mut ClingoControl,
    //                                     name: *const c_char,
    //                                     exists: *mut u8)
    //                                    -> u8;
    pub fn symbolic_atoms(&mut self) -> std::result::Result<&mut ClingoSymbolicAtoms, u8> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoControl(control) = self;
            let mut atoms = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
            let err = clingo_control_symbolic_atoms(control, &mut atoms);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *(atoms as *mut ClingoSymbolicAtoms))
            }
        }
    }
    pub fn theory_atoms(&mut self) -> std::result::Result<&mut ClingoTheoryAtoms, u8> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoControl(control) = self;
            let mut atoms = std::ptr::null_mut() as *mut clingo_theory_atoms_t;
            let err = clingo_control_theory_atoms(control, &mut atoms);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *(atoms as *mut ClingoTheoryAtoms))
            }
        }
    }
    pub fn backend(&mut self) -> std::result::Result<&mut ClingoBackend, u8> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoControl(control) = self;
            let mut backend = std::ptr::null_mut() as *mut clingo_backend_t;
            let err = clingo_control_backend(control, &mut backend);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *(backend as *mut ClingoBackend))
            }
        }
    }
    //     pub fn clingo_control_program_builder(control: *mut ClingoControl,
    //                                           builder: *mut *mut clingo_program_builder_t)
    //                                          -> u8;
}


pub struct ClingoConfiguration(clingo_configuration_t);
impl ClingoConfiguration {
    pub fn configuration_root(&mut self) -> std::result::Result<clingo_id_t, u8> {
        unsafe {
            let conf = &mut self.0;
            // let mut ClingoConfiguration(conf) = self;
            let mut root_key = 0 as clingo_id_t;
            let err = clingo_configuration_root(conf, &mut root_key);
            if err == 0 {
                Err(err)
            } else {
                Ok(root_key)
            }
        }
    }
    //     pub fn clingo_configuration_type(configuration: *mut ClingoConfiguration,
    //                                      key: clingo_id_t,
    //                                      type_: *mut clingo_configuration_type_bitset_t)
    //                                      -> u8;
    //     pub fn clingo_configuration_description(configuration: *mut ClingoConfiguration,
    //                                             key: clingo_id_t,
    //                                             description: *mut *const c_char)
    //                                             -> u8;
    //     pub fn clingo_configuration_array_size(configuration: *mut ClingoConfiguration,
    //                                            key: clingo_id_t,
    //                                            size: *mut size_t)
    //                                            -> u8;
    pub fn configuration_array_at(&mut self,
                                  key: clingo_id_t,
                                  offset: size_t)
                                  -> std::result::Result<clingo_id_t, u8> {
        unsafe {
            let conf = &mut self.0;
            // let mut ClingoConfiguration(conf) = self;
            let mut nkey = 0 as clingo_id_t;
            let err = clingo_configuration_array_at(conf, key, offset, &mut nkey);
            if err == 0 {
                Err(err)
            } else {
                Ok(nkey)
            }
        }
    }
    //     pub fn clingo_configuration_map_size(configuration: *mut ClingoConfiguration,
    //                                          key: clingo_id_t,
    //                                          size: *mut size_t)
    //                                          -> u8;
    //     pub fn clingo_configuration_map_subkey_name(configuration: *mut ClingoConfiguration,
    //                                                 key: clingo_id_t,
    //                                                 offset: size_t,
    //                                                 name: *mut *const c_char)
    //                                                 -> u8;
    pub fn configuration_map_at(&mut self,
                                key: clingo_id_t,
                                name: &str)
                                -> std::result::Result<clingo_id_t, u8> {
        unsafe {
            let conf = &mut self.0;
            // let mut ClingoConfiguration(conf) = self;
            let mut nkey = 0 as clingo_id_t;
            let err = clingo_configuration_map_at(conf,
                                                  key,
                                                  CString::new(name).unwrap().as_ptr(),
                                                  &mut nkey);

            if err == 0 {
                Err(err)
            } else {
                Ok(nkey)
            }
        }
    }
    //     pub fn clingo_configuration_value_is_assigned(configuration: *mut ClingoConfiguration,
    //                                                   key: clingo_id_t,
    //                                                   assigned: *mut u8)
    //                                                   -> u8;
    //     pub fn clingo_configuration_value_get_size(configuration: *mut ClingoConfiguration,
    //                                                key: clingo_id_t,
    //                                                size: *mut size_t)
    //                                                -> u8;
    //     pub fn clingo_configuration_value_get(configuration: *mut ClingoConfiguration,
    //                                           key: clingo_id_t,
    //                                           value: *mut c_char,
    //                                           size: size_t)
    //                                           -> u8;
    pub fn configuration_value_set(&mut self, key: clingo_id_t, value: &str) -> u8 {
        unsafe {
            let conf = &mut self.0;
            // let mut ClingoConfiguration(conf) = self;
            clingo_configuration_value_set(conf, key, CString::new(value).unwrap().as_ptr())
        }
    }
}

pub struct ClingoBackend(clingo_backend_t);
impl ClingoBackend {
    pub fn rule(&mut self,
                choice: bool,
                head_vector: &Vec<clingo_atom_t>,
                body_vector: &Vec<clingo_literal_t>)
                -> u8 {

        let head = head_vector.as_ptr();
        let head_size = head_vector.len();

        let body = body_vector.as_ptr();
        let body_size = body_vector.len();
        unsafe {
            let backend = &mut self.0;
            // let mut ClingoBackend(backend) = self;
            if choice {

                clingo_backend_rule(backend, 1, head, head_size, body, body_size)
            } else {
                clingo_backend_rule(backend, 0, head, head_size, body, body_size)
            }
        }
    }
    //     pub fn clingo_backend_weight_rule(backend: *mut ClingoBackend,
    //                                       choice: u8,
    //                                       head: *const clingo_atom_t,
    //                                       head_size: size_t,
    //                                       lower_bound: clingo_weight_t,
    //                                       body: *const clingo_weighted_literal_t,
    //                                       body_size: size_t)
    //                                       -> u8;
    //     pub fn clingo_backend_minimize(backend: *mut ClingoBackend,
    //                                    priority: clingo_weight_t,
    //                                    literals: *const clingo_weighted_literal_t,
    //                                    size: size_t)
    //                                    -> u8;
    //     pub fn clingo_backend_project(backend: *mut ClingoBackend,
    //                                   atoms: *const clingo_atom_t,
    //                                   size: size_t)
    //                                   -> u8;
    //     pub fn clingo_backend_external(backend: *mut ClingoBackend,
    //                                    atom: clingo_atom_t,
    //                                    type_: clingo_external_type_t)
    //                                    -> u8;
    pub fn assume(&mut self, literals: *const clingo_literal_t, size: size_t) -> u8 {

        unsafe {
            let backend = &mut self.0;
            // let mut ClingoBackend(backend) = self;
            clingo_backend_assume(backend, literals, size)
        }
    }
    //     pub fn clingo_backend_heuristic(backend: *mut ClingoBackend,
    //                                     atom: clingo_atom_t,
    //                                     type_: clingo_heuristic_type_t,
    //                                     bias: c_int,
    //                                     priority: ::std::os::raw::c_uint,
    //                                     condition: *const clingo_literal_t,
    //                                     size: size_t)
    //                                     -> u8;
    //     pub fn clingo_backend_acyc_edge(backend: *mut ClingoBackend,
    //                                     node_u: c_int,
    //                                     node_v: c_int,
    //                                     condition: *const clingo_literal_t,
    //                                     size: size_t)
    //                                     -> u8;
    pub fn add_atom(&mut self) -> std::result::Result<clingo_atom_t, u8> {
        unsafe {
            let backend = &mut self.0;
            // let mut ClingoBackend(backend) = self;
            let mut atom = 0 as clingo_atom_t;
            let err = clingo_backend_add_atom(backend, &mut atom);
            if err == 0 {
                Err(err)
            } else {
                Ok(atom)
            }
        }
    }
}

pub struct ClingoStatistics(clingo_statistics_t);
impl ClingoStatistics {
    pub fn statistics_root(&mut self) -> std::result::Result<uint64_t, u8> {
        unsafe {
            let stats = &mut self.0;
            // let mut ClingoStatistics(stats) = self;
            let mut root_key = 0 as uint64_t;
            let err = clingo_statistics_root(stats, &mut root_key);
            if err == 0 {
                Err(err)
            } else {
                Ok(root_key)
            }
        }
    }
    pub fn statistics_type(&mut self,
                           key: uint64_t)
                           -> std::result::Result<clingo_statistics_type_t, u8> {
        unsafe {
            let stats = &mut self.0;
            // let mut ClingoStatistics(stats) = self;
            let mut stype = 0 as clingo_statistics_type_t;
            let err = clingo_statistics_type(stats, key, &mut stype);
            if err == 0 {
                Err(err)
            } else {
                Ok(stype)
            }
        }
    }
    pub fn statistics_array_size(&mut self, key: uint64_t) -> std::result::Result<size_t, u8> {
        unsafe {
            let stats = &mut self.0;
            // let mut ClingoStatistics(stats) = self;
            let mut size = 0 as size_t;
            let err = clingo_statistics_array_size(stats, key, &mut size);
            if err == 0 {
                Err(err)
            } else {
                Ok(size)
            }
        }
    }
    pub fn statistics_array_at(&mut self,
                               key: uint64_t,
                               offset: size_t)
                               -> std::result::Result<uint64_t, u8> {
        unsafe {
            let stats = &mut self.0;
            // let mut ClingoStatistics(stats) = self;
            let mut subkey = 0 as uint64_t;
            let err = clingo_statistics_array_at(stats, key, offset, &mut subkey);
            if err == 0 {
                Err(err)
            } else {
                Ok(subkey)
            }
        }
    }
    pub fn statistics_map_size(&mut self, key: uint64_t) -> std::result::Result<size_t, u8> {
        unsafe {
            let stats = &mut self.0;
            // let mut ClingoStatistics(stats) = self;
            let mut size = 0 as size_t;
            let err = clingo_statistics_map_size(stats, key, &mut size);
            if err == 0 {
                Err(err)
            } else {
                Ok(size)
            }
        }
    }
    pub fn statistics_map_subkey_name<'a>(&mut self,
                                          key: uint64_t,
                                          offset: size_t)
                                          -> std::result::Result<&'a str, u8> {
        unsafe {
            let stats = &mut self.0;
            // let mut ClingoStatistics(stats) = self;
            let mut name = std::ptr::null() as *const c_char;

            let err = clingo_statistics_map_subkey_name(stats, key, offset, &mut name);
            if err == 0 {
                Err(err)
            } else {
                Ok(CStr::from_ptr(name).to_str().unwrap())
            }
        }
    }
    pub fn statistics_map_at(&mut self,
                             key: uint64_t,
                             name: &str)
                             -> std::result::Result<uint64_t, u8> {
        unsafe {
            let stats = &mut self.0;
            // let mut ClingoStatistics(stats) = self;
            let mut subkey = 0 as uint64_t;
            let err = clingo_statistics_map_at(stats,
                                               key,
                                               CString::new(name).unwrap().as_ptr(),
                                               &mut subkey);
            if err == 0 {
                Err(err)
            } else {
                Ok(subkey)
            }
        }
    }
    pub fn statistics_value_get(&mut self, key: uint64_t) -> std::result::Result<f64, u8> {
        unsafe {
            let stats = &mut self.0;
            // let mut ClingoStatistics(stats) = self;
            let mut value = 0.0 as f64;
            let err = clingo_statistics_value_get(stats, key, &mut value);
            if err == 0 {
                Err(err)
            } else {
                Ok(value)
            }
        }
    }
}

pub struct ClingoSymbolicAtoms(clingo_symbolic_atoms_t);
impl ClingoSymbolicAtoms {
    pub fn begin(&mut self,
                 signature: *const clingo_signature_t)
                 -> std::result::Result<clingo_symbolic_atom_iterator_t, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoSymbolicAtoms(atoms) = self;
            let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_begin(atoms, signature, &mut iterator);
            if err == 0 {
                Err(err)
            } else {
                Ok(iterator)
            }
        }
    }
    pub fn end(&mut self) -> std::result::Result<clingo_symbolic_atom_iterator_t, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoSymbolicAtoms(atoms) = self;
            let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_end(atoms, &mut iterator);
            if err == 0 {
                Err(err)
            } else {
                Ok(iterator)
            }
        }
    }
    pub fn find(&mut self,
                symbol: clingo_symbol_t)
                -> std::result::Result<clingo_symbolic_atom_iterator_t, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoSymbolicAtoms(atoms) = self;
            let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_find(atoms, symbol, &mut iterator);
            if err == 0 {
                Err(err)
            } else {
                Ok(iterator)
            }
        }
    }
    pub fn iterator_is_equal_to(&mut self,
                                a: clingo_symbolic_atom_iterator_t,
                                b: clingo_symbolic_atom_iterator_t)
                                -> std::result::Result<bool, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoSymbolicAtoms(atoms) = self;
            let mut equal = 0;
            let err = clingo_symbolic_atoms_iterator_is_equal_to(atoms, a, b, &mut equal);
            if err == 0 {
                Err(err)
            } else {
                Ok(equal == 1)
            }
        }
    }
    pub fn symbol(&mut self,
                  iterator: clingo_symbolic_atom_iterator_t)
                  -> std::result::Result<clingo_symbol_t, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoSymbolicAtoms(atoms) = self;
            let mut symbol = 0 as clingo_symbol_t;
            let err = clingo_symbolic_atoms_symbol(atoms, iterator, &mut symbol);
            if err == 0 {
                Err(err)
            } else {
                Ok(symbol)
            }
        }
    }
    pub fn is_fact(&mut self,
                   iterator: clingo_symbolic_atom_iterator_t)
                   -> std::result::Result<bool, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoSymbolicAtoms(atoms) = self;
            let mut fact = 0;
            let err = clingo_symbolic_atoms_is_fact(atoms, iterator, &mut fact);
            if err == 0 {
                Err(err)
            } else {
                Ok(fact == 1)
            }
        }
    }
    pub fn is_external(&mut self,
                       iterator: clingo_symbolic_atom_iterator_t)
                       -> std::result::Result<bool, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoSymbolicAtoms(atoms) = self;
            let mut external = 0;
            let err = clingo_symbolic_atoms_is_external(atoms, iterator, &mut external);
            if err == 0 {
                Err(err)
            } else {
                Ok(external == 1)
            }
        }
    }
    pub fn literal(&mut self,
                   iterator: clingo_symbolic_atom_iterator_t)
                   -> std::result::Result<clingo_literal_t, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoSymbolicAtoms(atoms) = self;
            let mut literal = 0 as clingo_literal_t;
            let err = clingo_symbolic_atoms_literal(atoms, iterator, &mut literal);
            if err == 0 {
                Err(err)
            } else {
                Ok(literal)
            }
        }
    }
    //     pub fn clingo_symbolic_atoms_signatures_size(atoms: *mut ClingoSymbolicAtoms,
    //                                                  size: *mut size_t)
    //                                                  -> u8;
    //     pub fn clingo_symbolic_atoms_signatures(atoms: *mut ClingoSymbolicAtoms,
    //                                             signatures: *mut clingo_signature_t,
    //                                             size: size_t)
    //                                             -> u8;
    pub fn next(&mut self,
                iterator: clingo_symbolic_atom_iterator_t)
                -> std::option::Option<clingo_symbolic_atom_iterator_t> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoSymbolicAtoms(atoms) = self;
            let mut next = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_next(atoms, iterator, &mut next);
            if err == 0 {
                None
            } else {
                Some(next)
            }
        }
    }
    //     pub fn clingo_symbolic_atoms_is_valid(atoms: *mut ClingoSymbolicAtoms,
    //                                           iterator: clingo_symbolic_atom_iterator_t,
    //                                           valid: *mut u8)
    //                                           -> u8;
}

pub struct ClingoTheoryAtoms(clingo_theory_atoms_t);
impl ClingoTheoryAtoms {
    //     pub fn clingo_theory_atoms_term_type(atoms: *mut ClingoTheoryAtoms,
    //                                          term: clingo_id_t,
    //                                          type_: *mut clingo_theory_term_type_t)
    //                                          -> u8;
    //     pub fn clingo_theory_atoms_term_number(atoms: *mut ClingoTheoryAtoms,
    //                                            term: clingo_id_t,
    //                                            number: *mut c_int)
    //                                            -> u8;
    pub fn term_name<'a>(&mut self, term: clingo_id_t) -> std::result::Result<&'a str, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoTheoryAtoms(atoms) = self;
            let mut char_ptr = std::ptr::null() as *const c_char;
            let err = clingo_theory_atoms_term_name(atoms, term, &mut char_ptr);
            if err == 0 {
                Err(err)
            } else {
                let c_str = CStr::from_ptr(char_ptr);
                Ok(c_str.to_str().unwrap())
            }
        }
    }
    //     pub fn clingo_theory_atoms_term_arguments(atoms: *mut ClingoTheoryAtoms,
    //                                               term: clingo_id_t,
    //                                               arguments: *mut *const clingo_id_t,
    //                                               size: *mut size_t)
    //                                               -> u8;
    //     pub fn clingo_theory_atoms_term_to_string_size(atoms: *mut ClingoTheoryAtoms,
    //                                                    term: clingo_id_t,
    //                                                    size: *mut size_t)
    //                                                    -> u8;
    //     pub fn clingo_theory_atoms_term_to_string(atoms: *mut ClingoTheoryAtoms,
    //                                               term: clingo_id_t,
    //                                               string: *mut c_char,
    //                                               size: size_t)
    //                                               -> u8;
    //     pub fn clingo_theory_atoms_element_tuple(atoms: *mut ClingoTheoryAtoms,
    //                                              element: clingo_id_t,
    //                                              tuple: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;
    //     pub fn clingo_theory_atoms_element_condition(atoms: *mut ClingoTheoryAtoms,
    //                                                  element: clingo_id_t,
    //                                                  condition: *mut *const clingo_literal_t,
    //                                                  size: *mut size_t)
    //                                                  -> u8;
    //     pub fn clingo_theory_atoms_element_condition_id(atoms: *mut ClingoTheoryAtoms,
    //                                                     element: clingo_id_t,
    //                                                     condition: *mut clingo_literal_t)
    //                                                     -> u8;
    //     pub fn clingo_theory_atoms_element_to_string_size(atoms: *mut ClingoTheoryAtoms,
    //                                                       element: clingo_id_t,
    //                                                       size: *mut size_t)
    //                                                       -> u8;
    //     pub fn clingo_theory_atoms_element_to_string(atoms: *mut ClingoTheoryAtoms,
    //                                                  element: clingo_id_t,
    //                                                  string: *mut c_char,
    //                                                  size: size_t)
    //                                                  -> u8;
    pub fn size(&mut self) -> std::result::Result<size_t, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoTheoryAtoms(atoms) = self;
            let mut size = 0 as size_t;
            let err = clingo_theory_atoms_size(atoms, &mut size);
            if err == 0 {
                Err(err)
            } else {
                Ok(size)
            }
        }
    }
    pub fn atom_term(&mut self, atom: clingo_id_t) -> std::result::Result<clingo_id_t, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoTheoryAtoms(atoms) = self;
            let mut term = 0 as clingo_id_t;
            let err = clingo_theory_atoms_atom_term(atoms, atom, &mut term);
            if err == 0 {
                Err(err)
            } else {
                Ok(term)
            }
        }
    }
    //     pub fn clingo_theory_atoms_atom_elements(atoms: *mut ClingoTheoryAtoms,
    //                                              atom: clingo_id_t,
    //                                              elements: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;
    pub fn atom_has_guard(&mut self, atom: clingo_id_t) -> std::result::Result<bool, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoTheoryAtoms(atoms) = self;
            let mut has_guard = 0;
            let err = clingo_theory_atoms_atom_has_guard(atoms, atom, &mut has_guard);
            if err == 0 {
                Err(err)
            } else {
                if has_guard == 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }
    //     pub fn clingo_theory_atoms_atom_guard(atoms: *mut ClingoTheoryAtoms,
    //                                           atom: clingo_id_t,
    //                                           connective: *mut *const c_char,
    //                                           term: *mut clingo_id_t)
    //                                           -> u8;
    pub fn atom_literal(&mut self, atom: clingo_id_t) -> std::result::Result<clingo_literal_t, u8> {
        unsafe {
            let atoms = &mut self.0;
            // let mut ClingoTheoryAtoms(atoms) = self;
            let mut literal = 0 as clingo_literal_t;
            let err = clingo_theory_atoms_atom_literal(atoms, atom, &mut literal);
            if err == 0 {
                Err(err)
            } else {
                Ok(literal)
            }
        }
    }
    //     pub fn clingo_theory_atoms_atom_to_string_size(atoms: *mut ClingoTheoryAtoms,
    //                                                    atom: clingo_id_t,
    //                                                    size: *mut size_t)
    //                                                    -> u8;
    //     pub fn clingo_theory_atoms_atom_to_string(atoms: *mut ClingoTheoryAtoms,
    //                                               atom: clingo_id_t,
    //                                               string: *mut c_char,
    //                                               size: size_t)
    //                                               -> u8;
}

pub struct ClingoModel(clingo_model_t);
impl ClingoModel {
    pub fn model_type(&mut self) -> std::result::Result<clingo_model_type_t, u8> {
        unsafe {
            let model = &mut self.0;
            // let mut ClingoModel(model) = self;
            let mut mtype = 0 as clingo_model_type_t;
            let err = clingo_model_type(model, &mut mtype);
            if err == 0 {
                Err(err)
            } else {
                Ok(mtype)
            }
        }
    }
    pub fn number(&mut self) -> std::result::Result<uint64_t, u8> {

        unsafe {
            let model = &mut self.0;
            // let mut ClingoModel(model) = self;
            let mut number = 0;
            let err = clingo_model_number(model, &mut number);
            if err == 0 {
                Err(err)
            } else {
                Ok(number)
            }
        }
    }
    //     pub fn clingo_model_symbols_size(model: *mut ClingoModel,
    //                                      show: clingo_show_type_bitset_t,
    //                                      size: *mut size_t)
    //                                      -> u8;
    pub fn symbols(&mut self,
                   show: clingo_show_type_bitset_t)
                   -> std::result::Result<Vec<clingo_symbol_t>, u8> {
        let mut size: usize = 0;
        let size_p = &mut size as *mut usize;
        unsafe {
            let model = &mut self.0;
            //         let mut ClingoModel(model) = self;
            let err1 = clingo_model_symbols_size(model, show, size_p);
            if err1 == 0 {
                Err(err1)
            } else {
                let mut a1 = Vec::<clingo_symbol_t>::with_capacity(size);
                let slice = a1.as_mut_slice();
                let symbols = slice.as_ptr() as *mut clingo_symbol_t;
                let err2 = clingo_model_symbols(model, show, symbols, size);
                if err2 == 0 {
                    Err(err2)
                } else {
                    let res = Vec::from_raw_parts(symbols, size, size);
                    Ok(res)
                }
            }
        }
    }
    //     pub fn clingo_model_contains(model: *mut ClingoModel,
    //                                  atom: clingo_symbol_t,
    //                                  contained: *mut u8)
    //                                  -> u8;
    //     pub fn clingo_model_cost_size(model: *mut ClingoModel, size: *mut size_t) -> u8;
    //     pub fn clingo_model_cost(model: *mut ClingoModel, costs: *mut int64_t, size: size_t) -> u8;
    //     pub fn clingo_model_optimality_proven(model: *mut ClingoModel, proven: *mut u8) -> u8;
    //     pub fn clingo_model_context(model: *mut ClingoModel,
    //                                 control: *mut *mut ClingoSolveControl)
    //                                 -> u8;
}

pub struct ClingoSolveIteratively(clingo_solve_iteratively_t);
impl ClingoSolveIteratively {
    pub fn next(&mut self) -> std::option::Option<&mut ClingoModel> {
        unsafe {
            let ClingoSolveIteratively(ref mut handle) = *self;
            let mut model = std::ptr::null_mut();
            let err = clingo_solve_iteratively_next(handle, &mut model);
            if err == 0 {
                None
            } else {
                Some(&mut *(model as *mut ClingoModel))
            }
        }
    }
    //     pub fn clingo_solve_iteratively_get(handle: *mut ClingoSolveIteratively,
    //                                         result: *mut clingo_solve_result_bitset_t)
    //                                         -> u8;
    //     pub fn clingo_solve_iteratively_close(handle: *mut ClingoSolveIteratively) -> u8;
}
// impl clingo_symbol_t {
//     pub fn clingo_symbol_create_number(number: c_int, symbol: *mut clingo_symbol_t);
//     pub fn clingo_symbol_create_supremum(symbol: *mut clingo_symbol_t);
//     pub fn clingo_symbol_create_infimum(symbol: *mut clingo_symbol_t);
//     pub fn clingo_symbol_create_string(string: *const c_char, symbol: *mut clingo_symbol_t) -> u8;
//     pub fn clingo_symbol_create_id(name: *const c_char,
//                                    positive: u8,
//                                    symbol: *mut clingo_symbol_t)
//                                    -> u8;
//     pub fn clingo_symbol_create_function(name: *const c_char,
//                                          arguments: *const clingo_symbol_t,
//                                          arguments_size: size_t,
//                                          positive: u8,
//                                          symbol: *mut clingo_symbol_t)
//                                          -> u8;
//     pub fn clingo_symbol_number(symbol: clingo_symbol_t, number: *mut c_int) -> u8;
//     pub fn clingo_symbol_name(symbol: clingo_symbol_t, name: *mut *const c_char) -> u8;
//     pub fn clingo_symbol_string(symbol: clingo_symbol_t, string: *mut *const c_char) -> u8;
//     pub fn clingo_symbol_is_positive(symbol: clingo_symbol_t, positive: *mut u8) -> u8;
//     pub fn clingo_symbol_is_negative(symbol: clingo_symbol_t, negative: *mut u8) -> u8;
//     pub fn clingo_symbol_arguments(symbol: clingo_symbol_t,
//                                    arguments: *mut *const clingo_symbol_t,
//                                    arguments_size: *mut size_t)
//                                    -> u8;
//     pub fn clingo_symbol_type(symbol: clingo_symbol_t) -> clingo_symbol_type_t;
//     pub fn clingo_symbol_to_string_size(symbol: clingo_symbol_t, size: *mut size_t) -> u8;
//     pub fn to_string(&mut self) -> std::result::Result<CString, u8> {
//
//         let mut size: usize = 0;
//         let size_p = &mut size as *mut usize;
//         unsafe {
//             let err1 = clingo_symbol_to_string_size(self, size_p);
//             if err1 == 0 {
//                 Err(err1)
//             } else {
//                 let a1 = vec![1; size];
//                 let string = CString::from_vec_unchecked(a1);
//
//                 let err2 = clingo_symbol_to_string(self, string.as_ptr() as *mut c_char, size);
//                 if err2 == 0 {
//                     Err(err2)
//                 } else {
//                     Ok(string)
//                 }
//             }
//         }
//     }
//     pub fn clingo_symbol_is_equal_to(a: clingo_symbol_t, b: clingo_symbol_t) -> u8;
//     pub fn clingo_symbol_is_less_than(a: clingo_symbol_t, b: clingo_symbol_t) -> u8;
//     pub fn clingo_symbol_hash(symbol: clingo_symbol_t) -> size_t;
// }

pub struct ClingoSolveControl(clingo_solve_control_t);
impl ClingoSolveControl {
    pub fn thread_id(&mut self) -> std::option::Option<clingo_id_t> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoSolveControl(control) = self;
            let mut id = 0 as clingo_id_t;
            let err = clingo_solve_control_thread_id(control, &mut id);
            if err == 0 {
                None
            } else {
                Some(id)
            }
        }
    }
    pub fn add_clause(&mut self, clause: *const clingo_symbolic_literal_t) -> bool {

        unsafe {
            let control = &mut self.0;
            // let mut ClingoSolveControl(control) = self;
            let size = 0; //TODO: comute size of clause
            clingo_solve_control_add_clause(control, clause, size) == 1
        }
    }
}

pub struct ClingoPropagateControl(clingo_propagate_control_t);
impl ClingoPropagateControl {
    pub fn thread_id(&mut self) -> clingo_id_t {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoPropagateControl(control) = self;
            clingo_propagate_control_thread_id(control)
        }
    }
    //     pub fn clingo_propagate_control_assignment(control: *mut ClingoPropagateControl)
    //                                                -> *mut clingo_assignment_t;
    pub fn add_clause(&mut self,
                      clause: &[clingo_literal_t],
                      type_: clingo_clause_type_t)
                      -> std::option::Option<u8> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoPropagateControl(control) = self;
            let size = 0; //TODO: comute size of claus
            let mut result = 0;
            let err = clingo_propagate_control_add_clause(control,
                                                          clause.as_ptr(),
                                                          size,
                                                          type_,
                                                          &mut result);
            if err == 0 {
                None
            } else {
                Some(result)
            }
        }
    }
    pub fn propagate(&mut self) -> std::option::Option<u8> {
        unsafe {
            let control = &mut self.0;
            // let mut ClingoPropagateControl(control) = self;
            let mut result = 0;
            let err = clingo_propagate_control_propagate(control, &mut result);
            if err == 0 {
                None
            } else {
                Some(result)
            }
        }
    }
}

pub struct ClingoPropagateInit(clingo_propagate_init_t);
impl ClingoPropagateInit {
    //     pub fn clingo_propagate_init_solver_literal(init: *mut ClingoPropagateInit,
    //                                                 aspif_literal: clingo_literal_t,
    //                                                 solver_literal: *mut clingo_literal_t)
    //                                                 -> u8;
    //     pub fn clingo_propagate_init_add_watch(init: *mut ClingoPropagateInit,
    //                                            solver_literal: clingo_literal_t)
    //                                            -> u8;
    //     pub fn clingo_propagate_init_symbolic_atoms(init: *mut ClingoPropagateInit,
    //                                                 atoms: *mut *mut ClingoSymbolicAtoms)
    //                                                 -> u8;
    //     pub fn c_lingo_propagate_init_theory_atoms(init: *mut ClingoPropagateInit,
    //                                               atoms: *mut *mut ClingoTheoryAtoms)
    //                                               -> u8;
    pub fn number_of_threads(&mut self) -> size_t {
        unsafe {
            let mut init = &mut self.0;
            let ret = clingo_propagate_init_number_of_threads(init);
            (ret as size_t)
        }
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
