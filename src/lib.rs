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
pub fn safe_clingo_model_symbols(model: *mut clingo_model_t,
                                 show: clingo_show_type_bitset_t)
                                 -> std::result::Result<Vec<clingo_symbol_t>, u8> {
    let mut size: usize = 0;
    let size_p = &mut size as *mut usize;
    unsafe {
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

            let err2 = clingo_symbol_to_string(sym, string.as_ptr() as *mut c_char, size);
            if err2 == 0 {
                Err(err2)
            } else {
                Ok(string)
            }
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
pub fn safe_clingo_model_type(model: *mut clingo_model_t)
                              -> std::result::Result<clingo_model_type_t, u8> {
    unsafe {

        let mut mtype = 0 as clingo_model_type_t;
        let err = clingo_model_type(model, &mut mtype);
        if err == 0 {
            Err(err)
        } else {
            Ok(mtype)
        }
    }
}
pub fn safe_clingo_model_number(model: *mut clingo_model_t) -> std::result::Result<uint64_t, u8> {

    unsafe {
        let mut number = 0;
        let err = clingo_model_number(model, &mut number);
        if err == 0 {
            Err(err)
        } else {
            Ok(number)
        }
    }
}
pub fn safe_clingo_solve_iteratively_next(handle: *mut clingo_solve_iteratively_t)
                                          -> std::result::Result<*mut clingo_model_t, u8> {
    unsafe {

        let mut model = std::ptr::null_mut() as *mut clingo_model_t;
        let err = clingo_solve_iteratively_next(handle, &mut model);
        if err == 0 {
            Err(err)
        } else {
            Ok(model)
        }
    }
}
pub fn clingo_print_map_name(name: *const c_char) {
    unsafe {
        let string = CStr::from_ptr(name);
        print!("{}", string.to_str().unwrap());
    }
}


pub struct SafeClingoControl {
    control: *mut clingo_control_t,
}
impl Drop for SafeClingoControl {
    fn drop(&mut self) {
        unsafe { clingo_control_free(self.control) }

    }
}
impl SafeClingoControl {
    pub fn new(arguments: std::env::Args,
               logger: clingo_logger_t,
               logger_data: *mut c_void,
               message_limit: ::std::os::raw::c_uint)
               -> std::result::Result<SafeClingoControl, u8> {
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
                Ok(SafeClingoControl { control: ctl })
            }
        }
    }
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
            clingo_control_add(self.control,
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
            clingo_control_ground(self.control,
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
        let assumptions_size = assumptions.len();
        let mut solve_result = 0 as clingo_solve_result_bitset_t;
        unsafe {
            let err = clingo_control_solve(self.control,
                                           model_callback,
                                           model_callback_data,
                                           assumptions.as_ptr(),
                                           assumptions_size,
                                           &mut solve_result);
            if err == 0 {
                Err(err)
            } else {
                Ok(solve_result)
            }
        }
    }
    pub fn symbolic_atoms(&self) -> std::result::Result<*mut clingo_symbolic_atoms_t, u8> {
        unsafe {
            let mut ato = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
            let err = clingo_control_symbolic_atoms(self.control, &mut ato);
            if err == 0 {
                Err(err)
            } else {
                Ok(ato)
            }
        }
    }
    pub fn solve_iteratively(&mut self,
                             assumptions: *const clingo_symbolic_literal_t,
                             assumptions_size: size_t)
                             -> std::result::Result<*mut clingo_solve_iteratively_t, u8> {
        unsafe {
            let mut handle = std::ptr::null_mut() as *mut clingo_solve_iteratively_t;
            let err = clingo_control_solve_iteratively(self.control,
                                                       assumptions,
                                                       assumptions_size,
                                                       &mut handle);
            if err == 0 {
                Err(err)
            } else {
                Ok(handle)
            }
        }
    }
    pub fn configuration(&mut self) -> std::result::Result<&mut clingo_configuration_t, u8> {
        unsafe {
            let mut conf = std::ptr::null_mut() as *mut clingo_configuration_t;
            let err = clingo_control_configuration(self.control, &mut conf);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *conf)
            }
        }
    }
    pub fn statistics(&self) -> std::result::Result<&mut clingo_statistics_t, u8> {
        unsafe {
            let mut stat = std::ptr::null_mut() as *mut clingo_statistics_t;
            let err = clingo_control_statistics(self.control, &mut stat);
            if err == 0 {
                Err(err)
            } else {
                Ok(&mut *stat)

            }
        }
    }
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
    pub fn configuration_value_set(&mut self, key: clingo_id_t, value: &str) -> u8 {
        unsafe {
            clingo_configuration_value_set(self,
                                           key,
                                           CString::new(value).unwrap().as_ptr())
        }
    }
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
    pub fn statistics_map_subkey_name(&mut self,
                                      key: uint64_t,
                                      offset: size_t)
                                      -> std::result::Result<*const c_char, u8> {
        unsafe {
            let mut name = std::ptr::null() as *const c_char;

            let err = clingo_statistics_map_subkey_name(self, key, offset, &mut name);
            if err == 0 {
                Err(err)
            } else {
                //             Ok(CStr::from_ptr(name))
                Ok(name)
            }
        }
    }
    pub fn statistics_map_at(&mut self,
                             key: uint64_t,
                             name: *const c_char)
                             -> std::result::Result<uint64_t, u8> {
        unsafe {
            let mut subkey = 0 as uint64_t;
            let err = clingo_statistics_map_at(self, key, name, &mut subkey);
            if err == 0 {
                Err(err)
            } else {
                Ok(subkey)
            }
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
