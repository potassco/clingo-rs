#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate clingo_sys;

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

pub struct safe_clingo_part<'a> {
    pub name: CString,
    pub params: &'a [clingo_symbol_t],
}

fn from_clingo_part(spart: &safe_clingo_part) -> clingo_part {
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
            for i in 0..size {
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
                              -> std::result::Result<&'a mut clingo_control_t, u8> {
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
            Ok(&mut *ctl)
        }
    }
}
impl Drop for clingo_control_t {
    fn drop(&mut self) {
        unsafe { clingo_control_free(self) }

    }
}
impl clingo_control_t {
    //     pub fn clingo_control_load(control: *mut clingo_control_t, file: *const c_char) -> u8;
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
            clingo_control_add(self,
                               mname.as_ptr(),
                               c_args.as_ptr(),
                               parameters_size,
                               mprogram.as_ptr())
        }
    }
    pub fn ground(&mut self,
                  sparts: Vec<safe_clingo_part>,
                  ground_callback: clingo_ground_callback_t,
                  ground_callback_data: *mut c_void)
                  -> u8 {

        let parts = sparts.iter().map(|arg| from_clingo_part(arg)).collect::<Vec<clingo_part>>();
        let parts_size = parts.len();

        unsafe {
            clingo_control_ground(self,
                                  parts.as_ptr(),
                                  parts_size,
                                  ground_callback,
                                  ground_callback_data)
        }
    }
    pub fn solve(&mut self,
                 model_callback: clingo_model_callback_t,
                 model_callback_data: *mut c_void,
                 assumptions: Vec<clingo_symbolic_literal_t>)
                 -> std::result::Result<clingo_solve_result_bitset_t, u8> {
        let mut solve_result = 0 as clingo_solve_result_bitset_t;
        unsafe {
            let err = clingo_control_solve(self,
                                           model_callback,
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
    pub fn solve_iteratively(&mut self,
                             assumptions: Vec<clingo_symbolic_literal_t>)
                             -> std::result::Result<&mut clingo_solve_iteratively_t, u8> {
        unsafe {
            let mut handle = std::ptr::null_mut() as *mut clingo_solve_iteratively_t;
            let err = clingo_control_solve_iteratively(self,
                                                       assumptions.as_ptr(),
                                                       assumptions.len(),
                                                       &mut handle);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *handle)
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
            let mut handle = std::ptr::null_mut() as *mut clingo_solve_async_t;
            let err = clingo_control_solve_async(self,
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

    //     pub fn clingo_control_cleanup(control: *mut clingo_control_t) -> u8;
    //     pub fn clingo_control_assign_external(control: *mut clingo_control_t,
    //                                           atom: clingo_symbol_t,
    //                                           value: clingo_truth_value_t)
    //                                           -> u8;
    //     pub fn clingo_control_release_external(control: *mut clingo_control_t,
    //                                            atom: clingo_symbol_t)
    //                                            -> u8;
    //     pub fn clingo_control_register_propagator(control: *mut clingo_control_t,
    //                                               propagator: clingo_propagator_t,
    //                                               data: *mut c_void,
    //                                               sequential: u8)
    //                                               -> u8;
    pub fn statistics(&mut self) -> std::result::Result<&mut clingo_statistics_t, u8> {
        unsafe {
            let mut stat = std::ptr::null_mut() as *mut clingo_statistics_t;
            let err = clingo_control_statistics(self, &mut stat);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *stat)

            }
        }
    }
    //     pub fn clingo_control_interrupt(control: *mut clingo_control_t);
    pub fn configuration(&mut self) -> std::result::Result<&mut clingo_configuration_t, u8> {
        unsafe {
            let mut conf = std::ptr::null_mut() as *mut clingo_configuration_t;
            let err = clingo_control_configuration(self, &mut conf);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *conf)
            }
        }
    }
    //     pub fn clingo_control_use_enumeration_assumption(control: *mut clingo_control_t,
    //                                                      enable: u8)
    //                                                     -> u8;
    //     pub fn clingo_control_get_const(control: *mut clingo_control_t,
    //                                     name: *const c_char,
    //                                     symbol: *mut clingo_symbol_t)
    //                                    -> u8;
    //     pub fn clingo_control_has_const(control: *mut clingo_control_t,
    //                                     name: *const c_char,
    //                                     exists: *mut u8)
    //                                    -> u8;
    pub fn symbolic_atoms(&mut self) -> std::result::Result<&mut clingo_symbolic_atoms_t, u8> {
        unsafe {
            let mut atoms = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
            let err = clingo_control_symbolic_atoms(self, &mut atoms);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *atoms)
            }
        }
    }
    pub fn theory_atoms(&mut self) -> std::result::Result<&mut clingo_theory_atoms_t, u8> {
        unsafe {
            let mut atoms = std::ptr::null_mut() as *mut clingo_theory_atoms_t;
            let err = clingo_control_theory_atoms(self, &mut atoms);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *atoms)
            }
        }
    }
    pub fn backend(&mut self) -> std::result::Result<&mut clingo_backend_t, u8> {
        unsafe {
            let mut backend = std::ptr::null_mut() as *mut clingo_backend_t;
            let err = clingo_control_backend(self, &mut backend);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *backend)
            }
        }
    }
    //     pub fn clingo_control_program_builder(control: *mut clingo_control_t,
    //                                           builder: *mut *mut clingo_program_builder_t)
    //                                          -> u8;
}

impl clingo_configuration_t {
    pub fn configuration_root(&mut self) -> std::result::Result<clingo_id_t, u8> {
        unsafe {
            let mut root_key = 0 as clingo_id_t;
            let err = clingo_configuration_root(self, &mut root_key);
            if err == 0 {
                Err(err)
            } else {
                Ok(root_key)
            }
        }
    }
    //     pub fn clingo_configuration_type(configuration: *mut clingo_configuration_t,
    //                                      key: clingo_id_t,
    //                                      type_: *mut clingo_configuration_type_bitset_t)
    //                                      -> u8;
    //     pub fn clingo_configuration_description(configuration: *mut clingo_configuration_t,
    //                                             key: clingo_id_t,
    //                                             description: *mut *const c_char)
    //                                             -> u8;
    //     pub fn clingo_configuration_array_size(configuration: *mut clingo_configuration_t,
    //                                            key: clingo_id_t,
    //                                            size: *mut size_t)
    //                                            -> u8;
    pub fn configuration_array_at(&mut self,
                                  key: clingo_id_t,
                                  offset: size_t)
                                  -> std::result::Result<clingo_id_t, u8> {
        unsafe {
            let mut nkey = 0 as clingo_id_t;
            let err = clingo_configuration_array_at(self, key, offset, &mut nkey);
            if err == 0 {
                Err(err)
            } else {
                Ok(nkey)
            }
        }
    }
    //     pub fn clingo_configuration_map_size(configuration: *mut clingo_configuration_t,
    //                                          key: clingo_id_t,
    //                                          size: *mut size_t)
    //                                          -> u8;
    //     pub fn clingo_configuration_map_subkey_name(configuration: *mut clingo_configuration_t,
    //                                                 key: clingo_id_t,
    //                                                 offset: size_t,
    //                                                 name: *mut *const c_char)
    //                                                 -> u8;
    pub fn configuration_map_at(&mut self,
                                key: clingo_id_t,
                                name: &str)
                                -> std::result::Result<clingo_id_t, u8> {
        unsafe {
            let mut nkey = 0 as clingo_id_t;
            let err = clingo_configuration_map_at(self,
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
    //     pub fn clingo_configuration_value_is_assigned(configuration: *mut clingo_configuration_t,
    //                                                   key: clingo_id_t,
    //                                                   assigned: *mut u8)
    //                                                   -> u8;
    //     pub fn clingo_configuration_value_get_size(configuration: *mut clingo_configuration_t,
    //                                                key: clingo_id_t,
    //                                                size: *mut size_t)
    //                                                -> u8;
    //     pub fn clingo_configuration_value_get(configuration: *mut clingo_configuration_t,
    //                                           key: clingo_id_t,
    //                                           value: *mut c_char,
    //                                           size: size_t)
    //                                           -> u8;
    pub fn configuration_value_set(&mut self, key: clingo_id_t, value: &str) -> u8 {
        unsafe { clingo_configuration_value_set(self, key, CString::new(value).unwrap().as_ptr()) }
    }
}

impl clingo_backend_t {
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
            if choice {

                clingo_backend_rule(self, 1, head, head_size, body, body_size)
            } else {
                clingo_backend_rule(self, 0, head, head_size, body, body_size)
            }
        }
    }
    //     pub fn clingo_backend_weight_rule(backend: *mut clingo_backend_t,
    //                                       choice: u8,
    //                                       head: *const clingo_atom_t,
    //                                       head_size: size_t,
    //                                       lower_bound: clingo_weight_t,
    //                                       body: *const clingo_weighted_literal_t,
    //                                       body_size: size_t)
    //                                       -> u8;
    //     pub fn clingo_backend_minimize(backend: *mut clingo_backend_t,
    //                                    priority: clingo_weight_t,
    //                                    literals: *const clingo_weighted_literal_t,
    //                                    size: size_t)
    //                                    -> u8;
    //     pub fn clingo_backend_project(backend: *mut clingo_backend_t,
    //                                   atoms: *const clingo_atom_t,
    //                                   size: size_t)
    //                                   -> u8;
    //     pub fn clingo_backend_external(backend: *mut clingo_backend_t,
    //                                    atom: clingo_atom_t,
    //                                    type_: clingo_external_type_t)
    //                                    -> u8;
    pub fn assume(&mut self, literals: *const clingo_literal_t, size: size_t) -> u8 {

        unsafe { clingo_backend_assume(self, literals, size) }
    }
    //     pub fn clingo_backend_heuristic(backend: *mut clingo_backend_t,
    //                                     atom: clingo_atom_t,
    //                                     type_: clingo_heuristic_type_t,
    //                                     bias: c_int,
    //                                     priority: ::std::os::raw::c_uint,
    //                                     condition: *const clingo_literal_t,
    //                                     size: size_t)
    //                                     -> u8;
    //     pub fn clingo_backend_acyc_edge(backend: *mut clingo_backend_t,
    //                                     node_u: c_int,
    //                                     node_v: c_int,
    //                                     condition: *const clingo_literal_t,
    //                                     size: size_t)
    //                                     -> u8;
    pub fn add_atom(&mut self) -> std::result::Result<clingo_atom_t, u8> {
        unsafe {
            let mut atom = 0 as clingo_atom_t;
            let err = clingo_backend_add_atom(self, &mut atom);
            if err == 0 {
                Err(err)
            } else {
                Ok(atom)
            }
        }
    }
}

impl clingo_statistics_t {
    pub fn statistics_root(&mut self) -> std::result::Result<uint64_t, u8> {
        unsafe {
            let mut root_key = 0 as uint64_t;
            let err = clingo_statistics_root(self, &mut root_key);
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

            let mut stype = 0 as clingo_statistics_type_t;
            let err = clingo_statistics_type(self, key, &mut stype);
            if err == 0 {
                Err(err)
            } else {
                Ok(stype)
            }
        }
    }
    pub fn statistics_array_size(&mut self, key: uint64_t) -> std::result::Result<size_t, u8> {
        unsafe {
            let mut size = 0 as size_t;
            let err = clingo_statistics_array_size(self, key, &mut size);
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
            let mut subkey = 0 as uint64_t;
            let err = clingo_statistics_array_at(self, key, offset, &mut subkey);
            if err == 0 {
                Err(err)
            } else {
                Ok(subkey)
            }
        }
    }
    pub fn statistics_map_size(&mut self, key: uint64_t) -> std::result::Result<size_t, u8> {
        unsafe {
            let mut size = 0 as size_t;
            let err = clingo_statistics_map_size(self, key, &mut size);
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
            let mut name = std::ptr::null() as *const c_char;

            let err = clingo_statistics_map_subkey_name(self, key, offset, &mut name);
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
            let mut subkey = 0 as uint64_t;
            let err = clingo_statistics_map_at(self,
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
            let mut value = 0.0 as f64;
            let err = clingo_statistics_value_get(self, key, &mut value);
            if err == 0 {
                Err(err)
            } else {
                Ok(value)
            }
        }
    }
}
impl clingo_symbolic_atoms_t {
    pub fn begin(&mut self,
                 signature: *const clingo_signature_t)
                 -> std::result::Result<clingo_symbolic_atom_iterator_t, u8> {
        unsafe {
            let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_begin(self, signature, &mut iterator);
            if err == 0 {
                Err(err)
            } else {
                Ok(iterator)
            }
        }
    }
    pub fn end(&mut self) -> std::result::Result<clingo_symbolic_atom_iterator_t, u8> {
        unsafe {
            let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_end(self, &mut iterator);
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
            let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_find(self, symbol, &mut iterator);
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
            let mut equal = 0;
            let err = clingo_symbolic_atoms_iterator_is_equal_to(self, a, b, &mut equal);
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
            let mut symbol = 0 as clingo_symbol_t;
            let err = clingo_symbolic_atoms_symbol(self, iterator, &mut symbol);
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
            let mut fact = 0;
            let err = clingo_symbolic_atoms_is_fact(self, iterator, &mut fact);
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
            let mut external = 0;
            let err = clingo_symbolic_atoms_is_external(self, iterator, &mut external);
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
            let mut literal = 0 as clingo_literal_t;
            let err = clingo_symbolic_atoms_literal(self, iterator, &mut literal);
            if err == 0 {
                Err(err)
            } else {
                Ok(literal)
            }
        }
    }
    //     pub fn clingo_symbolic_atoms_signatures_size(atoms: *mut clingo_symbolic_atoms_t,
    //                                                  size: *mut size_t)
    //                                                  -> u8;
    //     pub fn clingo_symbolic_atoms_signatures(atoms: *mut clingo_symbolic_atoms_t,
    //                                             signatures: *mut clingo_signature_t,
    //                                             size: size_t)
    //                                             -> u8;
    pub fn next(&mut self,
                iterator: clingo_symbolic_atom_iterator_t)
                -> std::option::Option<clingo_symbolic_atom_iterator_t> {
        unsafe {
            let mut next = 0 as clingo_symbolic_atom_iterator_t;
            let err = clingo_symbolic_atoms_next(self, iterator, &mut next);
            if err == 0 {
                None
            } else {
                Some(next)
            }
        }
    }
    //     pub fn clingo_symbolic_atoms_is_valid(atoms: *mut clingo_symbolic_atoms_t,
    //                                           iterator: clingo_symbolic_atom_iterator_t,
    //                                           valid: *mut u8)
    //                                           -> u8;
}
impl clingo_theory_atoms_t {
    //     pub fn clingo_theory_atoms_term_type(atoms: *mut clingo_theory_atoms_t,
    //                                          term: clingo_id_t,
    //                                          type_: *mut clingo_theory_term_type_t)
    //                                          -> u8;
    //     pub fn clingo_theory_atoms_term_number(atoms: *mut clingo_theory_atoms_t,
    //                                            term: clingo_id_t,
    //                                            number: *mut c_int)
    //                                            -> u8;
    pub fn term_name<'a>(&mut self, term: clingo_id_t) -> std::result::Result<&'a str, u8> {
        unsafe {
            let mut char_ptr = std::ptr::null() as *const c_char;
            let err = clingo_theory_atoms_term_name(self, term, &mut char_ptr);
            if err == 0 {
                Err(err)
            } else {
                let c_str = CStr::from_ptr(char_ptr);
                Ok(c_str.to_str().unwrap())
            }
        }
    }
    //     pub fn clingo_theory_atoms_term_arguments(atoms: *mut clingo_theory_atoms_t,
    //                                               term: clingo_id_t,
    //                                               arguments: *mut *const clingo_id_t,
    //                                               size: *mut size_t)
    //                                               -> u8;
    //     pub fn clingo_theory_atoms_term_to_string_size(atoms: *mut clingo_theory_atoms_t,
    //                                                    term: clingo_id_t,
    //                                                    size: *mut size_t)
    //                                                    -> u8;
    //     pub fn clingo_theory_atoms_term_to_string(atoms: *mut clingo_theory_atoms_t,
    //                                               term: clingo_id_t,
    //                                               string: *mut c_char,
    //                                               size: size_t)
    //                                               -> u8;
    //     pub fn clingo_theory_atoms_element_tuple(atoms: *mut clingo_theory_atoms_t,
    //                                              element: clingo_id_t,
    //                                              tuple: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;
    //     pub fn clingo_theory_atoms_element_condition(atoms: *mut clingo_theory_atoms_t,
    //                                                  element: clingo_id_t,
    //                                                  condition: *mut *const clingo_literal_t,
    //                                                  size: *mut size_t)
    //                                                  -> u8;
    //     pub fn clingo_theory_atoms_element_condition_id(atoms: *mut clingo_theory_atoms_t,
    //                                                     element: clingo_id_t,
    //                                                     condition: *mut clingo_literal_t)
    //                                                     -> u8;
    //     pub fn clingo_theory_atoms_element_to_string_size(atoms: *mut clingo_theory_atoms_t,
    //                                                       element: clingo_id_t,
    //                                                       size: *mut size_t)
    //                                                       -> u8;
    //     pub fn clingo_theory_atoms_element_to_string(atoms: *mut clingo_theory_atoms_t,
    //                                                  element: clingo_id_t,
    //                                                  string: *mut c_char,
    //                                                  size: size_t)
    //                                                  -> u8;
    pub fn size(&mut self) -> std::result::Result<size_t, u8> {
        unsafe {
            let mut size = 0 as size_t;
            let err = clingo_theory_atoms_size(self, &mut size);
            if err == 0 {
                Err(err)
            } else {
                Ok(size)
            }
        }
    }
    pub fn atom_term(&mut self, atom: clingo_id_t) -> std::result::Result<clingo_id_t, u8> {
        unsafe {
            let mut term = 0 as clingo_id_t;
            let err = clingo_theory_atoms_atom_term(self, atom, &mut term);
            if err == 0 {
                Err(err)
            } else {
                Ok(term)
            }
        }
    }
    //     pub fn clingo_theory_atoms_atom_elements(atoms: *mut clingo_theory_atoms_t,
    //                                              atom: clingo_id_t,
    //                                              elements: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;
    pub fn atom_has_guard(&mut self, atom: clingo_id_t) -> std::result::Result<bool, u8> {
        unsafe {
            let mut has_guard = 0;
            let err = clingo_theory_atoms_atom_has_guard(self, atom, &mut has_guard);
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
    //     pub fn clingo_theory_atoms_atom_guard(atoms: *mut clingo_theory_atoms_t,
    //                                           atom: clingo_id_t,
    //                                           connective: *mut *const c_char,
    //                                           term: *mut clingo_id_t)
    //                                           -> u8;
    pub fn atom_literal(&mut self, atom: clingo_id_t) -> std::result::Result<clingo_literal_t, u8> {
        unsafe {
            let mut literal = 0 as clingo_literal_t;
            let err = clingo_theory_atoms_atom_literal(self, atom, &mut literal);
            if err == 0 {
                Err(err)
            } else {
                Ok(literal)
            }
        }
    }
    //     pub fn clingo_theory_atoms_atom_to_string_size(atoms: *mut clingo_theory_atoms_t,
    //                                                    atom: clingo_id_t,
    //                                                    size: *mut size_t)
    //                                                    -> u8;
    //     pub fn clingo_theory_atoms_atom_to_string(atoms: *mut clingo_theory_atoms_t,
    //                                               atom: clingo_id_t,
    //                                               string: *mut c_char,
    //                                               size: size_t)
    //                                               -> u8;
}
impl clingo_model_t {
    pub fn model_type(&mut self) -> std::result::Result<clingo_model_type_t, u8> {
        unsafe {

            let mut mtype = 0 as clingo_model_type_t;
            let err = clingo_model_type(self, &mut mtype);
            if err == 0 {
                Err(err)
            } else {
                Ok(mtype)
            }
        }
    }
    pub fn number(&mut self) -> std::result::Result<uint64_t, u8> {

        unsafe {
            let mut number = 0;
            let err = clingo_model_number(self, &mut number);
            if err == 0 {
                Err(err)
            } else {
                Ok(number)
            }
        }
    }
    //     pub fn clingo_model_symbols_size(model: *mut clingo_model_t,
    //                                      show: clingo_show_type_bitset_t,
    //                                      size: *mut size_t)
    //                                      -> u8;
    pub fn symbols(&mut self,
                   show: clingo_show_type_bitset_t)
                   -> std::result::Result<Vec<clingo_symbol_t>, u8> {
        let mut size: usize = 0;
        let size_p = &mut size as *mut usize;
        unsafe {
            let err1 = clingo_model_symbols_size(self, show, size_p);
            if err1 == 0 {
                Err(err1)
            } else {
                let mut a1 = Vec::<clingo_symbol_t>::with_capacity(size);
                let slice = a1.as_mut_slice();
                let symbols = slice.as_ptr() as *mut clingo_symbol_t;
                let err2 = clingo_model_symbols(self, show, symbols, size);
                if err2 == 0 {
                    Err(err2)
                } else {
                    let res = Vec::from_raw_parts(symbols, size, size);
                    Ok(res)
                }
            }
        }
    }
    //     pub fn clingo_model_contains(model: *mut clingo_model_t,
    //                                  atom: clingo_symbol_t,
    //                                  contained: *mut u8)
    //                                  -> u8;
    //     pub fn clingo_model_cost_size(model: *mut clingo_model_t, size: *mut size_t) -> u8;
    //     pub fn clingo_model_cost(model: *mut clingo_model_t, costs: *mut int64_t, size: size_t) -> u8;
    //     pub fn clingo_model_optimality_proven(model: *mut clingo_model_t, proven: *mut u8) -> u8;
    //     pub fn clingo_model_context(model: *mut clingo_model_t,
    //                                 control: *mut *mut clingo_solve_control_t)
    //                                 -> u8;
}
impl clingo_solve_iteratively_t {
    pub fn next(&mut self) -> std::option::Option<&mut clingo_model_t> {
        unsafe {

            let mut model = std::ptr::null_mut() as *mut clingo_model_t;
            let err = clingo_solve_iteratively_next(self, &mut model);
            if err == 0 {
                None
            } else {
                Some(&mut *model)
            }
        }
    }
    //     pub fn clingo_solve_iteratively_get(handle: *mut clingo_solve_iteratively_t,
    //                                         result: *mut clingo_solve_result_bitset_t)
    //                                         -> u8;
    //     pub fn clingo_solve_iteratively_close(handle: *mut clingo_solve_iteratively_t) -> u8;
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
impl clingo_solve_control_t {
    pub fn thread_id(&mut self) -> std::option::Option<clingo_id_t> {
        unsafe {
            let mut id = 0 as clingo_id_t;
            let err = clingo_solve_control_thread_id(self, &mut id);
            if err == 0 {
                None
            } else {
                Some(id)
            }
        }
    }
    pub fn add_clause(&mut self, clause: *const clingo_symbolic_literal_t) -> bool {

        unsafe {
            let size = 0; //TODO: comute size of clause
            clingo_solve_control_add_clause(self, clause, size) == 1
        }
    }
}
impl clingo_propagate_control_t {
    pub fn thread_id(&mut self) -> clingo_id_t {
        unsafe { clingo_propagate_control_thread_id(self) }
    }
    //     pub fn clingo_propagate_control_assignment(control: *mut clingo_propagate_control_t)
    //                                                -> *mut clingo_assignment_t;
    pub fn add_clause(&mut self,
                      clause: &[clingo_literal_t],
                      type_: clingo_clause_type_t)
                      -> std::option::Option<u8> {
        unsafe {
            let size = 0; //TODO: comute size of claus
            let mut result = 0;
            let err = clingo_propagate_control_add_clause(self,
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
            let mut result = 0;
            let err = clingo_propagate_control_propagate(self, &mut result);
            if err == 0 {
                None
            } else {
                Some(result)
            }
        }
    }
}

impl clingo_propagate_init_t {
    //     pub fn clingo_propagate_init_solver_literal(init: *mut clingo_propagate_init_t,
    //                                                 aspif_literal: clingo_literal_t,
    //                                                 solver_literal: *mut clingo_literal_t)
    //                                                 -> u8;
    //     pub fn clingo_propagate_init_add_watch(init: *mut clingo_propagate_init_t,
    //                                            solver_literal: clingo_literal_t)
    //                                            -> u8;
    //     pub fn clingo_propagate_init_symbolic_atoms(init: *mut clingo_propagate_init_t,
    //                                                 atoms: *mut *mut clingo_symbolic_atoms_t)
    //                                                 -> u8;
    //     pub fn clingo_propagate_init_theory_atoms(init: *mut clingo_propagate_init_t,
    //                                               atoms: *mut *mut clingo_theory_atoms_t)
    //                                               -> u8;
    pub fn number_of_threads(&mut self) -> size_t {
        unsafe {
            let ret = clingo_propagate_init_number_of_threads(self);
            (ret as size_t)
        }
    }
}


// pub type us_clingo_model_callback_t = unsafe extern "C" fn(model: *mut clingo_model_t,
//                                                data: *mut c_void,
//                                                goon: *mut u8)
//                                                  -> u8;
//
// pub type sa_clingo_model_callback_t = unsafe extern "C" fn(model: &mut clingo_model_t,
//                                                data: *mut c_void,
//                                                goon: &mut bool)
//                                                  -> u8;

// pub fn tr_solve_call_back ( safe_fn: sa_clingo_model_callback_t) -> us_clingo_model_callback_t {
//     let retfun = unsafe extern "C" fn(model: *mut clingo_model_t, data: *mut c_void, goon: *mut u8) -> u8 {
//         return 1;
//     };
//     return retfun;
// }


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
