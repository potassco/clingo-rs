
extern crate libc;
extern crate clingo_sys;

use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use libc::c_int;
use libc::c_char;
use clingo_sys::*;

pub use clingo_sys::{clingo_literal_t, clingo_ast_statement_t, clingo_truth_value,
                     clingo_ast_sign, clingo_solve_mode, clingo_clause_type,
                     clingo_statistics_type, clingo_ast_body_literal_type, clingo_show_type,
                     clingo_ast_literal_type, clingo_ast_term_type, clingo_ast_statement_type,
                     clingo_ast_term_type_t, clingo_solve_event_type_t, clingo_show_type_bitset_t,
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
    pub fn get_endfile(&self) -> Option<usize> {
        println!("woah:{:?}", self.end_file);
        None
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

    pub fn create_id(name: &str, positive: bool) -> Option<ClingoSymbol> {

        let mut symbol = 0 as clingo_symbol_t;
        let name_c_str = CString::new(name).unwrap();
        let err = unsafe { clingo_symbol_create_id(name_c_str.as_ptr(), positive, &mut symbol) };
        if !err {
            None
        } else {
            Some(ClingoSymbol(symbol))
        }
    }

    pub fn create_function(
        name: &str,
        arguments: &[ClingoSymbol],
        positive: bool,
    ) -> Option<ClingoSymbol> {

        let mut symbol = 0 as clingo_symbol_t;
        let name_c_str = CString::new(name).unwrap();
        let err = unsafe {
            clingo_symbol_create_function(
                name_c_str.as_ptr(),
                arguments.as_ptr() as *const clingo_symbol_t,
                arguments.len(),
                positive,
                &mut symbol,
            )
        };
        if !err {
            None
        } else {
            Some(ClingoSymbol(symbol))
        }
    }

    pub fn number(&self) -> Option<i32> {

        let mut number = 0;
        let err = unsafe { clingo_symbol_number(self.0, &mut number) };
        if !err { None } else { Some(number) }
    }

    //     pub fn clingo_symbol_name(symbol: clingo_symbol_t, name: *mut *const c_char) -> u8;
    //     pub fn clingo_symbol_string(symbol: clingo_symbol_t, string: *mut *const c_char) -> u8;
    //     pub fn clingo_symbol_is_positive(symbol: clingo_symbol_t, positive: *mut u8) -> u8;
    //     pub fn clingo_symbol_is_negative(symbol: clingo_symbol_t, negative: *mut u8) -> u8;

    pub fn arguments(&self) -> Option<Vec<ClingoSymbol>> {

        let mut symbol_ptr = std::ptr::null() as *const clingo_symbol_t;
        let mut size: usize = 0;
        let err = unsafe { clingo_symbol_arguments(self.0, &mut symbol_ptr, &mut size) };
        if !err {
            None
        } else {
            let mut symbols = Vec::<ClingoSymbol>::with_capacity(size);
            for _ in 0..size {
                let nsymbol = unsafe { *symbol_ptr };
                symbols.push(ClingoSymbol(nsymbol));
                symbol_ptr = unsafe { symbol_ptr.offset(1) };
            }
            Some(symbols)
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
        let part = ClingoPart {
            name: CString::new(name).unwrap(),
            params: params,
        };
        part
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
impl ClingoPropagator {
    pub fn new(
        init: Option<
            unsafe extern "C" fn(init: *mut clingo_propagate_init_t,
                                 data: *mut ::std::os::raw::c_void)
                                 -> bool,
        >,
        propagate: Option<
            unsafe extern "C" fn(control: *mut clingo_propagate_control_t,
                                 changes: *const clingo_literal_t,
                                 size: usize,
                                 data: *mut ::std::os::raw::c_void)
                                 -> bool,
        >,
        undo: Option<
            unsafe extern "C" fn(control: *mut clingo_propagate_control_t,
                                 changes: *const clingo_literal_t,
                                 size: usize,
                                 data: *mut ::std::os::raw::c_void)
                                 -> bool,
        >,
        check: Option<
            unsafe extern "C" fn(control: *mut clingo_propagate_control_t,
                                 data: *mut ::std::os::raw::c_void)
                                 -> bool,
        >,
    ) -> ClingoPropagator {

        let prop = clingo_propagator_t {
            init: init,
            propagate: propagate,
            undo: undo,
            check: check,
        };
        ClingoPropagator(prop)
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
    /// **Parameters:**
    ///
    /// * `control` the target
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
    ///
    /// **Note:** Parts of a logic program without an explicit <tt>\#program</tt>
    /// specification are by default put into a program called `base` without
    /// arguments.
    ///
    /// **Parameters:**
    ///
    /// * `control` the target
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
    /// **Parameters:**
    ///
    /// * `control` the target
    /// * `atom` atom to assign
    /// * `value` the truth value
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
    /// **Parameters:**
    ///
    /// * `control` the target
    /// * `propagator` the propagator
    /// * `data` user data passed to the propagator functions
    /// * `sequential` whether the propagator should be called sequentially
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
    /// **Parameters:**
    ///
    /// * `control` the target
    /// * `builder` the program builder object
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
    /// **Parameters:**
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

#[derive(Debug, Clone, Copy)]
pub struct ClingoAstBodyLiteral(clingo_ast_body_literal_t);
impl ClingoAstBodyLiteral {
    pub fn new(
        location: ClingoLocation,
        sign: clingo_ast_sign,
        type_: clingo_ast_body_literal_type,
        lit_ref: &ClingoAstLiteral,
    ) -> ClingoAstBodyLiteral {
        let _bg_union_2 = clingo_ast_body_literal__bindgen_ty_1 {
            literal: __BindgenUnionField::new(),
            conditional: __BindgenUnionField::new(),
            aggregate: __BindgenUnionField::new(),
            body_aggregate: __BindgenUnionField::new(),
            theory_atom: __BindgenUnionField::new(),
            disjoint: __BindgenUnionField::new(),
            bindgen_union_field: (lit_ref as *const ClingoAstLiteral) as u64,
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
        let _bg_union_2 = clingo_ast_statement__bindgen_ty_1 {
            rule: __BindgenUnionField::new(),
            definition: __BindgenUnionField::new(),
            show_signature: __BindgenUnionField::new(),
            show_term: __BindgenUnionField::new(),
            minimize: __BindgenUnionField::new(),
            script: __BindgenUnionField::new(),
            program: __BindgenUnionField::new(),
            external: __BindgenUnionField::new(),
            edge: __BindgenUnionField::new(),
            heuristic: __BindgenUnionField::new(),
            project_atom: __BindgenUnionField::new(),
            project_signature: __BindgenUnionField::new(),
            theory_definition: __BindgenUnionField::new(),
            bindgen_union_field: external as u64,
        };
        let stm = clingo_ast_statement_t {
            location: location.clingo_location(),
            type_: type_ as clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        ClingoAstStatement(stm)
    }

    pub fn new_rule(location: ClingoLocation, rule_: &ClingoAstRule) -> ClingoAstStatement {

        let rule: *const ClingoAstRule = rule_;

        let _bg_union_2 = clingo_ast_statement__bindgen_ty_1 {
            rule: __BindgenUnionField::new(),
            definition: __BindgenUnionField::new(),
            show_signature: __BindgenUnionField::new(),
            show_term: __BindgenUnionField::new(),
            minimize: __BindgenUnionField::new(),
            script: __BindgenUnionField::new(),
            program: __BindgenUnionField::new(),
            external: __BindgenUnionField::new(),
            edge: __BindgenUnionField::new(),
            heuristic: __BindgenUnionField::new(),
            project_atom: __BindgenUnionField::new(),
            project_signature: __BindgenUnionField::new(),
            theory_definition: __BindgenUnionField::new(),
            bindgen_union_field: rule as u64,
        };
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
        let ast_rule_ptr = stm.__bindgen_anon_1.bindgen_union_field as *const clingo_ast_rule_t;
        (ast_rule_ptr as *const ClingoAstRule).as_ref().unwrap()
    }
}

pub struct ClingoAstTerm(clingo_ast_term_t);
impl ClingoAstTerm {
    pub fn new_symbol(location: ClingoLocation, symbol_: ClingoSymbol) -> ClingoAstTerm {
        let ClingoSymbol(symbol) = symbol_;
        let _bg_union_1 = clingo_ast_term__bindgen_ty_1 {
            symbol: __BindgenUnionField::new(),
            variable: __BindgenUnionField::new(),
            unary_operation: __BindgenUnionField::new(),
            binary_operation: __BindgenUnionField::new(),
            interval: __BindgenUnionField::new(),
            function: __BindgenUnionField::new(),
            external_function: __BindgenUnionField::new(),
            pool: __BindgenUnionField::new(),
            bindgen_union_field: symbol,
        };
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
            boolean: __BindgenUnionField::new(),
            symbol: __BindgenUnionField::new(),
            comparison: __BindgenUnionField::new(),
            csp_literal: __BindgenUnionField::new(),
            bindgen_union_field: symbol as u64,
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
    pub fn root(&mut self) -> Option<ClingoId> {
        unsafe {
            let ClingoConfiguration(ref mut conf) = *self;
            let mut root_key = 0 as clingo_id_t;
            let err = clingo_configuration_root(conf, &mut root_key);
            if !err { None } else { Some(ClingoId(root_key)) }
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

    pub fn array_at(&mut self, ClingoId(key): ClingoId, offset: usize) -> Option<ClingoId> {

        let mut nkey = 0 as clingo_id_t;
        let err = unsafe { clingo_configuration_array_at(&mut self.0, key, offset, &mut nkey) };
        if !err { None } else { Some(ClingoId(nkey)) }
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
    /// @pre The @link clingo_configuration_type() type@endlink of the entry must be @ref ::clingo_configuration_type_map.
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    ///
    /// **Parameters:**
    ///
    /// * `configuration` the target configuration
    /// * `key` the key
    /// * `name` the name to lookup the subkey
    /// * `subkey` the resulting subkey
    ///
    /// **Returns** whether the call was successful
    pub fn map_at(&mut self, ClingoId(key): ClingoId, name: &str) -> Option<ClingoId> {

        let mut nkey = 0 as clingo_id_t;
        let name_c_str = CString::new(name).unwrap();
        let err = unsafe {
            clingo_configuration_map_at(&mut self.0, key, name_c_str.as_ptr(), &mut nkey)
        };
        if !err { None } else { Some(ClingoId(nkey)) }
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
    /// @pre The @link clingo_configuration_type() type@endlink of the entry must be @ref ::clingo_configuration_type_value.
    ///
    /// **Parameters:**
    ///
    /// * `configuration` the target configuration
    /// * `key` the key
    /// * `value` the value to set
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
    /// **Parameters:**
    ///
    /// * `backend` the target backend
    /// * `choice` determines if the head is a choice or a disjunction
    /// * `head` the head atoms
    /// * `head_size` the number of atoms in the head
    /// * `body` the body literals
    /// * `body_size` the number of literals in the body
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
    /// **Parameters:**
    ///
    /// * `backend` the target backend
    /// * `literals` the literals to assume (positive literals are true and negative literals false for the next solve call)
    /// * `size` the number of atoms
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
    /// **Parameters:**
    ///
    /// * `backend` the target backend
    /// * `atom` the resulting atom
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
    pub fn statistics_root(&mut self) -> Option<u64> {

        let mut root_key = 0 as u64;
        let err = unsafe { clingo_statistics_root(&mut self.0, &mut root_key) };
        if !err { None } else { Some(root_key) }
    }

    pub fn statistics_type(&mut self, key: u64) -> Option<clingo_statistics_type> {

        let mut stype = 0 as clingo_statistics_type_t;
        let err = unsafe { clingo_statistics_type(&mut self.0, key, &mut stype) };
        if !err {
            None
        } else {
            match stype {

                0 => Some(clingo_statistics_type::clingo_statistics_type_empty),
                1 => Some(clingo_statistics_type::clingo_statistics_type_value),
                2 => Some(clingo_statistics_type::clingo_statistics_type_array),
                _ => Some(clingo_statistics_type::clingo_statistics_type_map), 
            }
        }
    }

    pub fn statistics_array_size(&mut self, key: u64) -> Option<usize> {

        let mut size = 0 as usize;
        let err = unsafe { clingo_statistics_array_size(&mut self.0, key, &mut size) };
        if !err { None } else { Some(size) }
    }

    pub fn statistics_array_at(&mut self, key: u64, offset: usize) -> Option<u64> {

        let mut subkey = 0 as u64;
        let err = unsafe { clingo_statistics_array_at(&mut self.0, key, offset, &mut subkey) };
        if !err { None } else { Some(subkey) }
    }

    pub fn statistics_map_size(&mut self, key: u64) -> Option<usize> {

        let mut size = 0 as usize;
        let err = unsafe { clingo_statistics_map_size(&mut self.0, key, &mut size) };
        if !err { None } else { Some(size) }
    }

    pub fn statistics_map_subkey_name<'a>(&mut self, key: u64, offset: usize) -> Option<&'a str> {

        let mut name = std::ptr::null() as *const c_char;
        let err = unsafe { clingo_statistics_map_subkey_name(&mut self.0, key, offset, &mut name) };
        if !err {
            None
        } else {
            Some(unsafe { CStr::from_ptr(name) }.to_str().unwrap())
        }
    }

    pub fn statistics_map_at(&mut self, key: u64, name: &str) -> Option<u64> {

        let mut subkey = 0 as u64;
        let name_c_str = CString::new(name).unwrap();
        let err =
            unsafe { clingo_statistics_map_at(&mut self.0, key, name_c_str.as_ptr(), &mut subkey) };
        if !err { None } else { Some(subkey) }
    }

    pub fn statistics_value_get(&mut self, key: u64) -> Option<f64> {

        let mut value = 0.0 as f64;
        let err = unsafe { clingo_statistics_value_get(&mut self.0, key, &mut value) };
        if !err { None } else { Some(value) }
    }
}

pub struct ClingoSignature(clingo_signature_t);
impl ClingoSignature {
    pub fn create(name_: &str, arity: u32, positive: bool) -> Option<ClingoSignature> {
        let name_c_str = CString::new(name_).unwrap();
        let mut signature = 0;
        let err = unsafe {
            clingo_signature_create(name_c_str.as_ptr(), arity, positive, &mut signature)
        };
        if !err {
            None
        } else {
            Some(ClingoSignature(signature))
        }
    }
}

#[derive(Debug)]
pub struct ClingoSymbolicAtoms(clingo_symbolic_atoms_t);
impl ClingoSymbolicAtoms {
    pub fn begin(
        &mut self,
        opt_sig: Option<&ClingoSignature>,
    ) -> Option<clingo_symbolic_atom_iterator_t> {

        match opt_sig {
            Some(sig) => {
                let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
                let err =
                    unsafe { clingo_symbolic_atoms_begin(&mut self.0, &sig.0, &mut iterator) };
                if !err { None } else { Some(iterator) }
            }
            None => {
                let signature = std::ptr::null();
                let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
                let err =
                    unsafe { clingo_symbolic_atoms_begin(&mut self.0, signature, &mut iterator) };
                if !err { None } else { Some(iterator) }
            }
        }
    }

    pub fn end(&mut self) -> Option<clingo_symbolic_atom_iterator_t> {

        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        let err = unsafe { clingo_symbolic_atoms_end(&mut self.0, &mut iterator) };
        if !err { None } else { Some(iterator) }
    }

    pub fn find(
        &mut self,
        ClingoSymbol(symbol): ClingoSymbol,
    ) -> Option<clingo_symbolic_atom_iterator_t> {

        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        let err = unsafe { clingo_symbolic_atoms_find(&mut self.0, symbol, &mut iterator) };
        if !err { None } else { Some(iterator) }
    }

    pub fn iterator_is_equal_to(
        &mut self,
        a: clingo_symbolic_atom_iterator_t,
        b: clingo_symbolic_atom_iterator_t,
    ) -> Option<bool> {

        let mut equal = false;
        let err =
            unsafe { clingo_symbolic_atoms_iterator_is_equal_to(&mut self.0, a, b, &mut equal) };
        if !err { None } else { Some(equal) }
    }

    pub fn symbol(&mut self, iterator: clingo_symbolic_atom_iterator_t) -> Option<ClingoSymbol> {

        let mut symbol = 0 as clingo_symbol_t;
        let err = unsafe { clingo_symbolic_atoms_symbol(&mut self.0, iterator, &mut symbol) };
        if !err {
            None
        } else {
            Some(ClingoSymbol(symbol))
        }
    }

    pub fn is_fact(&mut self, iterator: clingo_symbolic_atom_iterator_t) -> Option<bool> {

        let mut fact = false;
        let err = unsafe { clingo_symbolic_atoms_is_fact(&mut self.0, iterator, &mut fact) };
        if !err { None } else { Some(fact) }
    }

    pub fn is_external(&mut self, iterator: clingo_symbolic_atom_iterator_t) -> Option<bool> {

        let mut external = false;
        let err =
            unsafe { clingo_symbolic_atoms_is_external(&mut self.0, iterator, &mut external) };
        if !err { None } else { Some(external) }
    }

    pub fn literal(&mut self, iterator: clingo_symbolic_atom_iterator_t) -> Option<ClingoLiteral> {

        let mut literal = 0 as clingo_literal_t;
        let err = unsafe { clingo_symbolic_atoms_literal(&mut self.0, iterator, &mut literal) };
        if !err {
            None
        } else {
            Some(ClingoLiteral(literal))
        }
    }

    //     pub fn clingo_symbolic_atoms_signatures_size(atoms: *mut ClingoSymbolicAtoms,
    //                                                  size: *mut size_t)
    //                                                  -> u8;

    //     pub fn clingo_symbolic_atoms_signatures(atoms: *mut ClingoSymbolicAtoms,
    //                                             signatures: *mut clingo_signature_t,
    //                                             size: size_t)
    //                                             -> u8;

    pub fn next(
        &mut self,
        iterator: clingo_symbolic_atom_iterator_t,
    ) -> Option<clingo_symbolic_atom_iterator_t> {

        let mut next = 0 as clingo_symbolic_atom_iterator_t;
        let err = unsafe { clingo_symbolic_atoms_next(&mut self.0, iterator, &mut next) };
        if !err { None } else { Some(next) }
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
    pub fn term_name<'a>(&mut self, ClingoId(term): ClingoId) -> Option<&'a str> {

        let mut char_ptr = std::ptr::null() as *const c_char;
        let err = unsafe { clingo_theory_atoms_term_name(&mut self.0, term, &mut char_ptr) };
        if !err {
            None
        } else {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Some(c_str.to_str().unwrap())
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

    pub fn size(&mut self) -> Option<usize> {

        let mut size = 0 as usize;
        let err = unsafe { clingo_theory_atoms_size(&mut self.0, &mut size) };
        if !err { None } else { Some(size) }
    }

    pub fn atom_term(&mut self, ClingoId(atom): ClingoId) -> Option<ClingoId> {

        let mut term = 0 as clingo_id_t;
        let err = unsafe { clingo_theory_atoms_atom_term(&mut self.0, atom, &mut term) };
        if !err { None } else { Some(ClingoId(term)) }
    }

    //     pub fn clingo_theory_atoms_atom_elements(atoms: *mut ClingoTheoryAtoms,
    //                                              atom: clingo_id_t,
    //                                              elements: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;

    pub fn atom_has_guard(&mut self, ClingoId(atom): ClingoId) -> Option<bool> {

        let mut has_guard = false;
        let err = unsafe { clingo_theory_atoms_atom_has_guard(&mut self.0, atom, &mut has_guard) };
        if !err { None } else { Some(has_guard) }
    }

    //     pub fn clingo_theory_atoms_atom_guard(atoms: *mut ClingoTheoryAtoms,
    //                                           atom: clingo_id_t,
    //                                           connective: *mut *const c_char,
    //                                           term: *mut clingo_id_t)
    //                                           -> u8;

    pub fn atom_literal(&mut self, ClingoId(atom): ClingoId) -> Option<ClingoLiteral> {

        let mut literal = 0 as clingo_literal_t;
        let err = unsafe { clingo_theory_atoms_atom_literal(&mut self.0, atom, &mut literal) };
        if !err {
            None
        } else {
            Some(ClingoLiteral(literal))
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
    pub fn model_type(&mut self) -> Option<clingo_model_type_t> {

        let mut mtype = 0 as clingo_model_type_t;
        let err = unsafe { clingo_model_type(&mut self.0, &mut mtype) };
        if !err { None } else { Some(mtype) }
    }

    pub fn number(&mut self) -> Option<u64> {

        let mut number = 0;
        let err = unsafe { clingo_model_number(&mut self.0, &mut number) };
        if !err { None } else { Some(number) }
    }

    //     pub fn clingo_model_symbols_size(model: *mut ClingoModel,
    //                                      show: clingo_show_type_bitset_t,
    //                                      size: *mut size_t)
    //                                      -> u8;

    pub fn symbols(&mut self, show: clingo_show_type_bitset_t) -> Option<Vec<ClingoSymbol>> {
        let ClingoModel(ref mut model) = *self;
        let mut size: usize = 0;
        let size_p = &mut size as *mut usize;

        let err = unsafe { clingo_model_symbols_size(model, show, size_p) };
        if !err {
            None
        } else {
            let mut symbols = Vec::<ClingoSymbol>::with_capacity(size);
            let symbols_ptr = symbols.as_ptr();
            let err = unsafe {
                clingo_model_symbols(model, show, symbols_ptr as *mut clingo_symbol_t, size)
            };
            if !err {
                None
            } else {
                symbols =
                    unsafe { Vec::from_raw_parts(symbols_ptr as *mut ClingoSymbol, size, size) };
                Some(symbols)
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

pub struct ClingoSolveControl(clingo_solve_control_t);
impl ClingoSolveControl {
    /// Add a clause that applies to the current solving step during model
    /// enumeration.
    ///
    ///
    /// **Note:** The @ref Propagator module provides a more sophisticated
    /// interface to add clauses - even on partial assignments.
    ///
    /// **Parameters:**
    ///
    /// * `control` the target
    /// * `clause` array of literals representing the clause
    /// * `size` the size of the literal array
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    /// - ::clingo_error_runtime if adding the clause fails
    pub fn add_clause(&mut self, clause: &[ClingoLiteral]) -> Result<(), &'static str> {

        let size = mem::size_of_val(clause);
        if unsafe {
            clingo_solve_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                size,
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
    /// **Parameters:**
    ///
    /// * `control` the target
    /// * `clause` the clause to add
    /// * `size` the size of the clause
    /// * `type` the clause type determining its lifetime
    /// * `result` result indicating whether propagation has to be stopped
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_bad_alloc
    pub fn add_clause(
        &mut self,
        clause: &[ClingoLiteral],
        type_: clingo_clause_type,
    ) -> Result<bool, &'static str> {
        let size = mem::size_of_val(clause);
        let mut result = false;
        if unsafe {
            clingo_propagate_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                size,
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
    /// **Parameters:**
    ///
    /// * `control` the target
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
    pub fn solver_literal(
        &mut self,
        ClingoLiteral(aspif_literal): ClingoLiteral,
    ) -> Option<ClingoLiteral> {

        // let solver_literal = std::ptr::null();
        let mut solver_literal = 0 as clingo_literal_t;
        let err = unsafe {
            clingo_propagate_init_solver_literal(&mut self.0, aspif_literal, &mut solver_literal)
        };
        if !err {
            None
        } else {
            // let lit = unsafe { *solver_literal };
            Some(ClingoLiteral(solver_literal))
        }
    }

    /// Add a watch for the solver literal in the given phase.
    ///
    /// **Parameters:**
    ///
    /// * `init` the target
    /// * `solver_literal` the solver literal
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

    pub fn symbolic_atoms<'a>(&mut self) -> Option<&'a mut ClingoSymbolicAtoms> {

        let mut atoms_ptr = std::ptr::null_mut();
        let err = unsafe { clingo_propagate_init_symbolic_atoms(&mut self.0, &mut atoms_ptr) };
        if !err {
            None
        } else {
            unsafe { (atoms_ptr as *mut ClingoSymbolicAtoms).as_mut() }
        }
    }

    //     pub fn clingo_propagate_init_theory_atoms(init: &mut ClingoPropagateInit,
    //                                               atoms: *mut *mut ClingoTheoryAtoms)
    //                                               -> bool;

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

    pub fn model(&mut self) -> Option<&mut ClingoModel> {

        let ClingoSolveHandle(ref mut handle) = *self;
        let mut model = std::ptr::null_mut() as *mut clingo_model_t;
        let err = unsafe { clingo_solve_handle_model(handle, &mut model) };
        if !err {
            None
        } else {
            unsafe { (model as *mut ClingoModel).as_mut() }
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
        assert!(re == 0);
    }
}
