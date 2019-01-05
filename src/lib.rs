#![allow(non_upper_case_globals)]
use bitflags::bitflags;
use clingo_sys::*;
use libc::c_char;
use std::cmp::Ordering;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::NulError;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;
use std::str::Utf8Error;

use failure::*;

/// Functions and data structures to work with program ASTs.
pub mod ast;

/// Error from the clingo library.
///
/// **Note:** Errors can only be recovered from if explicitly mentioned; most
/// functions do not provide strong exception guarantees.  This means that in
/// case of errors associated objects cannot be used further.
#[derive(Debug, Fail)]
#[fail(display = "ErrorType::{:?}: {}", error_type, msg)]
pub struct ClingoError {
    pub error_type: ErrorType,
    pub msg: &'static str,
}

/// Error in the rust wrapper, like null pointers or failed calls to C functions.
#[derive(Debug, Fail)]
#[fail(display = "Error in the wrapper: {}", msg)]
pub struct WrapperError {
    msg: &'static str,
}

/// Enumeration of clingo error types for [`ClingoError`](struct.ClingoError.html).
#[derive(Debug, Copy, Clone)]
pub enum ErrorType {
    /// Successful API calls
    Success = clingo_error_clingo_error_success as isize,
    /// Errors only detectable at runtime like invalid input
    Runtime = clingo_error_clingo_error_runtime as isize,
    /// Wrong usage of the clingo API
    Logic = clingo_error_clingo_error_logic as isize,
    /// Memory could not be allocated
    BadAlloc = clingo_error_clingo_error_bad_alloc as isize,
    /// Errors unrelated to clingo
    Unknown = clingo_error_clingo_error_unknown as isize,
}
impl From<i32> for ErrorType {
    fn from(error: i32) -> Self {
        match error as u32 {
            clingo_error_clingo_error_success => ErrorType::Success,
            clingo_error_clingo_error_runtime => ErrorType::Runtime,
            clingo_error_clingo_error_logic => ErrorType::Logic,
            clingo_error_clingo_error_bad_alloc => ErrorType::BadAlloc,
            clingo_error_clingo_error_unknown => ErrorType::Unknown,
            x => panic!("Failed to match clingo_error: {}.", x),
        }
    }
}

/// Represents three-valued truth values.
#[derive(Debug, Copy, Clone)]
pub enum TruthValue {
    // No truth value
    Free = clingo_truth_value_clingo_truth_value_free as isize,
    //     True
    True = clingo_truth_value_clingo_truth_value_true as isize,
    //     False
    False = clingo_truth_value_clingo_truth_value_false as isize,
}

/// Enumeration of clause types determining the lifetime of a clause.
///
/// Clauses in the solver are either cleaned up based on a configurable deletion policy or at the end of a solving step.
/// The values of this enumeration determine if a clause is subject to one of the above deletion strategies.
#[derive(Debug, Copy, Clone)]
pub enum ClauseType {
    /// The clause is subject to the solvers deletion policy
    Learnt = clingo_clause_type_clingo_clause_type_learnt as isize,
    /// The clause is not subject to the solvers deletion policy
    Static = clingo_clause_type_clingo_clause_type_static as isize,
    /// Like `Learnt` but the clause is deleted after a solving step
    Volatile = clingo_clause_type_clingo_clause_type_volatile as isize,
    /// Like `Static` but the clause is deleted after a solving step
    VolatileStatic = clingo_clause_type_clingo_clause_type_volatile_static as isize,
}

/// Enumeration of solve events.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SolveEventType {
    /// Issued if a model is found.
    Model = clingo_solve_event_type_clingo_solve_event_type_model as isize,
    /// Issued when the statistics can be updated.
    Statistics = clingo_solve_event_type_clingo_solve_event_type_statistics as isize,
    /// Issued if the search has completed.
    Finish = clingo_solve_event_type_clingo_solve_event_type_finish as isize,
}

/// Enumeration for entries of the statistics.
#[derive(Debug, Copy, Clone)]
pub enum StatisticsType {
    /// The entry is invalid (has neither of the types below)
    Empty = clingo_statistics_type_clingo_statistics_type_empty as isize,
    /// The entry is a (double) value
    Value = clingo_statistics_type_clingo_statistics_type_value as isize,
    /// The entry is an array
    Array = clingo_statistics_type_clingo_statistics_type_array as isize,
    /// The entry is a map
    Map = clingo_statistics_type_clingo_statistics_type_map as isize,
}

/// Enumeration of available symbol types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SymbolType {
    /// The `#inf` symbol
    Infimum = clingo_symbol_type_clingo_symbol_type_infimum as isize,
    /// A numeric symbol, e.g., `1`
    Number = clingo_symbol_type_clingo_symbol_type_number as isize,
    /// A string symbol, e.g., `"a"`
    String = clingo_symbol_type_clingo_symbol_type_string as isize,
    /// A numeric symbol, e.g., `c`, `(1, "a")`, or `f(1,"a")`
    Function = clingo_symbol_type_clingo_symbol_type_function as isize,
    /// The `#sup` symbol
    Supremum = clingo_symbol_type_clingo_symbol_type_supremum as isize,
}

/// Enumeration of warning codes.
#[derive(Debug, Copy, Clone)]
pub enum Warning {
    /// Undefined arithmetic operation or weight of aggregate
    OperationUndefined = clingo_warning_clingo_warning_operation_undefined as isize,
    /// To report multiple errors; a corresponding runtime error is raised later
    RuntimeError = clingo_warning_clingo_warning_runtime_error as isize,
    /// An undefined atom in program
    AtomUndefined = clingo_warning_clingo_warning_atom_undefined as isize,
    /// The Same file included multiple times
    FileIncluded = clingo_warning_clingo_warning_file_included as isize,
    /// CSP variable with unbounded domain
    VariableUnbound = clingo_warning_clingo_warning_variable_unbounded as isize,
    /// A global variable in tuple of aggregate element
    GlobalVariable = clingo_warning_clingo_warning_global_variable as isize,
    /// Other kinds of warnings
    Other = clingo_warning_clingo_warning_other as isize,
}

/// Enumeration of different external statements.
#[derive(Debug, Copy, Clone)]
pub enum ExternalType {
    /// Allow an external to be assigned freely
    Free = clingo_external_type_clingo_external_type_free as isize,
    /// Assign an external to true
    True = clingo_external_type_clingo_external_type_true as isize,
    /// Assign an external to false
    False = clingo_external_type_clingo_external_type_false as isize,
    /// No longer treat an atom as external
    Release = clingo_external_type_clingo_external_type_release as isize,
}

/// Enumeration of different heuristic modifiers.
#[derive(Debug, Copy, Clone)]
pub enum HeuristicType {
    /// Set the level of an atom
    Level = clingo_heuristic_type_clingo_heuristic_type_level as isize,
    /// Configure which sign to chose for an atom
    Sign = clingo_heuristic_type_clingo_heuristic_type_sign as isize,
    /// Modify VSIDS factor of an atom
    Factor = clingo_heuristic_type_clingo_heuristic_type_factor as isize,
    /// Modify the initial VSIDS score of an atom
    Init = clingo_heuristic_type_clingo_heuristic_type_init as isize,
    /// Set the level of an atom and choose a positive sign
    True = clingo_heuristic_type_clingo_heuristic_type_true as isize,
    /// Set the level of an atom and choose a negative sign
    False = clingo_heuristic_type_clingo_heuristic_type_false as isize,
}

/// Enumeration of theory term types.
#[derive(Debug, Copy, Clone)]
pub enum TheoryTermType {
    /// A tuple term, e.g., `(1,2,3)`
    Tuple = clingo_theory_term_type_clingo_theory_term_type_tuple as isize,
    /// A list term, e.g., `[1,2,3]`
    List = clingo_theory_term_type_clingo_theory_term_type_list as isize,
    /// A set term, e.g., `{1,2,3}`
    Set = clingo_theory_term_type_clingo_theory_term_type_set as isize,
    /// A function term, e.g., `f(1,2,3)`
    Function = clingo_theory_term_type_clingo_theory_term_type_function as isize,
    /// A number term, e.g., `42`
    Number = clingo_theory_term_type_clingo_theory_term_type_number as isize,
    /// A symbol term, e.g., `c`
    Symbol = clingo_theory_term_type_clingo_theory_term_type_symbol as isize,
}

/// Enumeration for the different model types.
#[derive(Debug, Copy, Clone)]
pub enum ModelType {
    /// The model represents a stable model.
    StableModel = clingo_model_type_clingo_model_type_stable_model as isize,
    /// The model represents a set of brave consequences.
    BraveConsequences = clingo_model_type_clingo_model_type_brave_consequences as isize,
    /// The model represents a set of cautious consequences.
    CautiousConsequences = clingo_model_type_clingo_model_type_cautious_consequences as isize,
}

/// Supported check modes for propagators.
#[derive(Debug, Copy, Clone)]
pub enum PropagatorCheckMode {
    /// Do not call [`Propagator::check()`](trait.Propagator.html#method.check) at all
    None = clingo_propagator_check_mode_clingo_propagator_check_mode_none as isize,
    /// Call [`Propagator::check()`](trait.Propagator.html#method.check) on total assignment
    Total = clingo_propagator_check_mode_clingo_propagator_check_mode_total as isize,
    /// Call [`Propagator::check()`](trait.Propagator.html#method.check) on propagation fixpoints
    Fixpoint = clingo_propagator_check_mode_clingo_propagator_check_mode_fixpoint as isize,
}

bitflags! {
    /// Bit flags describing the entries of a configuration.
    pub struct ConfigurationType: u32 {
        /// The entry is a (string) value.
        const VALUE =
            clingo_configuration_type_clingo_configuration_type_value;
        /// The entry is an array.
        const ARRAY =
            clingo_configuration_type_clingo_configuration_type_array;
        /// The entry is a map.
        const MAP =
            clingo_configuration_type_clingo_configuration_type_map;
    }
}
bitflags! {
    /// Bit flags describing solve modes.
    pub struct SolveMode: u32 {
        /// Enable non-blocking search.
        const ASYNC = clingo_solve_mode_clingo_solve_mode_async;
        /// Yield models in calls to clingo_solve_handle_model.
        const YIELD = clingo_solve_mode_clingo_solve_mode_yield;
    }
}
bitflags! {
    /// Bit flags to select symbols in models.
    pub struct ShowType: u32 {
        /// Select CSP assignments.
        const CSP  = clingo_show_type_clingo_show_type_csp;
        /// Select shown atoms and terms.
        const SHOWN = clingo_show_type_clingo_show_type_shown;
        /// Select all atoms.
        const ATOMS = clingo_show_type_clingo_show_type_atoms;
        /// Select all terms.
        const TERMS = clingo_show_type_clingo_show_type_terms;
        /// Select everything.
        const ALL = clingo_show_type_clingo_show_type_all;
        /// Select false instead of true atoms (Atoms) or terms (Terms)."
        const COMPLEMENT = clingo_show_type_clingo_show_type_complement;
    }
}
bitflags! {
    /// Bit flags that describes the result of a solve call.
    pub struct SolveResult: u32 {
        /// The problem is satisfiable.
        const SATISFIABLE = clingo_solve_result_clingo_solve_result_satisfiable;
        /// The problem is unsatisfiable.
        const UNSATISFIABLE =
            clingo_solve_result_clingo_solve_result_unsatisfiable;
        /// The search space was exhausted.
        const EXHAUSTED = clingo_solve_result_clingo_solve_result_exhausted;
        /// The search was interupted.
        const INTERRUPTED = clingo_solve_result_clingo_solve_result_interrupted;
    }
}
type SolveEventCallback = unsafe extern "C" fn(
    type_: clingo_solve_event_type_t,
    event: *mut ::std::os::raw::c_void,
    data: *mut ::std::os::raw::c_void,
    goon: *mut bool,
) -> bool;
pub trait SolveEventHandler {
    /// Callback function called during search to notify when the search is finished or a model is ready.
    ///
    /// If a (non-recoverable) clingo API function fails in this callback, it must return false.
    /// In case of errors not related to clingo, set error code [`ErrorType::Unknown`](enum.ErrorType.html#variant.Unknown) and return false to stop solving with an error.
    ///
    /// **Attention:** If the search is finished, the model is NULL.
    ///
    /// # Arguments
    ///
    /// * `etype` - the type of the solve event
    /// * `goon` - can be set to false to stop solving
    ///
    /// **Returns** whether the call was successful
    ///
    /// **See:** [`Control::solve()`](struct.Control.html#method.solve)
    fn on_solve_event(&mut self, etype: SolveEventType, goon: &mut bool) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_solve_callback<T: SolveEventHandler>(
        etype: clingo_solve_event_type_t,
        event: *mut ::std::os::raw::c_void,
        data: *mut ::std::os::raw::c_void,
        goon: *mut bool,
    ) -> bool {
        // assert!(!event.is_null());
        let etype = match etype {
            clingo_solve_event_type_clingo_solve_event_type_model => SolveEventType::Model,
            clingo_solve_event_type_clingo_solve_event_type_statistics => {
                SolveEventType::Statistics
            }
            clingo_solve_event_type_clingo_solve_event_type_finish => SolveEventType::Finish,
            x => panic!("Failed to match clingo_solve_event_type: {}.", x),
        };

        let msg = match (data as *mut T).as_mut() {
            Some(event_handler) => {
                if let Some(goon) = goon.as_mut() {
                    return event_handler.on_solve_event(etype, goon);
                } else {
                    "unsafe_solve_callback tried casting a null pointer to &bool."
                }
            }
            None => "unsafe_solve_callback tried casting a null pointer to &SolveEventHandler.",
        };
        set_internal_error(ErrorType::Runtime, msg);
        false
    }
}

type AstCallback = unsafe extern "C" fn(
    arg1: *const clingo_ast_statement_t,
    arg2: *mut ::std::os::raw::c_void,
) -> bool;
pub trait AstStatementHandler {
    /// Callback function called on an ast statement while traversing the ast.
    ///
    /// **Returns** whether the call was successful
    fn on_statement<T>(&mut self, arg1: &AstStatement<T>) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_ast_callback<T: AstStatementHandler>(
        stm: *const clingo_ast_statement_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        if let Some(stm) = (stm as *const AstStatement<T>).as_ref() {
            if let Some(data) = (data as *mut T).as_mut() {
                return data.on_statement(stm);
            } else {
                set_internal_error(
                    ErrorType::Runtime,
                    "unsafe_ast_callback tried casting a null pointer to &mut AstStatementHandler.",
                );
            }
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_ast_callback tried casting a null pointer to &AstStatement.",
            );
        }
        false
    }
}

type LoggingCallback = unsafe extern "C" fn(
    code: clingo_warning_t,
    message: *const ::std::os::raw::c_char,
    data: *mut ::std::os::raw::c_void,
);
/// An instance of this trait has to be registered with a solver to implement a custom logging.
pub trait Logger {
    /// Callback to intercept warning messages.
    ///
    /// # Arguments
    ///
    /// * `code` - associated warning code
    /// * `message` - warning message
    ///
    /// **See:**
    ///
    /// * [`Control::new_with_logger()`](struct.Control.html#method.new_with_logger)
    /// * [`parse_term_with_logger()`](fn.parse_term_with_logger.html)
    /// * [`parse_program_with_logger()`](fn.parse_program_with_logger.html)
    fn log(&mut self, code: Warning, message: &str) {
        print!("warn {:?}: {}", code, message);
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_logging_callback<L: Logger>(
        code: clingo_warning_t,
        message: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
    ) {
        let code = match code as u32 {
            clingo_warning_clingo_warning_atom_undefined => Warning::AtomUndefined,
            clingo_warning_clingo_warning_file_included => Warning::FileIncluded,
            clingo_warning_clingo_warning_global_variable => Warning::GlobalVariable,
            clingo_warning_clingo_warning_operation_undefined => Warning::OperationUndefined,
            clingo_warning_clingo_warning_other => Warning::Other,
            clingo_warning_clingo_warning_runtime_error => Warning::RuntimeError,
            clingo_warning_clingo_warning_variable_unbounded => Warning::VariableUnbound,
            x => panic!("Failed to match clingo_warning: {}.", x),
        };

        assert!(!message.is_null());
        let c_str = CStr::from_ptr(message);
        match c_str.to_str() {
            Ok(message) => {
                if let Some(logger) = (data as *mut L).as_mut() {
                    logger.log(code, message);
                } else {
                    set_internal_error(
                        ErrorType::Runtime,
                        "unsafe_logging_callback tried casting a null pointer to &mut Logger.",
                    );
                }
            }
            Err(e) => {
                set_internal_error(
                    ErrorType::Runtime,
                    "unsafe_logging_callback message with invalid UTF-8 data.",
                );
            }
        }
    }
}

type GroundCallback = unsafe extern "C" fn(
    location: *const clingo_location_t,
    name: *const ::std::os::raw::c_char,
    arguments: *const clingo_symbol_t,
    arguments_size: usize,
    data: *mut ::std::os::raw::c_void,
    symbol_callback: clingo_symbol_callback_t,
    symbol_callback_data: *mut ::std::os::raw::c_void,
) -> bool;
pub trait ExternalFunctionHandler {
    /// Callback function to implement external functions.
    ///
    /// If an external function of form `@name(parameters)` occurs in a logic program,
    /// then this function is called with its location, name, parameters, and a callback to inject symbols as arguments.
    /// The callback can be called multiple times; all symbols passed are injected.
    ///
    /// If a (non-recoverable) clingo API function fails in this callback, for example, the symbol callback, the callback must return false.
    /// In case of errors not related to clingo, this function can set error [`ErrorType::Unknown`](enum.ErrorType.html#variant.Unknown) and return false to stop grounding with an error.
    ///
    /// # Arguments
    ///
    /// * `location` - location from which the external function was called
    /// * `name` - name of the called external function
    /// * `arguments` - arguments of the called external function
    ///
    /// **Returns** a vector of symbols
    ///
    /// **See:** [`Control::ground_with_event_handler()`](struct.Control.html#method.ground_with_event_handler)
    ///
    /// The following example implements the external function `@f()` returning 42.
    /// ```ignore
    /// fn on_external_function(
    ///     &mut self,
    ///     _location: &Location,
    ///     name: &str,
    ///     arguments: &[Symbol],
    /// ) -> Result<Vec<Symbol>,ClingoError> {
    ///     if name == "f" && arguments.len() == 0 {
    ///         let symbol = Symbol::create_number(42);
    ///         Ok(vec![symbol])
    ///     } else {
    ///        Err(ClingoError {
    ///          type_: ErrorType::Runtime,
    ///          msg: "function not found",
    ///        })
    ///    }
    /// }
    /// ```
    fn on_external_function(
        &mut self,
        location: &Location,
        name: &str,
        arguments: &[Symbol],
    ) -> Result<Vec<Symbol>, ClingoError>;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_ground_callback<T: ExternalFunctionHandler>(
        location: *const clingo_location_t,
        name: *const ::std::os::raw::c_char,
        arguments: *const clingo_symbol_t,
        arguments_size: usize,
        data: *mut ::std::os::raw::c_void,
        symbol_callback: clingo_symbol_callback_t,
        symbol_callback_data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!name.is_null());
        let c_str = CStr::from_ptr(name);
        match c_str.to_str() {
            Ok(name) => {
                if let Some(location) = (location as *const Location).as_ref() {
                    if arguments_size > 0 {
                        assert!(!arguments.is_null());
                    }
                    let arguments =
                        std::slice::from_raw_parts(arguments as *const Symbol, arguments_size);

                    if let Some(event_handler) = (data as *mut T).as_mut() {
                        match event_handler.on_external_function(location, name, arguments) {
                            Ok(symbols) => {
                                if let Some(symbol_injector) = symbol_callback {
                                    let v: Vec<clingo_symbol_t> =
                                        symbols.iter().map(|symbol| (*symbol).0).collect();
                                    return symbol_injector(
                                        v.as_slice().as_ptr(),
                                        v.len(),
                                        symbol_callback_data,
                                    );
                                } else {
                                    return true;
                                }
                            }
                            Err(e) => {
                                set_internal_error(e.error_type, e.msg);
                            }
                        }
                    } else {
                        set_internal_error(
                ErrorType::Runtime,"unsafe_ground_callback tried casting a null pointer to &mut ExternalFunctionHandler."
            );
                    }
                } else {
                    set_internal_error(
                        ErrorType::Runtime,
                        "unsafe_ground_callback tried casting a null pointer to &Location.",
                    );
                }
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        false
    }
}

/// Signed integer type used for aspif and solver literals.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Literal(clingo_literal_t);
impl Literal {
    pub fn negate(self) -> Literal {
        Literal(-(self.0))
    }
    pub fn from(Atom(atom): Atom) -> Literal {
        Literal(atom as clingo_literal_t)
    }
    pub fn get_integer(self) -> i32 {
        self.0
    }
}

/// Unsigned integer type used for aspif atoms.
#[derive(Debug, Copy, Clone)]
pub struct Atom(clingo_atom_t);

/// Unsigned integer type used in various places.
#[derive(Debug, Copy, Clone)]
pub struct Id(clingo_id_t);
impl Id {
    pub fn get_integer(self) -> u32 {
        self.0
    }
}

/// A Literal with an associated weight.
#[derive(Debug, Copy, Clone)]
pub struct WeightedLiteral(clingo_weighted_literal);
impl WeightedLiteral {
    pub fn literal(self) -> Literal {
        Literal(self.0.literal)
    }
    pub fn weight(self) -> i32 {
        self.0.weight
    }
}

/// Represents a source code location marking its beginning and end.
///
/// **Note:** Not all locations refer to physical files.
/// By convention, such locations use a name put in angular brackets as filename.
#[derive(Debug, Copy, Clone)]
pub struct Location(clingo_location);
impl Location {
    /// Create a new location.
    ///
    /// # Arguments
    ///
    /// - `begin_file` - the file where the location begins
    /// - `end_file` -  the file where the location ends
    /// - `begin_line` -  the line where the location begins
    /// - `end_line` -  the line where the location ends
    /// - `begin_column` -  the column where the location begins
    /// - `end_column` -  the column where the location ends
    ///
    /// # Errors
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `begin_file` `end_file` or contain a nul byte
    pub fn new(
        begin_file: &str,
        end_file: &str,
        begin_line: usize,
        end_line: usize,
        begin_column: usize,
        end_column: usize,
    ) -> Result<Location, NulError> {
        let begin_file = CString::new(begin_file)?;
        let end_file = CString::new(end_file)?;
        let loc = clingo_location {
            begin_line,
            end_line,
            begin_column,
            end_column,
            begin_file: begin_file.as_ptr(),
            end_file: end_file.as_ptr(),
        };
        Ok(Location(loc))
    }
    /// the file where the location begins
    pub fn begin_file(&self) -> Result<&str, Utf8Error> {
        if self.0.begin_file.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.begin_file) };
            c_str.to_str()
        }
    }
    /// the file where the location ends
    pub fn end_file(&self) -> Result<&str, Utf8Error> {
        if self.0.end_file.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.end_file) };
            c_str.to_str()
        }
    }
    /// the line where the location begins
    pub fn begin_line(&self) -> usize {
        self.0.begin_line
    }
    /// the line where the location ends
    pub fn end_line(&self) -> usize {
        self.0.end_line
    }
    /// the column where the location begins
    pub fn begin_column(&self) -> usize {
        self.0.begin_column
    }
    /// the column where the location ends
    pub fn end_column(&self) -> usize {
        self.0.end_column
    }
}

/// Represents a predicate signature.
///
/// Signatures have a name and an arity, and can be positive or negative (to
/// represent classical negation).
#[derive(Debug, Copy, Clone)]
pub struct Signature(clingo_signature_t);
impl PartialEq for Signature {
    /// Check if two signatures are equal.
    fn eq(&self, other: &Signature) -> bool {
        unsafe { clingo_signature_is_equal_to(self.0, other.0) }
    }
}
impl Eq for Signature {}
impl PartialOrd for Signature {
    /// Compare two signatures.
    ///
    /// Signatures are compared first by sign (unsigned < signed), then by arity,
    /// then by name.
    fn partial_cmp(&self, other: &Signature) -> Option<Ordering> {
        if unsafe { clingo_signature_is_less_than(self.0, other.0) } {
            Some(Ordering::Less)
        } else if unsafe { clingo_signature_is_less_than(other.0, self.0) } {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
impl Hash for Signature {
    /// Calculate a hash code of a signature.
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { clingo_signature_hash(self.0) }.hash(state);
    }
}
impl Signature {
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
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `name` contains a nul byte
    pub fn new(name: &str, arity: u32, positive: bool) -> Result<Signature, Error> {
        let name = CString::new(name)?;
        let mut signature = 0;
        if unsafe { clingo_signature_create(name.as_ptr(), arity, positive, &mut signature) } {
            Ok(Signature(signature))
        } else {
            Err(error())?
        }
    }

    /// Create a statement for the signature.
    pub fn ast_statement(&self, Location(loc): Location) -> AstStatement<Signature> {
        let _bg_union_2 = clingo_ast_statement__bindgen_ty_1 {
            project_signature: self.0 as clingo_signature_t,
        };
        let stm = clingo_ast_statement_t {
            location: loc,
            type_: ast::StatementType::ProjectAtomSignature as clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        AstStatement {
            data: stm,
            phantom: PhantomData,
        }
    }

    /// Get the name of a signature.
    ///
    /// # Errors
    ///
    /// - [`Utf8Error`](https://doc.rust-lang.org/std/str/struct.Utf8Error.html)
    pub fn name(&self) -> Result<&str, Utf8Error> {
        let char_ptr: *const c_char = unsafe { clingo_signature_name(self.0) };
        if char_ptr.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            c_str.to_str()
        }
    }

    /// Get the arity of a signature.
    pub fn arity(self) -> u32 {
        unsafe { clingo_signature_arity(self.0) }
    }

    /// Whether the signature is positive (is not classically negated).
    pub fn is_positive(self) -> bool {
        unsafe { clingo_signature_is_positive(self.0) }
    }

    /// Whether the signature is negative (is classically negated).
    pub fn is_negative(self) -> bool {
        unsafe { clingo_signature_is_negative(self.0) }
    }
}

/// Represents a symbol.
///
/// This includes numbers, strings, functions (including constants when
/// arguments are empty and tuples when the name is empty), \#inf and \#sup.
#[derive(Debug, Copy, Clone)]
pub struct Symbol(clingo_symbol_t);
impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        unsafe { clingo_symbol_is_equal_to(self.0, other.0) }
    }
}
impl Eq for Symbol {}
impl PartialOrd for Symbol {
    /// Compare two symbols.
    ///
    /// Symbols are first compared by type.  If the types are equal, the values are
    /// compared (where strings are compared using strcmp).  Functions are first
    /// compared by signature and then lexicographically by arguments.
    fn partial_cmp(&self, other: &Symbol) -> Option<Ordering> {
        if unsafe { clingo_symbol_is_less_than(self.0, other.0) } {
            Some(Ordering::Less)
        } else if unsafe { clingo_symbol_is_less_than(other.0, self.0) } {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
impl Hash for Symbol {
    /// Calculate a hash code of a symbol.
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { clingo_symbol_hash(self.0) }.hash(state);
    }
}
impl Symbol {
    /// Construct a symbol representing a number.
    pub fn create_number(number: i32) -> Symbol {
        let mut symbol = 0 as clingo_symbol_t;
        unsafe { clingo_symbol_create_number(number, &mut symbol) };
        Symbol(symbol)
    }

    /// Construct a symbol representing \#sup.
    pub fn create_supremum() -> Symbol {
        let mut symbol = 0 as clingo_symbol_t;
        unsafe { clingo_symbol_create_supremum(&mut symbol) };
        Symbol(symbol)
    }

    /// Construct a symbol representing \#inf
    pub fn create_infimum() -> Symbol {
        let mut symbol = 0 as clingo_symbol_t;
        unsafe { clingo_symbol_create_infimum(&mut symbol) };
        Symbol(symbol)
    }

    /// Construct a symbol representing a string.
    ///
    /// # Arguments
    ///
    /// * `string` - the string
    ///
    /// #  Errors:
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `string` contains a nul byte
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn create_string(string: &str) -> Result<Symbol, Error> {
        let mut symbol = 0 as clingo_symbol_t;
        let c_str = CString::new(string)?;
        if unsafe { clingo_symbol_create_string(c_str.as_ptr(), &mut symbol) } {
            Ok(Symbol(symbol))
        } else {
            Err(error())?
        }
    }

    /// Construct a symbol representing an id.
    ///
    /// **Note:** This is just a shortcut for [`create_function()`](struct.Symbol.html#method.create_function) with
    /// empty arguments.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the symbol
    /// * `positive` - whether the symbol has a classical negation sign
    ///
    /// # Errors
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `name` contains a nul byte
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn create_id(name: &str, positive: bool) -> Result<Symbol, Error> {
        let mut symbol = 0 as clingo_symbol_t;
        let name = CString::new(name)?;
        if unsafe { clingo_symbol_create_id(name.as_ptr(), positive, &mut symbol) } {
            Ok(Symbol(symbol))
        } else {
            Err(error())?
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
    /// * `positive` - whether the symbol has a classical negation sign
    ///
    /// # Errors
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `name` contains a nul byte
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn create_function(
        name: &str,
        arguments: &[Symbol],
        positive: bool,
    ) -> Result<Symbol, Error> {
        let mut symbol = 0 as clingo_symbol_t;
        let name = CString::new(name)?;
        if unsafe {
            clingo_symbol_create_function(
                name.as_ptr(),
                arguments.as_ptr() as *const clingo_symbol_t,
                arguments.len(),
                positive,
                &mut symbol,
            )
        } {
            Ok(Symbol(symbol))
        } else {
            Err(error())?
        }
    }

    //     pub fn term(&self, Location(location): Location) -> ast::Term {
    //         let _bg_union_1 = clingo_ast_term__bindgen_ty_1 { symbol: self.0 };
    //         let term = clingo_ast_term_t {
    //             location: location,
    //             type_: ast::TermType::Symbol as clingo_ast_term_type_t,
    //             __bindgen_anon_1: _bg_union_1,
    //         };
    //         ast::Term(term)
    //     }

    /// Get the number of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type [`SymbolType::Number`](enum.SymbolType.html#variant.Number)
    pub fn number(self) -> Result<i32, ClingoError> {
        let mut number = 0;
        if unsafe { clingo_symbol_number(self.0, &mut number) } {
            Ok(number)
        } else {
            Err(error())
        }
    }

    /// Get the name of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type [`SymbolType::Function`](enum.SymbolType.html#variant.Function)
    /// - [`Utf8Error`](https://doc.rust-lang.org/std/str/struct.Utf8Error.html)
    pub fn name(&self) -> Result<&str, Error> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_symbol_name(self.0, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Ok(c_str.to_str()?)
        } else {
            Err(error())?
        }
    }

    /// Get the string of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type [`SymbolType::String`](enum.SymbolType.html#variant.String)
    /// - [`Utf8Error`](https://doc.rust-lang.org/std/str/struct.Utf8Error.html)
    pub fn string(&self) -> Result<&str, Error> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_symbol_string(self.0, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Ok(c_str.to_str()?)
        } else {
            Err(error())?
        }
    }

    /// Check if a function is positive (does not have a sign).
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type [`SymbolType::Function`](enum.SymbolType.html#variant.Function)
    pub fn is_positive(self) -> Result<bool, ClingoError> {
        let mut positive = false;
        if unsafe { clingo_symbol_is_positive(self.0, &mut positive) } {
            Ok(positive)
        } else {
            Err(error())
        }
    }

    /// Check if a function is negative (has a sign).
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type [`SymbolType::Function`](enum.SymbolType.html#variant.Function)
    pub fn is_negative(self) -> Result<bool, ClingoError> {
        let mut negative = false;
        if unsafe { clingo_symbol_is_negative(self.0, &mut negative) } {
            Ok(negative)
        } else {
            Err(error())
        }
    }

    /// Get the arguments of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type [`SymbolType::Function`](enum.SymbolType.html#variant.Function)
    pub fn arguments(self) -> Result<Vec<Symbol>, ClingoError> {
        let mut symbol_ptr = std::ptr::null() as *const clingo_symbol_t;
        let mut size: usize = 0;
        if unsafe { clingo_symbol_arguments(self.0, &mut symbol_ptr, &mut size) } {
            let mut symbols = Vec::<Symbol>::with_capacity(size);
            for _ in 0..size {
                let nsymbol = unsafe { *symbol_ptr };
                symbols.push(Symbol(nsymbol));
                symbol_ptr = unsafe { symbol_ptr.offset(1) };
            }
            Ok(symbols)
        } else {
            Err(error())
        }
    }

    /// Get the type of a symbol.
    ///
    /// # Errors
    ///
    /// - may failed to match clingo symbol type
    pub fn symbol_type(self) -> SymbolType {
        let stype = unsafe { clingo_symbol_type(self.0) };
        match stype as u32 {
            clingo_symbol_type_clingo_symbol_type_infimum => SymbolType::Infimum,
            clingo_symbol_type_clingo_symbol_type_number => SymbolType::Number,
            clingo_symbol_type_clingo_symbol_type_string => SymbolType::String,
            clingo_symbol_type_clingo_symbol_type_function => SymbolType::Function,
            clingo_symbol_type_clingo_symbol_type_supremum => SymbolType::Supremum,
            x => panic!("Failed to match clingo_symbol_type: {}.", x),
        }
    }

    /// Get the string representation of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`Utf8Error`](https://doc.rust-lang.org/std/str/struct.Utf8Error.html)
    pub fn to_string(self) -> Result<String, Error> {
        let mut size: usize = 0;
        if unsafe { clingo_symbol_to_string_size(self.0, &mut size) } {
            let a1 = vec![1; size];
            let cstring = unsafe { CString::from_vec_unchecked(a1) };
            if unsafe { clingo_symbol_to_string(self.0, cstring.as_ptr() as *mut c_char, size) } {
                match cstring.into_string() {
                    Ok(string) => Ok(string.trim_matches(char::from(0)).to_string()),
                    Err(e) => Err(e)?,
                }
            } else {
                Err(error())?
            }
        } else {
            Err(error())?
        }
    }
}

// impl Logger for u32 {
//     fn log(&mut self, code: Warning, message: &str) {
//         print!("log {}: {}", self, message);
//         println!("warn: {:?}", code);
//         *self += 1;
//     }
// }

/// Parse the given program and return an abstract syntax tree for each statement via a callback.
///
/// # Arguments
///
/// * `program` - the program in gringo syntax
/// * `handler` - implementing the trait [`AstStatementHandler`](trait.AstStatementHandler.html)
///
/// # Errors
///
/// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `program` contains a nul byte
/// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if parsing fails
/// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
pub fn parse_program<T: AstStatementHandler>(program: &str, handler: &mut T) -> Result<(), Error> {
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let program = CString::new(program)?;
    let data = handler as *mut T;
    if unsafe {
        clingo_parse_program(
            program.as_ptr(),
            Some(T::unsafe_ast_callback::<T> as AstCallback),
            data as *mut ::std::os::raw::c_void,
            logger,
            logger_data,
            0,
        )
    } {
        Ok(())
    } else {
        Err(error())?
    }
}

/// Parse the given program and return an abstract syntax tree for each statement via a callback.
///
/// # Arguments
///
/// * `program` - the program in gringo syntax
/// * `handler` - implementating the trait [`AstStatementHandler`](trait.AstStatementHandler.html)
/// * `logger` - implementing the trait [`Logger`](trait.Logger.html) to report messages during parsing
/// * `message_limit` - the maximum number of times the logger is called
///
/// # Errors
///
/// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `program` contains a nul byte
/// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if parsing fails
/// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
pub fn parse_program_with_logger<T: AstStatementHandler, L: Logger>(
    program: &str,
    handler: &mut T,
    logger: &mut L,
    message_limit: u32,
) -> Result<(), Error> {
    let handler_data = handler as *mut T;
    let logger_data = logger as *mut L;
    let program = CString::new(program)?;
    if unsafe {
        clingo_parse_program(
            program.as_ptr(),
            Some(T::unsafe_ast_callback::<T> as AstCallback),
            handler_data as *mut ::std::os::raw::c_void,
            Some(L::unsafe_logging_callback::<L> as LoggingCallback),
            logger_data as *mut ::std::os::raw::c_void,
            message_limit,
        )
    } {
        Ok(())
    } else {
        Err(error())?
    }
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

/// Struct used to specify the program parts that have to be grounded.
///
/// Programs may be structured into parts, which can be grounded independently with [`Control::ground()`](struct.Control.html#method.ground).
/// Program parts are mainly interesting for incremental grounding and multi-shot solving.
/// For single-shot solving, program parts are not needed.
///
/// **Note:** Parts of a logic program without an explicit `#program`
/// specification are by default put into a program called `base` - without
/// arguments.
///
/// **See:** [`Control::ground()`](struct.Control.html#method.ground)
pub struct Part<'a> {
    name: CString,
    params: &'a [Symbol],
}
impl<'a> Part<'a> {
    /// Create a new program part object.
    ///
    /// # Arguments
    ///
    /// * `name` - the identifier of the program
    /// * `params` - the parameter of the program
    ///
    /// # Errors
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `name` contains a nul byte
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if argument parsing fails
    /// - [`WrapperError`](struct.WrapperError.html)
    pub fn new(name: &str, params: &'a [Symbol]) -> Result<Part<'a>, Error> {
        Ok(Part {
            name: CString::new(name)?,
            params,
        })
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
fn error() -> ClingoError {
    ClingoError {
        error_type: ErrorType::from(unsafe { clingo_error_code() }),
        msg: error_message(),
    }
}

/// Get the last error message set if an API call fails.
///
/// **Note:** Each thread has its own local error message.
fn error_message() -> &'static str {
    let char_ptr: *const c_char = unsafe { clingo_error_message() };
    if char_ptr.is_null() {
        "Ooops, original error message is null."
    } else {
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        c_str
            .to_str()
            .unwrap_or("Ooops, original error message was no valid utf8 string.")
    }
}

/// Set an error code and message in the active thread.
///
/// # Errors
///
/// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `name` contains a nul byte
pub fn set_error(code: ErrorType, message: &str) -> Result<(), NulError> {
    let message = CString::new(message)?;
    unsafe { clingo_set_error(code as clingo_error_t, message.as_ptr()) }
    Ok(())
}

fn set_internal_error(code: ErrorType, message: &'static str) {
    // unwrap won't panic, because the function only used internally on valid UTF-8 strings
    let message = CString::new(message).unwrap();
    unsafe { clingo_set_error(code as clingo_error_t, message.as_ptr()) }
}

/// An instance of this trait has to be registered with a solver to implement a custom propagator.
///
/// For all functions exist default implementations and they must not be implemented manually.
pub trait Propagator {
    /// This function is called once before each solving step.
    /// It is used to map relevant program literals to solver literals, add watches for solver
    /// literals, and initialize the data structures used during propagation.
    ///
    /// **Note:** This is the last point to access symbolic and theory atoms.
    /// Once the search has started, they are no longer accessible.
    ///
    /// # Arguments
    ///
    /// * `init` - initizialization object
    ///
    /// **Returns** whether the call was successful
    fn init(&mut self, _init: &mut PropagateInit) -> bool {
        true
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_init<T: Propagator>(
        init: *mut clingo_propagate_init_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!init.is_null());
        if let Some(init) = (init as *mut PropagateInit).as_mut() {
            assert!(!data.is_null());
            if let Some(propagator) = (data as *mut T).as_mut() {
                return propagator.init(init);
            } else {
                set_internal_error(
                    ErrorType::Runtime,
                    "unsafe_init tried casting a null pointer to &mut Propagator.",
                );
            }
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_init tried casting a null pointer to &mut PropagateInit.",
            );
        }
        false
    }
    /// Can be used to propagate solver literals given a
    /// [partial assignment](struct.Assignment.html).
    ///
    /// Called during propagation with a non-empty array of
    /// [watched solver literals](struct.PropagateInit.html#method.add_watch)
    /// that have been assigned to true since the last call to either propagate, undo, (or the start
    /// of the search) - the change set.
    /// Only watched solver literals are contained in the change set.
    /// Each literal in the change set is true w.r.t. the current
    /// [assignment](struct.Assignment.html).
    /// [`PropagateControl::add_clause()`](struct.PropagateControl.html#method.add_clause) can be
    /// used to add clauses.
    /// If a clause is unit resulting, it can be propagated using
    /// [`PropagateControl::propagate()`](struct.PropagateControl.html#method.propagate).
    /// If the result of either of the two methods is false, the propagate function must return
    /// immediately.
    ///
    /// The following snippet shows how to use the methods to add clauses and propagate consequences
    /// within the callback.
    /// The important point is to return true (true to indicate there was no error) if the result of
    /// either of the methods is false.
    /// ```ignore
    /// let clause= &[ ... ];
    /// // add a clause
    /// if let Ok(x) = control.add_clause(clause, ClauseType::Learnt) {
    ///     if !x {
    ///         true
    ///     }
    /// } else {
    ///     false
    /// }
    /// // propagate its consequences
    /// if let Ok(x) = control.propagate() {
    ///     if !x {
    ///         true
    ///     }
    /// } else {
    ///     false
    /// }
    /// // add further clauses and propagate them
    /// ...
    /// true
    /// ```
    ///
    /// **Note:**
    /// This function can be called from different solving threads.
    /// Each thread has its own assignment and id, which can be obtained using
    /// [`PropagateControl::thread_id()`](struct.PropagateControl.html#method.thread_id).
    ///
    /// # Arguments
    ///
    /// * `control` - control object for the target solver
    /// * `changes` - the change set
    ///
    /// **Returns** whether the call was successful
    fn propagate(&mut self, _control: &mut PropagateControl, _changes: &[Literal]) -> bool {
        true
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_propagate<T: Propagator>(
        control: *mut clingo_propagate_control_t,
        changes: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!control.is_null());
        if let Some(control) = (control as *mut PropagateControl).as_mut() {
            assert!(!changes.is_null());
            let changes = std::slice::from_raw_parts(changes as *const Literal, size);

            assert!(!data.is_null());
            if let Some(propagator) = (data as *mut T).as_mut() {
                return propagator.propagate(control, changes);
            } else {
                set_internal_error(
                    ErrorType::Runtime,
                    "unsafe_propagate tried casting a null pointer to &mut Propagator.",
                );
            }
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_propagate tried casting a null pointer to &mut PropagateControl.",
            );
        }
        false
    }
    /// Called whenever a solver undoes assignments to watched solver literals.
    ///
    /// This callback is meant to update assignment dependent state in the propagator.
    ///
    /// **Note:** No clauses must be propagated in this callback.
    ///
    /// # Arguments
    ///
    /// * `control` - control object for the target solver
    /// * `changes` - the change set
    fn undo(&mut self, _control: &mut PropagateControl, _changes: &[Literal]) -> bool {
        true
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_undo<T: Propagator>(
        control: *mut clingo_propagate_control_t,
        changes: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        if let Some(control) = (control as *mut PropagateControl).as_mut() {
            assert!(!changes.is_null());
            let changes = std::slice::from_raw_parts(changes as *const Literal, size);

            if let Some(propagator) = (data as *mut T).as_mut() {
                return propagator.undo(control, changes);
            } else {
                set_internal_error(
                    ErrorType::Runtime,
                    "unsafe_undo tried casting a null pointer to &mut Propagator.",
                );
            }
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_undo tried casting a null pointer to &mut PropagateControl.",
            );
        }
        false
    }
    /// This function is similar to
    /// [`PropagateControl::propagate()`](struct.PropagateControl.html#method.propagate) but is only
    /// called on total assignments without a change set.
    ///
    /// When exactly this function is called, can be configured using the
    /// [`PropagateInit::set_check_mode()`](struct.PropagateInit.html#method.set_check_mode)
    /// function.
    ///
    /// **Note:** This function is called even if no watches have been added.
    ///
    /// # Arguments
    ///
    /// * `control` - control object for the target solver
    ///
    /// **Returns** whether the call was successful
    fn check(&mut self, _control: &mut PropagateControl) -> bool {
        true
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_check<T: Propagator>(
        control: *mut clingo_propagate_control_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        if let Some(control) = (control as *mut PropagateControl).as_mut() {
            if let Some(propagator) = (data as *mut T).as_mut() {
                return propagator.check(control);
            } else {
                set_internal_error(
                    ErrorType::Runtime,
                    "unsafe_check tried casting a null pointer to &mut Propagator.",
                );
            }
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_check tried casting a null pointer to &mut PropagateControl.",
            );
        }
        false
    }
}

/// Control object holding grounding and solving state.
#[derive(Debug)]
pub struct Control {
    ctl: NonNull<clingo_control_t>,
}
impl Drop for Control {
    fn drop(&mut self) {
        //         println!("drop Control");
        unsafe { clingo_control_free(self.ctl.as_ptr()) }
    }
}
impl Control {
    /// Create a new control object.
    ///
    /// **Note:** Only gringo options (without `--output`) and clasp's options are supported as
    /// arguments,  except basic options such as `--help`.
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
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if an argument contains a nul byte
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if argument parsing fails
    /// - [`WrapperError`](struct.WrapperError.html)
    pub fn new(arguments: std::vec::Vec<String>) -> Result<Control, Error> {
        let logger = None;
        let logger_data = std::ptr::null_mut();

        // create a vector of zero terminated strings
        let mut args = vec![];
        for arg in arguments {
            args.push(CString::new(arg)?);
        }

        // convert the strings to raw pointers
        let c_args = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let mut ctl_ptr = unsafe { mem::uninitialized() };

        if unsafe {
            clingo_control_new(
                c_args.as_ptr(),
                c_args.len(),
                logger,
                logger_data,
                0,
                &mut ctl_ptr,
            )
        } {
            match NonNull::new(ctl_ptr) {
                Some(ctl) => Ok(Control { ctl }),
                None => Err(WrapperError {
                    msg: "tried creating NonNull from a null pointer.",
                })?,
            }
        } else {
            Err(error())?
        }
    }

    /// Create a new control object.
    ///
    /// **Note:** Only gringo options (without `--output`) and clasp's options are supported as
    /// arguments,
    /// except basic options such as `--help`.
    /// Furthermore, a control object is blocked while a search call is active;
    /// you must not call any member function during search.
    ///
    /// # Arguments
    ///
    /// * `arguments` - string array of command line arguments
    /// * `logger` - callback functions for warnings and info messages
    /// * `message_limit` - maximum number of times the logger callback is called
    ///
    /// # Errors
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if an argument contains a nul byte
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if argument parsing fails

    pub fn new_with_logger<L: Logger>(
        arguments: Vec<String>,
        logger: &mut L,
        message_limit: u32,
    ) -> Result<Control, Error> {
        let mut args = vec![];
        for arg in arguments {
            args.push(CString::new(arg)?);
        }

        // convert the strings to raw pointers
        let c_args = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let mut ctl_ptr = unsafe { mem::uninitialized() };

        let data = logger as *mut L;
        if unsafe {
            clingo_control_new(
                c_args.as_ptr(),
                c_args.len(),
                Some(L::unsafe_logging_callback::<L> as LoggingCallback),
                data as *mut ::std::os::raw::c_void,
                message_limit,
                &mut ctl_ptr,
            )
        } {
            match NonNull::new(ctl_ptr) {
                Some(ctl) => Ok(Control { ctl }),
                None => Err(WrapperError {
                    msg: "tried creating NonNull from a null pointer.",
                })?,
            }
        } else {
            Err(error())?
        }
    }

    //NODO: pub fn clingo_control_load(control: *mut Control, file: *const c_char) -> bool;

    /// Extend the logic program with the given non-ground logic program in string form.
    ///
    /// This function puts the given program into a block of form: `#program name(parameters).`
    ///
    /// After extending the logic program, the corresponding program parts are typically grounded
    /// with `ground()`.
    ///
    /// # Arguments
    ///
    /// * `name` - name of the program block
    /// * `parameters` - string array of parameters of the program block
    /// * `program` - string representation of the program
    ///
    /// # Errors
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if a any argument contains a nul byte
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if parsing fails
    pub fn add(&mut self, name: &str, parameters: &[&str], program: &str) -> Result<(), Error> {
        let name = CString::new(name)?;

        let program = CString::new(program)?;
        let program_ptr = program.as_ptr();

        let parameters_size = parameters.len();

        // create a vector of zero terminated strings
        let mut l_parameters = vec![];
        for arg in parameters {
            l_parameters.push(CString::new(*arg)?);
        }

        // convert the strings to raw pointers
        let c_parameters = l_parameters
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        if unsafe {
            clingo_control_add(
                self.ctl.as_ptr(),
                name.as_ptr(),
                c_parameters.as_ptr(),
                parameters_size,
                program_ptr,
            )
        } {
            Ok(())
        } else {
            Err(error())?
        }
    }

    /// Ground the selected [parts](struct.Part.html) of the current (non-ground) logic
    /// program.
    ///
    /// After grounding, logic programs can be solved with [`solve()`](struct.Control.html.method.solve).
    ///
    /// **Note:** Parts of a logic program without an explicit `#program`
    /// specification are by default put into a program called `base` - without
    /// arguments.
    ///
    /// # Arguments
    ///
    /// * `parts` -  array of [parts](struct.Part.html) to ground
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn ground(&mut self, parts: &[Part]) -> Result<(), ClingoError> {
        let parts_size = parts.len();
        let parts = parts
            .iter()
            .map(|arg| arg.from())
            .collect::<Vec<clingo_part>>();

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
            Err(error())
        }
    }

    /// Ground the selected [parts](struct.Part.html) of the current (non-ground) logic
    /// program.
    ///
    /// After grounding, logic programs can be solved with [`solve()`](struct.Control.html.method.solve).
    ///
    /// **Note:** Parts of a logic program without an explicit `#program`
    /// specification are by default put into a program called `base` - without
    /// arguments.
    ///
    /// # Arguments
    ///
    /// * `parts` - array of [parts](struct.Part.html) to ground
    /// * `handler` - implementing the trait [`ExternalFunctionHandler`](trait.ExternalFunctionHandler.html) to evaluate external functions
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    //TODO: the error code set in ExternalFunctionHandler is overwritten
    pub fn ground_with_event_handler<T: ExternalFunctionHandler>(
        &mut self,
        parts: &[Part],
        handler: &mut T,
    ) -> Result<(), ClingoError> {
        let parts_size = parts.len();
        let parts = parts
            .iter()
            .map(|arg| arg.from())
            .collect::<Vec<clingo_part>>();

        let data = handler as *mut T;
        if unsafe {
            clingo_control_ground(
                self.ctl.as_ptr(),
                parts.as_ptr(),
                parts_size,
                Some(T::unsafe_ground_callback::<T> as GroundCallback),
                data as *mut ::std::os::raw::c_void,
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Solve the currently [grounded](struct.Control.html#method.ground) logic program
    /// enumerating its models.
    ///
    /// # Arguments
    ///
    /// * `mode` - configures the search mode
    /// * `assumptions` - array of assumptions to solve under
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving could not be started
    /// - [`WrapperError`](struct.WrapperError.html)
    pub fn solve(
        &mut self,
        mode: SolveMode,
        assumptions: &[Literal],
    ) -> Result<SolveHandle, Error> {
        let mut handle = std::ptr::null_mut() as *mut clingo_solve_handle_t;
        if unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode.bits(),
                assumptions.as_ptr() as *const clingo_literal_t,
                assumptions.len(),
                None,
                std::ptr::null_mut() as *mut ::std::os::raw::c_void,
                &mut handle,
            )
        } {
            match unsafe { handle.as_mut() } {
                Some(handle_ref) => Ok(SolveHandle { theref: handle_ref }),
                None => Err(WrapperError {
                    msg: "tried casting a null pointer to &mut clingo_solve_handle.",
                })?,
            }
        } else {
            Err(error())?
        }
    }

    /// Solve the currently [grounded](struct.Control.html#method.ground) logic program
    /// enumerating its models.
    ///
    /// # Arguments
    ///
    /// * `mode` - configures the search mode
    /// * `assumptions` - array of assumptions to solve under
    /// * `handler` - implementing the trait [`SolveEventHandler`](trait.SolveEventHandler.html)
    ///
    /// # Errors
    ///
    /// - [`WrapperError`](struct.WrapperError.html)
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving could not be started
    pub fn solve_with_event_handler<T: SolveEventHandler>(
        &mut self,
        mode: SolveMode,
        assumptions: &[Literal],
        handler: &mut T,
    ) -> Result<SolveHandle, Error> {
        let mut handle = std::ptr::null_mut() as *mut clingo_solve_handle_t;
        let data = handler as *mut T;
        if unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode.bits(),
                assumptions.as_ptr() as *const clingo_literal_t,
                assumptions.len(),
                Some(T::unsafe_solve_callback::<T> as SolveEventCallback),
                data as *mut ::std::os::raw::c_void,
                &mut handle,
            )
        } {
            match unsafe { handle.as_mut() } {
                Some(handle_ref) => Ok(SolveHandle { theref: handle_ref }),
                None => Err(WrapperError {
                    msg: "tried casting a null pointer to &mut clingo_solve_handle.",
                })?,
            }
        } else {
            Err(error())?
        }
    }

    /// Clean up the domains of clingo's grounding component using the solving
    /// component's top level assignment.
    ///
    /// This function removes atoms from domains that are false and marks atoms as
    /// facts that are true.  With multi-shot solving, this can result in smaller
    /// groundings because less rules have to be instantiated and more
    /// simplifications can be applied.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn cleanup(&mut self) -> Result<(), ClingoError> {
        if unsafe { clingo_control_cleanup(self.ctl.as_ptr()) } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Assign a truth value to an external atom.
    ///
    /// If a negative literal is passed, the corresponding atom is assigned the
    /// inverted truth value.
    ///
    /// If the atom does not exist or is not external, this is a noop.
    ///
    /// # Arguments
    ///
    /// * `literal` - literal to assign
    /// * `value` - the truth value
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn assign_external(
        &mut self,
        literal: Literal,
        value: TruthValue,
    ) -> Result<(), ClingoError> {
        if unsafe {
            clingo_control_assign_external(
                self.ctl.as_ptr(),
                literal.0,
                value as clingo_truth_value_t,
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Release an external atom.
    ///
    /// If a negative literal is passed, the corresponding atom is released.
    ///
    /// After this call, an external atom is no longer external and subject to
    /// program simplifications.  If the atom does not exist or is not external,
    /// this is a noop.
    ///
    /// # Arguments
    ///
    /// * `literal` - literal to release
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn release_external(&mut self, Literal(literal): Literal) -> Result<(), ClingoError> {
        if unsafe { clingo_control_release_external(self.ctl.as_ptr(), literal) } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Register a custom propagator with the control object.
    ///
    /// If the sequential flag is set to true, the propagator is called
    /// sequentially when solving with multiple threads.
    ///
    /// # Arguments
    ///
    /// * `propagator` - implementing the trait [`Propagator`](trait.Propagator.html)
    /// * `sequential` - whether the propagator should be called sequentially
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn register_propagator<T: Propagator>(
        &mut self,
        propagator: &mut T,
        sequential: bool,
    ) -> Result<(), ClingoError> {
        let data_ptr = propagator as *mut T;
        let propagator = clingo_propagator_t {
            init: Some(T::unsafe_init::<T>),
            propagate: Some(T::unsafe_propagate::<T>),
            undo: Some(T::unsafe_undo::<T>),
            check: Some(T::unsafe_check::<T>),
        };
        if unsafe {
            clingo_control_register_propagator(
                self.ctl.as_ptr(),
                &propagator,
                data_ptr as *mut ::std::os::raw::c_void,
                sequential,
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Check if the solver has determined that the internal program representation is conflicting.
    ///
    /// If this function returns true, solve calls will return immediately with an unsatisfiable solve result.
    /// Note that conflicts first have to be detected, e.g. -
    /// initial unit propagation results in an empty clause,
    /// or later if an empty clause is resolved during solving.
    /// Hence, the function might return false even if the problem is unsatisfiable.
    pub fn clingo_control_is_conflicting(&self) -> bool {
        unsafe { clingo_control_is_conflicting(self.ctl.as_ptr()) }
    }

    /// Get a statistics object to inspect solver statistics.
    ///
    /// Statistics are updated after a solve call.
    ///
    /// **Attention:**
    /// The level of detail of the statistics depends on the stats option
    /// (which can be set using [`Configuration`](struct.Configuration.html) or passed as an
    /// option when [creating the control object](struct.Control.html#method.new)).
    /// The default level zero only provides basic statistics,
    /// level one provides extended and accumulated statistics,
    /// and level two provides per-thread statistics.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn statistics(&self) -> Result<&Statistics, Error> {
        let mut stat = std::ptr::null() as *const clingo_statistics_t;
        if unsafe { clingo_control_statistics(self.ctl.as_ptr(), &mut stat) } {
            match unsafe { (stat as *mut Statistics).as_ref() } {
                Some(x) => Ok(x),
                None => Err(WrapperError {
                    msg: "tried casting a null pointer to &Statistics.",
                })?,
            }
        } else {
            Err(error())?
        }
    }

    /// Interrupt the active solve call (or the following solve call right at the beginning).
    pub fn interrupt(&mut self) {
        unsafe {
            clingo_control_interrupt(self.ctl.as_ptr());
        }
    }

    /// Get a configuration object to change the solver configuration.
    pub fn configuration_mut(&mut self) -> Option<&mut Configuration> {
        let mut conf = std::ptr::null_mut() as *mut clingo_configuration_t;
        if unsafe { clingo_control_configuration(self.ctl.as_ptr(), &mut conf) } {
            unsafe { (conf as *mut Configuration).as_mut() }
        } else {
            None
        }
    }

    /// Get a configuration object to change the solver configuration.
    pub fn configuration(&self) -> Option<&Configuration> {
        let mut conf = std::ptr::null_mut() as *mut clingo_configuration_t;
        if unsafe { clingo_control_configuration(self.ctl.as_ptr(), &mut conf) } {
            unsafe { (conf as *const Configuration).as_ref() }
        } else {
            None
        }
    }

    /// Configure how learnt constraints are handled during enumeration.
    ///
    /// If the enumeration assumption is enabled, then all information learnt from
    /// the solver's various enumeration modes is removed after a solve call. This
    /// includes enumeration of cautious or brave consequences, enumeration of
    /// answer sets with or without projection, or finding optimal models, as well
    /// as clauses added with clingo_solve_control_add_clause().
    ///
    /// **Attention:** For practical purposes, this option is only interesting for single-shot
    /// solving or before the last solve call to squeeze out a tiny bit of performance.
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

    /// Return the symbol for a constant definition of form: `#const name = symbol`.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the constant if it exists
    pub fn get_const(&self, name: &str) -> Option<Symbol> {
        if let Ok(name) = CString::new(name) {
            let mut symbol = 0 as clingo_symbol_t;
            if unsafe { clingo_control_get_const(self.ctl.as_ptr(), name.as_ptr(), &mut symbol) } {
                Some(Symbol(symbol))
            } else {
                None
            }
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
    /// **See:** [`Part::get_const()`](struct.Part.html#method.get_const)
    pub fn has_const(&self, name: &str) -> bool {
        if let Ok(name) = CString::new(name) {
            let mut exist = false;
            if unsafe { clingo_control_has_const(self.ctl.as_ptr(), name.as_ptr(), &mut exist) } {
                return exist;
            }
        }
        false
    }

    /// Get an object to inspect symbolic atoms (the relevant Herbrand base) used
    /// for grounding.
    pub fn symbolic_atoms(&self) -> Option<&SymbolicAtoms> {
        let mut atoms = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
        if unsafe { clingo_control_symbolic_atoms(self.ctl.as_ptr(), &mut atoms) } {
            unsafe { (atoms as *const SymbolicAtoms).as_ref() }
        } else {
            None
        }
    }

    /// Get an object to inspect theory atoms that occur in the grounding.
    pub fn theory_atoms(&self) -> Option<&TheoryAtoms> {
        let mut atoms = std::ptr::null_mut() as *mut clingo_theory_atoms_t;
        if unsafe { clingo_control_theory_atoms(self.ctl.as_ptr(), &mut atoms) } {
            unsafe { (atoms as *const TheoryAtoms).as_ref() }
        } else {
            None
        }
    }

    /// Register a program observer with the control object.
    ///
    /// # Arguments
    ///
    /// * `observer` - the observer to register
    /// * `replace` - just pass the grounding to the observer but not the solver
    ///
    /// **Returns** whether the call was successful
    pub fn register_observer<T: GroundProgramObserver>(
        &mut self,
        observer: &mut T,
        replace: bool,
    ) -> bool {
        let data = observer as *mut T;
        let gpo = clingo_ground_program_observer_t {
            init_program: Some(T::unsafe_init_program::<T>),
            begin_step: Some(T::unsafe_begin_step::<T>),
            end_step: Some(T::unsafe_end_step::<T>),
            rule: Some(T::unsafe_rule::<T>),
            weight_rule: Some(T::unsafe_weight_rule::<T>),
            minimize: Some(T::unsafe_minimize::<T>),
            project: Some(T::unsafe_project::<T>),
            output_atom: Some(T::unsafe_output_atom::<T>),
            output_term: Some(T::unsafe_output_term::<T>),
            output_csp: Some(T::unsafe_output_csp::<T>),
            external: Some(T::unsafe_external::<T>),
            assume: Some(T::unsafe_assume::<T>),
            heuristic: Some(T::unsafe_heuristic::<T>),
            acyc_edge: Some(T::unsafe_acyc_edge::<T>),
            theory_term_number: Some(T::unsafe_theory_term_number::<T>),
            theory_term_string: Some(T::unsafe_theory_term_string::<T>),
            theory_term_compound: Some(T::unsafe_theory_term_compound::<T>),
            theory_element: Some(T::unsafe_theory_element::<T>),
            theory_atom: Some(T::unsafe_theory_atom::<T>),
            theory_atom_with_guard: Some(T::unsafe_theory_atom_with_guard::<T>),
        };
        unsafe {
            clingo_control_register_observer(
                self.ctl.as_ptr(),
                &gpo,
                replace,
                data as *mut ::std::os::raw::c_void,
            )
        }
    }

    /// Get an object to add ground directives to the program.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn backend(&mut self) -> Result<Backend, WrapperError> {
        let mut backend = std::ptr::null_mut();
        if unsafe { clingo_control_backend(self.ctl.as_ptr(), &mut backend) } {
            if unsafe { clingo_backend_begin(backend) } {
                match unsafe { backend.as_mut() } {
                    Some(backend_ref) => Ok(Backend {
                        theref: backend_ref,
                    }),
                    None => Err(WrapperError {
                        msg: "tried casting a null pointer to &mut clingo_backend.",
                    }),
                }
            } else {
                Err(WrapperError {
                    msg: "clingo_backend_begin() failed.",
                })
            }
        } else {
            Err(WrapperError {
                msg: "clingo_control_backend() failed.",
            })
        }
    }

    /// Get an object to add non-ground directives to the program.
    pub fn program_builder(&mut self) -> Result<ProgramBuilder, WrapperError> {
        let mut builder = std::ptr::null_mut() as *mut clingo_program_builder_t;
        if unsafe { clingo_control_program_builder(self.ctl.as_ptr(), &mut builder) } {
            // begin building the program
            if unsafe { clingo_program_builder_begin(builder) } {
                match unsafe { builder.as_mut() } {
                    Some(builder_ref) => Ok(ProgramBuilder {
                        theref: builder_ref,
                    }),
                    None => Err(WrapperError {
                        msg: "tried casting a null pointer to &mut clingo_program_builder.",
                    }),
                }
            } else {
                Err(WrapperError {
                    msg: "clingo_program_builder_begin() failed.",
                })
            }
        } else {
            Err(WrapperError {
                msg: "clingo_control_program_builder() failed.",
            })
        }
    }

    // NODO: pub fn clingo_control_clasp_facade()
}

/// Representation of a program statement.
#[derive(Clone)]
pub struct AstStatement<'a, T: 'a> {
    data: clingo_ast_statement_t,
    phantom: PhantomData<&'a T>,
}
impl<'a, T> AstStatement<'a, T> {
    /// Get the location of the statement.
    pub fn location(&self) -> Location {
        Location(self.data.location)
    }

    /// Get the type of the statement.
    pub fn statement_type(&self) -> ast::StatementType {
        match self.data.type_ as u32 {
            clingo_ast_statement_type_clingo_ast_statement_type_rule => ast::StatementType::Rule,
            clingo_ast_statement_type_clingo_ast_statement_type_const => ast::StatementType::Const,
            clingo_ast_statement_type_clingo_ast_statement_type_show_signature => {
                ast::StatementType::ShowSignature
            }
            clingo_ast_statement_type_clingo_ast_statement_type_show_term => {
                ast::StatementType::ShowTerm
            }
            clingo_ast_statement_type_clingo_ast_statement_type_minimize => {
                ast::StatementType::Minimize
            }
            clingo_ast_statement_type_clingo_ast_statement_type_script => {
                ast::StatementType::Script
            }
            clingo_ast_statement_type_clingo_ast_statement_type_program => {
                ast::StatementType::Program
            }
            clingo_ast_statement_type_clingo_ast_statement_type_external => {
                ast::StatementType::External
            }
            clingo_ast_statement_type_clingo_ast_statement_type_edge => ast::StatementType::Edge,
            clingo_ast_statement_type_clingo_ast_statement_type_heuristic => {
                ast::StatementType::Heuristic
            }
            clingo_ast_statement_type_clingo_ast_statement_type_project_atom => {
                ast::StatementType::ProjectAtom
            }
            clingo_ast_statement_type_clingo_ast_statement_type_project_atom_signature => {
                ast::StatementType::ProjectAtomSignature
            }
            clingo_ast_statement_type_clingo_ast_statement_type_theory_definition => {
                ast::StatementType::TheoryDefinition
            }
            x => panic!("Failed to match clingo_ast_statement_type: {}.", x),
        }
    }

    /// Get a reference to the rule if the statement is a rule.
    pub fn rule(&self) -> Result<&ast::Rule, WrapperError> {
        match self.statement_type() {
            ast::StatementType::Rule => {
                let rule = unsafe { self.data.__bindgen_anon_1.rule as *const clingo_ast_rule_t };
                match unsafe { (rule as *const ast::Rule).as_ref() } {
                    Some(reference) => Ok(reference),
                    None => Err(WrapperError {
                        msg: "tried casting a null pointer to &ast::Rule.",
                    }),
                }
            }
            _ => Err(WrapperError {
                msg: "Wrong StatementType,",
            }),
        }
    }

    /// Get a reference to the external if the [statement type](#method.statement_type) is [`External`](ast/enum.StatementType.html#variant.External).
    pub fn external(&self) -> Result<&ast::External, WrapperError> {
        match self.statement_type() {
            ast::StatementType::External => {
                let external =
                    unsafe { self.data.__bindgen_anon_1.external as *const clingo_ast_external_t };
                match unsafe { (external as *const ast::External).as_ref() } {
                    Some(reference) => Ok(reference),
                    None => Err(WrapperError {
                        msg: "tried casting a null pointer to &ast::External.",
                    }),
                }
            }
            _ => Err(WrapperError {
                msg: "Wrong StatementType,",
            }),
        }
    }

    /// Get project signature if the [statement type](#method.statement_type) is [`ProjectAtomSignature`](ast/enum.StatementType.html#variant.ProjectAtomSignature).
    pub fn project_signature(&self) -> Result<Signature, WrapperError> {
        match self.statement_type() {
            ast::StatementType::ProjectAtomSignature => {
                let project_signature = unsafe { self.data.__bindgen_anon_1.project_signature };
                Ok(Signature(project_signature))
            }
            _ => Err(WrapperError {
                msg: "Wrong StatementType,",
            }),
        }
    }
}

/// Object to build non-ground programs.
pub struct ProgramBuilder<'a> {
    theref: &'a mut clingo_program_builder_t,
}
impl<'a> ProgramBuilder<'a> {
    /// Adds a statement to the program.
    ///
    /// **Attention:** The [`end()`](struct.ProgramBuilder.html#method.end) must be called after
    /// all statements have been added.
    ///
    /// # Arguments
    ///
    /// * `statement` - the statement to add
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) for statements of invalid form
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn add<T>(&mut self, stm: &AstStatement<T>) -> Result<(), ClingoError> {
        if unsafe { clingo_program_builder_add(self.theref, &stm.data) } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// End building a program.
    /// The method consumes the program builder.
    pub fn end(self) -> Option<()> {
        if unsafe { clingo_program_builder_end(self.theref) } {
            Some(())
        } else {
            None
        }
    }
}

/// Handle for the solver configuration.
#[derive(Debug)]
pub struct Configuration(clingo_configuration_t);
impl Configuration {
    /// Get the root key of the configuration.
    pub fn root(&self) -> Option<Id> {
        let mut root_key = 0 as clingo_id_t;
        if unsafe { clingo_configuration_root(&self.0, &mut root_key) } {
            Some(Id(root_key))
        } else {
            None
        }
    }

    /// Get the type of a key.
    /// The type is a bitset, an entry can have multiple (but at least one) type.
    pub fn configuration_type(&self, Id(key): Id) -> Option<ConfigurationType> {
        let mut ctype = 0 as clingo_configuration_type_bitset_t;
        if unsafe { clingo_configuration_type(&self.0, key, &mut ctype) } {
            ConfigurationType::from_bits(ctype)
        } else {
            None
        }
    }

    /// Get the description of an entry.
    pub fn description(&self, Id(key): Id) -> Result<&str, WrapperError> {
        let mut description_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_configuration_description(
                &self.0,
                key,
                &mut description_ptr as *mut *const c_char,
            )
        } {
            let cstr = unsafe { CStr::from_ptr(description_ptr) };
            // all descriptions should be valid UTF-8 strings
            Ok(cstr.to_str().unwrap())
        } else {
            Err(WrapperError {
                msg: "clingo_configuration_description() failed.",
            })
        }
    }

    /// Get the size of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be  [`ConfigurationType::ARRAY`](struct.ConfigurationType.html#associatedconstant.ARRAY).
    pub fn array_size(&self, Id(key): Id) -> Option<usize> {
        let mut size = 0;
        if unsafe { clingo_configuration_array_size(&self.0, key, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Get the subkey at the given offset of an array entry.
    ///
    /// **Note:** Some array entries, like fore example the solver configuration, can be accessed
    /// past there actual size to add subentries.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::ARRAY`](struct.ConfigurationType.html#associatedconstant.ARRAY).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset in the array
    pub fn array_at(&self, Id(key): Id, offset: usize) -> Option<Id> {
        let mut nkey = 0 as clingo_id_t;
        if unsafe { clingo_configuration_array_at(&self.0, key, offset, &mut nkey) } {
            Some(Id(nkey))
        } else {
            None
        }
    }

    /// Get the number of subkeys of a map entry.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::MAP`](struct.ConfigurationType.html#associatedconstant.MAP).
    pub fn map_size(&self, Id(key): Id) -> Option<usize> {
        let mut size = 0;
        if unsafe { clingo_configuration_map_size(&self.0, key, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Query whether the map has a key.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::Map`](enum.ConfigurationType.html#variant.Map).
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `name` - the name to lookup the subkey
    ///
    /// **Returns** whether the key is in the map
    pub fn map_has_subkey(&self, Id(key): Id, name: &str) -> Option<bool> {
        let mut result = false;
        if let Ok(name) = CString::new(name) {
            if unsafe {
                clingo_configuration_map_has_subkey(&self.0, key, name.as_ptr(), &mut result)
            } {
                Some(result)
            } else {
                None
            }
        } else {
            Some(false)
        }
    }

    /// Get the name associated with the offset-th subkey.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::MAP`](struct.ConfigurationType.html#associatedconstant.MAP).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset of the name
    pub fn map_subkey_name(&self, Id(key): Id, offset: usize) -> Result<&str, WrapperError> {
        let mut name_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_configuration_map_subkey_name(
                &self.0,
                key,
                offset,
                &mut name_ptr as *mut *const c_char,
            )
        } {
            let cstr = unsafe { CStr::from_ptr(name_ptr) };
            // all configuration keys should be valid UTF-8 strings
            Ok(cstr.to_str().unwrap())
        } else {
            Err(WrapperError {
                msg: "clingo_configuration_map_subkey_name() failed.",
            })
        }
    }

    /// Lookup a subkey under the given name.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::MAP`](struct.ConfigurationType.html#associatedconstant.MAP).
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    pub fn map_at(&self, Id(key): Id, name: &str) -> Option<Id> {
        let mut nkey = 0 as clingo_id_t;
        if let Ok(name) = CString::new(name) {
            if unsafe { clingo_configuration_map_at(&self.0, key, name.as_ptr(), &mut nkey) } {
                return Some(Id(nkey));
            }
        }
        None
    }

    /// Check whether a entry has a value.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::VALUE`](struct.ConfigurationType.html#associatedconstant.VALUE).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn value_is_assigned(&self, Id(key): Id) -> Option<bool> {
        let mut assigned = false;
        if unsafe { clingo_configuration_value_is_assigned(&self.0, key, &mut assigned) } {
            Some(assigned)
        } else {
            None
        }
    }

    //NODO: clingo_configuration_value_get_size(&mut self.0, key, &mut size)

    /// Get the string value of the given entry.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::VALUE`](struct.ConfigurationType.html#associatedconstant.VALUE).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    ///
    /// # Errors
    ///
    /// - [`WrapperError`](struct.WrapperError.html)
    /// - [`Utf8Error`](https://doc.rust-lang.org/std/str/struct.Utf8Error.html)
    pub fn value_get(&self, Id(key): Id) -> Result<&str, Error> {
        let mut size = 0;
        if unsafe { clingo_configuration_value_get_size(&self.0, key, &mut size) } {
            let mut value_ptr = unsafe { mem::uninitialized() };
            if unsafe { clingo_configuration_value_get(&self.0, key, &mut value_ptr, size) } {
                let cstr = unsafe { CStr::from_ptr(&value_ptr) };
                Ok(cstr.to_str()?)
            } else {
                Err(WrapperError {
                    msg: "clingo_configuration_value_get() failed.",
                })?
            }
        } else {
            Err(WrapperError {
                msg: "clingo_configuration_value_get_size() failed.",
            })?
        }
    }

    /// Set the value of an entry.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::VALUE`](struct.ConfigurationType.html#associatedconstant.VALUE).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `value` - the value to set
    ///
    /// # Errors
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `value` contains a nul byte
    /// - [`WrapperError`](struct.WrapperError.html)
    pub fn value_set(&mut self, Id(key): Id, value: &str) -> Result<(), Error> {
        let value = CString::new(value)?;
        if unsafe { clingo_configuration_value_set(&mut self.0, key, value.as_ptr()) } {
            Ok(())
        } else {
            Err(WrapperError {
                msg: "clingo_configuration_value_set() failed.",
            })?
        }
    }
}

/// Handle to the backend to add directives in aspif format.
#[derive(Debug)]
pub struct Backend<'a> {
    theref: &'a mut clingo_backend_t,
}
impl<'a> Drop for Backend<'a> {
    /// Finalize the backend after using it.
    fn drop(&mut self) {
        // println!("drop Backend");
        if !unsafe { clingo_backend_end(self.theref) } {
            panic!("Failed to finalize Backend!");
        }
    }
}
impl<'a> Backend<'a> {
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
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn rule(
        &mut self,
        choice: bool,
        head: &[Atom],
        body: &[Literal],
    ) -> Result<(), ClingoError> {
        if unsafe {
            clingo_backend_rule(
                self.theref,
                choice,
                head.as_ptr() as *const clingo_atom_t,
                head.len(),
                body.as_ptr() as *const clingo_literal_t,
                body.len(),
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Add a weight rule to the program.
    ///
    /// **Attention:** All weights and the lower bound must be positive.
    ///
    /// # Arguments
    /// * `choice` - determines if the head is a choice or a disjunction
    /// * `head` - the head atoms
    /// * `lower_bound` - the lower bound of the weight rule
    /// * `body` - the weighted body literals
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn weight_rule(
        &mut self,
        choice: bool,
        head: &[Atom],
        lower_bound: i32,
        body: &[WeightedLiteral],
    ) -> Result<(), ClingoError> {
        if unsafe {
            clingo_backend_weight_rule(
                self.theref,
                choice,
                head.as_ptr() as *const clingo_atom_t,
                head.len(),
                lower_bound,
                body.as_ptr() as *const clingo_weighted_literal_t,
                body.len(),
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Add a minimize constraint (or weak constraint) to the program.
    ///
    /// # Arguments
    ///
    /// * `priority` - the priority of the constraint
    /// * `literals` - the weighted literals whose sum to minimize
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn minimize(
        &mut self,
        priority: i32,
        literals: &[WeightedLiteral],
    ) -> Result<(), ClingoError> {
        if unsafe {
            clingo_backend_minimize(
                self.theref,
                priority,
                literals.as_ptr() as *const clingo_weighted_literal_t,
                literals.len(),
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Add a projection directive.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the atoms to project on
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn project(&mut self, atoms: &[Atom]) -> Result<(), ClingoError> {
        if unsafe {
            clingo_backend_project(
                self.theref,
                atoms.as_ptr() as *const clingo_atom_t,
                atoms.len(),
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Add an external statement.
    ///
    /// # Arguments
    ///
    /// * `atom` - the external atom
    /// * `type` - the type of the external statement
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn external(&mut self, atom: Atom, type_: ExternalType) -> Result<(), ClingoError> {
        if unsafe { clingo_backend_external(self.theref, atom.0, type_ as clingo_external_type_t) }
        {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Add an assumption directive.
    ///
    /// # Arguments
    ///
    /// * `literals` - the literals to assume (positive literals are true and negative literals
    /// false for the next solve call)
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn assume(&mut self, literals: &[Literal]) -> Result<(), ClingoError> {
        let size = literals.len();
        if unsafe {
            clingo_backend_assume(
                self.theref,
                literals.as_ptr() as *const clingo_literal_t,
                size,
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Add an heuristic directive.
    ///
    /// # Arguments
    ///
    /// * `atom` - the target atom
    /// * `htype` - the type of the heuristic modification
    /// * `bias` - the heuristic bias
    /// * `priority` - the heuristic priority
    /// * `condition` - the condition under which to apply the heuristic modification
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn heuristic(
        &mut self,
        atom: Atom,
        htype: HeuristicType,
        bias: i32,
        priority: u32,
        condition: &[Literal],
    ) -> Result<(), ClingoError> {
        let size = condition.len();
        if unsafe {
            clingo_backend_heuristic(
                self.theref,
                atom.0,
                htype as clingo_heuristic_type_t,
                bias,
                priority,
                condition.as_ptr() as *const clingo_literal_t,
                size,
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Add an edge directive.
    ///
    /// # Arguments
    ///
    /// * `node_u` - the start vertex of the edge
    /// * `node_v` - the end vertex of the edge
    /// * `condition` - the condition under which the edge is part of the graph
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn acyc_edge(
        &mut self,
        node_u: i32,
        node_v: i32,
        condition: &[Literal],
    ) -> Result<(), ClingoError> {
        let size = condition.len();
        if unsafe {
            clingo_backend_acyc_edge(
                self.theref,
                node_u,
                node_v,
                condition.as_ptr() as *const clingo_literal_t,
                size,
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Get a fresh atom to be used in aspif directives.
    /// # Arguments
    ///
    /// * `symbol` - optional symbol to associate the atom with
    pub fn add_atom(&mut self, symbol: Option<Symbol>) -> Option<Atom> {
        match symbol {
            Some(Symbol(mut symbol)) => {
                let mut atom = 0 as clingo_atom_t;
                if unsafe { clingo_backend_add_atom(self.theref, &mut symbol, &mut atom) } {
                    Some(Atom(atom))
                } else {
                    None
                }
            }
            None => {
                let mut atom = 0 as clingo_atom_t;
                let null = std::ptr::null_mut();
                if unsafe { clingo_backend_add_atom(self.theref, null, &mut atom) } {
                    Some(Atom(atom))
                } else {
                    None
                }
            }
        }
    }
}

/// Handle for to the solver statistics.
#[derive(Debug)]
pub struct Statistics(clingo_statistics_t);
impl Statistics {
    /// Get the root key of the statistics.
    pub fn root(&self) -> Option<u64> {
        let mut root_key = 0 as u64;
        if unsafe { clingo_statistics_root(&self.0, &mut root_key) } {
            Some(root_key)
        } else {
            None
        }
    }

    /// Get the type of a key.
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn statistics_type(&self, key: u64) -> Option<StatisticsType> {
        let mut stype = 0 as clingo_statistics_type_t;
        if unsafe { clingo_statistics_type(&self.0, key, &mut stype) } {
            match stype as u32 {
                clingo_statistics_type_clingo_statistics_type_empty => Some(StatisticsType::Empty),
                clingo_statistics_type_clingo_statistics_type_value => Some(StatisticsType::Value),
                clingo_statistics_type_clingo_statistics_type_array => Some(StatisticsType::Array),
                clingo_statistics_type_clingo_statistics_type_map => Some(StatisticsType::Map),
                x => panic!("Failed to match clingo_statistics_type: {}.", x),
            }
        } else {
            None
        }
    }

    /// Get the size of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Array`](enum.StatisticsType.html#variant.Array).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn array_size(&self, key: u64) -> Option<usize> {
        let mut size = 0 as usize;
        if unsafe { clingo_statistics_array_size(&self.0, key, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Get the subkey at the given offset of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Array`](enum.StatisticsType.html#variant.Array).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset in the array
    pub fn statistics_array_at(&self, key: u64, offset: usize) -> Option<u64> {
        let mut subkey = 0 as u64;
        if unsafe { clingo_statistics_array_at(&self.0, key, offset, &mut subkey) } {
            Some(subkey)
        } else {
            None
        }
    }

    /// Create the subkey at the end of an array entry.
    ///
    /// #Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must
    /// be [`StatisticsType::Array`](enum.StatisticsType.html#variant.Array)
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `stype` -  the type of the new subkey
    pub fn array_push(&mut self, key: u64, stype: StatisticsType) -> Option<u64> {
        let mut subkey = 0 as u64;
        if unsafe {
            clingo_statistics_array_push(
                &mut self.0,
                key,
                stype as clingo_statistics_type_t,
                &mut subkey,
            )
        } {
            Some(subkey)
        } else {
            None
        }
    }

    /// Get the number of subkeys of a map entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must
    /// be [`StatisticsType::Map`](enum.StatisticsType.html#variant.Map).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn map_size(&self, key: u64) -> Option<usize> {
        let mut size = 0 as usize;
        if unsafe { clingo_statistics_map_size(&self.0, key, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Test if the given map contains a specific subkey.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must
    /// be [`StatisticsType::Map`](enum.StatisticsType.html#variant.Map).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `name` - name of the subkey
    pub fn map_has_subkey(&self, key: u64, name: &str) -> Option<bool> {
        let mut result = false;
        if let Ok(name) = CString::new(name) {
            if unsafe { clingo_statistics_map_has_subkey(&self.0, key, name.as_ptr(), &mut result) }
            {
                Some(result)
            } else {
                None
            }
        } else {
            Some(false)
        }
    }

    /// Get the name associated with the offset-th subkey.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Map`](enum.StatisticsType.html#variant.Map).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset of the name
    pub fn map_subkey_name(&self, key: u64, offset: usize) -> Result<&str, Error> {
        let mut name = std::ptr::null() as *const c_char;
        if unsafe { clingo_statistics_map_subkey_name(&self.0, key, offset, &mut name) } {
            Ok(unsafe { CStr::from_ptr(name) }.to_str()?)
        } else {
            Err(WrapperError {
                msg: "clingo_statistics_map_subkey_name() failed.",
            })?
        }
    }

    /// Lookup a subkey under the given name.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Map`](enum.StatisticsType.html#variant.Map).
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `name` - the name to lookup the subkey
    pub fn map_at(&self, key: u64, name: &str) -> Option<u64> {
        let mut subkey = 0 as u64;
        if let Ok(name) = CString::new(name) {
            if unsafe { clingo_statistics_map_at(&self.0, key, name.as_ptr(), &mut subkey) } {
                return Some(subkey);
            }
        }
        None
    }

    /// Add a subkey with the given name.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Map`](enum.StatisticsType.html#variant.Map).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `name` - the name to lookup the subkey
    /// * `stype` - the type of the new subkey
    ///
    /// **Returns** the index of the resulting subkey
    pub fn map_add_subkey(&mut self, key: u64, name: &str, stype: StatisticsType) -> Option<u64> {
        let mut subkey = 0 as u64;
        if let Ok(name) = CString::new(name) {
            if unsafe {
                clingo_statistics_map_add_subkey(
                    &mut self.0,
                    key,
                    name.as_ptr(),
                    stype as clingo_statistics_type_t,
                    &mut subkey,
                )
            } {
                return Some(subkey);
            }
        }
        None
    }

    /// Get the value of the given entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Value`](enum.StatisticsType.html#variant.Value).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn value_get(&self, key: u64) -> Option<f64> {
        let mut value = 0.0 as f64;
        if unsafe { clingo_statistics_value_get(&self.0, key, &mut value) } {
            Some(value)
        } else {
            None
        }
    }

    /// Set the value of the given entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Value`](enum.StatisticsType.html#variant.Value).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `value` - the new value
    ///
    /// **Returns** whether the call was successful
    pub fn value_set(&mut self, key: u64, value: f64) -> bool {
        unsafe { clingo_statistics_value_set(&mut self.0, key, value) }
    }
}
/// Container that stores symbolic atoms in a program -- the relevant Herbrand base
/// gringo uses to instantiate programs.
///
/// **See:** [`Control::symbolic_atoms()`](struct.Control.html#method.symbolic_atoms)
#[derive(Debug)]
pub struct SymbolicAtoms(clingo_symbolic_atoms_t);
impl SymbolicAtoms {
    /// Get the number of different atoms occurring in a logic program.
    pub fn size(&self) -> Option<usize> {
        let mut size = 0 as usize;
        if unsafe { clingo_symbolic_atoms_size(&self.0, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Get a forward iterator of the sequence of all symbolic atoms.
    pub fn iter(&self) -> SymbolicAtomsIterator {
        let mut begin = 0 as clingo_symbolic_atom_iterator_t;
        if !unsafe { clingo_symbolic_atoms_begin(&self.0, std::ptr::null(), &mut begin) } {
            panic!("Failed to create iterator for clingo_symbolic_atoms.");
        }
        let mut end = 0 as clingo_symbolic_atom_iterator_t;
        if !unsafe { clingo_symbolic_atoms_end(&self.0, &mut end) } {
            panic!("Failed to create iterator for clingo_symbolic_atoms.");
        }
        SymbolicAtomsIterator {
            cur: begin,
            end,
            atoms: &self.0,
        }
    }
    /// Get a forward iterator of the sequence of all symbolic atoms restricted to a given signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - the signature
    pub fn iter_with_signature(&self, sig: &Signature) -> SymbolicAtomsIterator {
        let mut begin = 0 as clingo_symbolic_atom_iterator_t;
        if !unsafe { clingo_symbolic_atoms_begin(&self.0, &sig.0, &mut begin) } {
            panic!("Failed to create iterator for clingo_symbolic_atoms.");
        }
        let mut end = 0 as clingo_symbolic_atom_iterator_t;
        if !unsafe { clingo_symbolic_atoms_end(&self.0, &mut end) } {
            panic!("Failed to create iterator for clingo_symbolic_atoms.");
        }
        SymbolicAtomsIterator {
            cur: begin,
            end,
            atoms: &self.0,
        }
    }

    //NODO: fn clingo_symbolic_atoms_signatures_size()

    /// Get the predicate signatures occurring in a logic program.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if the size is too small
    pub fn signatures(&self) -> Result<Vec<Signature>, ClingoError> {
        let mut size = 0;
        if unsafe { clingo_symbolic_atoms_signatures_size(&self.0, &mut size) } {
            let signatures = Vec::<Signature>::with_capacity(size);
            let signatures_ptr = signatures.as_ptr();
            if unsafe {
                clingo_symbolic_atoms_signatures(
                    &self.0,
                    signatures_ptr as *mut clingo_signature_t,
                    size,
                )
            } {
                let signatures_ref =
                    unsafe { std::slice::from_raw_parts(signatures_ptr as *const Signature, size) };
                Ok(signatures_ref.to_owned())
            } else {
                Err(error())
            }
        } else {
            Err(error())
        }
    }

    //NODO clingo_symbolic_atoms_is_valid()
}
/// An iterator over symbolic atoms.
pub struct SymbolicAtomsIterator<'a> {
    cur: clingo_symbolic_atom_iterator_t,
    end: clingo_symbolic_atom_iterator_t,
    atoms: &'a clingo_symbolic_atoms_t,
}
impl<'a> Iterator for SymbolicAtomsIterator<'a> {
    type Item = SymbolicAtom<'a>;

    fn next(&mut self) -> Option<SymbolicAtom<'a>> {
        let mut equal = false;
        if unsafe {
            clingo_symbolic_atoms_iterator_is_equal_to(self.atoms, self.cur, self.end, &mut equal)
        } {
            if equal {
                None
            } else {
                let ret = SymbolicAtom {
                    cur: self.cur,
                    atoms: self.atoms,
                };
                if !unsafe { clingo_symbolic_atoms_next(self.atoms, self.cur, &mut self.cur) } {
                    panic!("Failure in SymbolicAtomsIterator.");
                }
                Some(ret)
            }
        } else {
            None
        }
    }
}
/// A symbolic atom in a program.
pub struct SymbolicAtom<'a> {
    cur: clingo_symbolic_atom_iterator_t,
    atoms: &'a clingo_symbolic_atoms_t,
}
impl<'a> SymbolicAtom<'a> {
    /// Check whether an atom is a fact.
    ///
    /// **Note:** This does not determine if an atom is a cautious consequence. The
    /// grounding or solving component's simplifications can only detect this in
    /// some cases.
    pub fn is_fact(&self) -> Option<bool> {
        let mut fact = false;
        if unsafe { clingo_symbolic_atoms_is_fact(self.atoms, self.cur, &mut fact) } {
            Some(fact)
        } else {
            None
        }
    }

    /// Check whether an atom is external.
    ///
    /// An atom is external if it has been defined using an external directive and
    /// has not been released or defined by a rule.
    pub fn is_external(&self) -> Option<bool> {
        let mut external = false;
        if unsafe { clingo_symbolic_atoms_is_external(self.atoms, self.cur, &mut external) } {
            Some(external)
        } else {
            None
        }
    }

    /// Get the symbolic representation of an atom.
    pub fn symbol(&self) -> Option<Symbol> {
        let mut symbol = 0 as clingo_symbol_t;
        if unsafe { clingo_symbolic_atoms_symbol(self.atoms, self.cur, &mut symbol) } {
            Some(Symbol(symbol))
        } else {
            None
        }
    }

    /// Returns the (numeric) aspif literal corresponding to the given symbolic atom.
    ///
    /// Such a literal can be mapped to a solver literal (see [`Propagator`](struct.Propagator)).
    /// or be used in rules in aspif format (see [`ProgramBuilder`](struct.ProgramBuilder.html)).
    pub fn literal(&self) -> Option<Literal> {
        let mut literal = 0 as clingo_literal_t;
        if unsafe { clingo_symbolic_atoms_literal(self.atoms, self.cur, &mut literal) } {
            Some(Literal(literal))
        } else {
            None
        }
    }
}

/// Container that stores theory atoms, elements, and terms of a program.
///
/// **See:** [`Control::theory_atoms()`](struct.Control.html#method.theory_atoms)
#[derive(Debug)]
pub struct TheoryAtoms(clingo_theory_atoms_t);
impl TheoryAtoms {
    /// Get the total number of theory atoms.
    pub fn size(&self) -> Result<usize, WrapperError> {
        let mut size = 0 as usize;
        if unsafe { clingo_theory_atoms_size(&self.0, &mut size) } {
            Ok(size)
        } else {
            Err(WrapperError {
                msg: "clingo_theory_atoms_size() failed.",
            })
        }
    }

    ///  Returns an iterator over the theory atoms.
    pub fn iter(&self) -> TheoryAtomsIterator {
        TheoryAtomsIterator {
            count: 0,
            atoms: &self,
            atoms_size: self.size().unwrap(),
        }
    }

    /// Get the type of the given theory term.
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    pub fn term_type(&self, Id(term): Id) -> Option<TheoryTermType> {
        let mut ttype = 0 as clingo_theory_term_type_t;
        if unsafe { clingo_theory_atoms_term_type(&self.0, term, &mut ttype) } {
            match ttype as u32 {
                clingo_theory_term_type_clingo_theory_term_type_tuple => {
                    Some(TheoryTermType::Tuple)
                }
                clingo_theory_term_type_clingo_theory_term_type_list => Some(TheoryTermType::List),
                clingo_theory_term_type_clingo_theory_term_type_set => Some(TheoryTermType::Set),
                clingo_theory_term_type_clingo_theory_term_type_function => {
                    Some(TheoryTermType::Function)
                }
                clingo_theory_term_type_clingo_theory_term_type_number => {
                    Some(TheoryTermType::Number)
                }
                clingo_theory_term_type_clingo_theory_term_type_symbol => {
                    Some(TheoryTermType::Symbol)
                }
                x => panic!("Failed to match clingo_theory_term_type: {}.", x),
            }
        } else {
            None
        }
    }

    /// Get the number of the given numeric theory term.
    ///
    /// # Pre-condition
    ///
    /// The term must be of type [`TermType::Number`](enum.TermType.html#variant.Number).
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    pub fn term_number(&self, Id(term): Id) -> Option<i32> {
        let mut number = 0;
        if unsafe { clingo_theory_atoms_term_number(&self.0, term, &mut number) } {
            Some(number)
        } else {
            None
        }
    }

    /// Get the name of the given constant or function theory term.
    ///
    /// # Pre-condition
    ///
    /// The term must be of type [`TermType::Function`](enum.TermType.html#variant.Function) or
    /// [`TermType::Symbol`](enum.TermType.html#variant.Symbol).
    ///
    /// # Arguments
    ///
    /// * `term` id of the term
    pub fn term_name<'a>(&self, Id(term): Id) -> Result<&'a str, Error> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_theory_atoms_term_name(&self.0, term, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Ok(c_str.to_str()?)
        } else {
            Err(WrapperError {
                msg: "clingo_theory_atoms_term_name() failed.",
            })?
        }
    }

    /// Get the arguments of the given function theory term.
    ///
    /// # Pre-condition
    ///
    /// The term must be of type [`TermType::Function`](enum.TermType.html#variant.Function).
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    pub fn term_arguments(&self, Id(term): Id) -> Option<Vec<Id>> {
        let mut size = 0;
        let mut c_ptr = unsafe { mem::uninitialized() };
        if unsafe { clingo_theory_atoms_term_arguments(&self.0, term, &mut c_ptr, &mut size) } {
            let arguments_ref = unsafe { std::slice::from_raw_parts(c_ptr as *const Id, size) };
            Some(arguments_ref.to_owned())
        } else {
            None
        }
    }

    //NODO: pub fn clingo_theory_atoms_term_to_string_size()

    /// Get the string representation of the given theory term.
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if the size is too small
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn term_to_string(&self, Id(term): Id) -> Result<&str, Error> {
        let mut size = 0;
        if unsafe { clingo_theory_atoms_term_to_string_size(&self.0, term, &mut size) } {
            let mut c_ptr = unsafe { mem::uninitialized() };
            if unsafe { clingo_theory_atoms_term_to_string(&self.0, term, &mut c_ptr, size) } {
                let cstr = unsafe { CStr::from_ptr(&c_ptr) };
                Ok(cstr.to_str()?)
            } else {
                Err(error())?
            }
        } else {
            Err(error())?
        }
    }

    /// Get the tuple (array of theory terms) of the given theory element.
    ///
    /// # Arguments
    ///
    /// * `element` - id of the element
    pub fn element_tuple(&self, Id(element): Id) -> Option<Vec<Id>> {
        let mut size = 0;
        let mut tuple_ptr = unsafe { mem::uninitialized() };
        if unsafe { clingo_theory_atoms_element_tuple(&self.0, element, &mut tuple_ptr, &mut size) }
        {
            let tuple_ref = unsafe { std::slice::from_raw_parts(tuple_ptr as *const Id, size) };
            Some(tuple_ref.to_owned())
        } else {
            None
        }
    }

    /// Get the condition (array of aspif literals) of the given theory element.
    ///
    /// # Arguments
    ///
    /// * `element` - id of the element
    pub fn element_condition(&self, Id(element): Id) -> Option<Vec<Literal>> {
        let mut size = 0;
        let mut condition_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_theory_atoms_element_condition(&self.0, element, &mut condition_ptr, &mut size)
        } {
            let condition_ref =
                unsafe { std::slice::from_raw_parts(condition_ptr as *const Literal, size) };
            Some(condition_ref.to_owned())
        } else {
            None
        }
    }

    /// Get the id of the condition of the given theory element.
    ///
    /// **Note:**
    /// This id can be mapped to a solver literal using [`PropagateInit::solver_literal()`](struct.PropagateInit.html#method.solver_literal).
    /// This id is not (necessarily) an aspif literal;
    /// to get aspif literals use [`TheoryAtoms::element_condition()`](struct.TheoryAtoms.html#method.element_condition).
    ///
    /// # Arguments
    ///
    /// * `element` - id of the element
    pub fn element_condition_id(&self, Id(element): Id) -> Option<Literal> {
        let mut condition = unsafe { mem::uninitialized() };
        if unsafe { clingo_theory_atoms_element_condition_id(&self.0, element, &mut condition) } {
            Some(Literal(condition))
        } else {
            None
        }
    }

    //NODO: pub fn clingo_theory_atoms_element_to_string_size()

    /// Get the string representation of the given theory element.
    ///
    /// # Arguments
    ///
    /// * `element` - id of the element
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if the size is too small
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn element_to_string(&self, Id(element): Id) -> Result<&str, Error> {
        let mut size = 0;
        if unsafe { clingo_theory_atoms_element_to_string_size(&self.0, element, &mut size) } {
            let mut c_ptr = unsafe { mem::uninitialized() };
            if unsafe { clingo_theory_atoms_element_to_string(&self.0, element, &mut c_ptr, size) }
            {
                let cstr = unsafe { CStr::from_ptr(&c_ptr) };
                Ok(cstr.to_str()?)
            } else {
                Err(error())?
            }
        } else {
            Err(error())?
        }
    }

    /// Get the theory term associated with the theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` id of the atom
    pub fn atom_term(&self, Id(atom): Id) -> Option<Id> {
        let mut term = 0 as clingo_id_t;
        if unsafe { clingo_theory_atoms_atom_term(&self.0, atom, &mut term) } {
            Some(Id(term))
        } else {
            None
        }
    }

    /// Get the theory elements associated with the theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` - id of the atom
    pub fn atom_elements(&self, Id(atom): Id) -> Option<Vec<Id>> {
        let mut size = 0;
        let mut elements_ptr = unsafe { mem::uninitialized() };
        if unsafe { clingo_theory_atoms_atom_elements(&self.0, atom, &mut elements_ptr, &mut size) }
        {
            let elements = unsafe { std::slice::from_raw_parts(elements_ptr as *const Id, size) };
            Some(elements.to_owned())
        } else {
            None
        }
    }

    /// Whether the theory atom has a guard.
    ///
    /// # Arguments
    ///
    /// * `atom` id of the atom
    pub fn atom_has_guard(&self, Id(atom): Id) -> Option<bool> {
        let mut has_guard = false;
        if unsafe { clingo_theory_atoms_atom_has_guard(&self.0, atom, &mut has_guard) } {
            Some(has_guard)
        } else {
            None
        }
    }

    /// Get the guard consisting of a theory operator and a theory term of the given theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` - id of the atom
    pub fn atom_guard(&self, Id(atom): Id) -> Result<(&str, Id), Error> {
        let mut c_ptr = unsafe { mem::uninitialized() };
        let mut term = 0 as clingo_id_t;
        if unsafe { clingo_theory_atoms_atom_guard(&self.0, atom, &mut c_ptr, &mut term) } {
            let cstr = unsafe { CStr::from_ptr(c_ptr) };
            Ok((cstr.to_str()?, Id(term)))
        } else {
            Err(WrapperError {
                msg: "clingo_theory_atoms_atom_guard() failed.",
            })?
        }
    }

    /// Get the aspif literal associated with the given theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` id of the atom
    pub fn atom_literal(&self, Id(atom): Id) -> Option<Literal> {
        let mut literal = 0 as clingo_literal_t;
        if unsafe { clingo_theory_atoms_atom_literal(&self.0, atom, &mut literal) } {
            Some(Literal(literal))
        } else {
            None
        }
    }

    //NODO: pub fn clingo_theory_atoms_atom_to_string_size()

    /// Get the string representation of the given theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` - id of the element
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if the size is too small
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn atom_to_string(&self, Id(atom): Id) -> Result<&str, Error> {
        let mut size = 0;
        if unsafe { clingo_theory_atoms_atom_to_string_size(&self.0, atom, &mut size) } {
            let mut c_ptr = unsafe { mem::uninitialized() };
            if unsafe { clingo_theory_atoms_atom_to_string(&self.0, atom, &mut c_ptr, size) } {
                let cstr = unsafe { CStr::from_ptr(&c_ptr) };
                Ok(cstr.to_str()?)
            } else {
                Err(error())?
            }
        } else {
            Err(error())?
        }
    }
}

/// Iterator over theory atoms.
pub struct TheoryAtomsIterator<'a> {
    count: usize,
    atoms: &'a TheoryAtoms,
    atoms_size: usize,
}
impl<'a> Iterator for TheoryAtomsIterator<'a> {
    type Item = Id;

    fn next(&mut self) -> Option<Id> {
        // check to see if we've finished counting or not.
        if self.count < self.atoms_size {
            let ret = Id(self.count as clingo_id_t);
            // increment our count.
            self.count += 1;
            Some(ret)
        } else {
            None
        }
    }
}

/// Represents a model.
#[derive(Debug)]
pub struct Model(clingo_model_t);
impl Model {
    /// Get the type of the model.
    pub fn model_type(&self) -> Option<ModelType> {
        let mut mtype = 0 as clingo_model_type_t;
        if unsafe { clingo_model_type(&self.0, &mut mtype) } {
            match mtype as u32 {
                clingo_model_type_clingo_model_type_stable_model => Some(ModelType::StableModel),
                clingo_model_type_clingo_model_type_brave_consequences => {
                    Some(ModelType::BraveConsequences)
                }
                clingo_model_type_clingo_model_type_cautious_consequences => {
                    Some(ModelType::CautiousConsequences)
                }
                x => panic!("Failed to match clingo_model_type: {}.", x),
            }
        } else {
            None
        }
    }

    /// Get the running number of the model.
    pub fn number(&self) -> Option<u64> {
        let mut number = 0;
        if unsafe { clingo_model_number(&self.0, &mut number) } {
            Some(number)
        } else {
            None
        }
    }

    //NODO: pub fn clingo_model_symbols_size()

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
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if the size is too small
    pub fn symbols(&self, show: ShowType) -> Result<Vec<Symbol>, ClingoError> {
        let mut size: usize = 0;
        if unsafe { clingo_model_symbols_size(&self.0, show.bits(), &mut size) } {
            let symbols = Vec::<Symbol>::with_capacity(size);
            let symbols_ptr = symbols.as_ptr();
            if unsafe {
                clingo_model_symbols(
                    &self.0,
                    show.bits(),
                    symbols_ptr as *mut clingo_symbol_t,
                    size,
                )
            } {
                let symbols_ref =
                    unsafe { std::slice::from_raw_parts(symbols_ptr as *const Symbol, size) };
                Ok(symbols_ref.to_owned())
            } else {
                Err(error())
            }
        } else {
            Err(error())
        }
    }

    /// Constant time lookup to test whether an atom is in a model.
    ///
    /// # Arguments
    ///
    /// * `atom` - the atom to lookup
    pub fn contains(&self, Symbol(atom): Symbol) -> Option<bool> {
        let mut contained = false;
        if unsafe { clingo_model_contains(&self.0, atom, &mut contained) } {
            Some(contained)
        } else {
            None
        }
    }

    /// Check whether a program literal is true in a model.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal to lookup
    pub fn is_true(&self, literal: Literal) -> Option<bool> {
        let mut is_true = false;
        if unsafe { clingo_model_is_true(&self.0, literal.0, &mut is_true) } {
            Some(is_true)
        } else {
            None
        }
    }

    //NODO: pub fn clingo_model_cost_size(model: *mut Model, size: *mut size_t) -> u8;

    /// Get the cost vector of a model.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if the size is too small
    ///
    /// **See:** [`Model::optimality_proven()`](struct.Model.html#method.optimality_proven)
    pub fn cost(&self) -> Result<Vec<i64>, ClingoError> {
        let mut size: usize = 0;
        if unsafe { clingo_model_cost_size(&self.0, &mut size) } {
            let cost = Vec::<i64>::with_capacity(size);
            let cost_ptr = cost.as_ptr();
            if unsafe { clingo_model_cost(&self.0, cost_ptr as *mut i64, size) } {
                let cost_ref = unsafe { std::slice::from_raw_parts(cost_ptr as *const i64, size) };
                Ok(cost_ref.to_owned())
            } else {
                Err(error())
            }
        } else {
            Err(error())
        }
    }

    /// Whether the optimality of a model has been proven.
    ///
    /// **See:** [`Model::cost()`](struct.Model.html#method.cost)
    pub fn optimality_proven(&self) -> Option<bool> {
        let mut proven = false;
        if unsafe { clingo_model_optimality_proven(&self.0, &mut proven) } {
            Some(proven)
        } else {
            None
        }
    }

    /// Get the id of the solver thread that found the model.
    pub fn thread_id(&self) -> Option<Id> {
        let mut id = 0 as clingo_id_t;
        if unsafe { clingo_model_thread_id(&self.0, &mut id) } {
            Some(Id(id))
        } else {
            None
        }
    }

    /// Add symbols to the model."]
    ///
    /// These symbols will appear in clingo\'s output, which means that this
    /// function is only meaningful if there is an underlying clingo application."]
    /// Only models passed to the ::clingo_solve_event_callback_t are extendable."]
    ///
    /// # Arguments
    ///
    /// * `symbols` - the symbols to add
    ///
    /// **Returns** whether the call was successful
    pub fn extend(&mut self, symbols: &[Symbol]) -> bool {
        unsafe {
            clingo_model_extend(
                &mut self.0,
                symbols.as_ptr() as *const clingo_symbol_t,
                symbols.len(),
            )
        }
    }

    /// Get the associated solve control object of a model.
    ///
    /// This object allows for adding clauses during model enumeration.
    pub fn context(&self) -> Option<&mut SolveControl> {
        let mut control = unsafe { mem::uninitialized() };
        if unsafe { clingo_model_context(&self.0, &mut control) } {
            unsafe { (control as *mut SolveControl).as_mut() }
        } else {
            None
        }
    }
}

/// Object to add clauses during search.
#[derive(Debug)]
pub struct SolveControl(clingo_solve_control_t);
impl SolveControl {
    /// Add a clause that applies to the current solving step during model
    /// enumeration.
    ///
    /// **Note:** The [`Propagator`](trait.Propagator.html) trait provides a more sophisticated
    /// interface to add clauses - even on partial assignments.
    ///
    /// # Arguments
    ///
    /// * `clause` - array of literals representing the clause
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if adding the clause fails
    pub fn add_clause(&mut self, clause: &[Literal]) -> Result<(), ClingoError> {
        if unsafe {
            clingo_solve_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                clause.len(),
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Get an object to inspect the symbolic atoms.
    pub fn symbolic_atoms(&mut self) -> Option<&SymbolicAtoms> {
        let mut atoms = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
        if unsafe { clingo_solve_control_symbolic_atoms(&mut self.0, &mut atoms) } {
            unsafe { (atoms as *const SymbolicAtoms).as_ref() }
        } else {
            None
        }
    }
}

/// Represents a (partial) assignment of a particular solver.
///
/// An assignment assigns truth values to a set of literals.
/// A literal is assigned to either [true or false, or is unassigned](struct.Assignment.html#method.truth_value).
/// Furthermore, each assigned literal is associated with a [decision level](struct.Assignment.html#method.level).
/// There is exactly one [decision literal](struct.Assignment.html#method.decision) for each decision level greater than zero.
/// Assignments to all other literals on the same level are consequences implied by the current and possibly previous decisions.
/// Assignments on level zero are immediate consequences of the current program.
/// Decision levels are consecutive numbers starting with zero up to and including the [current decision level](struct.Assignment.html#method.decision_level).
#[derive(Debug)]
pub struct Assignment(clingo_assignment_t);
impl Assignment {
    /// Get the current decision level.
    pub fn decision_level(&self) -> u32 {
        unsafe { clingo_assignment_decision_level(&self.0) }
    }

    /// Check whether the given assignment is conflicting.
    pub fn has_conflict(&self) -> bool {
        unsafe { clingo_assignment_has_conflict(&self.0) }
    }

    /// Check whether the given literal is part of a (partial) assignment.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    pub fn has_literal(&self, literal: Literal) -> bool {
        unsafe { clingo_assignment_has_literal(&self.0, literal.0) }
    }

    /// Determine the decision level of a given literal.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    ///
    /// **Returns** the decision level of the given literal
    pub fn level(&self, literal: Literal) -> Option<u32> {
        let mut level = 0;
        if unsafe { clingo_assignment_level(&self.0, literal.0, &mut level) } {
            Some(level)
        } else {
            None
        }
    }

    /// Determine the decision literal given a decision level.
    ///
    /// # Arguments
    ///
    /// * `level` - the level
    ///
    /// **Returns** the decision literal for the given decision level
    pub fn decision(&self, level: u32) -> Option<Literal> {
        let mut lit = 0 as clingo_literal_t;
        if unsafe { clingo_assignment_decision(&self.0, level, &mut lit) } {
            Some(Literal(lit))
        } else {
            None
        }
    }

    /// Check if a literal has a fixed truth value.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    ///
    /// **Returns** whether the literal is fixed
    pub fn is_fixed(&self, literal: Literal) -> Option<bool> {
        let mut is_fixed = false;
        if unsafe { clingo_assignment_is_fixed(&self.0, literal.0, &mut is_fixed) } {
            Some(is_fixed)
        } else {
            None
        }
    }

    /// Check if a literal is true.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    /// **Returns** whether the literal is true (see [`Assignment::truth_value()`](struct.Assignment.html#method.truth_value))
    pub fn is_true(&self, literal: Literal) -> Option<bool> {
        let mut is_true = false;
        if unsafe { clingo_assignment_is_true(&self.0, literal.0, &mut is_true) } {
            Some(is_true)
        } else {
            None
        }
    }

    /// Check if a literal has a fixed truth value.
    ///
    /// # Arguments
    /// * `literal` - the literal
    ///
    /// **Returns** whether the literal is false (see [`Assignment::truth_value()`](struct.Assignment.html#method.truth_value))
    pub fn is_false(&self, literal: Literal) -> Option<bool> {
        let mut is_false = false;
        if unsafe { clingo_assignment_is_false(&self.0, literal.0, &mut is_false) } {
            Some(is_false)
        } else {
            None
        }
    }

    /// Determine the truth value of a given literal.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    /// * `value` - the resulting truth value
    ///
    /// **Returns** whether the call was successful
    pub fn truth_value(&self, literal: Literal) -> Option<TruthValue> {
        let mut value = 0;
        if unsafe { clingo_assignment_truth_value(&self.0, literal.0, &mut value) } {
            match value as u32 {
                clingo_truth_value_clingo_truth_value_false => Some(TruthValue::False),
                clingo_truth_value_clingo_truth_value_true => Some(TruthValue::True),
                clingo_truth_value_clingo_truth_value_free => Some(TruthValue::Free),
                x => panic!("Failed to match clingo_truth_value: {}.", x),
            }
        } else {
            None
        }
    }

    /// The number of assigned literals in the assignment.
    pub fn size(&self) -> usize {
        unsafe { clingo_assignment_size(&self.0) }
    }

    /// The maximum size of the assignment (if all literals are assigned).
    pub fn max_size(&self) -> usize {
        unsafe { clingo_assignment_max_size(&self.0) }
    }

    /// Check if the assignmen is total, i.e. - size == max_size.
    pub fn is_total(&self) -> bool {
        unsafe { clingo_assignment_is_total(&self.0) }
    }
}

/// This object can be used to add clauses and propagate literals while solving.
#[derive(Debug)]
pub struct PropagateControl(clingo_propagate_control_t);
impl PropagateControl {
    /// Get the id of the underlying solver thread.
    ///
    /// Thread ids are consecutive numbers starting with zero.
    pub fn thread_id(&mut self) -> u32 {
        unsafe { clingo_propagate_control_thread_id(&mut self.0) }
    }

    /// Get the assignment associated with the underlying solver.
    pub fn assignment(&self) -> Result<&Assignment, WrapperError> {
        match unsafe {
            (clingo_propagate_control_assignment(&self.0) as *const Assignment).as_ref()
        } {
            Some(stm) => Ok(stm),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &Assignment.",
            }),
        }
    }

    /// Get the assignment associated with the underlying solver.
    pub fn assignment_mut(&self) -> Result<&mut Assignment, WrapperError> {
        match unsafe { (clingo_propagate_control_assignment(&self.0) as *mut Assignment).as_mut() }
        {
            Some(stm) => Ok(stm),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &mut Assignment.",
            }),
        }
    }

    /// Adds a new volatile literal to the underlying solver thread.
    ///
    /// **Attention:** The literal is only valid within the current solving step and solver thread.
    /// All volatile literals and clauses involving a volatile literal are deleted after the current search.
    ///
    /// # Arguments
    ///
    /// * `result` - the (positive) solver literal
    ///
    /// **Errors:**
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Logic`](enum.ErrorType.html#variant.Logic) if the assignment is conflicting
    pub fn add_literal(&mut self, result: &mut Literal) -> Result<(), ClingoError> {
        if unsafe { clingo_propagate_control_add_literal(&mut self.0, &mut result.0) } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Add a watch for the solver literal in the given phase.
    ///
    /// **Note:** Unlike [`PropagateInit::add_watch()`](struct.PropagateInit.html#method.add_watch) this does not add a watch to all solver threads but just the current one.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal to watch
    ///
    /// **Errors:**
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Logic`](enum.ErrorType.html#variant.Logic) if the literal is invalid
    ///
    /// **See:** [`PropagateControl::remove_watch()`](struct.PropagateControl.html#method.remove_watch)
    pub fn add_watch(&mut self, literal: Literal) -> Result<(), ClingoError> {
        if unsafe { clingo_propagate_control_add_watch(&mut self.0, literal.0) } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Check whether a literal is watched in the current solver thread.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal to check
    pub fn has_watch(&self, literal: Literal) -> bool {
        unsafe { clingo_propagate_control_has_watch(&self.0, literal.0) }
    }

    /// Removes the watch (if any) for the given solver literal.
    ///
    /// **Note:** Similar to [`PropagateInit::add_watch()`](struct.PropagateInit.html#method.add_watch) this just removes the watch in the current solver thread.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal to remove
    pub fn remove_watch(&mut self, literal: Literal) {
        unsafe { clingo_propagate_control_remove_watch(&mut self.0, literal.0) }
    }

    /// Add the given clause to the solver.
    ///
    /// This method sets its result to false if the current propagation must be stopped for the solver to backtrack.
    ///
    /// **Attention:** No further calls on the control object or functions on the assignment should be called when the result of this method is false.
    ///
    /// # Arguments
    ///
    /// * `clause` - the clause to add
    /// * `ctype` - the clause type determining its lifetime
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn add_clause(
        &mut self,
        clause: &[Literal],
        ctype: ClauseType,
    ) -> Result<bool, ClingoError> {
        let mut result = false;
        if unsafe {
            clingo_propagate_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                clause.len(),
                ctype as clingo_clause_type_t,
                &mut result,
            )
        } {
            Ok(result)
        } else {
            Err(error())
        }
    }

    /// Propagate implied literals (resulting from added clauses).
    ///
    /// This method sets its result to false if the current propagation must be stopped for the
    /// solver to backtrack.
    ///
    /// **Attention:** No further calls on the control object or functions on the assignment should
    /// be called when the result of this method is false.
    ///
    /// **Returns** result indicating whether propagation has to be stopped
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn propagate(&mut self) -> Result<bool, ClingoError> {
        let mut result = false;
        if unsafe { clingo_propagate_control_propagate(&mut self.0, &mut result) } {
            Ok(result)
        } else {
            Err(error())
        }
    }
}

/// Object to initialize a user-defined propagator before each solving step.
///
/// Each [symbolic](struct.SymbolicAtoms.html) or [theory atom](struct.TheoryAtoms.html) is uniquely associated with an aspif atom in form of a positive integer ([`Literal`](struct.Literal.html)).
/// Aspif literals additionally are signed to represent default negation.
/// Furthermore, there are non-zero integer solver literals (also represented using [`Literal`](struct.Literal.html).
/// There is a surjective mapping from program atoms to solver literals.
///
/// All methods called during propagation use solver literals whereas [`SymbolicAtoms::literal()`](struct.SymbolicAtoms.html#method.literal) and [`TheoryAtoms::atom_literal()`](struct.TheoryAtoms.html#method.atom_literal) return program literals.
/// The function [`PropagateInit::solver_literal()`](struct.PropagateInit.html#method.solver_literal) can be used to map program literals or [condition ids](struct.TheoryAtoms.html#method.element_condition_id) to solver literals.
#[derive(Debug)]
pub struct PropagateInit(clingo_propagate_init_t);
impl PropagateInit {
    /// Map the given program literal or condition id to its solver literal.
    ///
    /// # Arguments
    ///
    /// * `aspif_literal` - the aspif literal to map
    ///
    /// **Returns** the corresponding solver literal
    pub fn solver_literal(&self, Literal(aspif_literal): Literal) -> Option<Literal> {
        let mut solver_literal = 0 as clingo_literal_t;
        if unsafe {
            clingo_propagate_init_solver_literal(&self.0, aspif_literal, &mut solver_literal)
        } {
            Some(Literal(solver_literal))
        } else {
            None
        }
    }

    /// Add a watch for the solver literal in the given phase.
    ///
    /// # Arguments
    ///
    /// * `solver_literal` - the solver literal
    pub fn add_watch(&mut self, Literal(solver_literal): Literal) -> Option<()> {
        if unsafe { clingo_propagate_init_add_watch(&mut self.0, solver_literal) } {
            Some(())
        } else {
            None
        }
    }

    /// Add a watch for the solver literal in the given phase to the given solver thread.
    ///
    /// # Arguments
    ///
    /// * `solver_literal` - the solver literal
    /// * `thread_id` - the id of the solver thread
    pub fn add_watch_to_thread(
        &mut self,
        Literal(solver_literal): Literal,
        thread_id: u32,
    ) -> Option<()> {
        if unsafe {
            clingo_propagate_init_add_watch_to_thread(&mut self.0, solver_literal, thread_id)
        } {
            Some(())
        } else {
            None
        }
    }

    /// Get an object to inspect the symbolic atoms.
    pub fn symbolic_atoms(&self) -> Option<&SymbolicAtoms> {
        let mut atoms_ptr = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
        if unsafe { clingo_propagate_init_symbolic_atoms(&self.0, &mut atoms_ptr) } {
            unsafe { (atoms_ptr as *const SymbolicAtoms).as_ref() }
        } else {
            None
        }
    }

    /// Get an object to inspect the theory atoms.
    pub fn theory_atoms(&self) -> Option<&TheoryAtoms> {
        let mut atoms_ptr = std::ptr::null_mut() as *mut clingo_theory_atoms_t;
        if unsafe { clingo_propagate_init_theory_atoms(&self.0, &mut atoms_ptr) } {
            unsafe { (atoms_ptr as *const TheoryAtoms).as_ref() }
        } else {
            None
        }
    }

    /// Get the number of threads used in subsequent solving.
    ///
    /// **See:** [`PropagateControl::thread_id()`](struct.PropagateControl.html#method.thread_id)
    pub fn number_of_threads(&self) -> usize {
        (unsafe { clingo_propagate_init_number_of_threads(&self.0) } as usize)
    }

    /// Configure when to call the check method of the propagator.
    ///
    /// # Arguments
    ///
    /// * `mode` - bitmask when to call the propagator
    ///
    /// **See:** [`Propagator::check()`](trait.Propagator.html#method.check)
    pub fn set_check_mode(&mut self, mode: PropagatorCheckMode) {
        unsafe {
            clingo_propagate_init_set_check_mode(
                &mut self.0,
                mode as clingo_propagator_check_mode_t,
            )
        }
    }

    /// Get the current check mode of the propagator.
    ///
    /// **Returns**  bitmask when to call the propagator
    ///
    /// **See:** [`PropagateInit::set_check_mode()`](struct.PropagateInit.html#method.set_check_mode)
    pub fn get_check_mode(&self) -> PropagatorCheckMode {
        match unsafe { clingo_propagate_init_get_check_mode(&self.0) } as u32 {
            clingo_propagator_check_mode_clingo_propagator_check_mode_fixpoint => {
                PropagatorCheckMode::Fixpoint
            }
            clingo_propagator_check_mode_clingo_propagator_check_mode_total => {
                PropagatorCheckMode::Total
            }
            clingo_propagator_check_mode_clingo_propagator_check_mode_none => {
                PropagatorCheckMode::None
            }
            x => panic!("Failed to match clingo_propagator_check_mode: {}.", x),
        }
    }

    /// Get the top level assignment solver.
    ///
    /// **Returns** the assignment
    pub fn assignment(&mut self) -> Result<&mut Assignment, WrapperError> {
        match unsafe { (clingo_propagate_init_assignment(&mut self.0) as *mut Assignment).as_mut() }
        {
            Some(stm) => Ok(stm),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &mut Assignment.",
            }),
        }
    }
}

/// Search handle to a solve call.
#[derive(Debug)]
pub struct SolveHandle<'a> {
    theref: &'a mut clingo_solve_handle_t,
}
impl<'a> SolveHandle<'a> {
    /// Get the next solve result.
    ///
    /// Blocks until the result is ready.
    /// When yielding partial solve results can be obtained, i.e.,
    /// when a model is ready, the result will be satisfiable but neither the search exhausted nor the optimality proven.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving fails
    pub fn get(&mut self) -> Result<SolveResult, ClingoError> {
        let mut result = 0;
        if unsafe { clingo_solve_handle_get(self.theref, &mut result) } {
            if let Some(res) = SolveResult::from_bits(result) {
                Ok(res)
            } else {
                panic!("Unknown bitflag in clingo_solve_result: {}.", result);
            }
        } else {
            Err(error())
        }
    }

    /// Wait for the specified amount of time to check if the next result is ready.
    ///
    /// If the time is set to zero, this function can be used to poll if the search is still active.
    /// If the time is negative, the function blocks until the search is finished.
    ///
    /// # Arguments
    ///
    /// * `timeout` - the maximum time to wait
    ///
    /// **Returns:**  whether the search has finished
    pub fn wait(&mut self, timeout: f64) -> bool {
        let mut result = false;
        unsafe { clingo_solve_handle_wait(self.theref, timeout, &mut result) };
        result
    }

    /// Get the next model or None if there are no more models.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving fails
    pub fn model(&mut self) -> Result<Option<&Model>, Error> {
        let mut model = std::ptr::null_mut() as *const clingo_model_t;
        if unsafe { clingo_solve_handle_model(self.theref, &mut model) } {
            Ok(unsafe { (model as *const Model).as_ref() })
        } else {
            Err(error())?
        }
    }

    /// Get the next model or None if there are no more models.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving fails
    pub fn model_mut(&mut self) -> Result<&mut Model, Error> {
        let mut model = std::ptr::null_mut() as *const clingo_model_t;
        if unsafe { clingo_solve_handle_model(self.theref, &mut model) } {
            match unsafe { (model as *mut Model).as_mut() } {
                Some(x) => Ok(x),
                None => Err(WrapperError {
                    msg: "tried casting a null pointer to &mut Model.",
                })?,
            }
        } else {
            Err(error())?
        }
    }
    /// Discards the last model and starts the search for the next one.
    ///
    /// If the search has been started asynchronously, this function continues the search in the
    /// background.
    ///
    /// **Note:** This function does not block.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving fails
    pub fn resume(&mut self) -> Result<(), ClingoError> {
        if unsafe { clingo_solve_handle_resume(self.theref) } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Stop the running search and block until done.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving fails
    pub fn cancel(&mut self) -> Result<(), ClingoError> {
        if unsafe { clingo_solve_handle_cancel(self.theref) } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Stops the running search and releases the handle.
    ///
    /// Blocks until the search is stopped (as if an implicit cancel was called before the handle is
    /// released).
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving fails
    pub fn close(self) -> Result<(), ClingoError> {
        if unsafe { clingo_solve_handle_close(self.theref) } {
            Ok(())
        } else {
            Err(error())
        }
    }
}

/// Internalize a string.
///
/// This functions takes a string as input and returns an equal unique string
/// that is (at the moment) not freed until the program is closed.  All strings
/// returned from clingo API functions are internalized and must not be freed.
///
/// # Arguments
///
/// * `string` - the string to internalize
/// * `result` - the internalized string
///
/// # Errors
///
/// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
pub fn add_string(string: &str) -> Result<&'static str, Error> {
    let in_cstr = CString::new(string)?;
    let mut out_ptr = unsafe { mem::uninitialized() };
    if unsafe { clingo_add_string(in_cstr.as_ptr(), &mut out_ptr) } {
        let out_cstr = unsafe { CStr::from_ptr(out_ptr) };
        Ok(out_cstr.to_str()?)
    } else {
        Err(error())?
    }
}

/// Parse a term in string form.
///
/// The result of this function is a symbol. The input term can contain
/// unevaluated functions, which are evaluated during parsing.
///
/// # Arguments
///
/// * `string` - the string to parse
///
/// # Errors
///
/// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `string` contains a nul byte
/// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
/// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if parsing fails
pub fn parse_term(string: &str) -> Result<Symbol, Error> {
    let c_str = CString::new(string)?;
    let mut symbol = 0 as clingo_symbol_t;
    if unsafe { clingo_parse_term(c_str.as_ptr(), None, std::ptr::null_mut(), 0, &mut symbol) } {
        Ok(Symbol(symbol))
    } else {
        Err(error())?
    }
}

/// Parse a term in string form.
///
/// The result of this function is a symbol. The input term can contain
/// unevaluated functions, which are evaluated during parsing.
///
/// # Arguments
///
/// * `string` - the string to parse
/// * `logger` -  logger to report warnings during parsing
/// * `message_limit` - maximum number of times to call the logger
///
/// # Errors
///
/// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `string` contains a nul byte
/// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
/// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if parsing fails
pub fn parse_term_with_logger<L: Logger>(
    string: &str,
    logger: &mut L,
    message_limit: u32,
) -> Result<Symbol, Error> {
    let c_str = CString::new(string)?;
    let data = logger as *mut L;
    let mut symbol = 0 as clingo_symbol_t;
    if unsafe {
        clingo_parse_term(
            c_str.as_ptr(),
            Some(L::unsafe_logging_callback::<L> as LoggingCallback),
            data as *mut ::std::os::raw::c_void,
            message_limit,
            &mut symbol,
        )
    } {
        Ok(Symbol(symbol))
    } else {
        Err(error())?
    }
}

pub trait GroundProgramObserver {
    /// Called once in the beginning.
    ///
    /// If the incremental flag is true, there can be multiple calls to
    /// [`Control::solve()`](struct.Control.html#method.solve).
    ///
    /// # Arguments
    ///
    /// * `incremental` - whether the program is incremental
    ///
    /// **Returns** whether the call was successful
    fn init_program(&mut self, incremental: bool) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_init_program<T: GroundProgramObserver>(
        incremental: bool,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.init_program(incremental)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_init_program tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Marks the beginning of a block of directives passed to the solver.
    ///
    /// **See:** [`end_step()`](trait.GroundProgramObserver.html#tymethod.end_step)
    ///
    /// **Returns** whether the call was successful
    fn begin_step(&mut self) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_begin_step<T: GroundProgramObserver>(
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.begin_step()
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_begin_step tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Marks the end of a block of directives passed to the solver.
    ///
    /// This function is called before solving starts.
    ///
    /// **See:** [`begin_step()`](trait.GroundProgramObserver.html#tymethod.begin_step)
    ///
    /// **Returns** whether the call was successful
    fn end_step(&mut self) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_end_step<T: GroundProgramObserver>(
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.end_step()
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_end_step tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe rules passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `choice` - determines if the head is a choice or a disjunction
    /// * `head` - the head atoms
    /// * `body` - the body literals
    ///
    /// **Returns** whether the call was successful
    fn rule(&mut self, choice: bool, head: &[Atom], body: &[Literal]) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_rule<T: GroundProgramObserver>(
        choice: bool,
        head: *const clingo_atom_t,
        head_size: usize,
        body: *const clingo_literal_t,
        body_size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!head.is_null());
        let head = std::slice::from_raw_parts(head as *const Atom, head_size);

        assert!(!body.is_null());
        let body = std::slice::from_raw_parts(body as *const Literal, body_size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.rule(choice, head, body)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_rule tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe weight rules passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `choice` - determines if the head is a choice or a disjunction
    /// * `head` - the head atoms
    /// * `lower_bound` - the lower bound of the weight rule
    /// * `body` - the weighted body literals
    ///
    /// **Returns** whether the call was successful
    fn weight_rule(
        &mut self,
        choice: bool,
        head: &[Atom],
        lower_bound: i32,
        body: &[WeightedLiteral],
    ) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_weight_rule<T: GroundProgramObserver>(
        choice: bool,
        head: *const clingo_atom_t,
        head_size: usize,
        lower_bound: clingo_weight_t,
        body: *const clingo_weighted_literal_t,
        body_size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!head.is_null());
        let head = std::slice::from_raw_parts(head as *const Atom, head_size);

        assert!(!body.is_null());
        let body = std::slice::from_raw_parts(body as *const WeightedLiteral, body_size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.weight_rule(choice, head, lower_bound, body)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_weight_rule tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe minimize constraints (or weak constraints) passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `priority` - the priority of the constraint
    /// * `literals` - the weighted literals whose sum to minimize
    ///
    /// **Returns** whether the call was successful
    fn minimize(&mut self, priority: i32, literals: &[WeightedLiteral]) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_minimize<T: GroundProgramObserver>(
        priority: clingo_weight_t,
        literals: *const clingo_weighted_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!literals.is_null());
        let literals = std::slice::from_raw_parts(literals as *const WeightedLiteral, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.minimize(priority, literals)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_minimize tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe projection directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the atoms to project on
    ///
    /// **Returns** whether the call was successful
    fn project(&mut self, atoms: &[Atom]) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_project<T: GroundProgramObserver>(
        atoms: *const clingo_atom_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!atoms.is_null());
        let atoms = std::slice::from_raw_parts(atoms as *const Atom, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.project(atoms)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_project tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe shown atoms passed to the solver.
    ///
    /// **Note:** Facts do not have an associated aspif atom.
    /// The value of the atom is set to zero.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the symbolic representation of the atom
    /// * `atom` - the aspif atom (0 for facts)
    ///
    /// **Returns** whether the call was successful
    fn output_atom(&mut self, symbol: Symbol, atom: Atom) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_output_atom<T: GroundProgramObserver>(
        symbol: clingo_symbol_t,
        atom: clingo_atom_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.output_atom(Symbol(symbol), Atom(atom))
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_output_atom tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe shown terms passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the symbolic representation of the term
    /// * `condition` - the literals of the condition
    ///
    /// **Returns** whether the call was successful
    fn output_term(&mut self, symbol: Symbol, condition: &[Literal]) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_output_term<T: GroundProgramObserver>(
        symbol: clingo_symbol_t,
        condition: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!condition.is_null());
        let condition = std::slice::from_raw_parts(condition as *const Literal, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.output_term(Symbol(symbol), condition)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_output_term tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe shown csp variables passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the symbolic representation of the variable
    /// * `value` - the value of the variable
    /// * `condition` - the literals of the condition
    ///
    /// **Returns** whether the call was successful
    fn output_csp(&mut self, symbol: Symbol, value: i32, condition: &[Literal]) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_output_csp<T: GroundProgramObserver>(
        symbol: clingo_symbol_t,
        value: ::std::os::raw::c_int,
        condition: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!condition.is_null());
        let condition = std::slice::from_raw_parts(condition as *const Literal, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.output_csp(Symbol(symbol), value, condition)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_output_csp tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe external statements passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `atom` - the external atom
    /// * `type` - the type of the external statement
    ///
    /// **Returns** whether the call was successful
    fn external(&mut self, atom: Atom, type_: ExternalType) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_external<T: GroundProgramObserver>(
        atom: clingo_atom_t,
        type_: clingo_external_type_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        let type_ = match type_ as u32 {
            clingo_external_type_clingo_external_type_false => ExternalType::False,
            clingo_external_type_clingo_external_type_free => ExternalType::Free,
            clingo_external_type_clingo_external_type_release => ExternalType::Release,
            clingo_external_type_clingo_external_type_true => ExternalType::True,
            x => panic!("Failed to match clingo_external_type: {}.", x),
        };

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.external(Atom(atom), type_)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_external tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe assumption directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `literals` - the literals to assume (positive literals are true and negative literals
    /// false for the next solve call)
    ///
    /// **Returns** whether the call was successful
    fn assume(&mut self, literals: &[Literal]) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_assume<T: GroundProgramObserver>(
        literals: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!literals.is_null());
        let literals = std::slice::from_raw_parts(literals as *const Literal, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.assume(literals)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_assume tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe heuristic directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `atom` - the target atom
    /// * `type` - the type of the heuristic modification
    /// * `bias` - the heuristic bias
    /// * `priority` - the heuristic priority
    /// * `condition` - the condition under which to apply the heuristic modification
    ///
    /// **Returns** whether the call was successful
    fn heuristic(
        &mut self,
        atom: Atom,
        type_: HeuristicType,
        bias: i32,
        priority: u32,
        condition: &[Literal],
    ) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_heuristic<T: GroundProgramObserver>(
        atom: clingo_atom_t,
        type_: clingo_heuristic_type_t,
        bias: ::std::os::raw::c_int,
        priority: ::std::os::raw::c_uint,
        condition: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        let type_ = match type_ as u32 {
            clingo_heuristic_type_clingo_heuristic_type_factor => HeuristicType::Factor,
            clingo_heuristic_type_clingo_heuristic_type_false => HeuristicType::False,
            clingo_heuristic_type_clingo_heuristic_type_init => HeuristicType::Init,
            clingo_heuristic_type_clingo_heuristic_type_level => HeuristicType::Level,
            clingo_heuristic_type_clingo_heuristic_type_sign => HeuristicType::Sign,
            clingo_heuristic_type_clingo_heuristic_type_true => HeuristicType::True,
            x => panic!("Failed to match clingo_heuristic_type: {}.", x),
        };

        assert!(!condition.is_null());
        let condition = std::slice::from_raw_parts(condition as *const Literal, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.heuristic(Atom(atom), type_, bias, priority, condition)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_heuristic tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe edge directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `node_u` - the start vertex of the edge
    /// * `node_v` - the end vertex of the edge
    /// * `condition` - the condition under which the edge is part of the graph
    ///
    /// **Returns** whether the call was successful
    fn acyc_edge(&mut self, node_u: i32, node_v: i32, condition: &[Literal]) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_acyc_edge<T: GroundProgramObserver>(
        node_u: ::std::os::raw::c_int,
        node_v: ::std::os::raw::c_int,
        condition: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!condition.is_null());
        let condition = std::slice::from_raw_parts(condition as *const Literal, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.acyc_edge(node_u, node_v, condition)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_acyc_edge tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe numeric theory terms.
    ///
    /// # Arguments
    ///
    /// * `term_id` - the id of the term
    /// * `number` - the value of the term
    ///
    /// **Returns** whether the call was successful
    fn theory_term_number(&mut self, term_id: Id, number: i32) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_theory_term_number<T: GroundProgramObserver>(
        term_id: clingo_id_t,
        number: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.theory_term_number(Id(term_id), number)
        } else {
            set_internal_error(
                        ErrorType::Runtime,
                            "unsafe_theory_term_number tried casting a null pointer to &mut GroundProgramObserver."
                    );
            false
        }
    }

    /// Observe string theory terms.
    ///
    /// # Arguments
    ///
    /// * `term_id` - the id of the term
    /// * `name` - the value of the term
    ///
    /// **Returns** whether the call was successful
    fn theory_term_string(&mut self, term_id: Id, name: &str) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_theory_term_string<T: GroundProgramObserver>(
        term_id: clingo_id_t,
        name: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!name.is_null());
        let cstr = CStr::from_ptr(name);
        let name = cstr.to_str().unwrap_or_else(|err| panic!(err));

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.theory_term_string(Id(term_id), name)
        } else {
            set_internal_error(
                        ErrorType::Runtime,
                            "unsafe_theory_term_string tried casting a null pointer to &mut GroundProgramObserver."
                    );
            false
        }
    }

    /// Observe compound theory terms.
    ///
    /// The name_id_or_type gives the type of the compound term:
    /// - if it is -1, then it is a tuple
    /// - if it is -2, then it is a set
    /// - if it is -3, then it is a list
    /// - otherwise, it is a function and name_id_or_type refers to the id of the name (in form of a
    /// string term)
    ///
    /// # Arguments
    ///
    /// * `term_id` - the id of the term
    /// * `name_id_or_type` - the name or type of the term
    /// * `arguments` - the arguments of the term
    ///
    /// **Returns** whether the call was successful
    fn theory_term_compound(&mut self, term_id: Id, name_id_or_type: i32, arguments: &[Id])
        -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_theory_term_compound<T: GroundProgramObserver>(
        term_id: clingo_id_t,
        name_id_or_type: ::std::os::raw::c_int,
        arguments: *const clingo_id_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!arguments.is_null());
        let arguments = std::slice::from_raw_parts(arguments as *const Id, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.theory_term_compound(Id(term_id), name_id_or_type, arguments)
        } else {
            set_internal_error(
                        ErrorType::Runtime,
                            "unsafe_theory_term_compound tried casting a null pointer to &mut GroundProgramObserver."
                    );
            false
        }
    }

    /// Observe theory elements.
    ///
    /// # Arguments
    ///
    /// * `element_id` - the id of the element
    /// * `terms` - the term tuple of the element
    /// * `condition` - the condition of the element
    ///
    /// **Returns** whether the call was successful
    fn theory_element(&mut self, element_id: Id, terms: &[Id], condition: &[Literal]) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_theory_element<T: GroundProgramObserver>(
        element_id: clingo_id_t,
        terms: *const clingo_id_t,
        terms_size: usize,
        condition: *const clingo_literal_t,
        condition_size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!terms.is_null());
        let terms = std::slice::from_raw_parts(terms as *const Id, terms_size);

        assert!(!condition.is_null());
        let condition = std::slice::from_raw_parts(condition as *const Literal, condition_size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.theory_element(Id(element_id), terms, condition)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_theory_element tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe theory atoms without guard.
    ///
    /// # Arguments
    ///
    /// * `atom_id_or_zero` - the id of the atom or zero for directives
    /// * `term_id` - the term associated with the atom
    /// * `elements` - the elements of the atom
    ///
    /// **Returns** whether the call was successful
    fn theory_atom(&mut self, atom_id_or_zero: Id, term_id: Id, elements: &[Id]) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_theory_atom<T: GroundProgramObserver>(
        atom_id_or_zero: clingo_id_t,
        term_id: clingo_id_t,
        elements: *const clingo_id_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!elements.is_null());
        let elements = std::slice::from_raw_parts(elements as *const Id, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.theory_atom(Id(atom_id_or_zero), Id(term_id), elements)
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_theory_atom tried casting a null pointer to &mut GroundProgramObserver.",
            );
            false
        }
    }

    /// Observe theory atoms with guard.
    ///
    /// # Arguments
    ///
    /// * `atom_id_or_zero` - the id of the atom or zero for directives
    /// * `term_id` - the term associated with the atom
    /// * `elements` - the elements of the atom
    /// * `operator_id` - the id of the operator (a string term)
    /// * `right_hand_side_id` - the id of the term on the right hand side of the atom
    ///
    /// **Returns** whether the call was successful
    fn theory_atom_with_guard(
        &mut self,
        atom_id_or_zero: Id,
        term_id: Id,
        elements: &[Id],
        operator_id: Id,
        right_hand_side_id: Id,
    ) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_theory_atom_with_guard<T: GroundProgramObserver>(
        atom_id_or_zero: clingo_id_t,
        term_id: clingo_id_t,
        elements: *const clingo_id_t,
        size: usize,
        operator_id: clingo_id_t,
        right_hand_side_id: clingo_id_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!elements.is_null());
        let elements = std::slice::from_raw_parts(elements as *const Id, size);

        if let Some(gpo) = (data as *mut T).as_mut() {
            gpo.theory_atom_with_guard(
                Id(atom_id_or_zero),
                Id(term_id),
                elements,
                Id(operator_id),
                Id(right_hand_side_id),
            )
        } else {
            set_internal_error(
                        ErrorType::Runtime,
                            "unsafe_theory_atom_with_guard tried casting a null pointer to &mut GroundProgramObserver."
                    );
            false
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
// }
