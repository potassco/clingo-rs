#![feature(ptr_internals)]
extern crate clingo_sys;
extern crate libc;
use std::mem;
use std::ptr::Unique;

use std::ffi::CStr;
use std::ffi::CString;
use libc::c_int;
use libc::c_char;
use clingo_sys::*;

pub use clingo_sys::{clingo_ast_statement_t, clingo_ast_term_type_t, clingo_logger_t};
pub use clingo_sys::clingo_show_type_bitset_t;
pub use clingo_sys::clingo_solve_mode_bitset_t;

#[derive(Debug, Copy, Clone)]
pub enum ClingoError {
    Success = clingo_error_clingo_error_success as isize,
    Runtime = clingo_error_clingo_error_runtime as isize,
    Logic = clingo_error_clingo_error_logic as isize,
    BadAlloc = clingo_error_clingo_error_bad_alloc as isize,
    Unknown = clingo_error_clingo_error_unknown as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoSolveMode {
    Async = clingo_solve_mode_clingo_solve_mode_async as isize,
    Yield = clingo_solve_mode_clingo_solve_mode_yield as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoShowType {
    CSP = clingo_show_type_clingo_show_type_csp as isize,
    Shown = clingo_show_type_clingo_show_type_shown as isize,
    Atoms = clingo_show_type_clingo_show_type_atoms as isize,
    Terms = clingo_show_type_clingo_show_type_terms as isize,
    Extra = clingo_show_type_clingo_show_type_extra as isize,
    All = clingo_show_type_clingo_show_type_all as isize,
    Complement = clingo_show_type_clingo_show_type_complement as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoTruthValue {
    Free = clingo_truth_value_clingo_truth_value_free as isize,
    True = clingo_truth_value_clingo_truth_value_true as isize,
    False = clingo_truth_value_clingo_truth_value_false as isize,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClingoAstStatementType {
    Rule = clingo_ast_statement_type_clingo_ast_statement_type_rule as isize,
    Const = clingo_ast_statement_type_clingo_ast_statement_type_const as isize,
    ShowSignature = clingo_ast_statement_type_clingo_ast_statement_type_show_signature as isize,
    ShowTerm = clingo_ast_statement_type_clingo_ast_statement_type_show_term as isize,
    Minimize = clingo_ast_statement_type_clingo_ast_statement_type_minimize as isize,
    Script = clingo_ast_statement_type_clingo_ast_statement_type_script as isize,
    Program = clingo_ast_statement_type_clingo_ast_statement_type_program as isize,
    External = clingo_ast_statement_type_clingo_ast_statement_type_external as isize,
    Edge = clingo_ast_statement_type_clingo_ast_statement_type_edge as isize,
    Heuristic = clingo_ast_statement_type_clingo_ast_statement_type_heuristic as isize,
    ProjectAtom = clingo_ast_statement_type_clingo_ast_statement_type_project_atom as isize,
    ProjectAtomSignature =
        clingo_ast_statement_type_clingo_ast_statement_type_project_atom_signature as isize,
    TheoryDefinition =
        clingo_ast_statement_type_clingo_ast_statement_type_theory_definition as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoAstSign {
    None = clingo_ast_sign_clingo_ast_sign_none as isize,
    Negation = clingo_ast_sign_clingo_ast_sign_negation as isize,
    DoubleNegation = clingo_ast_sign_clingo_ast_sign_double_negation as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoAstLiteralType {
    Boolean = clingo_ast_literal_type_clingo_ast_literal_type_boolean as isize,
    Symbolic = clingo_ast_literal_type_clingo_ast_literal_type_symbolic as isize,
    Comparison = clingo_ast_literal_type_clingo_ast_literal_type_comparison as isize,
    CSP = clingo_ast_literal_type_clingo_ast_literal_type_csp as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoAstBodyLiteralType {
    Literal = clingo_ast_body_literal_type_clingo_ast_body_literal_type_literal as isize,
    Conditional = clingo_ast_body_literal_type_clingo_ast_body_literal_type_conditional as isize,
    Aggregate = clingo_ast_body_literal_type_clingo_ast_body_literal_type_aggregate as isize,
    BodyAggregate =
        clingo_ast_body_literal_type_clingo_ast_body_literal_type_body_aggregate as isize,
    TheoryAtom = clingo_ast_body_literal_type_clingo_ast_body_literal_type_theory_atom as isize,
    Disjoint = clingo_ast_body_literal_type_clingo_ast_body_literal_type_disjoint as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoClauseType {
    Learnt = clingo_clause_type_clingo_clause_type_learnt as isize,
    Static = clingo_clause_type_clingo_clause_type_static as isize,
    Volatile = clingo_clause_type_clingo_clause_type_volatile as isize,
    VolatileStatic = clingo_clause_type_clingo_clause_type_volatile_static as isize,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClingoSolveEventType {
    Model = clingo_solve_event_type_clingo_solve_event_type_model as isize,
    Finish = clingo_solve_event_type_clingo_solve_event_type_finish as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoStatisticsType {
    Empty = clingo_statistics_type_clingo_statistics_type_empty as isize,
    Value = clingo_statistics_type_clingo_statistics_type_value as isize,
    Array = clingo_statistics_type_clingo_statistics_type_array as isize,
    Map = clingo_statistics_type_clingo_statistics_type_map as isize,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ClingoSymbolType {
    Infimum = clingo_symbol_type_clingo_symbol_type_infimum as isize,
    Number = clingo_symbol_type_clingo_symbol_type_number as isize,
    String = clingo_symbol_type_clingo_symbol_type_string as isize,
    Function = clingo_symbol_type_clingo_symbol_type_function as isize,
    Supremum = clingo_symbol_type_clingo_symbol_type_supremum as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoWarning {
    OperationUndefined = clingo_warning_clingo_warning_operation_undefined as isize,
    RuntimeError = clingo_warning_clingo_warning_runtime_error as isize,
    AtomUndefined = clingo_warning_clingo_warning_atom_undefined as isize,
    FileIncluded = clingo_warning_clingo_warning_file_included as isize,
    VariableUnbound = clingo_warning_clingo_warning_variable_unbounded as isize,
    GlobalVariable = clingo_warning_clingo_warning_global_variable as isize,
    Other = clingo_warning_clingo_warning_other as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClingoConfigurationType {
    Value = clingo_configuration_type_clingo_configuration_type_value as isize,
    Array = clingo_configuration_type_clingo_configuration_type_array as isize,
    Map = clingo_configuration_type_clingo_configuration_type_map as isize,
}
type ClingoSolveEventCallback = unsafe extern "C" fn(
    type_: clingo_solve_event_type_t,
    event: *mut ::std::os::raw::c_void,
    data: *mut ::std::os::raw::c_void,
    goon: *mut bool,
) -> bool;
pub trait ClingoSolveEventHandler<T> {
    fn on_solve_event(type_: ClingoSolveEventType, data: &mut T, goon: &mut bool) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_solve_callback(
        type_: clingo_solve_event_type_t,
        event: *mut ::std::os::raw::c_void,
        data_: *mut ::std::os::raw::c_void,
        goon_: *mut bool,
    ) -> bool {
        // TODO               assert!(!event.is_null());
        assert!(!data_.is_null());
        assert!(!goon_.is_null());
        let event_type = match type_ {
            clingo_solve_event_type_clingo_solve_event_type_model => ClingoSolveEventType::Model,
            clingo_solve_event_type_clingo_solve_event_type_finish => ClingoSolveEventType::Finish,
            _ => panic!("Rust binding failed to match clingo solve event type"),
        };
        let data = (data_ as *mut T).as_mut().unwrap();
        let goon = goon_.as_mut().unwrap();
        Self::on_solve_event(event_type, data, goon)
    }
}

type ClingoAstCallback =
    unsafe extern "C" fn(arg1: *const clingo_ast_statement_t, arg2: *mut ::std::os::raw::c_void)
        -> bool;
pub trait ClingoAstStatementHandler<T> {
    fn on_statement(arg1: &ClingoAstStatement, arg2: &mut T) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_ast_callback(
        stm_: *const clingo_ast_statement_t,
        data_: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!stm_.is_null());
        assert!(!data_.is_null());
        let stm = (stm_ as *const ClingoAstStatement).as_ref().unwrap();
        let data = (data_ as *mut T).as_mut().unwrap();
        Self::on_statement(stm, data)
    }
}

type ClingoLoggingCallback = unsafe extern "C" fn(
    code: clingo_warning_t,
    message: *const ::std::os::raw::c_char,
    data: *mut ::std::os::raw::c_void,
);
pub trait ClingoLogger<T> {
    fn log(code: ClingoWarning, message: &str, data: &mut T);
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_logging_callback(
        code_: clingo_warning_t,
        message_: *const ::std::os::raw::c_char,
        data_: *mut ::std::os::raw::c_void,
    ) {
        assert!(!message_.is_null());
        assert!(!data_.is_null());
        let warning = match code_ as u32 {
            clingo_warning_clingo_warning_atom_undefined => ClingoWarning::AtomUndefined,
            clingo_warning_clingo_warning_file_included => ClingoWarning::FileIncluded,
            clingo_warning_clingo_warning_global_variable => ClingoWarning::GlobalVariable,
            clingo_warning_clingo_warning_operation_undefined => ClingoWarning::OperationUndefined,
            clingo_warning_clingo_warning_other => ClingoWarning::Other,
            clingo_warning_clingo_warning_runtime_error => ClingoWarning::RuntimeError,
            clingo_warning_clingo_warning_variable_unbounded => ClingoWarning::VariableUnbound,
            _ => panic!("Rust binding failed to match clingo warning"),
        };
        let c_str = CStr::from_ptr(message_);
        let message = c_str.to_str().unwrap();
        let data = (data_ as *mut T).as_mut().unwrap();
        Self::log(warning, message, data)
    }
}

type ClingoGroundCallback = unsafe extern "C" fn(
    location: *const clingo_location_t,
    name: *const ::std::os::raw::c_char,
    arguments: *const clingo_symbol_t,
    arguments_size: usize,
    data: *mut ::std::os::raw::c_void,
    symbol_callback: clingo_symbol_callback_t,
    symbol_callback_data: *mut ::std::os::raw::c_void,
) -> bool;
pub trait ClingoGroundEventHandler<T> {
    fn on_ground_event(
        location: &ClingoLocation,
        name: &str,
        arguments: &[ClingoSymbol],
        data: &mut T,
        symbol_callback: clingo_symbol_callback_t,
        symbol_callback_data: *mut ::std::os::raw::c_void,
    ) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_ground_callback(
        location_: *const clingo_location_t,
        name_: *const ::std::os::raw::c_char,
        arguments_: *const clingo_symbol_t,
        arguments_size: usize,
        data_: *mut ::std::os::raw::c_void,
        symbol_callback: clingo_symbol_callback_t,
        symbol_callback_data: *mut ::std::os::raw::c_void,
        //TODO wrap symbol call back
    ) -> bool {
        assert!(!location_.is_null());
        assert!(!name_.is_null());
        assert!(!arguments_.is_null());
        assert!(!data_.is_null());
        assert!(!symbol_callback_data.is_null());

        let location = (location_ as *const ClingoLocation).as_ref().unwrap();
        let c_str = CStr::from_ptr(name_);
        let name = c_str.to_str().unwrap();
        let arguments =
            std::slice::from_raw_parts(arguments_ as *const ClingoSymbol, arguments_size);
        let data = (data_ as *mut T).as_mut().unwrap();
        Self::on_ground_event(
            location,
            name,
            arguments,
            data,
            symbol_callback,
            symbol_callback_data,
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ClingoSymbolicLiteral(clingo_symbolic_literal_t);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ClingoLiteral(clingo_literal_t);
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
#[derive(Debug, Copy, Clone)]
pub struct ClingoLocation(clingo_location);

#[derive(Debug, Clone)]
pub struct ClingoSymbol(clingo_symbol_t);
impl PartialEq for ClingoSymbol {
    fn eq(&self, other: &ClingoSymbol) -> bool {
        unsafe { clingo_symbol_is_equal_to(self.0, other.0) }
    }
}
impl Eq for ClingoSymbol {}

/// Construct a symbol representing a number.
// TODO replace c_int with u32 ?
pub fn create_number(number: c_int) -> ClingoSymbol {
    let mut symbol = 0 as clingo_symbol_t;
    unsafe { clingo_symbol_create_number(number, &mut symbol) };
    ClingoSymbol(symbol)
}

/// Construct a symbol representing \#sup.
pub fn create_supremum() -> ClingoSymbol {
    let mut symbol = 0 as clingo_symbol_t;
    unsafe { clingo_symbol_create_supremum(&mut symbol) };
    ClingoSymbol(symbol)
}

/// Construct a symbol representing \#inf<
pub fn create_infimum() -> ClingoSymbol {
    let mut symbol = 0 as clingo_symbol_t;
    unsafe { clingo_symbol_create_infimum(&mut symbol) };
    ClingoSymbol(symbol)
}

/// Construct a symbol representing a string.
///
/// #  Errors:
///
/// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
pub fn create_string(string: &str) -> Result<ClingoSymbol, &'static str> {
    let mut symbol = 0 as clingo_symbol_t;
    let c_str = CString::new(string).unwrap();
    if unsafe { clingo_symbol_create_string(c_str.as_ptr(), &mut symbol) } {
        Ok(ClingoSymbol(symbol))
    } else {
        Err(error_message())
    }
}

/// Construct a symbol representing an id.
///
/// **Note:** This is just a shortcut for `create_function()` with
/// empty arguments.
///
/// # Arguments
///
/// * `name` - the name of the symbol
/// * `positive` - whether the symbol has a classical negation sign
///
/// # Errors
///
/// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
pub fn create_id(name: &str, positive: bool) -> Result<ClingoSymbol, &'static str> {
    let mut symbol = 0 as clingo_symbol_t;
    let name_c_str = CString::new(name).unwrap();
    if unsafe { clingo_symbol_create_id(name_c_str.as_ptr(), positive, &mut symbol) } {
        //             println!("create ClingoSymbol! sym {} {:?}", symbol, name_c_str);
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
///
/// # Errors
///
/// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
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
    } {
        Ok(ClingoSymbol(symbol))
    } else {
        Err(error_message())
    }
}

impl ClingoSymbol {
    /// Get the number of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if symbol is not of type `ClingoSymbolType::Number`
    pub fn number(&self) -> Result<i32, &'static str> {
        let mut number = 0;
        if unsafe { clingo_symbol_number(self.0, &mut number) } {
            Ok(number)
        } else {
            Err(error_message())
        }
    }

    /// Get the name of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if symbol is not of type `ClingoSymbolType::Function`
    pub fn name(&self) -> Result<&str, &'static str> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_symbol_name(self.0, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Ok(c_str.to_str().unwrap())
        } else {
            Err(error_message())
        }
    }

    /// Get the string of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if symbol is not of type `ClingoSymbolType::String`
    pub fn string(&self) -> Result<&str, &'static str> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_symbol_string(self.0, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Ok(c_str.to_str().unwrap())
        } else {
            Err(error_message())
        }
    }

    /// Check if a function is positive (does not have a sign).
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if symbol is not of type `ClingoSymbolType::Function`
    pub fn is_positive(&self) -> Result<bool, &'static str> {
        let mut positive = false;
        if unsafe { clingo_symbol_is_positive(self.0, &mut positive) } {
            Ok(positive)
        } else {
            Err(error_message())
        }
    }

    /// Check if a function is negative (has a sign).
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if symbol is not of type `ClingoSymbolType::Function`
    pub fn is_negative(&self) -> Result<bool, &'static str> {
        let mut negative = false;
        if unsafe { clingo_symbol_is_negative(self.0, &mut negative) } {
            Ok(negative)
        } else {
            Err(error_message())
        }
    }

    /// Get the arguments of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if symbol is not of type `ClingoSymbolType::Function`
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

    /// Get the type of a symbol.
    ///
    /// # Errors
    ///
    /// - may failed to match clingo symbol type
    //TODO maybe unnecesary function in Rust API?
    pub fn get_type(&self) -> Result<ClingoSymbolType, &'static str> {
        let stype = unsafe { clingo_symbol_type(self.0) } as u32;
        match stype {
            clingo_symbol_type_clingo_symbol_type_infimum => Ok(ClingoSymbolType::Infimum),
            clingo_symbol_type_clingo_symbol_type_number => Ok(ClingoSymbolType::Number),
            clingo_symbol_type_clingo_symbol_type_string => Ok(ClingoSymbolType::String),
            clingo_symbol_type_clingo_symbol_type_function => Ok(ClingoSymbolType::Function),
            clingo_symbol_type_clingo_symbol_type_supremum => Ok(ClingoSymbolType::Supremum),
            _ => Err("Rust binding failed to match clingo symbol type"),
        }
    }

    /// Get the string representation of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    ///
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

    /// Check if a symbol is less than another symbol.
    ///
    /// Symbols are first compared by type.  If the types are equal, the values are
    /// compared (where strings are compared using strcmp).  Functions are first
    /// compared by signature and then lexicographically by arguments.
    ///
    /// # Arguments
    ///
    /// * `a` - first symbol
    /// * `b` - second symbol
    ///
    /// **Returns** whether a < b
    pub fn is_less_than(&self, other: &ClingoSymbol) -> bool {
        unsafe { clingo_symbol_is_less_than(self.0, other.0) }
    }

    /// Calculate a hash code of a symbol.
    pub fn hash(&self) -> usize {
        unsafe { clingo_symbol_hash(self.0) }
    }
}

// struct MaLogger;
// impl ClingoLogger<u32> for MaLogger {
//
//     fn log(code: ClingoWarning, message: &str, data: &mut u32){
//         println!("log: {}",message);
//         println!("warn: {:?}",code);
//     }
// }

/// Parse the given program and return an abstract syntax tree for each statement via a callback.
///
/// # Arguments
///
/// * `program` - the program in gringo syntax
/// * `callback` - the callback reporting statements
/// * `callback_data` - user data for the callback
///
/// # Errors
///
/// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if parsing fails
/// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
pub fn parse_program<D, T: ClingoAstStatementHandler<D>>(
    program_: &str,
    _callback: &T,
    callback_data: &mut D,
) -> Result<(), &'static str> {
    let logger = None;
    //         let logger = Some(MaLogger::unsafe_logging_callback as ClingoLoggingCallback);
    let logger_data = std::ptr::null_mut();
    let program = CString::new(program_).unwrap();
    let data = callback_data as *mut D;
    if unsafe {
        clingo_parse_program(
            program.as_ptr(),
            Some(T::unsafe_ast_callback as ClingoAstCallback),
            data as *mut ::std::os::raw::c_void,
            logger,
            logger_data,
            0,
        )
    } {
        Ok(())
    } else {
        Err(error_message())
    }
}

/// Parse the given program and return an abstract syntax tree for each statement via a callback.
///
/// # Arguments
///
/// * `program` - the program in gringo syntax
/// * `callback` - the callback reporting statements
/// * `callback_data` - user data for the callback
/// * `logger` - callback to report messages during parsing
/// * `logger_data` - user data for the logger
/// * `message_limit` - the maximum number of times the logger is called
///
/// # Errors
///
/// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if parsing fails
/// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
pub fn parse_program_with_logger<CD, C: ClingoAstStatementHandler<CD>, LD, L: ClingoLogger<LD>>(
    program_: &str,
    _callback: &C,
    cdata_: &mut CD,
    _logger: &L,
    ldata_: &mut LD,
    message_limit: u32,
) -> Result<(), &'static str> {
    let callback_data = cdata_ as *mut CD;
    let logger_data = ldata_ as *mut LD;
    let program = CString::new(program_).unwrap();
    if unsafe {
        clingo_parse_program(
            program.as_ptr(),
            Some(C::unsafe_ast_callback as ClingoAstCallback),
            callback_data as *mut ::std::os::raw::c_void,
            Some(L::unsafe_logging_callback as ClingoLoggingCallback),
            logger_data as *mut ::std::os::raw::c_void,
            message_limit,
        )
    } {
        Ok(())
    } else {
        Err(error_message())
    }
}
pub fn create_clingo_location(
    begin_line: usize,
    end_line: usize,
    begin_column: usize,
    end_column: usize,
    begin_file_: &str,
    end_file_: &str,
) -> ClingoLocation {
    let begin_file = CString::new(begin_file_).unwrap();
    let end_file = CString::new(end_file_).unwrap();
    let loc = clingo_location {
        begin_line: begin_line,
        end_line: end_line,
        begin_column: begin_column,
        end_column: end_column,
        begin_file: begin_file.as_ptr(),
        end_file: end_file.as_ptr(),
    };
    ClingoLocation(loc)
}

/// Obtain the clingo version.
///
/// `(major version, minor version, revision number)`
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
impl<'a> ClingoPart<'a> {
    pub fn new(name: &str, params: &'a [ClingoSymbol]) -> ClingoPart<'a> {
        ClingoPart {
            name: CString::new(name).unwrap(),
            params: params,
        }
    }

    fn from(&self) -> clingo_part {
        clingo_part {
            name: self.name.as_ptr(),
            params: self.params.as_ptr() as *const clingo_symbol_t,
            size: self.params.len(),
        }
    }
}
/// Get the last error code set by a clingo API call.
///
/// **Note:** Each thread has its own local error code.
pub fn error() -> ClingoError {
    let code = unsafe { clingo_error_code() };
    match code as u32 {
        clingo_error_clingo_error_success => ClingoError::Success,
        clingo_error_clingo_error_runtime => ClingoError::Runtime,
        clingo_error_clingo_error_logic => ClingoError::Logic,
        clingo_error_clingo_error_bad_alloc => ClingoError::BadAlloc,
        clingo_error_clingo_error_unknown => ClingoError::Unknown,
        _ => panic!("Rust binding failed to match clingo error"),
    }
}

/// Get the last error message set if an API call fails.
///
/// **Note:** Each thread has its own local error message.
pub fn error_message() -> &'static str {
    let char_ptr: *const c_char = unsafe { clingo_error_message() };
    if char_ptr.is_null() {
        ""
    } else {
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        c_str.to_str().unwrap()
    }
}

/// Set a custom error code and message in the active thread.
pub fn set_error(code: ClingoError, message: &str) {
    let message_c_str = CString::new(message).unwrap();
    unsafe { clingo_set_error(code as clingo_error_t, message_c_str.as_ptr()) }
}

pub struct ClingoPropagator(clingo_propagator_t);

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
        assert!(!init_.is_null());
        assert!(!data.is_null());
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
        assert!(!control_.is_null());
        assert!(!changes_.is_null());
        assert!(!data.is_null());
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
        assert!(!control_.is_null());
        assert!(!changes_.is_null());
        assert!(!data.is_null());
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
        assert!(!control_.is_null());
        assert!(!data.is_null());
        let control = (control_ as *mut ClingoPropagateControl).as_mut().unwrap();
        let propagator = (data as *mut T).as_mut().unwrap();
        Self::check(control, propagator)
    }
}

#[derive(Debug)]
pub struct ClingoControl {
    ctl: Unique<clingo_control_t>,
}
impl Drop for ClingoControl {
    fn drop(&mut self) {
        //         println!("drop ClingoControl");
        unsafe { clingo_control_free(self.ctl.as_ptr()) }
    }
}
impl ClingoControl {
    /// Create a new control object.
    ///
    /// **Note:** Only gringo options (without <code>\-\-output</code>) and clasp`s options are supported as arguments,
    /// except basic options such as <code>\-\-help</code>.
    /// Furthermore, a control object is blocked while a search call is active;
    /// you must not call any member function during search.
    ///
    /// Messages are printed to stderr.
    ///
    /// # Arguments
    ///
    /// * `arguments` - string array of command line arguments
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if argument parsing fails
    pub fn new(
        arguments: std::vec::Vec<String>,
        message_limit: u32,
    ) -> Result<ClingoControl, &'static str> {
        let logger = None;
        let logger_data = std::ptr::null_mut();

        // create a vector of zero terminated strings
        let mut args: Vec<CString> = Vec::new();
        for arg in arguments {
            args.push(CString::new(arg).unwrap());
        }

        // convert the strings to raw pointers
        let c_args = args.iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let mut ctl = unsafe { mem::uninitialized() };

        if unsafe {
            clingo_control_new(
                c_args.as_ptr(),
                c_args.len(),
                logger,
                logger_data,
                message_limit,
                &mut ctl,
            )
        } {
            Ok(ClingoControl {
                ctl: Unique::new(ctl).unwrap(),
            })
        } else {
            Err(error_message())
        }
    }

    /// Create a new control object.
    ///
    /// **Note:** Only gringo options (without <code>\-\-output</code>) and clasp`s options are supported as arguments,
    /// except basic options such as <code>\-\-help</code>.
    /// Furthermore, a control object is blocked while a search call is active;
    /// you must not call any member function during search.
    ///
    /// # Arguments
    ///
    /// * `arguments` - C string array of command line arguments
    /// * `logger` - callback functions for warnings and info messages
    /// * `logger_data` - user data for the logger callback
    /// * `message_limit` - maximum number of times the logger callback is called
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if argument parsing fails
    pub fn new_with_logger<D, T: ClingoLogger<D>>(
        arguments: std::vec::Vec<String>,
        _logger: &T,
        logger_data: &mut D,
        message_limit: u32,
    ) -> Result<ClingoControl, &'static str> {
        // create a vector of zero terminated strings
        let mut args: Vec<CString> = Vec::new();
        for arg in arguments {
            args.push(CString::new(arg).unwrap());
        }

        // convert the strings to raw pointers
        let c_args = args.iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let mut ctl = unsafe { mem::uninitialized() };

        let data = logger_data as *mut D;
        if unsafe {
            clingo_control_new(
                c_args.as_ptr(),
                c_args.len(),
                Some(T::unsafe_logging_callback as ClingoLoggingCallback),
                data as *mut ::std::os::raw::c_void,
                message_limit,
                &mut ctl,
            )
        } {
            Ok(ClingoControl {
                ctl: Unique::new(ctl).unwrap(),
            })
        } else {
            Err(error_message())
        }
    }

    //TODO     pub fn clingo_control_load(control: *mut ClingoControl, file: *const c_char) -> bool;

    /// Extend the logic program with the given non-ground logic program in string form.
    ///
    /// This function puts the given program into a block of form: <tt>\#program name(parameters).</tt>
    ///
    /// After extending the logic program, the corresponding program parts are typically grounded with `ground()`.
    ///
    /// # Arguments
    ///
    /// * `name` name of the program block
    /// * `parameters` string array of parameters of the program block
    /// * `program` string representation of the program
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if parsing fails
    pub fn add(
        &mut self,
        name_: &str,
        parameters: Vec<&str>,
        program_: &str,
    ) -> Result<(), &'static str> {
        let name = CString::new(name_).unwrap();
        let name_ptr = name.as_ptr();

        let program = CString::new(program_).unwrap();
        let program_ptr = program.as_ptr();

        let parameters_size = parameters.len();

        // create a vector of zero terminated strings
        let l_parameters = parameters
            .into_iter()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Vec<CString>>();

        // convert the strings to raw pointers
        let c_parameters = l_parameters
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        if unsafe {
            clingo_control_add(
                self.ctl.as_ptr(),
                name_ptr,
                c_parameters.as_ptr(),
                parameters_size,
                program_ptr,
            )
        } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// Ground the selected [`ClingoPart`](struct.ClingoPart.html) parts of the current (non-ground) logic program.
    ///
    /// After grounding, logic programs can be solved with `solve()`.
    ///
    /// **Note:** Parts of a logic program without an explicit <tt>\#program</tt>
    /// specification are by default put into a program called `base` - without
    /// arguments.
    ///
    /// # Arguments
    ///
    /// * `parts` array of parts to ground
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    ///
    /// @see `ClingoPart`
    pub fn ground(&mut self, sparts: &[ClingoPart]) -> Result<(), &'static str> {
        let parts = sparts
            .iter()
            .map(|arg| arg.from())
            .collect::<Vec<clingo_part>>();
        let parts_size = sparts.len();

        if unsafe {
            clingo_control_ground(
                self.ctl.as_ptr(),
                parts.as_ptr(),
                parts_size,
                None,
                std::ptr::null_mut() as *mut ::std::os::raw::c_void,
            )
        } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// Ground the selected [`ClingoPart`](struct.ClingoPart.html) parts of the current (non-ground) logic program.
    ///
    /// After grounding, logic programs can be solved with `solve()`.
    ///
    /// **Note:** Parts of a logic program without an explicit <tt>\#program</tt>
    /// specification are by default put into a program called `base` - without
    /// arguments.
    ///
    /// # Arguments
    ///
    /// * `parts` array of parts to ground
    /// * `ground_callback` callback to implement external functions
    /// * `ground_callback_data` user data for ground_callback
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - error code of ground callback
    ///
    /// @see `ClingoPart`
    pub fn ground_with_event_handler<D, T: ClingoGroundEventHandler<D>>(
        &mut self,
        sparts: &[ClingoPart],
        _ground_callback: &T,
        ground_callback_data: &mut D,
    ) -> Result<(), &'static str> {
        let parts = sparts
            .iter()
            .map(|arg| arg.from())
            .collect::<Vec<clingo_part>>();
        let parts_size = sparts.len();

        let data = ground_callback_data as *mut D;
        if unsafe {
            clingo_control_ground(
                self.ctl.as_ptr(),
                parts.as_ptr(),
                parts_size,
                Some(T::unsafe_ground_callback as ClingoGroundCallback),
                data as *mut ::std::os::raw::c_void,
            )
        } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// Solve the currently [`ground()`](struct.ClingoControl.html#method.ground) grounded logic program enumerating its models.
    ///
    /// See the [`SolveHandle`](struct.ClingoSolveHandle.html) module for more information.
    ///
    /// # Arguments
    ///
    /// * `mode` - configures the search mode
    /// * `assumptions` - array of assumptions to solve under
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if solving could not be started
    pub fn solve(
        &mut self,
        mode: ClingoSolveMode,
        assumptions: &[ClingoSymbolicLiteral],
    ) -> Result<&mut ClingoSolveHandle, &'static str> {
        let mut handle = std::ptr::null_mut() as *mut clingo_solve_handle_t;
        if unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode as clingo_solve_mode_bitset_t,
                assumptions.as_ptr() as *const clingo_symbolic_literal_t,
                assumptions.len(),
                None,
                std::ptr::null_mut() as *mut ::std::os::raw::c_void,
                &mut handle,
            )
        } {
            unsafe { (handle as *mut ClingoSolveHandle).as_mut() }
                .ok_or("Rust binding failed to dereference pointer to clingo solve handle")
        } else {
            Err(error_message())
        }
    }

    /// Solve the currently [`ground()`](struct.ClingoControl.html#method.ground) grounded logic program enumerating its models.
    ///
    /// See the [`SolveHandle`](struct.ClingoSolveHandle.html) module for more information.
    ///
    /// # Arguments
    ///
    /// * `mode` - configures the search mode
    /// * `assumptions` - array of assumptions to solve under
    /// * `notify` - the event handler to register
    /// * `data` - the user data for the event handler
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if solving could not be started
    pub fn solve_with_event_handler<D, T: ClingoSolveEventHandler<D>>(
        &mut self,
        mode: clingo_solve_mode_bitset_t,
        assumptions: &[ClingoSymbolicLiteral],
        _notify: &T,
        data_: &mut D,
    ) -> Result<&mut ClingoSolveHandle, &'static str> {
        let mut handle = std::ptr::null_mut() as *mut clingo_solve_handle_t;

        let data = data_ as *mut D;
        if unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode,
                assumptions.as_ptr() as *const clingo_symbolic_literal_t,
                assumptions.len(),
                Some(T::unsafe_solve_callback as ClingoSolveEventCallback),
                data as *mut ::std::os::raw::c_void,
                &mut handle,
            )
        } {
            unsafe { (handle as *mut ClingoSolveHandle).as_mut() }
                .ok_or("Rust binding failed to dereference pointer to clingo solve handle")
        } else {
            Err(error_message())
        }
    }

    /// Clean up the domains of clingo`s grounding component using the solving
    /// component`s top level assignment.
    ///
    /// This function removes atoms from domains that are false and marks atoms as
    /// facts that are true.  With multi-shot solving, this can result in smaller
    /// groundings because less rules have to be instantiated and more
    /// simplifications can be applied.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    pub fn cleanup(&mut self) -> Result<(), &'static str> {
        if unsafe { clingo_control_cleanup(self.ctl.as_ptr()) } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// Assign a truth value to an external atom.
    ///
    /// If the atom does not exist or is not external, this is a noop.
    ///
    /// # Arguments
    ///
    /// * `atom` atom to assign
    /// * `value` - the truth value
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    pub fn assign_external(
        &mut self,
        symbol: &ClingoSymbol,
        value: ClingoTruthValue,
    ) -> Result<(), &'static str> {
        if unsafe {
            clingo_control_assign_external(
                self.ctl.as_ptr(),
                symbol.0,
                value as clingo_truth_value_t,
            )
        } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// Release an external atom.
    ///
    /// After this call, an external atom is no longer external and subject to
    /// program simplifications.  If the atom does not exist or is not external,
    /// this is a noop.
    ///
    /// # Arguments
    ///
    /// * `atom` - atom to release
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    //     pub fn clingo_control_release_external(control: *mut ClingoControl,
    //                                            atom: clingo_symbol_t)
    //                                            -> u8;
    /// Register a custom propagator with the control object.
    ///
    /// If the sequential flag is set to true, the propagator is called
    /// sequentially when solving with multiple threads.
    ///
    /// See the [`ClingoPropagator`](struct.ClingoPropagator) module for more information.
    ///
    /// # Arguments
    ///
    /// * `propagator` - the propagator
    /// * `data` user data passed to the propagator functions
    /// * `sequential` - whether the propagator should be called sequentially
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    pub fn register_propagator<D, T: ClingoPropagatorBuilder<D>>(
        &mut self,
        _propagator_builder: &T,
        data: &mut D,
        sequential: bool,
    ) -> Result<(), &'static str> {
        let propagator = T::new();
        let propagator_ptr: *const ClingoPropagator = &propagator;
        let data_ptr = data as *mut D;
        if unsafe {
            clingo_control_register_propagator(
                self.ctl.as_ptr(),
                propagator_ptr as *const clingo_propagator,
                data_ptr as *mut ::std::os::raw::c_void,
                sequential,
            )
        } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// Get a statistics object to inspect solver statistics.
    ///
    /// Statistics are updated after a solve call.
    ///
    /// See the [`ClingoStatistics`](struct.ClingoStatistics.html) module for more information.
    ///
    /// **Attention**
    /// The level of detail of the statistics depends on the stats option
    /// (which can be set using [`ClingoConfiguration`](struct.ClingoConfiguration.html) module or passed as an option when [`new()`](struct.ClingoControl.html#method.new)  creating the control object).
    /// The default level zero only provides basic statistics,
    /// level one provides extended and accumulated statistics,
    /// and level two provides per-thread statistics.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    pub fn statistics(&mut self) -> Result<&mut ClingoStatistics, &'static str> {
        let mut stat = std::ptr::null_mut() as *mut clingo_statistics_t;
        if unsafe { clingo_control_statistics(self.ctl.as_ptr(), &mut stat) } {
            unsafe { (stat as *mut ClingoStatistics).as_mut() }
                .ok_or("Rust bindings failed to dereference pointer to clingo statistics")
        } else {
            Err(error_message())
        }
    }

    /// Interrupt the active solve call (or the following solve call right at the beginning).
    pub fn interrupt(&mut self) {
        unsafe {
            clingo_control_interrupt(self.ctl.as_ptr());
        }
    }

    /// Get a configuration object to change the solver configuration.
    ///
    /// See the [`ClingoConfiguration`](struct.ClingoConfiguration.html) module for more information.
    pub fn configuration(&mut self) -> Result<&mut ClingoConfiguration, &'static str> {
        let mut conf = std::ptr::null_mut() as *mut clingo_configuration_t;
        if unsafe { clingo_control_configuration(self.ctl.as_ptr(), &mut conf) } {
            unsafe { (conf as *mut ClingoConfiguration).as_mut() }
                .ok_or("Rust binding failed to dereference pointer to clingo configuration")
        } else {
            Err(error_message())
        }
    }

    /// Configure how learnt constraints are handled during enumeration.
    ///
    /// If the enumeration assumption is enabled, then all information learnt from
    /// the solver`s various enumeration modes is removed after a solve call. This
    /// includes enumeration of cautious or brave consequences, enumeration of
    /// answer sets with or without projection, or finding optimal models, as well
    /// as clauses added with clingo_solve_control_add_clause().
    ///
    /// @attention For practical purposes, this option is only interesting for single-shot solving
    /// or before the last solve call to squeeze out a tiny bit of performance.
    /// Initially, the enumeration assumption is enabled.
    ///
    /// # Arguments
    ///
    /// * `enable` - whether to enable the assumption
    pub fn use_enumeration_assumption(&mut self, enable: bool) -> Option<()> {
        if unsafe { clingo_control_use_enumeration_assumption(self.ctl.as_ptr(), enable) } {
            Some(())
        } else {
            None
        }
    }

    /// Return the symbol for a constant definition of form: <tt>\#const name = symbol</tt>.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the constant
    pub fn get_const(&mut self, name: &str) -> Option<ClingoSymbol> {
        let c_str_name = CString::new(name).unwrap();
        let mut symbol = 0 as clingo_symbol_t;
        if unsafe { clingo_control_get_const(self.ctl.as_ptr(), c_str_name.as_ptr(), &mut symbol) }
        {
            Some(ClingoSymbol(symbol))
        } else {
            None
        }
    }

    /// Check if there is a constant definition for the given constant.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the constant
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if constant definition does not exist
    ///
    /// @see clingo_control_get_const()
    pub fn has_const(&mut self, name: &str) -> Result<bool, &'static str> {
        let c_str_name = CString::new(name).unwrap();
        let mut exist = false;
        if unsafe { clingo_control_has_const(self.ctl.as_ptr(), c_str_name.as_ptr(), &mut exist) } {
            Ok(exist)
        } else {
            Err(error_message())
        }
    }

    /// Get an object to inspect symbolic atoms (the relevant Herbrand base) used
    /// for grounding.
    ///
    /// See the [`ClingoSymbolicAtoms`](struct.ClingoSymbolicAtoms.html) module for more information.
    pub fn symbolic_atoms(&mut self) -> Option<&mut ClingoSymbolicAtoms> {
        let mut atoms = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
        if unsafe { clingo_control_symbolic_atoms(self.ctl.as_ptr(), &mut atoms) } {
            unsafe { (atoms as *mut ClingoSymbolicAtoms).as_mut() }
        } else {
            None
        }
    }

    /// Get an object to inspect theory atoms that occur in the grounding.
    ///
    /// See the [`ClingoTheoryAtoms`](struct.ClingoTheoryAtoms.html) module for more information.
    pub fn theory_atoms(&mut self) -> Option<&mut ClingoTheoryAtoms> {
        let mut atoms = std::ptr::null_mut() as *mut clingo_theory_atoms_t;
        if unsafe { clingo_control_theory_atoms(self.ctl.as_ptr(), &mut atoms) } {
            unsafe { (atoms as *mut ClingoTheoryAtoms).as_mut() }
        } else {
            None
        }
    }

    /// Get an object to add ground directives to the program.
    ///
    /// See the [`ClingoProgramBuilder`](struct.ClingoProgramBuilder.html) module for more information.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    pub fn backend(&mut self) -> Option<&mut ClingoBackend> {
        let mut backend = std::ptr::null_mut();
        if unsafe { clingo_control_backend(self.ctl.as_ptr(), &mut backend) } {
            unsafe { (backend as *mut ClingoBackend).as_mut() }
        } else {
            None
        }
    }

    /// Get an object to add non-ground directives to the program.
    ///
    /// See the [`ClingoProgramBuilder`](struct.ClingoProgramBuilder.html) module for more information.
    pub fn program_builder(&mut self) -> Option<&mut ClingoProgramBuilder> {
        let mut builder = std::ptr::null_mut() as *mut clingo_program_builder_t;
        if unsafe { clingo_control_program_builder(self.ctl.as_ptr(), &mut builder) } {
            unsafe { (builder as *mut ClingoProgramBuilder).as_mut() }
        } else {
            None
        }
    }
}

pub struct ClingoProgramBuilder(clingo_program_builder_t);
impl ClingoProgramBuilder {
    /// Begin building a program.
    pub fn begin(&mut self) -> Option<()> {
        if unsafe { clingo_program_builder_begin(&mut self.0) } {
            Some(())
        } else {
            None
        }
    }

    /// Adds a statement to the program.
    ///
    /// @attention @ref clingo_program_builder_begin() must be called before adding statements and @ref clingo_program_builder_end() must be called after all statements have been added.
    ///
    /// # Arguments
    ///
    /// * `statement` - the statement to add
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) for statements of invalid form
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    pub fn add(&mut self, statement: &ClingoAstStatement) -> Result<(), &'static str> {
        let ClingoAstStatement(ref stm) = *statement;
        if unsafe { clingo_program_builder_add(&mut self.0, stm) } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// End building a program.
    pub fn end(&mut self) -> Option<()> {
        if unsafe { clingo_program_builder_end(&mut self.0) } {
            Some(())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct ClingoAstHeadLiteral(clingo_ast_head_literal_t);

#[derive(Clone, Copy)]
pub struct ClingoAstBodyLiteral(clingo_ast_body_literal_t);
impl ClingoAstBodyLiteral {
    pub fn new(
        ClingoLocation(location): ClingoLocation,
        sign: ClingoAstSign,
        type_: ClingoAstBodyLiteralType,
        lit_ref: &ClingoAstLiteral,
    ) -> ClingoAstBodyLiteral {
        let _bg_union_2 = clingo_ast_body_literal__bindgen_ty_1 {
            literal: (lit_ref as *const ClingoAstLiteral) as *const clingo_ast_literal,
        };
        ClingoAstBodyLiteral(clingo_ast_body_literal_t {
            location: location,
            sign: sign as clingo_ast_sign_t,
            type_: type_ as clingo_ast_body_literal_type_t,
            __bindgen_anon_1: _bg_union_2,
        })
    }
}

#[derive(Clone, Copy)]
pub struct ClingoAstRule(clingo_ast_rule_t);
impl ClingoAstRule {
    pub fn new(
        ClingoAstHeadLiteral(head): ClingoAstHeadLiteral,
        body: &[ClingoAstBodyLiteral],
    ) -> ClingoAstRule {
        let rule = clingo_ast_rule {
            head: head,
            body: body.as_ptr() as *const clingo_ast_body_literal_t,
            size: body.len(),
        };
        ClingoAstRule(rule)
    }

    pub fn head(&self) -> ClingoAstHeadLiteral {
        let ClingoAstRule(ref rule) = *self;
        ClingoAstHeadLiteral(rule.head)
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

#[derive(Clone, Copy)]
pub struct ClingoAstExternal(clingo_ast_external_t);
impl ClingoAstExternal {
    pub fn new(
        ClingoAstTerm(term): ClingoAstTerm,
        body: &[ClingoAstBodyLiteral],
    ) -> ClingoAstExternal {
        let ext = clingo_ast_external {
            atom: term,
            body: body.as_ptr() as *const clingo_ast_body_literal_t,
            size: body.len(),
        };
        ClingoAstExternal(ext)
    }
}

#[derive(Clone)]
pub struct ClingoAstStatement(clingo_ast_statement_t);
impl ClingoAstStatement {
    pub fn new_external(
        ClingoLocation(location): ClingoLocation,
        type_: ClingoAstStatementType,
        ext: &ClingoAstExternal,
    ) -> ClingoAstStatement {
        let external: *const ClingoAstExternal = ext;
        let _bg_union_2 = clingo_ast_statement__bindgen_ty_1 {
            external: external as *const clingo_ast_external,
        };
        let stm = clingo_ast_statement_t {
            location: location,
            type_: type_ as clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        ClingoAstStatement(stm)
    }

    pub fn new_rule(
        ClingoLocation(location): ClingoLocation,
        rule_: &ClingoAstRule,
    ) -> ClingoAstStatement {
        let rule: *const ClingoAstRule = rule_;

        let _bg_union_2 = clingo_ast_statement__bindgen_ty_1 {
            rule: rule as *const clingo_ast_rule,
        };
        let stm = clingo_ast_statement_t {
            location: location,
            type_: ClingoAstStatementType::Rule as clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        ClingoAstStatement(stm)
    }

    pub fn location(&self) -> ClingoLocation {
        ClingoLocation(self.0.location)
    }

    pub fn get_type(&self) -> Result<ClingoAstStatementType, &'static str> {
        let ClingoAstStatement(ref stm) = *self;
        match stm.type_ as u32 {
            clingo_ast_statement_type_clingo_ast_statement_type_rule => {
                Ok(ClingoAstStatementType::Rule)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_const => {
                Ok(ClingoAstStatementType::Const)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_show_signature => {
                Ok(ClingoAstStatementType::ShowSignature)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_show_term => {
                Ok(ClingoAstStatementType::ShowTerm)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_minimize => {
                Ok(ClingoAstStatementType::Minimize)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_script => {
                Ok(ClingoAstStatementType::Script)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_program => {
                Ok(ClingoAstStatementType::Program)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_external => {
                Ok(ClingoAstStatementType::External)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_edge => {
                Ok(ClingoAstStatementType::Edge)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_heuristic => {
                Ok(ClingoAstStatementType::Heuristic)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_project_atom => {
                Ok(ClingoAstStatementType::ProjectAtom)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_project_atom_signature => {
                Ok(ClingoAstStatementType::ProjectAtomSignature)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_theory_definition => {
                Ok(ClingoAstStatementType::TheoryDefinition)
            }
            _ => Err("Rust binding failed to match clingo ast statement type"),
        }
    }

    pub unsafe fn rule(&self) -> &ClingoAstRule {
        let ClingoAstStatement(ref stm) = *self;
        let ast_rule_ptr = stm.__bindgen_anon_1.rule as *const clingo_ast_rule_t;
        (ast_rule_ptr as *const ClingoAstRule).as_ref().unwrap()
    }
}

#[derive(Clone, Copy)]
pub struct ClingoAstTerm(clingo_ast_term_t);
impl ClingoAstTerm {
    pub fn new_symbol(
        ClingoLocation(location): ClingoLocation,
        ClingoSymbol(symbol): ClingoSymbol,
    ) -> ClingoAstTerm {
        let _bg_union_1 = clingo_ast_term__bindgen_ty_1 { symbol: symbol };
        let term = clingo_ast_term_t {
            location: location,
            type_: clingo_ast_term_type_clingo_ast_term_type_symbol as clingo_ast_term_type_t,
            __bindgen_anon_1: _bg_union_1,
        };
        ClingoAstTerm(term)
    }

    pub fn location(&self) -> ClingoLocation {
        ClingoLocation(self.0.location)
    }
}

#[derive(Clone, Copy)]
pub struct ClingoAstLiteral(clingo_ast_literal_t);
impl ClingoAstLiteral {
    pub fn new(
        ClingoLocation(location): ClingoLocation,
        sign: ClingoAstSign,
        type_: ClingoAstLiteralType,
        sym: &ClingoAstTerm,
    ) -> ClingoAstLiteral {
        let symbol: *const ClingoAstTerm = sym;
        let _bg_union_2 = clingo_ast_literal__bindgen_ty_1 {
            symbol: symbol as *const clingo_sys::clingo_ast_term,
        };
        let lit = clingo_ast_literal_t {
            location: location,
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
    pub fn root(&mut self) -> Result<ClingoId, &'static str> {
        let ClingoConfiguration(ref mut conf) = *self;
        let mut root_key = 0 as clingo_id_t;
        if unsafe { clingo_configuration_root(conf, &mut root_key) } {
            Ok(ClingoId(root_key))
        } else {
            Err(error_message())
        }
    }

    /// Get the type of a key.
    // TODO: The type is bitset, an entry can have multiple (but at least one) type.
    pub fn configuration_type(
        &mut self,
        ClingoId(key): ClingoId,
    ) -> Result<ClingoConfigurationType, &'static str> {
        let ClingoConfiguration(ref mut conf) = *self;
        let mut ctype = 0 as clingo_configuration_type_bitset_t;
        if unsafe { clingo_configuration_type(conf, key, &mut ctype) } {
            match ctype as u32 {
                clingo_configuration_type_clingo_configuration_type_value => {
                    Ok(ClingoConfigurationType::Value)
                }
                clingo_configuration_type_clingo_configuration_type_array => {
                    Ok(ClingoConfigurationType::Array)
                }
                clingo_configuration_type_clingo_configurations_type_map => {
                    Ok(ClingoConfigurationType::Map)
                }
                _ => Err("Rust binding failed to match clingo configuration type"),
            }
        } else {
            Err("Rust binding failed to detect clingo configuration type")
        }
    }

    /// Get the description of an entry.
    pub fn description(&mut self, ClingoId(key): ClingoId) -> Option<&str> {
        let ClingoConfiguration(ref mut conf) = *self;
        let mut description_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_configuration_description(
                conf,
                key,
                &mut description_ptr as *mut *const ::std::os::raw::c_char,
            )
        } {
            let cstr = unsafe { CStr::from_ptr(description_ptr) };
            Some(cstr.to_str().unwrap())
        } else {
            None
        }
    }

    /// Get the size of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [`configuration_type()`](struct.ClingoConfiguration.html#method.configuration_type) type of the entry must be  [`ClingoConfigurationType::Array`](enum.ClingoConfigurationType.html#variant.Array).
    pub fn array_size(&mut self, ClingoId(key): ClingoId) -> Option<usize> {
        let ClingoConfiguration(ref mut conf) = *self;
        let mut size = 0;
        if unsafe { clingo_configuration_array_size(conf, key, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Get the subkey at the given offset of an array entry.
    ///
    /// **Note:** Some array entries, like fore example the solver configuration, can be accessed past there actual size to add subentries.
    /// # Pre-condition
    ///
    /// The [`configuration_type()`](struct.ClingoConfiguration.html#method.configuration_type) type of the entry must be [`ClingoConfigurationType::Array`](enum.ClingoConfigurationType.html#variant.Array).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset in the array
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

    /// Get the number of subkeys of a map entry.
    ///
    /// # Pre-condition
    ///
    /// The [`configuration_type()`](struct.ClingoConfiguration.html#method.configuration_type) type of the entry must be [`ClingoConfigurationType::Map`](enum.ClingoConfigurationType.html#variant.Map).
    pub fn map_size(&mut self, ClingoId(key): ClingoId) -> Option<usize> {
        let ClingoConfiguration(ref mut conf) = *self;
        let mut size = 0;
        if unsafe { clingo_configuration_map_size(conf, key, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    //TODO     pub fn clingo_configuration_map_subkey_name(configuration: *mut ClingoConfiguration,
    //                                                 key: clingo_id_t,
    //                                                 offset: size_t,
    //                                                 name: *mut *const c_char)
    //                                                 -> u8;

    /// Lookup a subkey under the given name.
    ///
    /// # Pre-condition
    ///
    /// The [`configuration_type()`](struct.ClingoConfiguration.html#method.configuration_type) type of the entry must be [`ClingoConfigurationType::Map`](enum.ClingoConfigurationType.html#variant.Map).
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    pub fn map_at(
        &mut self,
        ClingoId(key): ClingoId,
        name: &str,
    ) -> Result<ClingoId, &'static str> {
        let mut nkey = 0 as clingo_id_t;
        let name_c_str = CString::new(name).unwrap();
        if unsafe { clingo_configuration_map_at(&mut self.0, key, name_c_str.as_ptr(), &mut nkey) }
        {
            Ok(ClingoId(nkey))
        } else {
            Err(error_message())
        }
    }

    //TODO     pub fn clingo_configuration_value_is_assigned(configuration: *mut ClingoConfiguration,
    //                                                   key: clingo_id_t,
    //                                                   assigned: *mut u8)
    //                                                   -> u8;

    //TODO     pub fn clingo_configuration_value_get_size(configuration: *mut ClingoConfiguration,
    //                                                key: clingo_id_t,
    //                                                size: *mut size_t)
    //                                                -> u8;

    //TODO     pub fn clingo_configuration_value_get(configuration: *mut ClingoConfiguration,
    //                                           key: clingo_id_t,
    //                                           value: *mut c_char,
    //                                           size: size_t)
    //                                           -> u8;

    /// Set the value of an entry.
    ///
    /// # Pre-condition
    ///
    /// The [`configuration_type()`](struct.ClingoConfiguration.html#method.configuration_type) type of the entry must be [`ClingoConfigurationType::Value`](enum.ClingoConfigurationType.html#variant.Value).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `value` - the value to set
    pub fn value_set(&mut self, ClingoId(key): ClingoId, value: &str) -> Option<()> {
        let value_c_str = CString::new(value).unwrap();
        if unsafe { clingo_configuration_value_set(&mut self.0, key, value_c_str.as_ptr()) } {
            Some(())
        } else {
            None
        }
    }
}

pub struct ClingoBackend(clingo_backend_t);
impl ClingoBackend {
    /// Add a rule to the program.
    ///
    /// # Arguments
    ///
    /// * `choice` determines if the head is a choice or a disjunction
    /// * `head` - the head atoms
    /// * `body` - the body literals
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
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
        } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    //TODO     pub fn clingo_backend_weight_rule(backend: *mut ClingoBackend,
    //                                       choice: u8,
    //                                       head: *const clingo_atom_t,
    //                                       head_size: size_t,
    //                                       lower_bound: clingo_weight_t,
    //                                       body: *const clingo_weighted_literal_t,
    //                                       body_size: size_t)
    //                                       -> u8;

    //TODO     pub fn clingo_backend_minimize(backend: *mut ClingoBackend,
    //                                    priority: clingo_weight_t,
    //                                    literals: *const clingo_weighted_literal_t,
    //                                    size: size_t)
    //                                    -> u8;

    //TODO     pub fn clingo_backend_project(backend: *mut ClingoBackend,
    //                                   atoms: *const clingo_atom_t,
    //                                   size: size_t)
    //                                   -> u8;

    //TODO     pub fn clingo_backend_external(backend: *mut ClingoBackend,
    //                                    atom: clingo_atom_t,
    //                                    type_: clingo_external_type_t)
    //                                    -> u8;

    /// Add an assumption directive.
    ///
    /// # Arguments
    ///
    /// * `literals` - the literals to assume (positive literals are true and negative literals false for the next solve call)
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    pub fn assume(&mut self, literals: &[ClingoLiteral]) -> Result<(), &'static str> {
        let size = literals.len();
        if unsafe {
            clingo_backend_assume(
                &mut self.0,
                literals.as_ptr() as *const clingo_literal_t,
                size,
            )
        } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    //TODO     pub fn clingo_backend_heuristic(backend: *mut ClingoBackend,
    //                                     atom: clingo_atom_t,
    //                                     type_: clingo_heuristic_type_t,
    //                                     bias: c_int,
    //                                     priority: ::std::os::raw::c_uint,
    //                                     condition: *const clingo_literal_t,
    //                                     size: size_t)
    //                                     -> u8;

    //TODO     pub fn clingo_backend_acyc_edge(backend: *mut ClingoBackend,
    //                                     node_u: c_int,
    //                                     node_v: c_int,
    //                                     condition: *const clingo_literal_t,
    //                                     size: size_t)
    //                                     -> u8;

    /// Get a fresh atom to be used in aspif directives.
    pub fn add_atom(&mut self) -> Option<ClingoAtom> {
        let mut atom = 0 as clingo_atom_t;
        if unsafe { clingo_backend_add_atom(&mut self.0, &mut atom) } {
            Some(ClingoAtom(atom))
        } else {
            None
        }
    }
}

pub struct ClingoStatistics(clingo_statistics_t);
impl ClingoStatistics {
    /// Get the root key of the statistics.
    pub fn root(&mut self) -> Option<u64> {
        let mut root_key = 0 as u64;
        if unsafe { clingo_statistics_root(&mut self.0, &mut root_key) } {
            Some(root_key)
        } else {
            None
        }
    }

    /// Get the type of a key.
    ///
    /// # Errors
    ///
    /// - Failure to match clingo statistics type
    pub fn statistics_type(&mut self, key: u64) -> Result<ClingoStatisticsType, &'static str> {
        let mut stype = 0 as clingo_statistics_type_t;
        if unsafe { clingo_statistics_type(&mut self.0, key, &mut stype) } {
            match stype as u32 {
                clingo_statistics_type_clingo_statistics_type_empty => {
                    Ok(ClingoStatisticsType::Empty)
                }
                clingo_statistics_type_clingo_statistics_type_value => {
                    Ok(ClingoStatisticsType::Value)
                }
                clingo_statistics_type_clingo_statistics_type_array => {
                    Ok(ClingoStatisticsType::Array)
                }
                clingo_statistics_type_clingo_statistics_type_map => Ok(ClingoStatisticsType::Map),
                _ => Err("Rust binding failed to match clingo statistics type"),
            }
        } else {
            Err(error_message())
        }
    }

    /// Get the size of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be [`ClingoConfigurationType::Array`](enum.ClingoConfigurationType.html#variant.Array).
    ///
    /// **Returns** whether the call was successful
    pub fn array_size(&mut self, key: u64) -> Option<usize> {
        let mut size = 0 as usize;
        if unsafe { clingo_statistics_array_size(&mut self.0, key, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Get the subkey at the given offset of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be [`ClingoStatisticsType::Array`](enum.ClingoStatisticsType.html#variant.Array).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset in the array
    ///
    /// **Returns** whether the call was successful
    pub fn statistics_array_at(&mut self, key: u64, offset: usize) -> Option<u64> {
        let mut subkey = 0 as u64;
        if unsafe { clingo_statistics_array_at(&mut self.0, key, offset, &mut subkey) } {
            Some(subkey)
        } else {
            None
        }
    }

    /// Get the number of subkeys of a map entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be [`ClingoStatisticsType::Map`](enum.ClingoStatisticsType.html#variant.Map).
    pub fn map_size(&mut self, key: u64) -> Option<usize> {
        let mut size = 0 as usize;
        if unsafe { clingo_statistics_map_size(&mut self.0, key, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Get the name associated with the offset-th subkey.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be [`ClingoStatisticsType::Map`](enum.ClingoStatisticsType.html#variant.Map).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset of the name
    pub fn map_subkey_name<'a>(&mut self, key: u64, offset: usize) -> Option<&'a str> {
        let mut name = std::ptr::null() as *const c_char;
        if unsafe { clingo_statistics_map_subkey_name(&mut self.0, key, offset, &mut name) } {
            Some(unsafe { CStr::from_ptr(name) }.to_str().unwrap())
        } else {
            None
        }
    }

    /// Lookup a subkey under the given name.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be [`ClingoStatisticsType::Map`](enum.ClingoStatisticsType.html#variant.Map).
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `name` - the name to lookup the subkey
    pub fn map_at(&mut self, key: u64, name: &str) -> Option<u64> {
        let mut subkey = 0 as u64;
        let name_c_str = CString::new(name).unwrap();
        if unsafe { clingo_statistics_map_at(&mut self.0, key, name_c_str.as_ptr(), &mut subkey) } {
            Some(subkey)
        } else {
            None
        }
    }

    /// Get the value of the given entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.ClingoStatistics.html#method.statistics_type) of the entry must be [`ClingoStatisticsType::Value`](enum.ClingoStatisticsType.html#variant.Value).
    pub fn value_get(&mut self, key: u64) -> Option<f64> {
        let mut value = 0.0 as f64;
        if unsafe { clingo_statistics_value_get(&mut self.0, key, &mut value) } {
            Some(value)
        } else {
            None
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
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    pub fn create(
        name_: &str,
        arity: u32,
        positive: bool,
    ) -> Result<ClingoSignature, &'static str> {
        let name_c_str = CString::new(name_).unwrap();
        let mut signature = 0;
        if unsafe { clingo_signature_create(name_c_str.as_ptr(), arity, positive, &mut signature) }
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
    /// * `signature` optional signature
    pub fn begin(
        &mut self,
        opt_sig: Option<&ClingoSignature>,
    ) -> Option<clingo_symbolic_atom_iterator_t> {
        match opt_sig {
            Some(sig) => {
                let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
                if unsafe { clingo_symbolic_atoms_begin(&mut self.0, &sig.0, &mut iterator) } {
                    Some(iterator)
                } else {
                    None
                }
            }
            None => {
                let signature = std::ptr::null();
                let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
                if unsafe { clingo_symbolic_atoms_begin(&mut self.0, signature, &mut iterator) } {
                    Some(iterator)
                } else {
                    None
                }
            }
        }
    }

    /// Iterator pointing to the end of the sequence of symbolic atoms.
    pub fn end(&mut self) -> Option<clingo_symbolic_atom_iterator_t> {
        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        if unsafe { clingo_symbolic_atoms_end(&mut self.0, &mut iterator) } {
            Some(iterator)
        } else {
            None
        }
    }

    /// Find a symbolic atom given its symbolic representation.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the symbol to lookup
    /// * `iterator` iterator pointing to the symbolic atom or to the end
    /// of the sequence if no corresponding atom is found
    pub fn find(
        &mut self,
        ClingoSymbol(symbol): ClingoSymbol,
    ) -> Option<clingo_symbolic_atom_iterator_t> {
        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        if unsafe { clingo_symbolic_atoms_find(&mut self.0, symbol, &mut iterator) } {
            Some(iterator)
        } else {
            None
        }
    }

    /// Check if two iterators point to the same element (or end of the sequence).
    ///
    /// # Arguments
    ///
    /// * `a` - the first iterator
    /// * `b` - the second iterator
    pub fn iterator_is_equal_to(
        &mut self,
        a: clingo_symbolic_atom_iterator_t,
        b: clingo_symbolic_atom_iterator_t,
    ) -> Option<bool> {
        let mut equal = false;
        if unsafe { clingo_symbolic_atoms_iterator_is_equal_to(&mut self.0, a, b, &mut equal) } {
            Some(equal)
        } else {
            None
        }
    }

    /// Get the symbolic representation of an atom.
    ///
    /// # Arguments
    ///
    /// * `iterator` iterator to the atom
    pub fn symbol(&mut self, iterator: clingo_symbolic_atom_iterator_t) -> Option<ClingoSymbol> {
        let mut symbol = 0 as clingo_symbol_t;
        if unsafe { clingo_symbolic_atoms_symbol(&mut self.0, iterator, &mut symbol) } {
            Some(ClingoSymbol(symbol))
        } else {
            None
        }
    }

    /// Check whether an atom is a fact.
    ///
    /// **Note:** This does not determine if an atom is a cautious consequence. The
    /// grounding or solving component`s simplifications can only detect this in
    /// some cases.
    ///
    /// # Arguments
    ///
    /// * `iterator` iterator to the atom
    pub fn is_fact(&mut self, iterator: clingo_symbolic_atom_iterator_t) -> Option<bool> {
        let mut fact = false;
        if unsafe { clingo_symbolic_atoms_is_fact(&mut self.0, iterator, &mut fact) } {
            Some(fact)
        } else {
            None
        }
    }

    /// Check whether an atom is external.
    ///
    /// An atom is external if it has been defined using an external directive and
    /// has not been released or defined by a rule.
    ///
    /// # Arguments
    ///
    /// * `iterator` iterator to the atom
    pub fn is_external(&mut self, iterator: clingo_symbolic_atom_iterator_t) -> Option<bool> {
        let mut external = false;
        if unsafe { clingo_symbolic_atoms_is_external(&mut self.0, iterator, &mut external) } {
            Some(external)
        } else {
            None
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
    /// * `iterator` iterator to the atom
    pub fn literal(&mut self, iterator: clingo_symbolic_atom_iterator_t) -> Option<ClingoLiteral> {
        let mut literal = 0 as clingo_literal_t;
        if unsafe { clingo_symbolic_atoms_literal(&mut self.0, iterator, &mut literal) } {
            Some(ClingoLiteral(literal))
        } else {
            None
        }
    }

    //TODO     pub fn clingo_symbolic_atoms_signatures_size(atoms: *mut ClingoSymbolicAtoms,
    //                                                  size: *mut size_t)
    //                                                  -> u8;

    //TODO     pub fn clingo_symbolic_atoms_signatures(atoms: *mut ClingoSymbolicAtoms,
    //                                             signatures: *mut clingo_signature_t,
    //                                             size: size_t)
    //                                             -> u8;

    /// Get an iterator to the next element in the sequence of symbolic atoms.
    ///
    /// # Arguments
    ///
    /// * `iterator` - the current iterator
    pub fn next(
        &mut self,
        iterator: clingo_symbolic_atom_iterator_t,
    ) -> Option<clingo_symbolic_atom_iterator_t> {
        let mut next = 0 as clingo_symbolic_atom_iterator_t;
        if unsafe { clingo_symbolic_atoms_next(&mut self.0, iterator, &mut next) } {
            Some(next)
        } else {
            None
        }
    }

    //TODO     pub fn clingo_symbolic_atoms_is_valid(atoms: *mut ClingoSymbolicAtoms,
    //                                           iterator: clingo_symbolic_atom_iterator_t,
    //                                           valid: *mut u8)
    //                                           -> u8;
}

pub struct ClingoTheoryAtoms(clingo_theory_atoms_t);
impl ClingoTheoryAtoms {
    //TODO     pub fn clingo_theory_atoms_term_type(atoms: *mut ClingoTheoryAtoms,
    //                                          term: clingo_id_t,
    //                                          type_: *mut clingo_theory_term_type_t)
    //                                          -> u8;

    //TODO     pub fn clingo_theory_atoms_term_number(atoms: *mut ClingoTheoryAtoms,
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
    /// * `term` id of the term
    pub fn term_name<'a>(&mut self, ClingoId(term): ClingoId) -> Option<&'a str> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_theory_atoms_term_name(&mut self.0, term, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Some(c_str.to_str().unwrap())
        } else {
            None
        }
    }

    //TODO     pub fn clingo_theory_atoms_term_arguments(atoms: *mut ClingoTheoryAtoms,
    //                                               term: clingo_id_t,
    //                                               arguments: *mut *const clingo_id_t,
    //                                               size: *mut size_t)
    //                                               -> u8;

    //TODO     pub fn clingo_theory_atoms_term_to_string_size(atoms: *mut ClingoTheoryAtoms,
    //                                                    term: clingo_id_t,
    //                                                    size: *mut size_t)
    //                                                    -> u8;

    //TODO     pub fn clingo_theory_atoms_term_to_string(atoms: *mut ClingoTheoryAtoms,
    //                                               term: clingo_id_t,
    //                                               string: *mut c_char,
    //                                               size: size_t)
    //                                               -> u8;

    //TODO     pub fn clingo_theory_atoms_element_tuple(atoms: *mut ClingoTheoryAtoms,
    //                                              element: clingo_id_t,
    //                                              tuple: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;

    //TODO     pub fn clingo_theory_atoms_element_condition(atoms: *mut ClingoTheoryAtoms,
    //                                                  element: clingo_id_t,
    //                                                  condition: *mut *const clingo_literal_t,
    //                                                  size: *mut size_t)
    //                                                  -> u8;

    //TODO     pub fn clingo_theory_atoms_element_condition_id(atoms: *mut ClingoTheoryAtoms,
    //                                                     element: clingo_id_t,
    //                                                     condition: *mut clingo_literal_t)
    //                                                     -> u8;

    //TODO     pub fn clingo_theory_atoms_element_to_string_size(atoms: *mut ClingoTheoryAtoms,
    //                                                       element: clingo_id_t,
    //                                                       size: *mut size_t)
    //                                                       -> u8;

    //TODO     pub fn clingo_theory_atoms_element_to_string(atoms: *mut ClingoTheoryAtoms,
    //                                                  element: clingo_id_t,
    //                                                  string: *mut c_char,
    //                                                  size: size_t)
    //                                                  -> u8;

    /// Get the total number of theory atoms.
    pub fn size(&mut self) -> Option<usize> {
        let mut size = 0 as usize;
        if unsafe { clingo_theory_atoms_size(&mut self.0, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Get the theory term associated with the theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` id of the atom
    pub fn atom_term(&mut self, ClingoId(atom): ClingoId) -> Option<ClingoId> {
        let mut term = 0 as clingo_id_t;
        if unsafe { clingo_theory_atoms_atom_term(&mut self.0, atom, &mut term) } {
            Some(ClingoId(term))
        } else {
            None
        }
    }

    //TODO     pub fn clingo_theory_atoms_atom_elements(atoms: *mut ClingoTheoryAtoms,
    //                                              atom: clingo_id_t,
    //                                              elements: *mut *const clingo_id_t,
    //                                              size: *mut size_t)
    //                                              -> u8;

    /// Whether the theory atom has a guard.
    ///
    /// # Arguments
    ///
    /// * `atom` id of the atom
    pub fn atom_has_guard(&mut self, ClingoId(atom): ClingoId) -> Option<bool> {
        let mut has_guard = false;
        if unsafe { clingo_theory_atoms_atom_has_guard(&mut self.0, atom, &mut has_guard) } {
            Some(has_guard)
        } else {
            None
        }
    }

    //TODO     pub fn clingo_theory_atoms_atom_guard(atoms: *mut ClingoTheoryAtoms,
    //                                           atom: clingo_id_t,
    //                                           connective: *mut *const c_char,
    //                                           term: *mut clingo_id_t)
    //                                           -> u8;

    /// Get the aspif literal associated with the given theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` id of the atom
    pub fn atom_literal(&mut self, ClingoId(atom): ClingoId) -> Option<ClingoLiteral> {
        let mut literal = 0 as clingo_literal_t;
        if unsafe { clingo_theory_atoms_atom_literal(&mut self.0, atom, &mut literal) } {
            Some(ClingoLiteral(literal))
        } else {
            None
        }
    }

    //TODO     pub fn clingo_theory_atoms_atom_to_string_size(atoms: *mut ClingoTheoryAtoms,
    //                                                    atom: clingo_id_t,
    //                                                    size: *mut size_t)
    //                                                    -> u8;

    //TODO     pub fn clingo_theory_atoms_atom_to_string(atoms: *mut ClingoTheoryAtoms,
    //                                               atom: clingo_id_t,
    //                                               string: *mut c_char,
    //                                               size: size_t)
    //                                               -> u8;
}
pub struct UNSAFE_ClingoTheoryAtomsIterator {
    count: usize,
    size: usize,
}
impl Iterator for UNSAFE_ClingoTheoryAtomsIterator {
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
impl UNSAFE_ClingoTheoryAtomsIterator {
    pub fn from(cta: &mut ClingoTheoryAtoms) -> UNSAFE_ClingoTheoryAtomsIterator {
        UNSAFE_ClingoTheoryAtomsIterator {
            count: 0,
            size: cta.size().unwrap(),
        }
    }
}

pub struct ClingoModel(clingo_model_t);
impl ClingoModel {
    /// Get the type of the model.
    pub fn model_type(&mut self) -> Option<clingo_model_type_t> {
        let mut mtype = 0 as clingo_model_type_t;
        if unsafe { clingo_model_type(&mut self.0, &mut mtype) } {
            Some(mtype)
        } else {
            None
        }
    }

    /// Get the running number of the model.
    pub fn number(&mut self) -> Option<u64> {
        let mut number = 0;
        if unsafe { clingo_model_number(&mut self.0, &mut number) } {
            Some(number)
        } else {
            None
        }
    }

    //NOTTODO pub fn clingo_model_symbols_size(model: *mut ClingoModel,
    //  show: clingo_show_type_bitset_t,
    //  size: *mut size_t)
    //  -> bool;

    /// Get the symbols of the selected types in the model.
    ///
    /// **Note:** CSP assignments are represented using functions with name "$"
    /// where the first argument is the name of the CSP variable and the second one its
    /// value.
    ///
    /// # Arguments
    ///
    /// * `show` - which symbols to select
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if the size is too small
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
            let symbols = Vec::<ClingoSymbol>::with_capacity(size);
            let symbols_ptr = symbols.as_ptr();
            if unsafe {
                clingo_model_symbols(
                    model,
                    show as clingo_show_type_bitset_t,
                    symbols_ptr as *mut clingo_symbol_t,
                    size,
                )
            } {
                let symbols_ref =
                    unsafe { std::slice::from_raw_parts(symbols_ptr as *const ClingoSymbol, size) };
                Ok(symbols_ref.to_owned())
            } else {
                Err(error_message())
            }
        } else {
            Err(error_message())
        }
    }

    //TODO     pub fn clingo_model_contains(model: *mut ClingoModel,
    //                                  atom: clingo_symbol_t,
    //                                  contained: *mut u8)
    //                                  -> u8;

    //TODO     pub fn clingo_model_cost_size(model: *mut ClingoModel, size: *mut size_t) -> u8;

    //TODO     pub fn clingo_model_cost(model: *mut ClingoModel, costs: *mut int64_t, size: size_t) -> u8;

    //TODO     pub fn clingo_model_optimality_proven(model: *mut ClingoModel, proven: *mut u8) -> u8;

    //TODO     pub fn clingo_model_context(model: *mut ClingoModel,
    //                                 control: *mut *mut ClingoSolveControl)
    //                                 -> u8;
}

pub struct ClingoSolveControl(clingo_solve_control_t);
impl ClingoSolveControl {
    /// Add a clause that applies to the current solving step during model
    /// enumeration.
    ///
    /// **Note:** The @ref Propagator module provides a more sophisticated
    /// interface to add clauses - even on partial assignments.
    ///
    /// # Arguments
    ///
    /// * `clause` array of literals representing the clause
    ///
    /// **Returns** whether the call was successful; might set one of the following error codes:
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if adding the clause fails
    pub fn add_clause(&mut self, clause: &[ClingoLiteral]) -> Result<(), &'static str> {
        if unsafe {
            clingo_solve_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                clause.len(),
            )
        } {
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
    pub fn thread_id(&mut self) -> u32 {
        unsafe { clingo_propagate_control_thread_id(&mut self.0) }
    }

    //TODO     pub fn clingo_propagate_control_assignment(control: *mut ClingoPropagateControl)
    //                                           -> *mut clingo_assignment_t;

    /// Add the given clause to the solver.
    ///
    /// This method sets its result to false if the current propagation must be stopped for the solver to backtrack.
    ///
    /// @attention No further calls on the control object or functions on the assignment should be called when the result of this method is false.
    ///
    /// # Arguments
    ///
    /// * `clause` - the clause to add
    /// * `type` - the clause type determining its lifetime
    ///
    /// **Returns** result indicating whether propagation has to be stopped
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    pub fn add_clause(
        &mut self,
        clause: &[ClingoLiteral],
        type_: ClingoClauseType,
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
        } {
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
    /// **Returns** result indicating whether propagation has to be stopped
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
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
    /// * `aspif_literal` - the aspif literal to map
    ///
    /// **Returns** whether the corresponding solver literal
    pub fn solver_literal(
        &mut self,
        ClingoLiteral(aspif_literal): ClingoLiteral,
    ) -> Option<ClingoLiteral> {
        let mut solver_literal = 0 as clingo_literal_t;
        if unsafe {
            clingo_propagate_init_solver_literal(&mut self.0, aspif_literal, &mut solver_literal)
        } {
            Some(ClingoLiteral(solver_literal))
        } else {
            None
        }
    }

    /// Add a watch for the solver literal in the given phase.
    ///
    /// # Arguments
    ///
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
    pub fn symbolic_atoms<'a>(&mut self) -> Result<&'a mut ClingoSymbolicAtoms, &'static str> {
        let mut atoms_ptr = std::ptr::null_mut();
        if unsafe { clingo_propagate_init_symbolic_atoms(&mut self.0, &mut atoms_ptr) } {
            unsafe { (atoms_ptr as *mut ClingoSymbolicAtoms).as_mut() }
                .ok_or("Rust binding failed to dereference pointer to clingo symbolic atoms")
        } else {
            Err(error_message())
        }
    }

    //TODO     pub fn clingo_propagate_init_theory_atoms(init: &mut ClingoPropagateInit,
    //                                               atoms: *mut *mut ClingoTheoryAtoms)
    //                                               -> bool;

    /// Get the number of threads used in subsequent solving.
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
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if solving fails
    pub fn get(&mut self) -> Result<clingo_solve_result_bitset_t, &'static str> {
        let mut result = 0;
        if unsafe { clingo_solve_handle_get(&mut self.0, &mut result) } {
            Ok(result)
        } else {
            Err(error_message())
        }
    }

    /// Get the next model (or zero if there are no more models).
    /// (it is NULL if there are no more models)
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if solving fails
    pub fn model(&mut self) -> Result<&mut ClingoModel, &'static str> {
        let ClingoSolveHandle(ref mut handle) = *self;
        let mut model = std::ptr::null_mut() as *mut clingo_model_t;
        if unsafe { clingo_solve_handle_model(handle, &mut model) } {
            unsafe { (model as *mut ClingoModel).as_mut() }
                .ok_or("Rust binding failed to dereference pointer to clingo model")
        } else {
            Err(error_message())
        }
    }

    /// Discards the last model and starts the search for the next one.
    ///
    /// If the search has been started asynchronously, this function continues the search in the background.
    ///
    /// **Note:** This function does not block.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if solving fails
    pub fn resume(&mut self) -> Result<(), &'static str> {
        let ClingoSolveHandle(ref mut handle) = *self;
        if unsafe { clingo_solve_handle_resume(handle) } {
            Ok(())
        } else {
            Err(error_message())
        }
    }

    /// Stops the running search and releases the handle.
    ///
    /// Blocks until the search is stopped (as if an implicit cancel was called before the handle is released).
    ///
    /// # Errors
    ///
    /// - [`ClingoError::BadAlloc`](enum.ClingoError.html#variant.BadAlloc)
    /// - [`ClingoError::Runtime`](enum.ClingoError.html#variant.Runtime) if solving fails
    pub fn close(&mut self) -> Result<(), &'static str> {
        let ClingoSolveHandle(ref mut handle) = *self;
        if unsafe { clingo_solve_handle_close(handle) } {
            Ok(())
        } else {
            Err(error_message())
        }
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
        assert!(re == 2);
    }
    #[test]
    fn parse_program_test() {
        let mut sym = create_number(42);
        assert!(42 == sym.number().unwrap());
        sym = create_infimum();
        assert!(ClingoSymbolType::Infimum == sym.get_type().unwrap());
    }
}
