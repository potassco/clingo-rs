
extern crate libc;
extern crate clingo_sys;

use std::ffi::CStr;
use std::ffi::CString;
use libc::c_int;
use libc::c_char;
use clingo_sys::*;
pub use clingo_sys::clingo_show_type::*;
pub use clingo_sys::clingo_solve_mode::*;
pub use clingo_sys::clingo_solve_event_type::*;
pub use clingo_sys::clingo_statistics_type::*;
pub use clingo_sys::clingo_clause_type::*;
pub use clingo_sys::clingo_truth_value::*;
pub use clingo_sys::clingo_ast_sign::*;
pub use clingo_sys::clingo_ast_term_type::*;
pub use clingo_sys::clingo_ast_literal_type::*;
pub use clingo_sys::clingo_ast_body_literal_type::*;
pub use clingo_sys::clingo_ast_statement_type::*;

pub use clingo_sys::{clingo_literal_t, clingo_ast_statement_t, clingo_ast_term_type_t,
                     clingo_solve_event_type_t, clingo_show_type_bitset_t,
                     clingo_solve_mode_bitset_t, clingo_error, clingo_solve_result_bitset_t,
                     clingo_propagate_init_t, clingo_propagate_control_t, clingo_logger_t};

pub type ClingoAstCallback = clingo_ast_callback_t;
pub type ClingoSolveEventCallback = clingo_solve_event_callback_t;
pub type ClingoError = clingo_error_t;


#[derive(Debug, Copy, Clone)]
pub struct ClingoLiteral(clingo_literal_t);
impl PartialEq for ClingoLiteral {
    fn eq(&self, other: &ClingoLiteral) -> bool {
        self.0.eq(&other.0)
    }
}
impl Eq for ClingoLiteral {}
impl ClingoLiteral {
    pub fn negate(&self) -> ClingoLiteral {
        ClingoLiteral(-(self.0))
    }
    pub fn UNSAFE_from(ClingoAtom(atom): ClingoAtom) -> ClingoLiteral {
        ClingoLiteral(atom as clingo_literal_t)
    }
    pub fn get_integer(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ClingoAtom(clingo_atom_t);

#[derive(Debug, Copy, Clone)]
pub struct ClingoId(clingo_id_t);
impl ClingoId {
    pub fn get_integer(&self) -> u32 {
        self.0
    }
}

pub struct ClingoLocation {
    begin_line: usize,
    end_line: usize,
    begin_column: usize,
    end_column: usize,
    begin_file: CString,
    end_file: CString,
}
impl Drop for ClingoLocation {
    fn drop(&mut self) {
        // println!("droped ClingoLocation!");
    }
}
impl ClingoLocation {
    pub fn new(
        begin_line: usize,
        end_line: usize,
        begin_column: usize,
        end_column: usize,
        begin_file: &str,
        end_file: &str,
    ) -> ClingoLocation {
        ClingoLocation {
            begin_line: begin_line,
            end_line: end_line,
            begin_column: begin_column,
            end_column: end_column,
            begin_file: CString::new(begin_file).unwrap(),
            end_file: CString::new(end_file).unwrap(),
        }
    }
    fn from(location: clingo_location) -> ClingoLocation {
        ClingoLocation {
            begin_line: location.begin_line,
            end_line: location.end_line,
            begin_column: location.begin_column,
            end_column: location.end_column,
            begin_file: unsafe { CStr::from_ptr(location.begin_file) }.to_owned(),
            end_file: unsafe { CStr::from_ptr(location.end_file) }.to_owned(),
        }
    }
    fn clingo_location(&self) -> clingo_location {
        clingo_location {
            begin_line: self.begin_line,
            end_line: self.end_line,
            begin_column: self.begin_column,
            end_column: self.end_column,
            begin_file: self.begin_file.as_ptr(),
            end_file: self.end_file.as_ptr(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ClingoSymbol(clingo_symbol_t);
impl PartialEq for ClingoSymbol {
    fn eq(&self, other: &ClingoSymbol) -> bool {
        unsafe { clingo_symbol_is_equal_to(self.0, other.0) }
    }
}
impl Eq for ClingoSymbol {}
impl ClingoSymbol {
    pub fn create_number(number: c_int) -> ClingoSymbol {
        let mut symbol = 0 as clingo_symbol_t;
        unsafe { clingo_symbol_create_number(number, &mut symbol) };
        ClingoSymbol(symbol)
    }

    //     pub fn clingo_symbol_create_supremum(symbol: *mut clingo_symbol_t);
    //     pub fn clingo_symbol_create_infimum(symbol: *mut clingo_symbol_t);
    //     pub fn clingo_symbol_create_string(string: *const c_char, symbol: *mut clingo_symbol_t) -> u8;

    /// Construct a symbol representing an id.
    ///
    ///
    /// **Note:** This is just a shortcut for clingo_symbol_create_function() with
    /// empty arguments.
    ///
    /// # Arguments
    ///
    /// * `name` - the name
    /// * `positive` - whether the symbol has a classical negation sign
    /// * `symbol` - the resulting symbol
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn create_id(name: &str, positive: bool) -> Result<ClingoSymbol, &'static str> {

        let mut symbol = 0 as clingo_symbol_t;
        let name_c_str = CString::new(name).unwrap();
        if unsafe { clingo_symbol_create_id(name_c_str.as_ptr(), positive, &mut symbol) } {
            Ok(ClingoSymbol(symbol))
        } else {
            Err(error_message())
        }
    }

    /// Construct a symbol representing a function or tuple.
    ///
    ///
    /// **Note:** To create tuples, the empty string has to be used as name.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the function
    /// * `arguments` - the arguments of the function
    /// * `arguments_size` - the number of arguments
    /// * `positive` - whether the symbol has a classical negation sign
    /// * `symbol` - the resulting symbol
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn create_function(
        name: &str,
        arguments: &[ClingoSymbol],
        positive: bool,
    ) -> Result<ClingoSymbol, &'static str> {

        let mut symbol = 0 as clingo_symbol_t;
        let name_c_str = CString::new(name).unwrap();
        if unsafe {
            clingo_symbol_create_function(
                name_c_str.as_ptr(),
                arguments.as_ptr() as *const clingo_symbol_t,
                arguments.len(),
                positive,
                &mut symbol,
            )
        }
        {
            Ok(ClingoSymbol(symbol))
        } else {
            Err(error_message())
        }
    }

    /// Get the number of a symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the target symbol
    /// * `number` - the resulting number
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_runtime if symbol is not of type ::clingo_symbol_type_number
    pub fn number(&self) -> Result<i32, &'static str> {

        let mut number = 0;
        if unsafe { clingo_symbol_number(self.0, &mut number) } {
            Ok(number)
        } else {
            Err(error_message())
        }
    }

    //     pub fn clingo_symbol_name(symbol: clingo_symbol_t, name: *mut *const c_char) -> u8;
    //     pub fn clingo_symbol_string(symbol: clingo_symbol_t, string: *mut *const c_char) -> u8;
    //     pub fn clingo_symbol_is_positive(symbol: clingo_symbol_t, positive: *mut u8) -> u8;
    //     pub fn clingo_symbol_is_negative(symbol: clingo_symbol_t, negative: *mut u8) -> u8;

    /// Get the arguments of a symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the target symbol
    /// * `arguments` - the resulting arguments
    /// * `arguments_size` - the number of arguments
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_runtime if symbol is not of type ::clingo_symbol_type_function
    pub fn arguments(&self) -> Result<Vec<ClingoSymbol>, &'static str> {

        let mut symbol_ptr = std::ptr::null() as *const clingo_symbol_t;
        let mut size: usize = 0;
        if unsafe { clingo_symbol_arguments(self.0, &mut symbol_ptr, &mut size) } {
            let mut symbols = Vec::<ClingoSymbol>::with_capacity(size);
            for _ in 0..size {
                let nsymbol = unsafe { *symbol_ptr };
                symbols.push(ClingoSymbol(nsymbol));
                symbol_ptr = unsafe { symbol_ptr.offset(1) };
            }
            Ok(symbols)
        } else {
            Err(error_message())
        }
    }

    //     pub fn clingo_symbol_type(symbol: clingo_symbol_t) -> clingo_symbol_type_t;
    //     pub fn clingo_symbol_to_string_size(symbol: clingo_symbol_t, size: *mut size_t) -> u8;

    pub fn to_string(&self) -> Option<String> {

        let mut size: usize = 0;
        let err = unsafe { clingo_symbol_to_string_size(self.0, &mut size) };
        if !err {
            None
        } else {
            let a1 = vec![1; size];
            let cstring = unsafe { CString::from_vec_unchecked(a1) };
            let err =
                unsafe { clingo_symbol_to_string(self.0, cstring.as_ptr() as *mut c_char, size) };
            if !err {
                None
            } else {
                cstring.into_string().ok()
            }
        }
    }

    pub fn is_less_than(&self, other: &ClingoSymbol) -> bool {
        unsafe { clingo_symbol_is_less_than(self.0, other.0) }
    }

    pub fn hash(&self) -> usize {
        unsafe { clingo_symbol_hash(self.0) }
    }
}

pub fn version() -> (i32, i32, i32) {
    let mut major = 0;
    let mut minor = 0;
    let mut revision = 0;
    unsafe { clingo_version(&mut major, &mut minor, &mut revision) };

    (major, minor, revision)
}

pub struct ClingoPart<'a> {
    name: CString,
    params: &'a [ClingoSymbol],
}
impl<'a> Drop for ClingoPart<'a> {
    fn drop(&mut self) {
        // println!("droped ClingoPart!");
    }
}
impl<'a> ClingoPart<'a> {
    pub fn new_part(name: &'a str, params: &'a [ClingoSymbol]) -> ClingoPart<'a> {
        ClingoPart {
            name: CString::new(name).unwrap(),
            params: params,
        }
    }
}

fn from_clingo_part(spart: &ClingoPart) -> clingo_part {
    clingo_part {
        name: spart.name.as_ptr(),
        params: spart.params.as_ptr() as *const clingo_symbol_t,
        size: spart.params.len(),
    }
}

pub fn error_code() -> clingo_error_t {
    unsafe { clingo_error_code() }
}

pub fn error_message() -> &'static str {

    let char_ptr: *const c_char = unsafe { clingo_error_message() };
    if char_ptr.is_null() {
        ""
    } else {
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        c_str.to_str().unwrap()
    }
}

pub fn set_error(code: clingo_error, message: &str) {

    let message_c_str = CString::new(message).unwrap();
    unsafe { clingo_set_error(code as clingo_error_t, message_c_str.as_ptr()) }
}

pub fn parse_program(
    program_: &str,
    callback: clingo_ast_callback_t,
    callback_data: *mut ::std::os::raw::c_void,
    logger: clingo_logger_t,
    logger_data: *mut ::std::os::raw::c_void,
    message_limit: ::std::os::raw::c_uint,
) -> Result<(), &'static str> {

    let program = CString::new(program_).unwrap();
    let suc = unsafe {
        clingo_parse_program(
            program.as_ptr(),
            callback,
            callback_data,
            logger,
            logger_data,
            message_limit,
        )
    };
    if suc { Ok(()) } else { Err(error_message()) }
}

pub struct ClingoPropagator(clingo_propagator_t);

pub struct ClingoPropagatorInit(clingo_propagate_init_t);

pub trait ClingoPropagatorBuilder<T> {
    fn init(_init: &mut ClingoPropagateInit, _data: &mut T) -> bool {
        true
    }
    fn propagate(
        _control: &mut ClingoPropagateControl,
        _changes: &[ClingoLiteral],
        _data: &mut T,
    ) -> bool {
        true
    }
    fn undo(
        _control: &mut ClingoPropagateControl,
        _changes: &[ClingoLiteral],
        _data: &mut T,
    ) -> bool {
        true
    }
    fn check(_control: &mut ClingoPropagateControl, _data: &mut T) -> bool {
        true
    }
    /// Get a ClingoPropagator
    fn new() -> ClingoPropagator {

        let prop = clingo_propagator_t {
            init: Some(Self::unsafe_init),
            propagate: Some(Self::unsafe_propagate),
            undo: Some(Self::unsafe_undo),
            check: Some(Self::unsafe_check),
        };
        ClingoPropagator(prop)
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_init(
        init_: *mut clingo_propagate_init_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        let init = (init_ as *mut ClingoPropagateInit).as_mut().unwrap();
        let propagator = (data as *mut T).as_mut().unwrap();
        Self::init(init, propagator)
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_propagate(
        control_: *mut clingo_propagate_control_t,
        changes_: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        let control = (control_ as *mut ClingoPropagateControl).as_mut().unwrap();
        let changes = std::slice::from_raw_parts(changes_ as *const ClingoLiteral, size);
        let propagator = (data as *mut T).as_mut().unwrap();
        Self::propagate(control, changes, propagator)
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_undo(
        control_: *mut clingo_propagate_control_t,
        changes_: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        let control = (control_ as *mut ClingoPropagateControl).as_mut().unwrap();
        let changes = std::slice::from_raw_parts(changes_ as *const ClingoLiteral, size);
        let propagator = (data as *mut T).as_mut().unwrap();
        Self::undo(control, changes, propagator)
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_check(
        control_: *mut clingo_propagate_control_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        let control = (control_ as *mut ClingoPropagateControl).as_mut().unwrap();
        let propagator = (data as *mut T).as_mut().unwrap();
        Self::check(control, propagator)
    }
}

#[derive(Debug)]
pub struct ClingoControl(clingo_control_t);
impl Drop for ClingoControl {
    fn drop(&mut self) {
        unsafe { clingo_control_free(&mut self.0) }
    }
}
impl ClingoControl {
    pub fn new<'a>(
        arguments: std::vec::Vec<String>,
        logger: clingo_logger_t,
        logger_data: *mut ::std::os::raw::c_void,
        message_limit: ::std::os::raw::c_uint,
    ) -> Result<&'a mut ClingoControl, &'static str> {

        // create a vector of zero terminated strings
        let mut args: Vec<CString> = Vec::new();
        for arg in arguments {
            args.push(CString::new(arg).unwrap());
        }

        // convert the strings to raw pointers
        let c_args = args.iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let mut ctl = std::ptr::null_mut();

        let suc = unsafe {
            clingo_control_new(
                c_args.as_ptr(),
                c_args.len(),
                logger,
                logger_data,
                message_limit,
                &mut ctl,
            )
        };
        if suc {
            unsafe { Ok(&mut *(ctl as *mut ClingoControl)) }
        } else {
            Err(error_message())
        }
    }

    //     pub fn clingo_control_load(control: *mut ClingoControl, file: *const c_char) -> u8;

    /// Extend the logic program with the given non-ground logic program in string form.
    ///
    /// This function puts the given program into a block of form: <tt>\#program name(parameters).</tt>
    ///
    /// After extending the logic program, the corresponding program parts are typically grounded with ::clingo_control_ground.
    ///
    /// # Arguments
    ///
    /// * `control` - the target
    /// * `name` name of the program block
    /// * `parameters` string array of parameters of the program block
    /// * `parameters_size` number of parameters
    /// * `program` string representation of the program
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if parsing fails
    pub fn add(
        &mut self,
        name_: &str,
        parameters: Vec<&str>,
        program_: &str,
    ) -> Result<(), &'static str> {

        let name = CString::new(name_).unwrap();
        let program = CString::new(program_).unwrap();

        let parameters_size = parameters.len();

        // create a vector of zero terminated strings
        let args = parameters
            .into_iter()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<CString>>();

        // convert the strings to raw pointers
        let c_args = args.iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let suc = unsafe {
            clingo_control_add(
                &mut self.0,
                name.as_ptr(),
                c_args.as_ptr(),
                parameters_size,
                program.as_ptr(),
            )
        };
        if suc { Ok(()) } else { Err(error_message()) }
    }

    /// Ground the selected @link ::clingo_part parts @endlink of the current (non-ground) logic program.
    ///
    /// After grounding, logic programs can be solved with ::clingo_control_solve().
    ///
    /// **Note:** Parts of a logic program without an explicit <tt>\#program</tt>
    /// specification are by default put into a program called `base` - without
    /// arguments.
    ///
    /// # Arguments
    ///
    /// * `control` - the target
    /// * `parts` array of parts to ground
    /// * `parts_size` size of the parts array
    /// * `ground_callback` callback to implement external functions
    /// * `ground_callback_data` user data for ground_callback
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - error code of ground callback
    ///
    /// @see clingo_part
    pub fn ground(
        &mut self,
        sparts: Vec<ClingoPart>,
        ground_callback: clingo_ground_callback_t,
        ground_callback_data: *mut ::std::os::raw::c_void,
    ) -> Result<(), &'static str> {

        let parts = sparts
            .iter()
            .map(|arg| from_clingo_part(arg))
            .collect::<Vec<clingo_part>>();
        let parts_size = parts.len();

        let suc = unsafe {
            clingo_control_ground(
                &mut self.0,
                parts.as_ptr(),
                parts_size,
                ground_callback,
                ground_callback_data,
            )
        };
        if suc { Ok(()) } else { Err(error_message()) }
    }

    pub fn solve(
        &mut self,
        mode: clingo_solve_mode_bitset_t,
        assumptions: Vec<clingo_symbolic_literal_t>,
        notify: clingo_solve_event_callback_t,
        data: *mut ::std::os::raw::c_void,
    ) -> Option<&mut ClingoSolveHandle> {

        let mut handle = std::ptr::null_mut() as *mut clingo_solve_handle_t;

        let err = unsafe {
            clingo_control_solve(
                &mut self.0,
                mode,
                assumptions.as_ptr(),
                assumptions.len(),
                notify,
                data,
                &mut handle,
            )
        };
        if !err {
            None
        } else {
            unsafe { (handle as *mut ClingoSolveHandle).as_mut() }
        }
    }

    //     pub fn clingo_control_cleanup(control: *mut ClingoControl) -> u8;

    /// Assign a truth value to an external atom.
    ///
    /// If the atom does not exist or is not external, this is a noop.
    ///
    /// # Arguments
    ///
    /// * `control` - the target
    /// * `atom` atom to assign
    /// * `value` - the truth value
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn assign_external(
        &mut self,
        ClingoSymbol(symbol): ClingoSymbol,
        value: clingo_truth_value,
    ) -> Result<(), &'static str> {
        let suc = unsafe {
            clingo_control_assign_external(&mut self.0, symbol, value as clingo_truth_value_t)
        };
        if suc { Ok(()) } else { Err(error_message()) }
    }

    //     pub fn clingo_control_release_external(control: *mut ClingoControl,
    //                                            atom: clingo_symbol_t)
    //                                            -> u8;

    /// Register a custom propagator with the control object.
    ///
    /// If the sequential flag is set to true, the propagator is called
    /// sequentially when solving with multiple threads.
    ///
    /// See the @ref Propagator module for more information.
    ///
    /// # Arguments
    ///
    /// * `control` - the target
    /// * `propagator` - the propagator
    /// * `data` user data passed to the propagator functions
    /// * `sequential` - whether the propagator should be called sequentially
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn register_propagator(
        &mut self,
        propagator: &ClingoPropagator,
        data: *mut ::std::os::raw::c_void,
        sequential: bool,
    ) -> Result<(), &'static str> {

        let ptr: *const ClingoPropagator = propagator;
        let ptr2 = ptr as *const clingo_propagator;
        let suc =
            unsafe { clingo_control_register_propagator(&mut self.0, ptr2, data, sequential) };
        if suc { Ok(()) } else { Err(error_message()) }
    }

    pub fn statistics(&mut self) -> Option<&mut ClingoStatistics> {

        let mut stat = std::ptr::null_mut() as *mut clingo_statistics_t;

        let err = unsafe { clingo_control_statistics(&mut self.0, &mut stat) };
        if !err {
            None
        } else {
            unsafe { (stat as *mut ClingoStatistics).as_mut() }
        }
    }

    //     pub fn clingo_control_interrupt(control: *mut ClingoControl);

    pub fn configuration(&mut self) -> Option<&mut ClingoConfiguration> {

        let mut conf = std::ptr::null_mut() as *mut clingo_configuration_t;
        let err = unsafe { clingo_control_configuration(&mut self.0, &mut conf) };
        if !err {
            None
        } else {
            unsafe { (conf as *mut ClingoConfiguration).as_mut() }
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

    pub fn symbolic_atoms(&mut self) -> Option<&mut ClingoSymbolicAtoms> {

        let mut atoms = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
        let err = unsafe { clingo_control_symbolic_atoms(&mut self.0, &mut atoms) };
        if !err {
            None
        } else {
            unsafe { (atoms as *mut ClingoSymbolicAtoms).as_mut() }
        }
    }

    pub fn theory_atoms(&mut self) -> Option<&mut ClingoTheoryAtoms> {

        let mut atoms = std::ptr::null_mut() as *mut clingo_theory_atoms_t;
        let err = unsafe { clingo_control_theory_atoms(&mut self.0, &mut atoms) };
        if !err {
            None
        } else {
            unsafe { (atoms as *mut ClingoTheoryAtoms).as_mut() }
        }
    }

    pub fn backend(&mut self) -> Option<&mut ClingoBackend> {

        let mut backend = std::ptr::null_mut();
        let err = unsafe { clingo_control_backend(&mut self.0, &mut backend) };
        if !err {
            None
        } else {
            unsafe { (backend as *mut ClingoBackend).as_mut() }
        }
    }

    /// Get an object to add non-ground directives to the program.
    ///
    /// See the @ref ProgramBuilder module for more information.
    ///
    /// # Arguments
    ///
    /// * `control` - the target
    /// * `builder` - the program builder object
    ///
    /// **Returns** whether the call was successful
    pub fn program_builder(&mut self) -> Result<&mut ClingoProgramBuilder, &'static str> {

        let mut builder = std::ptr::null_mut() as *mut clingo_program_builder_t;
        if unsafe { clingo_control_program_builder(&mut self.0, &mut builder) } {
            unsafe { (builder as *mut ClingoProgramBuilder).as_mut() }
                .ok_or("Failed to obtain ProgramBuilder.")
        } else {
            Err(error_message())
        }
    }
}

pub struct ClingoProgramBuilder(clingo_program_builder_t);
impl ClingoProgramBuilder {
    /// Begin building a program.
    ///
    /// * `builder` - the target program builder
    ///
    /// **Returns** whether the call was successful
    pub fn begin(&mut self) -> Result<(), &'static str> {
        if unsafe { clingo_program_builder_begin(&mut self.0) } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// Adds a statement to the program.
    ///
    /// @attention @ref clingo_program_builder_begin() must be called before adding statements and @ref clingo_program_builder_end() must be called after all statements have been added.
    ///
    /// # Arguments
    ///
    /// * `builder` - the target program builder
    /// * `statement` - the statement to add
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_runtime for statements of invalid form
    /// - ::clingo_error_bad_alloc
    pub fn add(&mut self, statement: &ClingoAstStatement) -> Result<(), &'static str> {

        let ClingoAstStatement(ref stm) = *statement;
        if unsafe { clingo_program_builder_add(&mut self.0, stm) } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// End building a program.
    ///
    /// * `builder` - the target program builder
    ///
    /// **Returns** whether the call was successful
    pub fn end(&mut self) -> Result<(), &'static str> {
        if unsafe { clingo_program_builder_end(&mut self.0) } {
            Ok(())
        } else {
            Err(error_message())
        }
    }
}

#[derive(Clone, Copy)]
pub struct ClingoAstBodyLiteral(clingo_ast_body_literal_t);
impl ClingoAstBodyLiteral {
    pub fn new(
        location: ClingoLocation,
        sign: clingo_ast_sign,
        type_: clingo_ast_body_literal_type,
        lit_ref: &ClingoAstLiteral,
    ) -> ClingoAstBodyLiteral {
        let _bg_union_2 = clingo_ast_body_literal__bindgen_ty_1 {
            literal: (lit_ref as *const ClingoAstLiteral) as *const clingo_ast_literal,
        };
        ClingoAstBodyLiteral(clingo_ast_body_literal_t {
            location: location.clingo_location(),
            sign: sign as clingo_ast_sign_t,
            type_: type_ as clingo_ast_body_literal_type_t,
            __bindgen_anon_1: _bg_union_2,
        })
    }
}

pub struct ClingoAstRule(clingo_ast_rule_t);
impl ClingoAstRule {
    pub fn new(head: clingo_ast_head_literal_t, body: &[ClingoAstBodyLiteral]) -> ClingoAstRule {

        let rule = clingo_ast_rule {
            head: head,
            body: body.as_ptr() as *const clingo_ast_body_literal_t,
            size: body.len(),
        };
        ClingoAstRule(rule)
    }

    pub fn head(&self) -> clingo_ast_head_literal_t {
        let ClingoAstRule(ref rule) = *self;
        rule.head
    }

    pub fn body(&self) -> &[ClingoAstBodyLiteral] {
        let ClingoAstRule(ref rule) = *self;
        unsafe { std::slice::from_raw_parts(rule.body as *const ClingoAstBodyLiteral, rule.size) }
    }

    pub fn size(&self) -> usize {
        let ClingoAstRule(ref rule) = *self;
        rule.size
    }
}

pub struct ClingoAstExternal(clingo_ast_external_t);
impl ClingoAstExternal {
    pub fn new(sym: ClingoAstTerm, body: &[ClingoAstBodyLiteral]) -> ClingoAstExternal {

        let ClingoAstTerm(symbol) = sym;
        let ext = clingo_ast_external {
            atom: symbol,
            body: body.as_ptr() as *const clingo_ast_body_literal_t,
            size: body.len(),
        };
        ClingoAstExternal(ext)
    }
}

pub struct ClingoAstStatement(clingo_ast_statement_t);
impl ClingoAstStatement {
    pub fn new_external(
        location: ClingoLocation,
        type_: clingo_ast_statement_type,
        ext: &ClingoAstExternal,
    ) -> ClingoAstStatement {

        let external: *const ClingoAstExternal = ext;
        let _bg_union_2 =
            clingo_ast_statement__bindgen_ty_1 { external: external as *const clingo_ast_external };
        let stm = clingo_ast_statement_t {
            location: location.clingo_location(),
            type_: type_ as clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        ClingoAstStatement(stm)
    }

    pub fn new_rule(location: ClingoLocation, rule_: &ClingoAstRule) -> ClingoAstStatement {

        let rule: *const ClingoAstRule = rule_;

        let _bg_union_2 =
            clingo_ast_statement__bindgen_ty_1 { rule: rule as *const clingo_ast_rule };
        let stm = clingo_ast_statement_t {
            location: location.clingo_location(),
            type_: clingo_ast_statement_type::clingo_ast_statement_type_rule as
                clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        ClingoAstStatement(stm)
    }

    pub fn location(&self) -> ClingoLocation {
        let ClingoAstStatement(ref stm) = *self;
        ClingoLocation::from(stm.location)
    }

    pub fn get_type(&self) -> clingo_ast_statement_type {
        let ClingoAstStatement(ref stm) = *self;
        let t = stm.type_;
        match t {
            0 => clingo_ast_statement_type::clingo_ast_statement_type_rule,
            1 => clingo_ast_statement_type::clingo_ast_statement_type_const,
            2 => clingo_ast_statement_type::clingo_ast_statement_type_show_signature,
            3 => clingo_ast_statement_type::clingo_ast_statement_type_show_term,
            4 => clingo_ast_statement_type::clingo_ast_statement_type_minimize,
            5 => clingo_ast_statement_type::clingo_ast_statement_type_script,
            6 => clingo_ast_statement_type::clingo_ast_statement_type_program,
            7 => clingo_ast_statement_type::clingo_ast_statement_type_external,
            8 => clingo_ast_statement_type::clingo_ast_statement_type_edge,
            9 => clingo_ast_statement_type::clingo_ast_statement_type_heuristic,
            10 => clingo_ast_statement_type::clingo_ast_statement_type_project_atom,
            11 => clingo_ast_statement_type::clingo_ast_statement_type_project_atom_signature,
            _ => clingo_ast_statement_type::clingo_ast_statement_type_theory_definition,
        }
    }

    pub unsafe fn rule(&self) -> &ClingoAstRule {
        let ClingoAstStatement(ref stm) = *self;
        let ast_rule_ptr = stm.__bindgen_anon_1.rule as *const clingo_ast_rule_t;
        (ast_rule_ptr as *const ClingoAstRule).as_ref().unwrap()
    }
}

pub struct ClingoAstTerm(clingo_ast_term_t);
impl ClingoAstTerm {
    pub fn new_symbol(location: ClingoLocation, symbol_: ClingoSymbol) -> ClingoAstTerm {
        let ClingoSymbol(symbol) = symbol_;
        let _bg_union_1 = clingo_ast_term__bindgen_ty_1 { symbol: symbol };
        let term = clingo_ast_term_t {
            location: location.clingo_location(),
            type_: clingo_ast_term_type::clingo_ast_term_type_symbol as clingo_ast_term_type_t,
            __bindgen_anon_1: _bg_union_1,
        };
        ClingoAstTerm(term)
    }

    pub fn location(&self) -> ClingoLocation {

        let ClingoAstTerm(ref term) = *self;
        ClingoLocation::from(term.location)
    }
}

pub struct ClingoAstLiteral(clingo_ast_literal_t);
impl ClingoAstLiteral {
    pub fn new(
        location: ClingoLocation,
        sign: clingo_ast_sign,
        type_: clingo_ast_literal_type,
        sym: &ClingoAstTerm,
    ) -> ClingoAstLiteral {

        let symbol: *const ClingoAstTerm = sym;
        let _bg_union_2 = clingo_ast_literal__bindgen_ty_1 {
            symbol: symbol as *const clingo_sys::clingo_ast_term,
        };
        let lit = clingo_ast_literal_t {
            location: location.clingo_location(),
            type_: type_ as clingo_ast_literal_type_t,
            sign: sign as clingo_ast_sign_t,
            __bindgen_anon_1: _bg_union_2,
        };
        ClingoAstLiteral(lit)
    }
}

pub struct ClingoConfiguration(clingo_configuration_t);
impl ClingoConfiguration {
    /// Get the root key of the configuration.
    ///
    /// # Arguments
    ///
    /// * `configuration` - the target configuration
    /// * `key` - the root key
    ///
    /// **Returns** whether the call was successful
    pub fn root(&mut self) -> Result<ClingoId, &'static str> {
        let ClingoConfiguration(ref mut conf) = *self;
        let mut root_key = 0 as clingo_id_t;
        if unsafe { clingo_configuration_root(conf, &mut root_key) } {
            Ok(ClingoId(root_key))
        } else {
            Err(error_message())
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

    /// Get the subkey at the given offset of an array entry.
    ///
    ///
    /// **Note:** Some array entries, like fore example the solver configuration, can be accessed past there actual size to add subentries.
    /// # Pre-condition
    ///
    /// The @link clingo_configuration_type() type@endlink of the entry must be @ref ::clingo_configuration_type_array.
    ///
    /// # Arguments
    ///
    /// * `configuration` - the target configuration
    /// * `key` - the key
    /// * `offset` - the offset in the array
    /// * `subkey` - the resulting subkey
    ///
    /// **Returns** whether the call was successful
    pub fn array_at(
        &mut self,
        ClingoId(key): ClingoId,
        offset: usize,
    ) -> Result<ClingoId, &'static str> {

        let mut nkey = 0 as clingo_id_t;
        if unsafe { clingo_configuration_array_at(&mut self.0, key, offset, &mut nkey) } {
            Ok(ClingoId(nkey))
        } else {
            Err(error_message())
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

    /// Lookup a subkey under the given name.
    ///
    /// # Pre-condition
    ///
    /// The @link clingo_configuration_type() type@endlink of the entry must be @ref ::clingo_configuration_type_map.
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    ///
    /// # Arguments
    ///
    /// * `configuration` - the target configuration
    /// * `key` - the key
    /// * `name` - the name to lookup the subkey
    /// * `subkey` - the resulting subkey
    ///
    /// **Returns** whether the call was successful
    pub fn map_at(
        &mut self,
        ClingoId(key): ClingoId,
        name: &str,
    ) -> Result<ClingoId, &'static str> {

        let mut nkey = 0 as clingo_id_t;
        let name_c_str = CString::new(name).unwrap();
        if unsafe {
            clingo_configuration_map_at(&mut self.0, key, name_c_str.as_ptr(), &mut nkey)
        }
        {
            Ok(ClingoId(nkey))
        } else {
            Err(error_message())
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

    /// Set the value of an entry.
    ///
    /// # Pre-condition
    ///
    /// The @link clingo_configuration_type() type@endlink of the entry must be @ref ::clingo_configuration_type_value.
    ///
    /// # Arguments
    ///
    /// * `configuration` - the target configuration
    /// * `key` - the key
    /// * `value` - the value to set
    ///
    /// **Returns** whether the call was successful
    pub fn value_set(&mut self, ClingoId(key): ClingoId, value: &str) -> Result<(), &'static str> {

        let value_c_str = CString::new(value).unwrap();
        if unsafe { clingo_configuration_value_set(&mut self.0, key, value_c_str.as_ptr()) } {
            Ok(())
        } else {
            Err(error_message())
        }
    }
}

pub struct ClingoBackend(clingo_backend_t);
impl ClingoBackend {
    /// Add a rule to the program.
    ///
    /// # Arguments
    ///
    /// * `backend` - the target backend
    /// * `choice` determines if the head is a choice or a disjunction
    /// * `head` - the head atoms
    /// * `head_size` - the number of atoms in the head
    /// * `body` - the body literals
    /// * `body_size` - the number of literals in the body
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn rule(
        &mut self,
        choice: bool,
        head: &[ClingoAtom],
        body: &[ClingoLiteral],
    ) -> Result<(), &'static str> {
        if unsafe {
            clingo_backend_rule(
                &mut self.0,
                choice,
                head.as_ptr() as *const clingo_atom_t,
                head.len(),
                body.as_ptr() as *const clingo_literal_t,
                body.len(),
            )
        }
        {
            Ok(())
        } else {
            Err(error_message())
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

    /// Add an assumption directive.
    ///
    /// # Arguments
    ///
    /// * `backend` - the target backend
    /// * `literals` - the literals to assume (positive literals are true and negative literals false for the next solve call)
    /// * `size` - the number of atoms
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn assume(&mut self, literals: &[ClingoLiteral]) -> Result<(), &'static str> {
        let size = literals.len();
        if unsafe {
            clingo_backend_assume(
                &mut self.0,
                literals.as_ptr() as *const clingo_literal_t,
                size,
            )
        }
        {
            Ok(())
        } else {
            Err(error_message())
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

    /// Get a fresh atom to be used in aspif directives.
    ///
    /// # Arguments
    ///
    /// * `backend` - the target backend
    /// * `atom` - the resulting atom
    ///
    /// **Returns** whether the call was successful
    pub fn add_atom(&mut self) -> Result<ClingoAtom, &'static str> {

        let mut atom = 0 as clingo_atom_t;
        if unsafe { clingo_backend_add_atom(&mut self.0, &mut atom) } {
            Ok(ClingoAtom(atom))
        } else {
            Err(error_message())
        }
    }
}

pub struct ClingoStatistics(clingo_statistics_t);
impl ClingoStatistics {
    /// Get the root key of the statistics.
    ///
    /// # Arguments
    ///
    /// * `statistics` - the target statistics
    /// * `key` - the root key
    ///
    /// **Returns** whether the call was successful
    pub fn root(&mut self) -> Result<u64, &'static str> {

        let mut root_key = 0 as u64;
        if unsafe { clingo_statistics_root(&mut self.0, &mut root_key) } {
            Ok(root_key)
        } else {
            Err(error_message())
        }
    }

    /// Get the type of a key.
    ///
    /// # Arguments
    ///
    /// * `statistics` - the target statistics
    /// * `key` - the key
    /// * `type` - the resulting type
    ///
    /// **Returns** whether the call was successful
    pub fn statistics_type(&mut self, key: u64) -> Result<clingo_statistics_type, &'static str> {

        let mut stype = 0 as clingo_statistics_type_t;
        if unsafe { clingo_statistics_type(&mut self.0, key, &mut stype) } {
            match stype {
                0 => Ok(clingo_statistics_type::clingo_statistics_type_empty),
                1 => Ok(clingo_statistics_type::clingo_statistics_type_value),
                2 => Ok(clingo_statistics_type::clingo_statistics_type_array),
                _ => Ok(clingo_statistics_type::clingo_statistics_type_map),
            }
        } else {
            Err(error_message())
        }
    }

    /// Get the size of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be @ref ::clingo_statistics_type_array.
    ///
    /// # Arguments
    ///
    /// * `statistics` - the target statistics
    /// * `key` - the key
    /// * `size` - the resulting size
    ///
    /// **Returns** whether the call was successful
    pub fn array_size(&mut self, key: u64) -> Result<usize, &'static str> {

        let mut size = 0 as usize;
        if unsafe { clingo_statistics_array_size(&mut self.0, key, &mut size) } {
            Ok(size)
        } else {
            Err(error_message())
        }
    }

    /// Get the subkey at the given offset of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be @ref ::clingo_statistics_type_array.
    ///
    /// # Arguments
    ///
    /// * `statistics` - the target statistics
    /// * `key` - the key
    /// * `offset` - the offset in the array
    /// * `subkey` - the resulting subkey
    ///
    /// **Returns** whether the call was successful
    pub fn statistics_array_at(&mut self, key: u64, offset: usize) -> Result<u64, &'static str> {

        let mut subkey = 0 as u64;
        if unsafe { clingo_statistics_array_at(&mut self.0, key, offset, &mut subkey) } {
            Ok(subkey)
        } else {
            Err(error_message())
        }
    }

    /// Get the number of subkeys of a map entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be [clingo_statistics_type_map](clingo_sys/enum.clingo_statistics_type.html).
    ///
    /// # Arguments
    ///
    /// * `statistics` - the target statistics
    /// * `key` - the key
    /// * `size` - the resulting number
    ///
    /// **Returns** whether the call was successful
    pub fn map_size(&mut self, key: u64) -> Result<usize, &'static str> {

        let mut size = 0 as usize;
        if unsafe { clingo_statistics_map_size(&mut self.0, key, &mut size) } {
            Ok(size)
        } else {
            Err(error_message())
        }
    }

    /// Get the name associated with the offset-th subkey.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be @ref ::clingo_statistics_type_map.
    ///
    /// # Arguments
    ///
    /// * `statistics` - the target statistics
    /// * `key` - the key
    /// * `offset` - the offset of the name
    /// * `name` - the resulting name
    ///
    /// **Returns** whether the call was successful
    pub fn map_subkey_name<'a>(
        &mut self,
        key: u64,
        offset: usize,
    ) -> Result<&'a str, &'static str> {

        let mut name = std::ptr::null() as *const c_char;
        if unsafe { clingo_statistics_map_subkey_name(&mut self.0, key, offset, &mut name) } {
            Ok(unsafe { CStr::from_ptr(name) }.to_str().unwrap())
        } else {
            Err(error_message())
        }
    }

    /// Lookup a subkey under the given name.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be @ref ::clingo_statistics_type_map.
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    ///
    /// # Arguments
    ///
    /// * `statistics` - the target statistics
    /// * `key` - the key
    /// * `name` - the name to lookup the subkey
    /// * `subkey` - the resulting subkey
    ///
    /// **Returns** whether the call was successful
    pub fn map_at(&mut self, key: u64, name: &str) -> Result<u64, &'static str> {

        let mut subkey = 0 as u64;
        let name_c_str = CString::new(name).unwrap();
        if unsafe { clingo_statistics_map_at(&mut self.0, key, name_c_str.as_ptr(), &mut subkey) } {
            Ok(subkey)
        } else {
            Err(error_message())
        }
    }

    /// Get the value of the given entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be @ref ::clingo_statistics_type_value.
    ///
    /// # Arguments
    ///
    /// * `statistics` - the target statistics
    /// * `key` - the key
    /// * `value` - the resulting value
    ///
    /// **Returns** whether the call was successful
    pub fn value_get(&mut self, key: u64) -> Result<f64, &'static str> {

        let mut value = 0.0 as f64;
        if unsafe { clingo_statistics_value_get(&mut self.0, key, &mut value) } {
            Ok(value)
        } else {
            Err(error_message())
        }
    }
}

pub struct ClingoSignature(clingo_signature_t);
impl ClingoSignature {
    /// Create a new signature.
    ///
    /// # Arguments
    ///
    /// * `name` name of the signature
    /// * `arity` arity of the signature
    /// * `positive` false if the signature has a classical negation sign
    /// * `signature` - the resulting signature
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn create(
        name_: &str,
        arity: u32,
        positive: bool,
    ) -> Result<ClingoSignature, &'static str> {
        let name_c_str = CString::new(name_).unwrap();
        let mut signature = 0;
        if unsafe {
            clingo_signature_create(name_c_str.as_ptr(), arity, positive, &mut signature)
        }
        {
            Ok(ClingoSignature(signature))
        } else {
            Err(error_message())
        }
    }
}

#[derive(Debug)]
pub struct ClingoSymbolicAtoms(clingo_symbolic_atoms_t);
impl ClingoSymbolicAtoms {
    /// Get a forward iterator to the beginning of the sequence of all symbolic
    /// atoms optionally restricted to a given signature.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `signature` optional signature
    /// * `iterator` - the resulting iterator
    ///
    /// **Returns** whether the call was successful
    pub fn begin(
        &mut self,
        opt_sig: Option<&ClingoSignature>,
    ) -> Result<clingo_symbolic_atom_iterator_t, &'static str> {

        match opt_sig {
            Some(sig) => {
                let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
                if unsafe { clingo_symbolic_atoms_begin(&mut self.0, &sig.0, &mut iterator) } {
                    Ok(iterator)
                } else {
                    Err(error_message())
                }
            }
            None => {
                let signature = std::ptr::null();
                let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
                if unsafe { clingo_symbolic_atoms_begin(&mut self.0, signature, &mut iterator) } {
                    Ok(iterator)
                } else {
                    Err(error_message())
                }
            }
        }
    }

    /// Iterator pointing to the end of the sequence of symbolic atoms.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `iterator` - the resulting iterator
    ///
    /// **Returns** whether the call was successful
    pub fn end(&mut self) -> Result<clingo_symbolic_atom_iterator_t, &'static str> {

        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        if unsafe { clingo_symbolic_atoms_end(&mut self.0, &mut iterator) } {
            Ok(iterator)
        } else {
            Err(error_message())
        }
    }

    /// Find a symbolic atom given its symbolic representation.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `symbol` - the symbol to lookup
    /// * `iterator` iterator pointing to the symbolic atom or to the end
    /// of the sequence if no corresponding atom is found
    ///
    /// **Returns** whether the call was successful
    pub fn find(
        &mut self,
        ClingoSymbol(symbol): ClingoSymbol,
    ) -> Result<clingo_symbolic_atom_iterator_t, &'static str> {

        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        if unsafe { clingo_symbolic_atoms_find(&mut self.0, symbol, &mut iterator) } {
            Ok(iterator)
        } else {
            Err(error_message())
        }
    }

    /// Check if two iterators point to the same element (or end of the sequence).
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `a` - the first iterator
    /// * `b` - the second iterator
    /// * `equal` - whether the two iterators are equal
    ///
    /// **Returns** whether the call was successful
    pub fn iterator_is_equal_to(
        &mut self,
        a: clingo_symbolic_atom_iterator_t,
        b: clingo_symbolic_atom_iterator_t,
    ) -> Result<bool, &'static str> {

        let mut equal = false;
        if unsafe { clingo_symbolic_atoms_iterator_is_equal_to(&mut self.0, a, b, &mut equal) } {
            Ok(equal)
        } else {
            Err(error_message())
        }
    }

    /// Get the symbolic representation of an atom.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `iterator` iterator to the atom
    /// * `symbol` - the resulting symbol
    ///
    /// **Returns** whether the call was successful
    pub fn symbol(
        &mut self,
        iterator: clingo_symbolic_atom_iterator_t,
    ) -> Result<ClingoSymbol, &'static str> {

        let mut symbol = 0 as clingo_symbol_t;
        if unsafe { clingo_symbolic_atoms_symbol(&mut self.0, iterator, &mut symbol) } {
            Ok(ClingoSymbol(symbol))
        } else {
            Err(error_message())
        }
    }

    /// Check whether an atom is a fact.
    ///
    ///
    /// **Note:** This does not determine if an atom is a cautious consequence. The
    /// grounding or solving component`s simplifications can only detect this in
    /// some cases.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `iterator` iterator to the atom
    /// * `fact` - whether the atom is a fact
    ///
    /// **Returns** whether the call was successful
    pub fn is_fact(
        &mut self,
        iterator: clingo_symbolic_atom_iterator_t,
    ) -> Result<bool, &'static str> {

        let mut fact = false;
        if unsafe { clingo_symbolic_atoms_is_fact(&mut self.0, iterator, &mut fact) } {
            Ok(fact)
        } else {
            Err(error_message())
        }
    }

    /// Check whether an atom is external.
    ///
    /// An atom is external if it has been defined using an external directive and
    /// has not been released or defined by a rule.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `iterator` iterator to the atom
    /// * `external` - whether the atom is a external
    ///
    /// **Returns** whether the call was successful
    pub fn is_external(
        &mut self,
        iterator: clingo_symbolic_atom_iterator_t,
    ) -> Result<bool, &'static str> {

        let mut external = false;
        if unsafe { clingo_symbolic_atoms_is_external(&mut self.0, iterator, &mut external) } {
            Ok(external)
        } else {
            Err(error_message())
        }
    }

    /// Returns the (numeric) aspif literal corresponding to the given symbolic atom.
    ///
    /// Such a literal can be mapped to a solver literal (see the \ref Propagator
    /// module) or be used in rules in aspif format (see the \ref ProgramBuilder
    /// module).
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `iterator` iterator to the atom
    /// * `literal` - the associated literal
    ///
    /// **Returns** whether the call was successful
    pub fn literal(
        &mut self,
        iterator: clingo_symbolic_atom_iterator_t,
    ) -> Result<ClingoLiteral, &'static str> {

        let mut literal = 0 as clingo_literal_t;
        if unsafe { clingo_symbolic_atoms_literal(&mut self.0, iterator, &mut literal) } {
            Ok(ClingoLiteral(literal))
        } else {
            Err(error_message())
        }
    }

    //     pub fn clingo_symbolic_atoms_signatures_size(atoms: *mut ClingoSymbolicAtoms,
    //                                                  size: *mut size_t)
    //                                                  -> u8;

    //     pub fn clingo_symbolic_atoms_signatures(atoms: *mut ClingoSymbolicAtoms,
    //                                             signatures: *mut clingo_signature_t,
    //                                             size: size_t)
    //                                             -> u8;

    /// Get an iterator to the next element in the sequence of symbolic atoms.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `iterator` - the current iterator
    /// * `next` - the succeeding iterator
    ///
    /// **Returns** whether the call was successful
    pub fn next(
        &mut self,
        iterator: clingo_symbolic_atom_iterator_t,
    ) -> Result<clingo_symbolic_atom_iterator_t, &'static str> {

        let mut next = 0 as clingo_symbolic_atom_iterator_t;
        if unsafe { clingo_symbolic_atoms_next(&mut self.0, iterator, &mut next) } {
            Ok(next)
        } else {
            Err(error_message())
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

    /// Get the name of the given constant or function theory term.
    ///
    /// # Pre-condition
    ///
    /// The term must be of type ::clingo_theory_term_type_function or ::clingo_theory_term_type_symbol.
    ///
    /// # Arguments
    ///
    /// * `atoms` container where the term is stored
    /// * `term` id of the term
    /// * `name` - the resulting name
    ///
    /// **Returns** whether the call was successful
    pub fn term_name<'a>(&mut self, ClingoId(term): ClingoId) -> Result<&'a str, &'static str> {

        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_theory_atoms_term_name(&mut self.0, term, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Ok(c_str.to_str().unwrap())
        } else {
            Err(error_message())
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

    /// Get the total number of theory atoms.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the target
    /// * `size` - the resulting number
    ///
    /// **Returns** whether the call was successful
    pub fn size(&mut self) -> Result<usize, &'static str> {

        let mut size = 0 as usize;
        if unsafe { clingo_theory_atoms_size(&mut self.0, &mut size) } {
            Ok(size)
        } else {
            Err(error_message())
        }
    }

    /// Get the theory term associated with the theory atom.
    ///
    /// # Arguments
    ///
    /// * `atoms` container where the atom is stored
    /// * `atom` id of the atom
    /// * `term` - the resulting term id
    ///
    /// **Returns** whether the call was successful
    pub fn atom_term(&mut self, ClingoId(atom): ClingoId) -> Result<ClingoId, &'static str> {

        let mut term = 0 as clingo_id_t;
        if unsafe { clingo_theory_atoms_atom_term(&mut self.0, atom, &mut term) } {
            Ok(ClingoId(term))
        } else {
            Err(error_message())
        }
    }

    //     pub fn clingo_theory_atoms_atom_elements(atoms: *mut ClingoTheoryAtoms,
    //                                              atom: clingo_id_t,
    //                                              elements: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;

    /// Whether the theory atom has a guard.
    ///
    /// # Arguments
    ///
    /// * `atoms` container where the atom is stored
    /// * `atom` id of the atom
    /// * `has_guard` - whether the theory atom has a guard
    ///
    /// **Returns** whether the call was successful
    pub fn atom_has_guard(&mut self, ClingoId(atom): ClingoId) -> Result<bool, &'static str> {

        let mut has_guard = false;
        if unsafe { clingo_theory_atoms_atom_has_guard(&mut self.0, atom, &mut has_guard) } {
            Ok(has_guard)
        } else {
            Err(error_message())
        }
    }

    //     pub fn clingo_theory_atoms_atom_guard(atoms: *mut ClingoTheoryAtoms,
    //                                           atom: clingo_id_t,
    //                                           connective: *mut *const c_char,
    //                                           term: *mut clingo_id_t)
    //                                           -> u8;

    /// Get the aspif literal associated with the given theory atom.
    ///
    /// # Arguments
    ///
    /// * `atoms` container where the atom is stored
    /// * `atom` id of the atom
    /// * `literal` - the resulting literal
    ///
    /// **Returns** whether the call was successful
    pub fn atom_literal(
        &mut self,
        ClingoId(atom): ClingoId,
    ) -> Result<ClingoLiteral, &'static str> {

        let mut literal = 0 as clingo_literal_t;
        if unsafe { clingo_theory_atoms_atom_literal(&mut self.0, atom, &mut literal) } {
            Ok(ClingoLiteral(literal))
        } else {
            Err(error_message())
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
pub struct UNSAFEClingoTheoryAtomsIterator {
    count: usize,
    size: usize,
}
impl Iterator for UNSAFEClingoTheoryAtomsIterator {
    type Item = ClingoId;

    fn next(&mut self) -> Option<ClingoId> {
        // increment our count. This is why we started at zero.
        self.count += 1;

        // check to see if we've finished counting or not.
        if self.count < self.size {
            Some(ClingoId(self.count as u32))
        } else {
            None
        }
    }
}
impl UNSAFEClingoTheoryAtomsIterator {
    pub fn from(cta: &mut ClingoTheoryAtoms) -> UNSAFEClingoTheoryAtomsIterator {
        UNSAFEClingoTheoryAtomsIterator {
            count: 0,
            size: cta.size().unwrap(),
        }
    }
}

pub struct ClingoModel(clingo_model_t);
impl ClingoModel {
    /// Get the type of the model.
    ///
    /// # Arguments
    ///
    /// * `model` - the target
    /// * `type` - the type of the model
    ///
    /// **Returns** whether the call was successful
    pub fn model_type(&mut self) -> Result<clingo_model_type_t, &'static str> {

        let mut mtype = 0 as clingo_model_type_t;
        if unsafe { clingo_model_type(&mut self.0, &mut mtype) } {
            Ok(mtype)
        } else {
            Err(error_message())
        }
    }

    /// Get the running number of the model.
    ///
    /// # Arguments
    ///
    /// * `model` - the target
    /// * `number` - the number of the model
    ///
    /// **Returns** whether the call was successful
    pub fn number(&mut self) -> Result<u64, &'static str> {

        let mut number = 0;
        if unsafe { clingo_model_number(&mut self.0, &mut number) } {
            Ok(number)
        } else {
            Err(error_message())
        }
    }

    /// Get the number of symbols of the selected types in the model.
    ///
    /// # Arguments
    ///
    /// * `model` - the target
    /// * `show` - which symbols to select
    /// * `size` - the number symbols
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    // pub fn clingo_model_symbols_size(model: *mut ClingoModel,
    //  show: clingo_show_type_bitset_t,
    //  size: *mut size_t)
    //  -> u8;
    /// Get the symbols of the selected types in the model.
    ///
    ///
    /// **Note:** CSP assignments are represented using functions with name "$"
    /// where the first argument is the name of the CSP variable and the second one its
    /// value.
    ///
    /// # Arguments
    ///
    /// * `model` - the target
    /// * `show` - which symbols to select
    /// * `symbols` - the resulting symbols
    /// * `size` - the number of selected symbols
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if the size is too small
    ///
    /// @see clingo_model_symbols_size()
    pub fn symbols(
        &mut self,
        show: clingo_show_type_bitset_t,
    ) -> Result<Vec<ClingoSymbol>, &'static str> {
        let ClingoModel(ref mut model) = *self;
        let mut size: usize = 0;
        let size_p = &mut size as *mut usize;

        if unsafe { clingo_model_symbols_size(model, show, size_p) } {
            let mut symbols = Vec::<ClingoSymbol>::with_capacity(size);
            let symbols_ptr = symbols.as_ptr();
            if unsafe {
                clingo_model_symbols(model, show, symbols_ptr as *mut clingo_symbol_t, size)
            }
            {
                symbols =
                    unsafe { Vec::from_raw_parts(symbols_ptr as *mut ClingoSymbol, size, size) };
                Ok(symbols)
            } else {
                Err(error_message())
            }
        } else {
            Err(error_message())
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

pub struct ClingoSolveControl(clingo_solve_control_t);
impl ClingoSolveControl {
    /// Add a clause that applies to the current solving step during model
    /// enumeration.
    ///
    ///
    /// **Note:** The @ref Propagator module provides a more sophisticated
    /// interface to add clauses - even on partial assignments.
    ///
    /// # Arguments
    ///
    /// * `control` - the target
    /// * `clause` array of literals representing the clause
    /// * `size` - the size of the literal array
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if adding the clause fails
    pub fn add_clause(&mut self, clause: &[ClingoLiteral]) -> Result<(), &'static str> {

        if unsafe {
            clingo_solve_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                clause.len(),
            )
        }
        {
            Ok(())
        } else {
            Err(error_message())
        }
    }
}

pub struct ClingoPropagateControl(clingo_propagate_control_t);
impl ClingoPropagateControl {
    /// Get the id of the underlying solver thread.
    ///
    /// Thread ids are consecutive numbers starting with zero.
    ///
    /// # Arguments
    ///
    /// * `control` - the target
    ///
    /// **Returns** the thread id
    pub fn thread_id(&mut self) -> u32 {
        unsafe { clingo_propagate_control_thread_id(&mut self.0) }
    }

    //     pub fn clingo_propagate_control_assignment(control: *mut ClingoPropagateControl)
    //                                           -> *mut clingo_assignment_t;

    /// Add the given clause to the solver.
    ///
    /// This method sets its result to false if the current propagation must be stopped for the solver to backtrack.
    ///
    /// @attention No further calls on the control object or functions on the assignment should be called when the result of this method is false.
    ///
    /// # Arguments
    ///
    /// * `control` - the target
    /// * `clause` - the clause to add
    /// * `size` - the size of the clause
    /// * `type` - the clause type determining its lifetime
    /// * `result` result indicating whether propagation has to be stopped
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn add_clause(
        &mut self,
        clause: &[ClingoLiteral],
        type_: clingo_clause_type,
    ) -> Result<bool, &'static str> {
        let mut result = false;
        if unsafe {
            clingo_propagate_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                clause.len(),
                type_ as clingo_clause_type_t,
                &mut result,
            )
        }
        {
            Ok(result)
        } else {
            Err(error_message())
        }

    }

    /// Propagate implied literals (resulting from added clauses).
    ///
    /// This method sets its result to false if the current propagation must be stopped for the solver to backtrack.
    ///
    /// @attention No further calls on the control object or functions on the assignment should be called when the result of this method is false.
    ///
    /// # Arguments
    ///
    /// * `control` - the target
    /// * `result` result indicating whether propagation has to be stopped
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn propagate(&mut self) -> Result<bool, &'static str> {
        let mut result = false;
        if unsafe { clingo_propagate_control_propagate(&mut self.0, &mut result) } {
            Ok(result)
        } else {
            Err(error_message())
        }
    }
}

pub struct ClingoPropagateInit(clingo_propagate_init_t);
impl ClingoPropagateInit {
    /// Map the given program literal or condition id to its solver literal.
    ///
    /// # Arguments
    ///
    /// * `init` - the target
    /// * `aspif_literal` - the aspif literal to map
    /// * `solver_literal` - the resulting solver literal
    ///
    /// **Returns** whether the call was successful
    pub fn solver_literal(
        &mut self,
        ClingoLiteral(aspif_literal): ClingoLiteral,
    ) -> Result<ClingoLiteral, &'static str> {

        let mut solver_literal = 0 as clingo_literal_t;
        if unsafe {
            clingo_propagate_init_solver_literal(&mut self.0, aspif_literal, &mut solver_literal)
        }
        {
            Ok(ClingoLiteral(solver_literal))
        } else {
            Err(error_message())
        }
    }

    /// Add a watch for the solver literal in the given phase.
    ///
    /// # Arguments
    ///
    /// * `init` - the target
    /// * `solver_literal` - the solver literal
    ///
    /// **Returns** whether the call was successful
    pub fn add_watch(
        &mut self,
        ClingoLiteral(solver_literal): ClingoLiteral,
    ) -> Result<(), &'static str> {
        if unsafe { clingo_propagate_init_add_watch(&mut self.0, solver_literal) } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// Get an object to inspect the symbolic atoms.
    ///
    /// # Arguments
    ///
    /// * `init` - the target
    /// * `atoms` - the resulting object
    ///
    /// **Returns** whether the call was successful
    pub fn symbolic_atoms<'a>(&mut self) -> Result<&'a mut ClingoSymbolicAtoms, &'static str> {

        let mut atoms_ptr = std::ptr::null_mut();
        if unsafe { clingo_propagate_init_symbolic_atoms(&mut self.0, &mut atoms_ptr) } {
            unsafe { (atoms_ptr as *mut ClingoSymbolicAtoms).as_mut() }
                .ok_or("Failed to dereference pointer.")
        } else {
            Err(error_message())
        }
    }

    //     pub fn clingo_propagate_init_theory_atoms(init: &mut ClingoPropagateInit,
    //                                               atoms: *mut *mut ClingoTheoryAtoms)
    //                                               -> bool;

    /// Get the number of threads used in subsequent solving.
    ///
    /// # Arguments
    ///
    /// * `init` - the target
    ///
    /// **Returns** the number of threads
    /// @see clingo_propagate_control_thread_id()
    pub fn number_of_threads(&mut self) -> usize {
        (unsafe { clingo_propagate_init_number_of_threads(&mut self.0) } as usize)
    }
}

pub struct ClingoSolveHandle(clingo_solve_handle);
impl ClingoSolveHandle {
    /// Get the next solve result.
    ///
    /// Blocks until the result is ready.
    /// When yielding partial solve results can be obtained, i.e.,
    /// when a model is ready, the result will be satisfiable but neither the search exhausted nor the optimality proven.
    ///
    /// @param[in] handle the target
    /// @param[out] result the solve result
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if solving fails
    pub fn get(&mut self) -> Result<clingo_solve_result_bitset_t, &'static str> {

        let mut result = 0;
        let suc = unsafe { clingo_solve_handle_get(&mut self.0, &mut result) };
        if suc {
            Ok(result)
        } else {
            Err(error_message())
        }
    }

    /// Get the next model (or zero if there are no more models).
    ///
    /// @param[in] handle the target
    /// @param[out] model the model (it is NULL if there are no more models)
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if solving fails

    pub fn model(&mut self) -> Result<&mut ClingoModel, &'static str> {

        let ClingoSolveHandle(ref mut handle) = *self;
        let mut model = std::ptr::null_mut() as *mut clingo_model_t;
        if unsafe { clingo_solve_handle_model(handle, &mut model) } {
            unsafe { (model as *mut ClingoModel).as_mut() }.ok_or("Failed to dereference pointer.")
        } else {
            Err(error_message())
        }
    }

    /// Discards the last model and starts the search for the next one.
    ///
    /// If the search has been started asynchronously, this function continues the search in the background.
    ///
    /// @note This function does not block.
    ///
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if solving fails

    pub fn resume(&mut self) -> Result<(), &'static str> {
        let ClingoSolveHandle(ref mut handle) = *self;
        let suc = unsafe { clingo_solve_handle_resume(handle) };
        if suc { Ok(()) } else { Err(error_message()) }
    }
    /// Stops the running search and releases the handle.
    ///
    /// Blocks until the search is stopped (as if an implicit cancel was called before the handle is released).
    ///
    /// @param[in] handle the target
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if solving fails
    pub fn close(&mut self) -> Result<(), &'static str> {
        let ClingoSolveHandle(ref mut handle) = *self;
        let suc = unsafe { clingo_solve_handle_close(handle) };
        if suc { Ok(()) } else { Err(error_message()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_test() {
        let (ma, mi, re) = version();
        assert!(ma == 5);
        assert!(mi == 2);
        assert!(re == 1);
    }
}
