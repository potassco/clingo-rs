#![feature(ptr_internals)]
#![allow(non_upper_case_globals)]
extern crate clingo_sys;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate libc;

use std::mem;
use std::ptr::Unique;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ffi::CStr;
use std::ffi::CString;
use libc::c_char;
use clingo_sys::*;
pub use failure::Error;

/// Functions and data structures to work with program ASTs
pub mod ast;

/// Error from the clingo library.
///
/// **Note:** Errors can only be recovered from if explicitly mentioned; most
/// functions do not provide strong exception guarantees.  This means that in
/// case of errors associated objects cannot be used further.
#[derive(Debug, Fail)]
#[fail(display = "ErrorType::{:?}: {}", type_, msg)]
pub struct ClingoError {
    pub type_: ErrorType,
    pub msg: &'static str,
}

/// Error discovered in the bindings
#[derive(Debug, Fail)]
#[fail(display = "Error discovered in the bindings: {}", msg)]
pub struct BindingError {
    msg: &'static str,
}

/// Enumeration of error types.
#[derive(Debug, Copy, Clone)]
pub enum ErrorType {
    /// successful API calls
    Success = clingo_error_clingo_error_success as isize,
    /// errors only detectable at runtime like invalid input
    Runtime = clingo_error_clingo_error_runtime as isize,
    /// wrong usage of the clingo API
    Logic = clingo_error_clingo_error_logic as isize,
    /// memory could not be allocated
    BadAlloc = clingo_error_clingo_error_bad_alloc as isize,
    /// errors unrelated to clingo
    Unknown = clingo_error_clingo_error_unknown as isize,
    /// custom error set by the user
    CustomError,
}
impl From<i32> for ErrorType {
    fn from(error: i32) -> Self {
        match error as u32 {
            clingo_error_clingo_error_success => ErrorType::Success,
            clingo_error_clingo_error_runtime => ErrorType::Runtime,
            clingo_error_clingo_error_logic => ErrorType::Logic,
            clingo_error_clingo_error_bad_alloc => ErrorType::BadAlloc,
            clingo_error_clingo_error_unknown => ErrorType::Unknown,
            _ => ErrorType::CustomError,
        }
    }
}

/// Represents three-valued truth values.
#[derive(Debug, Copy, Clone)]
pub enum TruthValue {
    // no truth value
    Free = clingo_truth_value_clingo_truth_value_free as isize,
    //     true
    True = clingo_truth_value_clingo_truth_value_true as isize,
    //     false
    False = clingo_truth_value_clingo_truth_value_false as isize,
}

/// Enumeration of clause types determining the lifetime of a clause.
///
/// Clauses in the solver are either cleaned up based on a configurable deletion policy or at the end of a solving step.
/// The values of this enumeration determine if a clause is subject to one of the above deletion strategies.
#[derive(Debug, Copy, Clone)]
pub enum ClauseType {
    ///  clause is subject to the solvers deletion policy
    Learnt = clingo_clause_type_clingo_clause_type_learnt as isize,
    /// clause is not subject to the solvers deletion policy
    Static = clingo_clause_type_clingo_clause_type_static as isize,
    /// like `Learnt` but the clause is deleted after a solving step
    Volatile = clingo_clause_type_clingo_clause_type_volatile as isize,
    /// like `Static` but the clause is deleted after a solving step
    VolatileStatic = clingo_clause_type_clingo_clause_type_volatile_static as isize,
}

/// Enumeration of solve events.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SolveEventType {
    /// Issued if a model is found.
    Model = clingo_solve_event_type_clingo_solve_event_type_model as isize,
    /// Issued if the search has completed.
    Finish = clingo_solve_event_type_clingo_solve_event_type_finish as isize,
}

/// Enumeration for entries of the statistics.
#[derive(Debug, Copy, Clone)]
pub enum StatisticsType {
    /// the entry is invalid (has neither of the types below)
    Empty = clingo_statistics_type_clingo_statistics_type_empty as isize,
    /// the entry is a (string) value
    Value = clingo_statistics_type_clingo_statistics_type_value as isize,
    /// the entry is an array
    Array = clingo_statistics_type_clingo_statistics_type_array as isize,
    /// the entry is a map
    Map = clingo_statistics_type_clingo_statistics_type_map as isize,
}

/// Enumeration of available symbol types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SymbolType {
    /// the `#inf` symbol
    Infimum = clingo_symbol_type_clingo_symbol_type_infimum as isize,
    /// a numeric symbol, e.g., `1`
    Number = clingo_symbol_type_clingo_symbol_type_number as isize,
    /// a string symbol, e.g., `"a"`
    String = clingo_symbol_type_clingo_symbol_type_string as isize,
    /// a numeric symbol, e.g., `c`, `(1, "a")`, or `f(1,"a")`
    Function = clingo_symbol_type_clingo_symbol_type_function as isize,
    /// the `#sup` symbol
    Supremum = clingo_symbol_type_clingo_symbol_type_supremum as isize,
}

/// Enumeration of warning codes.
#[derive(Debug, Copy, Clone)]
pub enum Warning {
    /// undefined arithmetic operation or weight of aggregate
    OperationUndefined = clingo_warning_clingo_warning_operation_undefined as isize,
    /// to report multiple errors; a corresponding runtime error is raised later
    RuntimeError = clingo_warning_clingo_warning_runtime_error as isize,
    /// undefined atom in program
    AtomUndefined = clingo_warning_clingo_warning_atom_undefined as isize,
    /// same file included multiple times
    FileIncluded = clingo_warning_clingo_warning_file_included as isize,
    /// CSP variable with unbounded domain
    VariableUnbound = clingo_warning_clingo_warning_variable_unbounded as isize,
    /// global variable in tuple of aggregate element
    GlobalVariable = clingo_warning_clingo_warning_global_variable as isize,
    /// other kinds of warnings
    Other = clingo_warning_clingo_warning_other as isize,
}

/// Enumeration of different external statements.
#[derive(Debug, Copy, Clone)]
pub enum ExternalType {
    /// allow an external to be assigned freely
    Free = clingo_external_type_clingo_external_type_free as isize,
    /// assign an external to true
    True = clingo_external_type_clingo_external_type_true as isize,
    /// assign an external to false
    False = clingo_external_type_clingo_external_type_false as isize,
    /// no longer treat an atom as external
    Release = clingo_external_type_clingo_external_type_release as isize,
}

/// Enumeration of different heuristic modifiers.
#[derive(Debug, Copy, Clone)]
pub enum HeuristicType {
    /// set the level of an atom
    Level = clingo_heuristic_type_clingo_heuristic_type_level as isize,
    /// configure which sign to chose for an atom
    Sign = clingo_heuristic_type_clingo_heuristic_type_sign as isize,
    /// modify VSIDS factor of an atom
    Factor = clingo_heuristic_type_clingo_heuristic_type_factor as isize,
    /// modify the initial VSIDS score of an atom
    Init = clingo_heuristic_type_clingo_heuristic_type_init as isize,
    /// set the level of an atom and choose a positive sign
    True = clingo_heuristic_type_clingo_heuristic_type_true as isize,
    /// set the level of an atom and choose a negative sign
    False = clingo_heuristic_type_clingo_heuristic_type_false as isize,
}

/// Enumeration of theory term types.
#[derive(Debug, Copy, Clone)]
pub enum TheoryTermType {
    /// a tuple term, e.g., `(1,2,3)`
    Tuple = clingo_theory_term_type_clingo_theory_term_type_tuple as isize,
    /// a list term, e.g., `[1,2,3]`
    List = clingo_theory_term_type_clingo_theory_term_type_list as isize,
    /// a set term, e.g., `{1,2,3}`
    Set = clingo_theory_term_type_clingo_theory_term_type_set as isize,
    /// a function term, e.g., `f(1,2,3)`
    Function = clingo_theory_term_type_clingo_theory_term_type_function as isize,
    /// a number term, e.g., `42`
    Number = clingo_theory_term_type_clingo_theory_term_type_number as isize,
    /// a symbol term, e.g., `c`
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
    /// do not call @ref ::clingo_propagator::check() at all
    None = clingo_propagator_check_mode_clingo_propagator_check_mode_none as isize,
    /// call @ref ::clingo_propagator::check() on total assignment
    Total = clingo_propagator_check_mode_clingo_propagator_check_mode_total as isize,
    /// call @ref ::clingo_propagator::check() on propagation fixpoints
    Fixpoint = clingo_propagator_check_mode_clingo_propagator_check_mode_fixpoint as isize,
}

/// Bit flags for entries of the configuration
#[derive(Debug, Copy, Clone)]
pub struct ConfigurationType(clingo_configuration_type);
impl ConfigurationType {
    pub const VALUE: ConfigurationType =
        ConfigurationType(clingo_configuration_type_clingo_configuration_type_value);
    pub const ARRAY: ConfigurationType =
        ConfigurationType(clingo_configuration_type_clingo_configuration_type_array);
    pub const MAP: ConfigurationType =
        ConfigurationType(clingo_configuration_type_clingo_configuration_type_map);
}
impl ::std::ops::BitOr<ConfigurationType> for ConfigurationType {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        ConfigurationType(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for ConfigurationType {
    #[inline]
    fn bitor_assign(&mut self, rhs: ConfigurationType) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<ConfigurationType> for ConfigurationType {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        ConfigurationType(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for ConfigurationType {
    #[inline]
    fn bitand_assign(&mut self, rhs: ConfigurationType) {
        self.0 &= rhs.0;
    }
}

/// Bit flags of solve modes.
#[derive(Debug, Copy, Clone)]
pub struct SolveMode(clingo_solve_mode);
impl SolveMode {
    pub const ASYNC: SolveMode = SolveMode(clingo_solve_mode_clingo_solve_mode_async);
    pub const YIELD: SolveMode = SolveMode(clingo_solve_mode_clingo_solve_mode_yield);
}
impl ::std::ops::BitOr<SolveMode> for SolveMode {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        SolveMode(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for SolveMode {
    #[inline]
    fn bitor_assign(&mut self, rhs: SolveMode) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<SolveMode> for SolveMode {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        SolveMode(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for SolveMode {
    #[inline]
    fn bitand_assign(&mut self, rhs: SolveMode) {
        self.0 &= rhs.0;
    }
}

/// Bit flags to select symbols in models.
#[derive(Debug, Copy, Clone)]
pub struct ShowType(clingo_show_type);
impl ShowType {
    pub const CSP: ShowType = ShowType(clingo_show_type_clingo_show_type_csp);
    pub const SHOWN: ShowType = ShowType(clingo_show_type_clingo_show_type_shown);
    pub const ATOMS: ShowType = ShowType(clingo_show_type_clingo_show_type_atoms);
    pub const TERMS: ShowType = ShowType(clingo_show_type_clingo_show_type_terms);
    pub const EXTRA: ShowType = ShowType(clingo_show_type_clingo_show_type_extra);
    pub const ALL: ShowType = ShowType(clingo_show_type_clingo_show_type_all);
    pub const COMPLEMENT: ShowType = ShowType(clingo_show_type_clingo_show_type_complement);
}
impl ::std::ops::BitOr<ShowType> for ShowType {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        ShowType(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for ShowType {
    #[inline]
    fn bitor_assign(&mut self, rhs: ShowType) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<ShowType> for ShowType {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        ShowType(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for ShowType {
    #[inline]
    fn bitand_assign(&mut self, rhs: ShowType) {
        self.0 &= rhs.0;
    }
}

/// Bit flags for solve call results
#[derive(Debug, Copy, Clone)]
pub struct SolveResult(clingo_solve_result);
impl SolveResult {
    pub const SATISFIABLE: SolveResult =
        SolveResult(clingo_solve_result_clingo_solve_result_satisfiable);
    pub const UNSATISFIABLE: SolveResult =
        SolveResult(clingo_solve_result_clingo_solve_result_unsatisfiable);
    pub const EXHAUSTED: SolveResult =
        SolveResult(clingo_solve_result_clingo_solve_result_exhausted);
    pub const INTERRUPTED: SolveResult =
        SolveResult(clingo_solve_result_clingo_solve_result_interrupted);
}
impl ::std::ops::BitOr<SolveResult> for SolveResult {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        SolveResult(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for SolveResult {
    #[inline]
    fn bitor_assign(&mut self, rhs: SolveResult) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<SolveResult> for SolveResult {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        SolveResult(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for SolveResult {
    #[inline]
    fn bitand_assign(&mut self, rhs: SolveResult) {
        self.0 &= rhs.0;
    }
}

type SolveEventCallback = unsafe extern "C" fn(
    type_: clingo_solve_event_type_t,
    event: *mut ::std::os::raw::c_void,
    data: *mut ::std::os::raw::c_void,
    goon: *mut bool,
) -> bool;
pub trait SolveEventHandler {
    // TODO: check documentation and solve_event
    /// Callback function called during search to notify when the search is finished or a model is ready.
    ///
    /// If a (non-recoverable) clingo API function fails in this callback, it must return false.
    /// In case of errors not related to clingo, set error code ::clingo_error_unknown and return false to stop solving with an error.
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
        // TODO               assert!(!event.is_null());
        let event_type = match etype {
            clingo_solve_event_type_clingo_solve_event_type_model => SolveEventType::Model,
            clingo_solve_event_type_clingo_solve_event_type_finish => SolveEventType::Finish,
            _ => panic!("Failed to match clingo_solve_event_type."),
        };

        assert!(!data.is_null());
        let event_handler = (data as *mut T)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        assert!(!goon.is_null());
        let goon = goon.as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        event_handler.on_solve_event(event_type, goon)
    }
}

type AstCallback =
    unsafe extern "C" fn(arg1: *const clingo_ast_statement_t, arg2: *mut ::std::os::raw::c_void)
        -> bool;
pub trait AstStatementHandler {
    /// Callback function called on an AstStatement while traversing the Ast.
    ///
    /// **Returns** whether the call was successful
    fn on_statement(&mut self, arg1: &AstStatement) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_ast_callback<T: AstStatementHandler>(
        stm: *const clingo_ast_statement_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!stm.is_null());
        let stm = (stm as *const AstStatement)
            .as_ref()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        assert!(!data.is_null());
        let data = (data as *mut T)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        data.on_statement(stm)
    }
}

type LoggingCallback = unsafe extern "C" fn(
    code: clingo_warning_t,
    message: *const ::std::os::raw::c_char,
    data: *mut ::std::os::raw::c_void,
);
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
    fn log(&mut self, code: Warning, message: &str);
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_logging_callback<L: Logger>(
        code: clingo_warning_t,
        message: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
    ) {
        let warning = match code as u32 {
            clingo_warning_clingo_warning_atom_undefined => Warning::AtomUndefined,
            clingo_warning_clingo_warning_file_included => Warning::FileIncluded,
            clingo_warning_clingo_warning_global_variable => Warning::GlobalVariable,
            clingo_warning_clingo_warning_operation_undefined => Warning::OperationUndefined,
            clingo_warning_clingo_warning_other => Warning::Other,
            clingo_warning_clingo_warning_runtime_error => Warning::RuntimeError,
            clingo_warning_clingo_warning_variable_unbounded => Warning::VariableUnbound,
            _ => panic!("Failed to match clingo_warning."),
        };

        assert!(!message.is_null());
        let c_str = CStr::from_ptr(message);
        let message = c_str.to_str().unwrap_or_else(|err| {
            panic!(err);
        });

        assert!(!data.is_null());
        let logger = (data as *mut L)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        logger.log(warning, message)
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
    /// In case of errors not related to clingo, this function can set error ::clingo_error_unknown and return false to stop grounding with an error.
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
    /// ```
    /// fn on_external_function(
    ///     &mut self,
    ///     _location: &Location,
    ///     name: &str,
    ///     arguments: &[Symbol],
    /// ) -> Result<Vec<Symbol>,Error> {
    ///     if name == "f" && arguments.len() == 0 {
    ///         let symbol = Symbol::create_number(42);
    ///         Ok(vec![symbol])
    ///     } else {
    ///        Err(ClingoError {
    ///          type_: ErrorType::Runtime,
    ///          msg: "function not found",
    ///        })?
    ///    }
    /// }
    /// ```
    fn on_external_function(
        &mut self,
        location: &Location,
        name: &str,
        arguments: &[Symbol],
    ) -> Result<Vec<Symbol>, Error>;
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
        assert!(!location.is_null());
        let location = (location as *const Location)
            .as_ref()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        assert!(!name.is_null());
        let c_str = CStr::from_ptr(name);
        let name = c_str.to_str().unwrap();

        if arguments_size > 0 {
            assert!(!arguments.is_null());
        }
        let arguments = std::slice::from_raw_parts(arguments as *const Symbol, arguments_size);

        assert!(!data.is_null());
        let event_handler = (data as *mut T)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        //         assert!(!symbol_callback_data.is_null());

        match event_handler.on_external_function(location, name, arguments) {
            Ok(symbols) => {
                let symbol_injector = symbol_callback.unwrap();
                let mut v: Vec<clingo_symbol_t> =
                    symbols.iter().map(|symbol| symbol.clone().0).collect();
                symbol_injector(v.as_slice().as_ptr(), v.len(), symbol_callback_data)
            }
            Err(e) => false,
        }
    }
}

/// Represents a symbolic literal.
#[derive(Debug, Copy, Clone)]
pub struct SymbolicLiteral(clingo_symbolic_literal_t);
impl SymbolicLiteral {
    /// Get the associated symbol (must be a function)
    pub fn symbol(&self) -> Symbol {
        Symbol(self.0.symbol)
    }
    /// Whether the literal has a sign
    pub fn positive(&self) -> bool {
        self.0.positive
    }
}

/// Signed integer type used for aspif and solver literals.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Literal(clingo_literal_t);
impl Literal {
    pub fn negate(&self) -> Literal {
        Literal(-(self.0))
    }
    pub fn UNSAFE_from(Atom(atom): Atom) -> Literal {
        Literal(atom as clingo_literal_t)
    }
    pub fn get_integer(&self) -> i32 {
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
    pub fn get_integer(&self) -> u32 {
        self.0
    }
}

/// A Literal with an associated weight.
/// @ingroup ProgramInspection
#[derive(Debug, Copy, Clone)]
pub struct WeightedLiteral(clingo_weighted_literal);
impl WeightedLiteral {
    pub fn literal(&self) -> Literal {
        Literal(self.0.literal)
    }
    pub fn weight(&self) -> i32 {
        self.0.weight
    }
}

/// Represents a source code location marking its beginnig and end.
///
/// **Note:** Not all locations refer to physical files.
/// By convention, such locations use a name put in angular brackets as filename.
#[derive(Debug, Copy, Clone)]
pub struct Location(clingo_location);
impl Location {
    /// Create a new location.
    ///
    /// # Arguments
    // TODO
    pub fn new(
        begin_line: usize,
        end_line: usize,
        begin_column: usize,
        end_column: usize,
        begin_file_: &str,
        end_file_: &str,
    ) -> Location {
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
        Location(loc)
    }
    /// the file where the location begins
    pub fn begin_file(&self) -> &str {
        if self.0.begin_file.is_null() {
            ""
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.begin_file) };
            c_str.to_str().unwrap()
        }
    }
    /// the file where the location ends
    pub fn end_file(&self) -> &str {
        if self.0.end_file.is_null() {
            ""
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.end_file) };
            c_str.to_str().unwrap()
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
    pub fn create(name_: &str, arity: u32, positive: bool) -> Result<Signature, Error> {
        let name_c_str = CString::new(name_).unwrap();
        let mut signature = 0;
        if unsafe { clingo_signature_create(name_c_str.as_ptr(), arity, positive, &mut signature) }
        {
            Ok(Signature(signature))
        } else {
            Err(error())?
        }
    }
    // TODO
    //     /// Get the name of a signature.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `signature` - the target signature
    //     ///
    //     /// **Returns** the name of the signature
    //     pub fn clingo_signature_name(signature: clingo_signature_t) -> *const ::std::os::raw::c_char;

    // TODO
    //     /// Get the arity of a signature.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `signature` - the target signature
    //     ///
    //     /// **Returns** the arity of the signature
    //     pub fn clingo_signature_arity(signature: clingo_signature_t) -> u32;

    // TODO
    //     /// Whether the signature is positive (is not classically negated).
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `signature` - the target signature
    //     ///
    //     /// **Returns** whether the signature has no sign
    //     pub fn clingo_signature_is_positive(signature: clingo_signature_t) -> bool;

    // TODO
    //     /// Whether the signature is negative (is classically negated).
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `signature` - the target signature
    //     ///
    //     /// **Returns** whether the signature has a sign
    //     pub fn clingo_signature_is_negative(signature: clingo_signature_t) -> bool;
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
    /// #  Errors:
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn create_string(string: &str) -> Result<Symbol, Error> {
        let mut symbol = 0 as clingo_symbol_t;
        let c_str = CString::new(string).unwrap();
        if unsafe { clingo_symbol_create_string(c_str.as_ptr(), &mut symbol) } {
            Ok(Symbol(symbol))
        } else {
            Err(error())?
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
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn create_id(name: &str, positive: bool) -> Result<Symbol, Error> {
        let mut symbol = 0 as clingo_symbol_t;
        let name_c_str = CString::new(name).unwrap();
        if unsafe { clingo_symbol_create_id(name_c_str.as_ptr(), positive, &mut symbol) } {
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
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn create_function(
        name: &str,
        arguments: &[Symbol],
        positive: bool,
    ) -> Result<Symbol, Error> {
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
            Ok(Symbol(symbol))
        } else {
            Err(error())?
        }
    }

    /// Get the number of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type `SymbolType::Number`
    pub fn number(&self) -> Result<i32, Error> {
        let mut number = 0;
        if unsafe { clingo_symbol_number(self.0, &mut number) } {
            Ok(number)
        } else {
            Err(error())?
        }
    }

    /// Get the name of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type `SymbolType::Function`
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
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type `SymbolType::String`
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
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type `SymbolType::Function`
    pub fn is_positive(&self) -> Result<bool, Error> {
        let mut positive = false;
        if unsafe { clingo_symbol_is_positive(self.0, &mut positive) } {
            Ok(positive)
        } else {
            Err(error())?
        }
    }

    /// Check if a function is negative (has a sign).
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type `SymbolType::Function`
    pub fn is_negative(&self) -> Result<bool, Error> {
        let mut negative = false;
        if unsafe { clingo_symbol_is_negative(self.0, &mut negative) } {
            Ok(negative)
        } else {
            Err(error())?
        }
    }

    /// Get the arguments of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if symbol is not of type `SymbolType::Function`
    pub fn arguments(&self) -> Result<Vec<Symbol>, Error> {
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
            Err(error())?
        }
    }

    /// Get the type of a symbol.
    ///
    /// # Errors
    ///
    /// - may failed to match clingo symbol type
    //TODO maybe unnecesary function in Rust API?
    pub fn symbol_type(&self) -> SymbolType {
        let stype = unsafe { clingo_symbol_type(self.0) };
        match stype as u32 {
            clingo_symbol_type_clingo_symbol_type_infimum => SymbolType::Infimum,
            clingo_symbol_type_clingo_symbol_type_number => SymbolType::Number,
            clingo_symbol_type_clingo_symbol_type_string => SymbolType::String,
            clingo_symbol_type_clingo_symbol_type_function => SymbolType::Function,
            clingo_symbol_type_clingo_symbol_type_supremum => SymbolType::Supremum,
            _ => panic!("Failed to match clingo_symbol_type."),
        }
    }

    /// Get the string representation of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn to_string(&self) -> Result<String, Error> {
        let mut size: usize = 0;
        if unsafe { clingo_symbol_to_string_size(self.0, &mut size) } {
            let a1 = vec![1; size];
            let cstring = unsafe { CString::from_vec_unchecked(a1) };
            if unsafe { clingo_symbol_to_string(self.0, cstring.as_ptr() as *mut c_char, size) } {
                Ok(cstring.into_string()?)
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
/// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if parsing fails
/// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
pub fn parse_program<T: AstStatementHandler>(program_: &str, handler: &mut T) -> Result<(), Error> {
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let program = CString::new(program_).unwrap();
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
/// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if parsing fails
/// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
pub fn parse_program_with_logger<T: AstStatementHandler, L: Logger>(
    program_: &str,
    handler: &mut T,
    logger: &mut L,
    message_limit: u32,
) -> Result<(), Error> {
    let handler_data = handler as *mut T;
    let logger_data = logger as *mut L;
    let program = CString::new(program_).unwrap();
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
    pub fn new(name: &str, params: &'a [Symbol]) -> Part<'a> {
        Part {
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
fn error() -> ClingoError {
    ClingoError {
        type_: ErrorType::from(unsafe { clingo_error_code() }),
        msg: error_message(),
    }
}

/// Get the last error message set if an API call fails.
///
/// **Note:** Each thread has its own local error message.
fn error_message() -> &'static str {
    let char_ptr: *const c_char = unsafe { clingo_error_message() };
    if char_ptr.is_null() {
        ""
    } else {
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        c_str.to_str().unwrap()
    }
}

/// Set a custom error code and message in the active thread.
pub fn set_error(code: ErrorType, message: &str) {
    let message_c_str = CString::new(message).unwrap();
    unsafe { clingo_set_error(code as clingo_error_t, message_c_str.as_ptr()) }
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
        let init = (init as *mut PropagateInit)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        assert!(!data.is_null());
        let propagator = (data as *mut T)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        propagator.init(init)
    }
    //TODO
    /// Can be used to propagate solver literals given a [partial assignment](struct.Assignment.html).
    ///
    /// Called during propagation with a non-empty array of [watched solver literals](struct.PropagateInit.html#method.add_watch)
    /// that have been assigned to true since the last call to either propagate, undo, (or the start of the search) - the change set.
    /// Only watched solver literals are contained in the change set.
    /// Each literal in the change set is true w.r.t. the current [assignment](struct.Assignment.html).
    /// [`PropagateControl::add_clause()`](struct.PropagateControl.html#method.add_clause) can be used to add clauses.
    /// If a clause is unit resulting, it can be propagated using [`PropagateControl::propagate()`](struct.PropagateControl.html#method.propagate).
    /// If the result of either of the two methods is false, the propagate function must return
    /// immediately.
    ///
    /// The following snippet shows how to use the methods to add clauses and propagate consequences
    /// within the callback.
    /// The important point is to return true (true to indicate there was no error) if the result of
    /// either of the methods is false.
    /// ```
    /// bool result;
    /// clingo_literal_t clause[] = { ... };
    ///
    /// // add a clause
    /// if (!clingo_propagate_control_add_clause(control, clause, clingo_clause_type_learnt, &result) { return false; }
    /// if (!result) { return true; }
    /// // propagate its consequences
    /// if (!clingo_propagate_control_propagate(control, &result) { return false; }
    /// if (!result) { return true; }
    ///
    /// // add further clauses and propagate them
    /// ...
    ///
    /// return true;
    /// ```
    ///
    /// **Note:**
    /// This function can be called from different solving threads.
    /// Each thread has its own assignment and id, which can be obtained using [`PropagateControl::thread_id()`](struct.PropagateControl.html#method.thread_id).
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
        let control = (control as *mut PropagateControl)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        assert!(!changes.is_null());
        let changes = std::slice::from_raw_parts(changes as *const Literal, size);

        assert!(!data.is_null());
        let propagator = (data as *mut T)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        propagator.propagate(control, changes)
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
        assert!(!control.is_null());
        let control = (control as *mut PropagateControl)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        assert!(!changes.is_null());
        let changes = std::slice::from_raw_parts(changes as *const Literal, size);

        assert!(!data.is_null());
        let propagator = (data as *mut T)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        propagator.undo(control, changes)
    }
    /// This function is similar to [`PropagateControl::propagate()`](struct.PropagateControl.html#method.propagate) but is only called on total assignments without a change set.
    ///
    /// When exactly this function is called, can be configured using the [`PropagateInit::set_check_mode()`](struct.PropagateInit.html#method.set_check_mode) function.
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
        assert!(!control.is_null());
        let control = (control as *mut PropagateControl)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));

        assert!(!data.is_null());
        let propagator = (data as *mut T)
            .as_mut()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."));
        propagator.check(control)
    }
}

/// Control object holding grounding and solving state.
#[derive(Debug)]
pub struct Control {
    ctl: Unique<clingo_control_t>,
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
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if argument parsing fails
    pub fn new(arguments: std::vec::Vec<String>) -> Result<Control, Error> {
        let logger = None;
        let logger_data = std::ptr::null_mut();

        // create a vector of zero terminated strings
        let mut args: Vec<CString> = arguments
            .into_iter()
            .map(|arg| CString::new(arg).unwrap())
            .collect();

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
                0,
                &mut ctl,
            )
        } {
            Ok(Control {
                ctl: Unique::new(ctl).unwrap(),
            })
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
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if argument parsing fails

    pub fn new_with_logger<L: Logger>(
        arguments: Vec<String>,
        logger: &mut L,
        message_limit: u32,
    ) -> Result<Control, Error> {
        let mut args: Vec<CString> = arguments
            .into_iter()
            .map(|arg| CString::new(arg).unwrap())
            .collect();

        // convert the strings to raw pointers
        let c_args = args.iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let mut ctl = unsafe { mem::uninitialized() };

        let data = logger as *mut L;
        if unsafe {
            clingo_control_new(
                c_args.as_ptr(),
                c_args.len(),
                Some(L::unsafe_logging_callback::<L> as LoggingCallback),
                data as *mut ::std::os::raw::c_void,
                message_limit,
                &mut ctl,
            )
        } {
            Ok(Control {
                ctl: Unique::new(ctl).unwrap(),
            })
        } else {
            Err(error())?
        }
    }

    //NOTTODO: pub fn clingo_control_load(control: *mut Control, file: *const c_char) -> bool;

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
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if parsing fails
    pub fn add(&mut self, name_: &str, parameters: &[&str], program_: &str) -> Result<(), Error> {
        let name = CString::new(name_).unwrap();
        let name_ptr = name.as_ptr();

        let program = CString::new(program_).unwrap();
        let program_ptr = program.as_ptr();

        let parameters_size = parameters.len();

        // create a vector of zero terminated strings
        let l_parameters = parameters
            .into_iter()
            .map(|arg| CString::new(*arg).unwrap())
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
    pub fn ground(&mut self, parts: &[Part]) -> Result<(), Error> {
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
    ) -> Result<(), Error> {
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
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving could not be started
    pub fn solve(
        &mut self,
        mode: &SolveMode,
        assumptions: &[SymbolicLiteral],
    ) -> Result<SolveHandle, Error> {
        let mut handle = std::ptr::null_mut() as *mut clingo_solve_handle_t;
        if unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode.0,
                assumptions.as_ptr() as *const clingo_symbolic_literal_t,
                assumptions.len(),
                None,
                std::ptr::null_mut() as *mut ::std::os::raw::c_void,
                &mut handle,
            )
        } {
            Ok(SolveHandle {
                theref: unsafe { handle.as_mut() }.unwrap(),
            })
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
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving could not be started
    pub fn solve_with_event_handler<T: SolveEventHandler>(
        &mut self,
        mode: &SolveMode,
        assumptions: &[SymbolicLiteral],
        handler: &mut T,
    ) -> Result<SolveHandle, Error> {
        let mut handle = std::ptr::null_mut() as *mut clingo_solve_handle_t;
        let data = handler as *mut T;
        if unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode.0,
                assumptions.as_ptr() as *const clingo_symbolic_literal_t,
                assumptions.len(),
                Some(T::unsafe_solve_callback::<T> as SolveEventCallback),
                data as *mut ::std::os::raw::c_void,
                &mut handle,
            )
        } {
            Ok(SolveHandle {
                theref: unsafe { handle.as_mut() }.unwrap(),
            })
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
    pub fn cleanup(&mut self) -> Result<(), Error> {
        if unsafe { clingo_control_cleanup(self.ctl.as_ptr()) } {
            Ok(())
        } else {
            Err(error())?
        }
    }

    /// Assign a truth value to an external atom.
    ///
    /// If the atom does not exist or is not external, this is a noop.
    ///
    /// # Arguments
    ///
    /// * `atom` - atom to assign
    /// * `value` - the truth value
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn assign_external(&mut self, symbol: &Symbol, value: TruthValue) -> Result<(), Error> {
        if unsafe {
            clingo_control_assign_external(
                self.ctl.as_ptr(),
                symbol.0,
                value as clingo_truth_value_t,
            )
        } {
            Ok(())
        } else {
            Err(error())?
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
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn release_external(&mut self, Symbol(atom): Symbol) -> Result<(), Error> {
        if unsafe { clingo_control_release_external(self.ctl.as_ptr(), atom) } {
            Ok(())
        } else {
            Err(error())?
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
    ) -> Result<(), Error> {
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
            Err(error())?
        }
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
                None => Err(BindingError {
                    msg: "Failed dereferencing pointer to clingo_statistics.",
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
    pub fn configuration(&mut self) -> Option<&mut Configuration> {
        let mut conf = std::ptr::null_mut() as *mut clingo_configuration_t;
        if unsafe { clingo_control_configuration(self.ctl.as_ptr(), &mut conf) } {
            unsafe { (conf as *mut Configuration).as_mut() }
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
    /// * `name` - the name of the constant
    pub fn get_const(&self, name: &str) -> Option<Symbol> {
        let c_str_name = CString::new(name).unwrap();
        let mut symbol = 0 as clingo_symbol_t;
        if unsafe { clingo_control_get_const(self.ctl.as_ptr(), c_str_name.as_ptr(), &mut symbol) }
        {
            Some(Symbol(symbol))
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
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if constant definition does not exist
    ///
    /// **See:** [`Part::get_const()`](struct.Part.html#method.get_const)
    pub fn has_const(&self, name: &str) -> Result<bool, Error> {
        let c_str_name = CString::new(name).unwrap();
        let mut exist = false;
        if unsafe { clingo_control_has_const(self.ctl.as_ptr(), c_str_name.as_ptr(), &mut exist) } {
            Ok(exist)
        } else {
            Err(error())?
        }
    }

    /// Get an object to inspect symbolic atoms (the relevant Herbrand base) used
    /// for grounding.
    pub fn symbolic_atoms(&self) -> Option<&SymbolicAtoms> {
        let mut atoms = std::ptr::null() as *const clingo_symbolic_atoms_t;
        if unsafe { clingo_control_symbolic_atoms(self.ctl.as_ptr(), &mut atoms) } {
            unsafe { (atoms as *const SymbolicAtoms).as_ref() }
        } else {
            None
        }
    }

    /// Get an object to inspect theory atoms that occur in the grounding.
    pub fn theory_atoms(&self) -> Option<&TheoryAtoms> {
        let mut atoms = std::ptr::null() as *const clingo_theory_atoms_t;
        if unsafe { clingo_control_theory_atoms(self.ctl.as_ptr(), &mut atoms) } {
            unsafe { (atoms as *const TheoryAtoms).as_ref() }
        } else {
            None
        }
    }

    // TODO
    //     /// Register a program observer with the control object.
    //     ///
    //     /// # Arguments
    //     ///
    //     /// * `control` - the target
    //     /// * `observer` - the observer to register
    //     /// * `replace` - just pass the grounding to the observer but not the solver
    //     /// * `data` - user data passed to the observer functions
    //     ///
    //     /// **Returns** whether the call was successful
    //     pub fn clingo_control_register_observer(
    //         control: *mut clingo_control_t,
    //         observer: *const clingo_ground_program_observer_t,
    //         replace: bool,
    //         data: *mut ::std::os::raw::c_void,
    //     ) -> bool;

    /// Get an object to add ground directives to the program.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn backend(&mut self) -> Option<&mut Backend> {
        let mut backend = std::ptr::null_mut();
        if unsafe { clingo_control_backend(self.ctl.as_ptr(), &mut backend) } {
            unsafe { (backend as *mut Backend).as_mut() }
        } else {
            None
        }
    }

    /// Get an object to add non-ground directives to the program.
    pub fn program_builder(&mut self) -> Option<ProgramBuilder> {
        let mut builder = std::ptr::null_mut() as *mut clingo_program_builder_t;
        if unsafe { clingo_control_program_builder(self.ctl.as_ptr(), &mut builder) } {
            // begin building the program
            if unsafe { clingo_program_builder_begin(builder) } {
                Some(ProgramBuilder {
                    theref: unsafe { builder.as_mut() }.unwrap(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    // NOTTODO: pub fn clingo_control_clasp_facade()
}
#[derive(Clone)]
pub struct AstStatement(clingo_ast_statement_t);
impl AstStatement {
    pub fn new_external(
        Location(location): Location,
        type_: ast::StatementType,
        ext: &ast::External,
    ) -> AstStatement {
        let external: *const ast::External = ext;
        let _bg_union_2 = clingo_ast_statement__bindgen_ty_1 {
            external: external as *const clingo_ast_external,
        };
        let stm = clingo_ast_statement_t {
            location: location,
            type_: type_ as clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        AstStatement(stm)
    }

    pub fn new_rule(Location(location): Location, rule_: &ast::Rule) -> AstStatement {
        let rule: *const ast::Rule = rule_;

        let _bg_union_2 = clingo_ast_statement__bindgen_ty_1 {
            rule: rule as *const clingo_ast_rule,
        };
        let stm = clingo_ast_statement_t {
            location: location,
            type_: ast::StatementType::Rule as clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        AstStatement(stm)
    }

    pub fn location(&self) -> Location {
        Location(self.0.location)
    }

    pub fn statement_type(&self) -> ast::StatementType {
        let AstStatement(ref stm) = *self;
        match stm.type_ as u32 {
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
            _ => panic!("Failed to match clingo_ast_statement_type."),
        }
    }

    pub unsafe fn rule(&self) -> &ast::Rule {
        let AstStatement(ref stm) = *self;
        let ast_rule_ptr = stm.__bindgen_anon_1.rule as *const clingo_ast_rule_t;
        (ast_rule_ptr as *const ast::Rule)
            .as_ref()
            .unwrap_or_else(|| panic!("Tried dereferencing a null pointer."))
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
    pub fn add(&mut self, stm: &AstStatement) -> Result<(), Error> {
        if unsafe { clingo_program_builder_add(self.theref, &stm.0) } {
            Ok(())
        } else {
            Err(error())?
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

/// Handle for to the solver configuration.
#[derive(Debug, Copy, Clone)]
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
    /// The type is bitset, an entry can have multiple (but at least one) type.
    pub fn configuration_type(&self, Id(key): Id) -> Option<ConfigurationType> {
        let mut ctype = 0 as clingo_configuration_type_bitset_t;
        if unsafe { clingo_configuration_type(&self.0, key, &mut ctype) } {
            Some(ConfigurationType(ctype))
        } else {
            None
        }
    }

    /// Get the description of an entry.
    pub fn description(&self, Id(key): Id) -> Option<&str> {
        let mut description_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_configuration_description(
                &self.0,
                key,
                &mut description_ptr as *mut *const c_char,
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

    //TODO     /// Query whether the map has a key.
    //     ///
    //     /// @pre The @link clingo_configuration_type() type@endlink of the entry must be @ref ::clingo_configuration_type_map.
    //     ///
    //     /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `configuration` - the target configuration
    //     /// * `key` - the key
    //     /// * `name` - the name to lookup the subkey
    //     /// * `result` - whether the key is in the map
    //     ///
    //     /// **Returns** whether the call was successful
    //     pub fn clingo_configuration_map_has_subkey(
    //         configuration: *mut clingo_configuration_t,
    //         key: clingo_id_t,
    //         name: *const ::std::os::raw::c_char,
    //         result: *mut bool,
    //     ) -> bool;

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
    pub fn map_subkey_name(&self, Id(key): Id, offset: usize) -> Option<&str> {
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
            Some(cstr.to_str().unwrap())
        } else {
            None
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
        let name_c_str = CString::new(name).unwrap();
        if unsafe { clingo_configuration_map_at(&self.0, key, name_c_str.as_ptr(), &mut nkey) } {
            Some(Id(nkey))
        } else {
            None
        }
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

    //NOTTODO: clingo_configuration_value_get_size(&mut self.0, key, &mut size)

    /// Get the string value of the given entry.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::VALUE`](struct.ConfigurationType.html#associatedconstant.VALUE).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn value_get(&self, Id(key): Id) -> Option<&str> {
        let mut size = 0;
        if unsafe { clingo_configuration_value_get_size(&self.0, key, &mut size) } {
            let mut value_ptr = unsafe { mem::uninitialized() };
            if unsafe { clingo_configuration_value_get(&self.0, key, &mut value_ptr, size) } {
                let cstr = unsafe { CStr::from_ptr(&value_ptr) };
                Some(cstr.to_str().unwrap())
            } else {
                None
            }
        } else {
            None
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
    pub fn value_set(&mut self, Id(key): Id, value: &str) -> Option<()> {
        let value_c_str = CString::new(value).unwrap();
        if unsafe { clingo_configuration_value_set(&mut self.0, key, value_c_str.as_ptr()) } {
            Some(())
        } else {
            None
        }
    }
}

/// Handle to the backend to add directives in aspif format.
#[derive(Debug, Copy, Clone)]
pub struct Backend(clingo_backend_t);
impl Backend {
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
    pub fn rule(&mut self, choice: bool, head: &[Atom], body: &[Literal]) -> Result<(), Error> {
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
            Err(error())?
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
    ) -> Result<(), Error> {
        if unsafe {
            clingo_backend_weight_rule(
                &mut self.0,
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
            Err(error())?
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
    pub fn minimize(&mut self, priority: i32, literals: &[WeightedLiteral]) -> Result<(), Error> {
        if unsafe {
            clingo_backend_minimize(
                &mut self.0,
                priority,
                literals.as_ptr() as *const clingo_weighted_literal_t,
                literals.len(),
            )
        } {
            Ok(())
        } else {
            Err(error())?
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
    pub fn project(&mut self, atoms: &[Atom]) -> Result<(), Error> {
        if unsafe {
            clingo_backend_project(
                &mut self.0,
                atoms.as_ptr() as *const clingo_atom_t,
                atoms.len(),
            )
        } {
            Ok(())
        } else {
            Err(error())?
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
    pub fn external(&mut self, atom: &Atom, type_: ExternalType) -> Result<(), Error> {
        if unsafe { clingo_backend_external(&mut self.0, atom.0, type_ as clingo_external_type_t) }
        {
            Ok(())
        } else {
            Err(error())?
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
    pub fn assume(&mut self, literals: &[Literal]) -> Result<(), Error> {
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
            Err(error())?
        }
    }

    /// Add an heuristic directive.
    ///
    /// # Arguments
    ///
    /// * `atom` - the target atom
    /// * `type` - the type of the heuristic modification
    /// * `bias` - the heuristic bias
    /// * `priority` - the heuristic priority
    /// * `condition` - the condition under which to apply the heuristic modification
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    pub fn heuristic(
        &mut self,
        atom: &Atom,
        type_: HeuristicType,
        bias: i32,
        priority: u32,
        condition: &[Literal],
    ) -> Result<(), Error> {
        let size = condition.len();
        if unsafe {
            clingo_backend_heuristic(
                &mut self.0,
                atom.0,
                type_ as clingo_heuristic_type_t,
                bias,
                priority,
                condition.as_ptr() as *const clingo_literal_t,
                size,
            )
        } {
            Ok(())
        } else {
            Err(error())?
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
    ) -> Result<(), Error> {
        let size = condition.len();
        if unsafe {
            clingo_backend_acyc_edge(
                &mut self.0,
                node_u,
                node_v,
                condition.as_ptr() as *const clingo_literal_t,
                size,
            )
        } {
            Ok(())
        } else {
            Err(error())?
        }
    }

    /// Get a fresh atom to be used in aspif directives.
    pub fn add_atom(&mut self) -> Option<Atom> {
        let mut atom = 0 as clingo_atom_t;
        if unsafe { clingo_backend_add_atom(&mut self.0, &mut atom) } {
            Some(Atom(atom))
        } else {
            None
        }
    }
}

/// Handle for to the solver statistics.
#[derive(Debug, Copy, Clone)]
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
    pub fn statistics_type(&self, key: u64) -> Option<StatisticsType> {
        let mut stype = 0 as clingo_statistics_type_t;
        if unsafe { clingo_statistics_type(&self.0, key, &mut stype) } {
            match stype as u32 {
                clingo_statistics_type_clingo_statistics_type_empty => Some(StatisticsType::Empty),
                clingo_statistics_type_clingo_statistics_type_value => Some(StatisticsType::Value),
                clingo_statistics_type_clingo_statistics_type_array => Some(StatisticsType::Array),
                clingo_statistics_type_clingo_statistics_type_map => Some(StatisticsType::Map),
                _ => None,
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

    /// Get the number of subkeys of a map entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must
    /// be [`StatisticsType::Map`](enum.StatisticsType.html#variant.Map).
    pub fn map_size(&self, key: u64) -> Option<usize> {
        let mut size = 0 as usize;
        if unsafe { clingo_statistics_map_size(&self.0, key, &mut size) } {
            Some(size)
        } else {
            None
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
    pub fn map_subkey_name<'a>(&self, key: u64, offset: usize) -> Option<&'a str> {
        let mut name = std::ptr::null() as *const c_char;
        if unsafe { clingo_statistics_map_subkey_name(&self.0, key, offset, &mut name) } {
            Some(unsafe { CStr::from_ptr(name) }.to_str().unwrap())
        } else {
            None
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
        let name_c_str = CString::new(name).unwrap();
        if unsafe { clingo_statistics_map_at(&self.0, key, name_c_str.as_ptr(), &mut subkey) } {
            Some(subkey)
        } else {
            None
        }
    }

    /// Get the value of the given entry.
    ///
    /// # Pre-condition
    ///
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Value`](enum.StatisticsType.html#variant.Value).
    pub fn value_get(&self, key: u64) -> Option<f64> {
        let mut value = 0.0 as f64;
        if unsafe { clingo_statistics_value_get(&self.0, key, &mut value) } {
            Some(value)
        } else {
            None
        }
    }
}

/// Object to inspect symbolic atoms in a program---the relevant Herbrand base
/// gringo uses to instantiate programs.
///
/// **See:** [`Control::symbolic_atoms()`](struct.Control.html#method.symbolic_atoms)
#[derive(Debug, Copy, Clone)]
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

    /// Get a forward iterator to the beginning of the sequence of all symbolic
    /// atoms optionally restricted to a given signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - optional signature
    // TODO implement Iterator trait
    pub fn begin(&self, opt_sig: Option<&Signature>) -> Option<clingo_symbolic_atom_iterator_t> {
        match opt_sig {
            Some(sig) => {
                let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
                if unsafe { clingo_symbolic_atoms_begin(&self.0, &sig.0, &mut iterator) } {
                    Some(iterator)
                } else {
                    None
                }
            }
            None => {
                let signature = std::ptr::null();
                let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
                if unsafe { clingo_symbolic_atoms_begin(&self.0, signature, &mut iterator) } {
                    Some(iterator)
                } else {
                    None
                }
            }
        }
    }

    /// Iterator pointing to the end of the sequence of symbolic atoms.
    pub fn end(&self) -> Option<clingo_symbolic_atom_iterator_t> {
        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        if unsafe { clingo_symbolic_atoms_end(&self.0, &mut iterator) } {
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
    /// * `iterator` - iterator pointing to the symbolic atom or to the end
    /// of the sequence if no corresponding atom is found
    pub fn find(&self, Symbol(symbol): Symbol) -> Option<clingo_symbolic_atom_iterator_t> {
        let mut iterator = 0 as clingo_symbolic_atom_iterator_t;
        if unsafe { clingo_symbolic_atoms_find(&self.0, symbol, &mut iterator) } {
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
        &self,
        a: clingo_symbolic_atom_iterator_t,
        b: clingo_symbolic_atom_iterator_t,
    ) -> Option<bool> {
        let mut equal = false;
        if unsafe { clingo_symbolic_atoms_iterator_is_equal_to(&self.0, a, b, &mut equal) } {
            Some(equal)
        } else {
            None
        }
    }

    /// Get the symbolic representation of an atom.
    ///
    /// # Arguments
    ///
    /// * `iterator` - iterator to the atom
    pub fn symbol(&self, iterator: clingo_symbolic_atom_iterator_t) -> Option<Symbol> {
        let mut symbol = 0 as clingo_symbol_t;
        if unsafe { clingo_symbolic_atoms_symbol(&self.0, iterator, &mut symbol) } {
            Some(Symbol(symbol))
        } else {
            None
        }
    }

    /// Check whether an atom is a fact.
    ///
    /// **Note:** This does not determine if an atom is a cautious consequence. The
    /// grounding or solving component's simplifications can only detect this in
    /// some cases.
    ///
    /// # Arguments
    ///
    /// * `iterator` - iterator to the atom
    pub fn is_fact(&self, iterator: clingo_symbolic_atom_iterator_t) -> Option<bool> {
        let mut fact = false;
        if unsafe { clingo_symbolic_atoms_is_fact(&self.0, iterator, &mut fact) } {
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
    /// * `iterator` - iterator to the atom
    pub fn is_external(&self, iterator: clingo_symbolic_atom_iterator_t) -> Option<bool> {
        let mut external = false;
        if unsafe { clingo_symbolic_atoms_is_external(&self.0, iterator, &mut external) } {
            Some(external)
        } else {
            None
        }
    }

    /// Returns the (numeric) aspif literal corresponding to the given symbolic atom.
    ///
    /// Such a literal can be mapped to a solver literal (see [`Propagator`](struct.Propagator)).
    /// or be used in rules in aspif format (see [`ProgramBuilder`](struct.ProgramBuilder.html)).
    ///
    /// # Arguments
    ///
    /// * `iterator` iterator to the atom
    pub fn literal(&self, iterator: clingo_symbolic_atom_iterator_t) -> Option<Literal> {
        let mut literal = 0 as clingo_literal_t;
        if unsafe { clingo_symbolic_atoms_literal(&self.0, iterator, &mut literal) } {
            Some(Literal(literal))
        } else {
            None
        }
    }

    //NOTTODO: fn clingo_symbolic_atoms_signatures_size()

    /// Get the predicate signatures occurring in a logic program.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if the size is too small
    pub fn signatures(&self) -> Result<Vec<Signature>, Error> {
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
                Err(error())?
            }
        } else {
            Err(error())?
        }
    }

    /// Get an iterator to the next element in the sequence of symbolic atoms.
    ///
    /// # Arguments
    ///
    /// * `iterator` - the current iterator
    pub fn next(
        &self,
        iterator: clingo_symbolic_atom_iterator_t,
    ) -> Option<clingo_symbolic_atom_iterator_t> {
        let mut next = 0 as clingo_symbolic_atom_iterator_t;
        if unsafe { clingo_symbolic_atoms_next(&self.0, iterator, &mut next) } {
            Some(next)
        } else {
            None
        }
    }

    /// Check whether the given iterator points to some element with the sequence
    /// of symbolic atoms or to the end of the sequence.
    ///
    /// # Arguments
    ///
    /// * `iterator` - the iterator
    pub fn is_valid(&self, iterator: clingo_symbolic_atom_iterator_t) -> Option<bool> {
        let mut valid = false;
        if unsafe { clingo_symbolic_atoms_is_valid(&self.0, iterator, &mut valid) } {
            Some(valid)
        } else {
            None
        }
    }
}

/// Container that stores theory atoms, elements, and terms (see @ref clingo_control_theory_atoms())
#[derive(Debug, Copy, Clone)]
pub struct TheoryAtoms(clingo_theory_atoms_t);
impl TheoryAtoms {
    /// Get the type of the given theory term.
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    // TODO ? is this needed in an Rust API
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
                _ => None,
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
    pub fn term_name<'a>(&self, Id(term): Id) -> Option<&'a str> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_theory_atoms_term_name(&self.0, term, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Some(c_str.to_str().unwrap())
        } else {
            None
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

    //NOTTODO: pub fn clingo_theory_atoms_term_to_string_size()

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
                Ok(cstr.to_str().unwrap())
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

    //NOTTODO: pub fn clingo_theory_atoms_element_to_string_size()

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
                Ok(cstr.to_str().unwrap())
            } else {
                Err(error())?
            }
        } else {
            Err(error())?
        }
    }

    /// Get the total number of theory atoms.
    pub fn size(&self) -> Option<usize> {
        let mut size = 0 as usize;
        if unsafe { clingo_theory_atoms_size(&self.0, &mut size) } {
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
            let elements_ref =
                unsafe { std::slice::from_raw_parts(elements_ptr as *const Id, size) };
            Some(elements_ref.to_owned())
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
    pub fn atom_guard(&self, Id(atom): Id) -> Option<(&str, Id)> {
        let mut c_ptr = unsafe { mem::uninitialized() };
        let mut term = 0 as clingo_id_t;
        if unsafe { clingo_theory_atoms_atom_guard(&self.0, atom, &mut c_ptr, &mut term) } {
            let cstr = unsafe { CStr::from_ptr(c_ptr) };
            Some((cstr.to_str().unwrap(), Id(term)))
        } else {
            None
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

    //NOTTODO: pub fn clingo_theory_atoms_atom_to_string_size()

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
                Ok(cstr.to_str().unwrap())
            } else {
                Err(error())?
            }
        } else {
            Err(error())?
        }
    }
}

//TODO: make safe
/// Object to iterate over symbolic atoms.
///
/// Such an iterator either points to a symbolic atom within a sequence of
/// symbolic atoms or to the end of the sequence.
///
/// **Note:** Iterators are valid as long as the underlying sequence is not modified.
/// Operations that can change this sequence are ::clingo_control_ground(),
/// ::clingo_control_cleanup(), and functions that modify the underlying
/// non-ground program.
pub struct UNSAFE_SymbolicAtomsIterator {
    count: usize,
    size: usize,
}
impl Iterator for UNSAFE_SymbolicAtomsIterator {
    type Item = Symbol;

    fn next(&mut self) -> Option<Symbol> {
        // increment our count. This is why we started at zero.
        self.count += 1;

        // check to see if we've finished counting or not.
        if self.count < self.size {
            Some(Symbol(self.count as u64))
        } else {
            None
        }
    }
}
// impl IntoIterator for SymbolicAtoms {
//     type Item = Symbol;
//     type IntoIter = UNSAFE_SymbolicAtomsIterator;
//     fn into_iter(self) -> Self::IntoIter {
//         UNSAFE_SymbolicAtomsIterator::from(&self)
//     }
// }

impl UNSAFE_SymbolicAtomsIterator {
    pub fn from(sa: &SymbolicAtoms) -> UNSAFE_SymbolicAtomsIterator {
        UNSAFE_SymbolicAtomsIterator {
            count: 0,
            size: sa.size().unwrap(),
        }
    }
}

//TODO: make safe
pub struct UNSAFE_TheoryAtomsIterator {
    count: usize,
    size: usize,
}
impl Iterator for UNSAFE_TheoryAtomsIterator {
    type Item = Id;

    fn next(&mut self) -> Option<Id> {
        // increment our count. This is why we started at zero.
        self.count += 1;

        // check to see if we've finished counting or not.
        if self.count < self.size {
            Some(Id(self.count as u32))
        } else {
            None
        }
    }
}
impl UNSAFE_TheoryAtomsIterator {
    pub fn from(cta: &TheoryAtoms) -> UNSAFE_TheoryAtomsIterator {
        UNSAFE_TheoryAtomsIterator {
            count: 0,
            size: cta.size().unwrap(),
        }
    }
}

/// Object representing a model.
#[derive(Debug, Copy, Clone)]
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
                _ => None,
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

    //NOTTODO: pub fn clingo_model_symbols_size()

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
    pub fn symbols(&self, show: &ShowType) -> Result<Vec<Symbol>, Error> {
        let mut size: usize = 0;
        if unsafe { clingo_model_symbols_size(&self.0, show.0, &mut size) } {
            let symbols = Vec::<Symbol>::with_capacity(size);
            let symbols_ptr = symbols.as_ptr();
            if unsafe {
                clingo_model_symbols(&self.0, show.0, symbols_ptr as *mut clingo_symbol_t, size)
            } {
                let symbols_ref =
                    unsafe { std::slice::from_raw_parts(symbols_ptr as *const Symbol, size) };
                Ok(symbols_ref.to_owned())
            } else {
                Err(error())?
            }
        } else {
            Err(error())?
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

    //TODO     /// Check if a program literal is true in a model.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `model` - the target
    //     /// * `literal` - the literal to lookup
    //     /// * `result` - whether the literal is true
    //     ///
    //     /// **Returns** whether the call was successful
    //     pub fn clingo_model_is_true(
    //         model: *mut clingo_model_t,
    //         literal: clingo_literal_t,
    //         result: *mut bool,
    //     ) -> bool;

    //NOTTODO: pub fn clingo_model_cost_size(model: *mut Model, size: *mut size_t) -> u8;

    /// Get the cost vector of a model.
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if the size is too small
    ///
    /// **See:** [`Model::optimality_proven()`](struct.Model.html#method.optimality_proven)
    pub fn cost(&self) -> Result<Vec<i64>, Error> {
        let mut size: usize = 0;
        if unsafe { clingo_model_cost_size(&self.0, &mut size) } {
            let cost = Vec::<i64>::with_capacity(size);
            let cost_ptr = cost.as_ptr();
            if unsafe { clingo_model_cost(&self.0, cost_ptr as *mut i64, size) } {
                let cost_ref = unsafe { std::slice::from_raw_parts(cost_ptr as *const i64, size) };
                Ok(cost_ref.to_owned())
            } else {
                Err(error())?
            }
        } else {
            Err(error())?
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

    //TODO     /// Get the id of the solver thread that found the model.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `model` - the target
    //     /// * `id` - the resulting thread id
    //     ///
    //     /// **Returns** whether the call was successful
    //     pub fn clingo_model_thread_id(model: *mut clingo_model_t, id: *mut clingo_id_t) -> bool;

    /// Get the associated solve control object of a model.
    ///
    /// This object allows for adding clauses during model enumeration.
    pub fn context(&mut self) -> Option<&mut SolveControl> {
        let mut control = unsafe { mem::uninitialized() };
        if unsafe { clingo_model_context(&mut self.0, &mut control) } {
            unsafe { (control as *mut SolveControl).as_mut() }
        } else {
            None
        }
    }
}

/// Object to add clauses during search.
#[derive(Debug, Copy, Clone)]
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
    pub fn add_clause(&mut self, clause: &[Literal]) -> Result<(), Error> {
        if unsafe {
            clingo_solve_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                clause.len(),
            )
        } {
            Ok(())
        } else {
            Err(error())?
        }
    }

    /// Get an object to inspect the symbolic atoms.
    pub fn symbolic_atoms(&mut self) -> Option<&SymbolicAtoms> {
        let mut atoms = std::ptr::null_mut() as *const clingo_symbolic_atoms_t;
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
/// A literal is assigned to either @link clingo_assignment_truth_value() true or false, or is unassigned@endlink.
/// Furthermore, each assigned literal is associated with a @link clingo_assignment_level() decision level@endlink.
/// There is exactly one @link clingo_assignment_decision() decision literal@endlink for each decision level greater than zero.
/// Assignments to all other literals on the same level are consequences implied by the current and possibly previous decisions.
/// Assignments on level zero are immediate consequences of the current program.
/// Decision levels are consecutive numbers starting with zero up to and including the @link clingo_assignment_decision_level() current decision level@endlink.
#[derive(Debug, Copy, Clone)]
pub struct Assignment(clingo_assignment_t);
// impl Assignment {
//TODO     /// Get the current decision level.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment` - the target assignment
//     ///
//     /// **Returns** the decision level
//     pub fn clingo_assignment_decision_level(assignment: *mut clingo_assignment_t) -> u32;

//TODO     /// Check if the given assignment is conflicting.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment` - the target assignment
//     ///
//     /// **Returns** whether the assignment is conflicting
//     pub fn clingo_assignment_has_conflict(assignment: *mut clingo_assignment_t) -> bool;

//TODO     /// Check if the given literal is part of a (partial) assignment.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment` - the target assignment
//     /// * `literal` - the literal
//     ///
//     /// **Returns** whether the literal is valid
//     pub fn clingo_assignment_has_literal(
//         assignment: *mut clingo_assignment_t,
//         literal: clingo_literal_t,
//     ) -> bool;

//TODO     /// Determine the decision level of a given literal.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment` - the target assignment
//     /// * `literal` - the literal
//     /// * `level` - the resulting level
//     ///
//     /// **Returns** whether the call was successful
//     pub fn clingo_assignment_level(
//         assignment: *mut clingo_assignment_t,
//         literal: clingo_literal_t,
//         level: *mut u32,
//     ) -> bool;

//TODO     /// Determine the decision literal given a decision level.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment` - the target assignment
//     /// * `level` - the level
//     /// * `literal` - the resulting literal
//     ///
//     /// **Returns** whether the call was successful
//     pub fn clingo_assignment_decision(
//         assignment: *mut clingo_assignment_t,
//         level: u32,
//         literal: *mut clingo_literal_t,
//     ) -> bool;

//TODO     /// Check if a literal has a fixed truth value.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment` - the target assignment
//     /// * `literal` - the literal
//     /// * `is_fixed` - whether the literal is fixed
//     ///
//     /// **Returns** whether the call was successful
//     pub fn clingo_assignment_is_fixed(
//         assignment: *mut clingo_assignment_t,
//         literal: clingo_literal_t,
//         is_fixed: *mut bool,
//     ) -> bool;

//TODO     /// Check if a literal is true.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment` - the target assignment
//     /// * `literal` - the literal
//     /// * `is_true` - whether the literal is true
//     ///
//     /// **Returns** whether the call was successful
//     /// @see clingo_assignment_truth_value()
//     pub fn clingo_assignment_is_true(
//         assignment: *mut clingo_assignment_t,
//         literal: clingo_literal_t,
//         is_true: *mut bool,
//     ) -> bool;

//TODO     /// Check if a literal has a fixed truth value.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment` - the target assignment
//     /// * `literal` - the literal
//     /// * `is_false` - whether the literal is false
//     ///
//     /// **Returns** whether the call was successful
//     /// @see clingo_assignment_truth_value()
//     pub fn clingo_assignment_is_false(
//         assignment: *mut clingo_assignment_t,
//         literal: clingo_literal_t,
//         is_false: *mut bool,
//     ) -> bool;

//TODO     /// Determine the truth value of a given literal.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment` - the target assignment
//     /// * `literal` - the literal
//     /// * `value` - the resulting truth value
//     ///
//     /// **Returns** whether the call was successful
//     pub fn clingo_assignment_truth_value(
//         assignment: *mut clingo_assignment_t,
//         literal: clingo_literal_t,
//         value: *mut clingo_truth_value_t,
//     ) -> bool;

//TODO     /// The number of assigned literals in the assignment.
//     /// **Parameters:**
//     ///
//     /// * `assignment`- the target
//     ///
//     /// **Returns** the number of literals
//     pub fn clingo_assignment_size(assignment: *mut clingo_assignment_t) -> usize;

//TODO     /// The maximum size of the assignment (if all literals are assigned).
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment`- the target
//     ///
//     /// **Returns** the maximum size
//     pub fn clingo_assignment_max_size(assignment: *mut clingo_assignment_t) -> usize;

//TODO     /// Check if the assignmen is total, i.e. - size == max_size.
//     ///
//     /// **Parameters:**
//     ///
//     /// * `assignment`- the target
//     ///
//     /// **Returns** wheather the assignment is total
//     pub fn clingo_assignment_is_total(assignment: *mut clingo_assignment_t) -> bool;

// }

/// This object can be used to add clauses and propagate literals while solving.
#[derive(Debug, Copy, Clone)]
pub struct PropagateControl(clingo_propagate_control_t);
impl PropagateControl {
    /// Get the id of the underlying solver thread.
    ///
    /// Thread ids are consecutive numbers starting with zero.
    pub fn thread_id(&mut self) -> u32 {
        unsafe { clingo_propagate_control_thread_id(&mut self.0) }
    }

    /// Get the assignment associated with the underlying solver.
    pub fn assignment(&mut self) -> &mut Assignment {
        unsafe { (clingo_propagate_control_assignment(&mut self.0) as *mut Assignment).as_mut() }
            .unwrap()
    }

    //TODO     /// Adds a new volatile literal to the underlying solver thread.
    //     ///
    //     /// @attention The literal is only valid within the current solving step and solver thread.
    //     /// All volatile literals and clauses involving a volatile literal are deleted after the current search.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `control` - the target
    //     /// * `result` - the (positive) solver literal
    //     ///
    //     /// **Returns** whether the call was successful; might set one of the following error codes:
    //     /// - ::clingo_error_bad_alloc
    //     /// - ::clingo_error_logic if the assignment is conflicting
    //     pub fn clingo_propagate_control_add_literal(
    //         control: *mut clingo_propagate_control_t,
    //         result: *mut clingo_literal_t,
    //     ) -> bool;

    //TODO     /// Add a watch for the solver literal in the given phase.
    //     ///
    //     /// **Note:** Unlike @ref clingo_propagate_init_add_watch() this does not add a watch to all solver threads but just the current one.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `control` - the target
    //     /// * `literal` - the literal to watch
    //     ///
    //     /// **Returns** whether the call was successful; might set one of the following error codes:
    //     /// - ::clingo_error_bad_alloc
    //     /// - ::clingo_error_logic if the literal is invalid
    //     /// @see clingo_propagate_control_remove_watch()
    //     pub fn clingo_propagate_control_add_watch(
    //         control: *mut clingo_propagate_control_t,
    //         literal: clingo_literal_t,
    //     ) -> bool;

    //TODO     /// Check whether a literal is watched in the current solver thread.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `control` - the target
    //     /// * `literal` - the literal to check
    //     ///
    //     /// **Returns** whether the literal is watched
    //     pub fn clingo_propagate_control_has_watch(
    //         control: *mut clingo_propagate_control_t,
    //         literal: clingo_literal_t,
    //     ) -> bool;

    //TODO     /// Removes the watch (if any) for the given solver literal.
    //     ///
    //     /// **Note:** Similar to @ref clingo_propagate_init_add_watch() this just removes the watch in the current solver thread.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `control` - the target
    //     /// * `literal` - the literal to remove
    //     pub fn clingo_propagate_control_remove_watch(
    //         control: *mut clingo_propagate_control_t,
    //         literal: clingo_literal_t,
    //     );

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
    pub fn add_clause(&mut self, clause: &[Literal], ctype: ClauseType) -> Result<bool, Error> {
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
            Err(error())?
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
    pub fn propagate(&mut self) -> Result<bool, Error> {
        let mut result = false;
        if unsafe { clingo_propagate_control_propagate(&mut self.0, &mut result) } {
            Ok(result)
        } else {
            Err(error())?
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
#[derive(Debug, Copy, Clone)]
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

    /// Get an object to inspect the symbolic atoms.
    pub fn symbolic_atoms<'a>(&self) -> Option<&'a SymbolicAtoms> {
        let mut atoms_ptr = std::ptr::null() as *const clingo_symbolic_atoms_t;
        if unsafe { clingo_propagate_init_symbolic_atoms(&self.0, &mut atoms_ptr) } {
            unsafe { (atoms_ptr as *const SymbolicAtoms).as_ref() }
        } else {
            None
        }
    }

    /// Get an object to inspect the theory atoms.
    pub fn theory_atoms(&self) -> Option<&TheoryAtoms> {
        let mut atoms_ptr = std::ptr::null() as *const clingo_theory_atoms_t;
        if unsafe { clingo_propagate_init_theory_atoms(&self.0, &mut atoms_ptr) } {
            unsafe { (atoms_ptr as *const TheoryAtoms).as_ref() }
        } else {
            None
        }
    }

    /// Get the number of threads used in subsequent solving.
    /// **See:** [`PropagateControl::thread_id()`](struct.PropagateControl.html#method.thread_id)
    pub fn number_of_threads(&self) -> usize {
        (unsafe { clingo_propagate_init_number_of_threads(&self.0) } as usize)
    }

    //TODO     /// Configure when to call the check method of the propagator.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `init` - the target
    //     /// *`mode`- bitmask when to call the propagator
    //     /// @see @ref ::clingo_propagator::check()
    //     pub fn clingo_propagate_init_set_check_mode(
    //         init: *mut clingo_propagate_init_t,
    //         mode: clingo_propagator_check_mode_t,
    //     );

    //TODO     /// Get the current check mode of the propagator.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `init`- the target
    //     ///
    //     /// **Returns**  bitmask when to call the propagator
    //     /// @see clingo_propagate_init_set_check_mode()
    //     pub fn clingo_propagate_init_get_check_mode(
    //         init: *mut clingo_propagate_init_t,
    //     ) -> clingo_propagator_check_mode_t;
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
    pub fn get(&mut self) -> Result<SolveResult, Error> {
        let mut result = 0;
        if unsafe { clingo_solve_handle_get(self.theref, &mut result) } {
            Ok(SolveResult(result))
        } else {
            Err(error())?
        }
    }

    //TODO     /// Wait for the specified amount of time to check if the next result is ready.
    //     ///
    //     /// If the time is set to zero, this function can be used to poll if the search is still active.
    //     /// If the time is negative, the function blocks until the search is finished.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `handle` - the target
    //     /// * `timeout` - the maximum time to wait
    //     /// * `result` - whether the search has finished
    //     pub fn clingo_solve_handle_wait(
    //         handle: *mut clingo_solve_handle_t,
    //         timeout: f64,
    //         result: *mut bool,
    //     );

    /// Get the next model (or zero if there are no more models).
    /// (it is NULL if there are no more models)
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving fails
    pub fn model(&mut self) -> Result<&mut Model, Error> {
        let mut model = std::ptr::null_mut() as *mut clingo_model_t;
        if unsafe { clingo_solve_handle_model(self.theref, &mut model) } {
            match unsafe { (model as *mut Model).as_mut() } {
                Some(x) => Ok(x),
                None => Err(BindingError {
                    msg: "Failed dereferencing pointer to clingo_model.",
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
    pub fn resume(&mut self) -> Result<(), Error> {
        if unsafe { clingo_solve_handle_resume(self.theref) } {
            Ok(())
        } else {
            Err(error())?
        }
    }

    // TODO    /// Stop the running search and block until done.
    //     ///
    //     /// **Parameters:**
    //     ///
    //     /// * `handle` - the target
    //     ///
    //     /// **Returns** whether the call was successful; might set one of the following error codes:
    //     /// - ::clingo_error_bad_alloc
    //     /// - ::clingo_error_runtime if solving fails
    //     pub fn clingo_solve_handle_cancel(handle: *mut clingo_solve_handle_t) -> bool;

    /// Stops the running search and releases the handle.
    ///
    /// Blocks until the search is stopped (as if an implicit cancel was called before the handle is
    /// released).
    ///
    /// # Errors
    ///
    /// - [`ErrorType::BadAlloc`](enum.ErrorType.html#variant.BadAlloc)
    /// - [`ErrorType::Runtime`](enum.ErrorType.html#variant.Runtime) if solving fails
    pub fn close(self) -> Result<(), Error> {
        if unsafe { clingo_solve_handle_close(self.theref) } {
            Ok(())
        } else {
            Err(error())?
        }
    }
}

//TODO     /// Internalize a string.
//     ///
//     /// This functions takes a string as input and returns an equal unique string
//     /// that is (at the moment) not freed until the program is closed.  All strings
//     /// returned from clingo API functions are internalized and must not be freed.
//     ///
//     /// # Arguments
//     ///
//     /// * `string` - the string to internalize
//     /// * `result` - the internalized string
//     ///
//     /// **Returns** whether the call was successful; might set one of the following error codes:
//     /// - ::clingo_error_bad_alloc
//     pub fn clingo_add_string(
//         string: *const ::std::os::raw::c_char,
//         result: *mut *const ::std::os::raw::c_char,
//     ) -> bool;

//TODO     /// Parse a term in string form.
//     ///
//     /// The result of this function is a symbol. The input term can contain
//     /// unevaluated functions, which are evaluated during parsing.
//     ///
//     /// # Arguments
//     ///
//     /// * `string` - the string to parse
//     /// * `logger` - ouptional logger to report warnings during parsing
//     /// * `logger_data` - user data for the logger
//     /// * `message_limit` - maximum number of times to call the logger
//     /// * `symbol` - the resulting symbol
//     ///
//     /// **Returns** whether the call was successful; might set one of the following error codes:
//     /// - ::clingo_error_bad_alloc
//     /// - ::clingo_error_runtime if parsing fails
//     pub fn clingo_parse_term(
//         string: *const ::std::os::raw::c_char,
//         logger: clingo_logger_t,
//         logger_data: *mut ::std::os::raw::c_void,
//         message_limit: ::std::os::raw::c_uint,
//         symbol: *mut clingo_symbol_t,
//     ) -> bool;

// TODO
pub trait GroundProgramObserver {
    /// Called once in the beginning.
    ///
    /// If the incremental flag is true, there can be multiple calls to @ref clingo_control_solve().
    ///
    /// # Arguments
    ///
    /// * `incremental` - whether the program is incremental
    ///
    /// **Returns** whether the call was successful
    fn init_program(&mut self, incremental: bool) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_init_program(
        incremental: bool,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Marks the beginning of a block of directives passed to the solver.
    ///
    /// @see @ref end_step
    ///
    /// # Arguments
    ///
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn begin_step(data: *mut ::std::os::raw::c_void) -> bool {
        false
    }

    /// Marks the end of a block of directives passed to the solver.
    ///
    /// This function is called before solving starts.
    ///
    /// @see @ref begin_step
    ///
    /// # Arguments
    ///
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn end_step(data: *mut ::std::os::raw::c_void) -> bool {
        false
    }

    /// Observe rules passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `choice` - determines if the head is a choice or a disjunction
    /// * `head` - the head atoms
    /// * `head_size` - the number of atoms in the head
    /// * `body` - the body literals
    /// * `body_size` - the number of literals in the body
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn rule(
        choice: bool,
        head: *const clingo_atom_t,
        head_size: usize,
        body: *const clingo_literal_t,
        body_size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe weight rules passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `choice` - determines if the head is a choice or a disjunction
    /// * `head` - the head atoms
    /// * `head_size` - the number of atoms in the head
    /// * `lower_bound` - the lower bound of the weight rule
    /// * `body` - the weighted body literals
    /// * `body_size` - the number of weighted literals in the body
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn weight_rule(
        choice: bool,
        head: *const clingo_atom_t,
        head_size: usize,
        lower_bound: clingo_weight_t,
        body: *const clingo_weighted_literal_t,
        body_size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe minimize constraints (or weak constraints) passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `priority` - the priority of the constraint
    /// * `literals` - the weighted literals whose sum to minimize
    /// * `size` - the number of weighted literals
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn minimize(
        priority: clingo_weight_t,
        literals: *const clingo_weighted_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe projection directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the atoms to project on
    /// * `size` - the number of atoms
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn project(
        atoms: *const clingo_atom_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe shown atoms passed to the solver.
    /// \note Facts do not have an associated aspif atom.
    /// The value of the atom is set to zero.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the symbolic representation of the atom
    /// * `atom` - the aspif atom (0 for facts)
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn output_atom(
        symbol: clingo_symbol_t,
        atom: clingo_atom_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe shown terms passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the symbolic representation of the term
    /// * `condition` - the literals of the condition
    /// * `size` - the size of the condition
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn output_term(
        symbol: clingo_symbol_t,
        condition: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe shown csp variables passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the symbolic representation of the variable
    /// * `value` - the value of the variable
    /// * `condition` - the literals of the condition
    /// * `size` - the size of the condition
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn output_csp(
        symbol: clingo_symbol_t,
        value: ::std::os::raw::c_int,
        condition: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe external statements passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `atom` - the external atom
    /// * `type` - the type of the external statement
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn external(
        atom: clingo_atom_t,
        type_: clingo_external_type_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe assumption directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `literals` - the literals to assume (positive literals are true and negative literals
    /// false for the next solve call)
    /// * `size` - the number of atoms
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn assume(
        literals: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
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
    /// * `size` - the number of atoms in the condition
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn heuristic(
        atom: clingo_atom_t,
        type_: clingo_heuristic_type_t,
        bias: ::std::os::raw::c_int,
        priority: ::std::os::raw::c_uint,
        condition: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe edge directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `node_u` - the start vertex of the edge
    /// * `node_v` - the end vertex of the edge
    /// * `condition` - the condition under which the edge is part of the graph
    /// * `size` - the number of atoms in the condition
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn acyc_edge(
        node_u: ::std::os::raw::c_int,
        node_v: ::std::os::raw::c_int,
        condition: *const clingo_literal_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe numeric theory terms.
    ///
    /// # Arguments
    ///
    /// * `term_id` - the id of the term
    /// * `number` - the value of the term
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn theory_term_number(
        term_id: clingo_id_t,
        number: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe string theory terms.
    ///
    /// # Arguments
    ///
    /// * `term_id` - the id of the term
    /// * `name` - the value of the term
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn theory_term_string(
        term_id: clingo_id_t,
        name: *const ::std::os::raw::c_char,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
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
    /// * `size` - the number of arguments
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn theory_term_compound(
        term_id: clingo_id_t,
        name_id_or_type: ::std::os::raw::c_int,
        arguments: *const clingo_id_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe theory elements.
    ///
    /// # Arguments
    ///
    /// * `element_id` - the id of the element
    /// * `terms` - the term tuple of the element
    /// * `terms_size` - the number of terms in the tuple
    /// * `condition` - the condition of the elemnt
    /// * `condition_size` - the number of literals in the condition
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn theory_element(
        element_id: clingo_id_t,
        terms: *const clingo_id_t,
        terms_size: usize,
        condition: *const clingo_literal_t,
        condition_size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe theory atoms without guard.
    ///
    /// # Arguments
    ///
    /// * `atom_id_or_zero` - the id of the atom or zero for directives
    /// * `term_id` - the term associated with the atom
    /// * `elements` - the elements of the atom
    /// * `size` - the number of elements
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn theory_atom(
        atom_id_or_zero: clingo_id_t,
        term_id: clingo_id_t,
        elements: *const clingo_id_t,
        size: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
    }

    /// Observe theory atoms with guard.
    ///
    /// # Arguments
    ///
    /// * `atom_id_or_zero` - the id of the atom or zero for directives
    /// * `term_id` - the term associated with the atom
    /// * `elements` - the elements of the atom
    /// * `size` - the number of elements
    /// * `operator_id` - the id of the operator (a string term)
    /// * `right_hand_side_id` - the id of the term on the right hand side of the atom
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    unsafe extern "C" fn theory_atom_with_guard(
        atom_id_or_zero: clingo_id_t,
        term_id: clingo_id_t,
        elements: *const clingo_id_t,
        size: usize,
        operator_id: clingo_id_t,
        right_hand_side_id: clingo_id_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        false
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
        let mut sym = Symbol::create_number(42);
        assert!(42 == sym.number().unwrap());
        sym = Symbol::create_infimum();
        assert!(SymbolType::Infimum == sym.symbol_type());
    }
}
