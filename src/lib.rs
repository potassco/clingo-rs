#![doc(html_root_url = "https://docs.rs/clingo/0.6.0")]
//! This crate provides bindings to the [clingo](https://github.com/potassco/clingo) library version 5.4.0.
//!
//! ## Requirements
//!
//! - a c++14 conforming compiler
//!   - *at least* [gcc](https://gcc.gnu.org/) version 4.9
//!   - [clang](http://clang.llvm.org/) version 3.1 (using either libstdc++
//!     provided by gcc 4.9 or libc++)
//!
//!
//! ## Using `derive` macro
//!
//! The crate provides a derive macro to help easing the use of rust data types as facts.
//!
//!
//! In your `Cargo.toml` add:
//! ```toml
//! [dependencies]
//! clingo = { version = "0.6", features = ["derive"] }
//! ```
//!      
//! In your source write:
//! ```ignore
//! use clingo::ToSymbol;
//! use clingo::ClingoError;
//! use clingo::FactBase;
//!
//! #[derive(ToSymbol)]
//! struct MyPoint {
//!     x: i32,
//!     y: i32,
//! }
//!
//! let p = MyPoint{ x:4, y:2 };
//! let fb = FactBase::new();
//! fb.insert(p);
//! ```
//!
//! The macro performs a conversion to snake case. This means the corresponing fact for `MyPoint{x:4,y:2}` is `my_point(4,2)`.
//!
//!
//! ## Using `dynamic_linking`
//!
//! The crate defines a [Cargo feature] that allows to use the clingo library via dynamic linking.
//!
//! [Cargo feature]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//!
//! With dynamic linking enabled the clingo library is not build for static linking but it is assumed that a
//! clingo dynamic library is installed on the system.
//!
//! The recommended way to use the optional dynamic linking support is as
//! follows.
//!
//! ```toml
//! [dependencies]
//! clingo = { version = "0.6.0", features = ["derive", "dynamic_linking"] }
//! ```
//!
#![allow(non_upper_case_globals)]
#![allow(clippy::try_err)]
use bitflags::bitflags;
use clingo_sys::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::NulError;
use std::hash::{Hash, Hasher};
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::str::Utf8Error;
use thiserror::Error;
use std::time::Duration;

/// Functions and data structures to work with program ASTs.
pub mod ast;

/// ClingoError in the rust wrapper, like null pointers or failed matches of C enums.
///
/// Includes internal error from the clingo library.
///
/// **Note:** Errors can only be recovered from if explicitly mentioned; most
/// functions do not provide strong exception guarantees.  This means that in
/// case of errors associated objects cannot be used further.
#[derive(Error, Debug)]
pub enum ClingoError {
    #[error("NulError: ")]
    NulError(#[from] NulError),
    #[error("Utf8Error: ")]
    Utf8Error(#[from] Utf8Error),
    #[error("FFIError: {msg}")]
    FFIError { msg: &'static str },
    #[error("InternalError: {msg}, code: {code:?}, last: {last}")]
    InternalError {
        msg: &'static str,
        code: ErrorCode,
        last: &'static str,
    },
    #[error("ExternalError: ")]
    ExternalError(#[from] ExternalError),
}
impl ClingoError {
    fn new_internal(msg: &'static str) -> ClingoError {
        ClingoError::InternalError {
            msg,
            code: error_code(),
            last: error_message(),
        }
    }
}
#[derive(Error, Debug)]
#[error("ExternalError: {msg}")]
pub struct ExternalError {
    pub msg: &'static str,
}

/// Enumeration of clingo error types
/// See: set_error()
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
/// Enumeration of clingo error codes for [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError).
#[derive(Debug, Copy, Clone)]
pub enum ErrorCode {
    /// Successful API calls
    Success,
    /// Errors only detectable at runtime like invalid input
    Runtime,
    /// Wrong usage of the clingo API
    Logic,
    /// Memory could not be allocated
    BadAlloc,
    /// Errors unrelated to clingo
    Unknown,
    /// FFI failed to match clingo_error
    FFIError,
}
impl From<i32> for ErrorCode {
    fn from(error: i32) -> Self {
        match error as u32 {
            clingo_error_clingo_error_success => ErrorCode::Success,
            clingo_error_clingo_error_runtime => ErrorCode::Runtime,
            clingo_error_clingo_error_logic => ErrorCode::Logic,
            clingo_error_clingo_error_bad_alloc => ErrorCode::BadAlloc,
            clingo_error_clingo_error_unknown => ErrorCode::Unknown,
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_error {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                ErrorCode::FFIError
            }
        }
    }
}

/// Get the last error code set by a clingo API call.
///
/// **Note:** Each thread has its own local error code.
fn error_code() -> ErrorCode {
    ErrorCode::from(unsafe { clingo_error_code() })
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
/// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `message` contains a nul byte
pub fn set_error(code: ErrorType, message: &str) -> Result<(), NulError> {
    let message = CString::new(message)?;
    unsafe { clingo_set_error(code as clingo_error_t, message.as_ptr()) }
    Ok(())
}

fn set_internal_error(code: ErrorType, message: &'static str) {
    // unwrap won't panic, because the function is only used internally on valid strings
    let message = CString::new(message).unwrap();
    unsafe { clingo_set_error(code as clingo_error_t, message.as_ptr()) }
}

/// Represents three-valued truth values.
#[derive(Debug, Copy, Clone)]
pub enum TruthValue {
    /// No truth value
    Free = clingo_truth_value_clingo_truth_value_free as isize,
    /// True
    True = clingo_truth_value_clingo_truth_value_true as isize,
    /// False
    False = clingo_truth_value_clingo_truth_value_false as isize,
}
impl TruthValue {
    fn try_from(code: i32) -> Result<TruthValue, ClingoError> {
        match code as u32 {
            clingo_truth_value_clingo_truth_value_false => Ok(TruthValue::False),
            clingo_truth_value_clingo_truth_value_true => Ok(TruthValue::True),
            clingo_truth_value_clingo_truth_value_free => Ok(TruthValue::Free),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_truth_value {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_truth_value.",
                })
            }
        }
    }
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
impl SolveEventType {
    fn try_from(code: u32) -> Result<SolveEventType, ClingoError> {
        match code {
            clingo_solve_event_type_clingo_solve_event_type_model => Ok(SolveEventType::Model),
            clingo_solve_event_type_clingo_solve_event_type_statistics => {
                Ok(SolveEventType::Statistics)
            }
            clingo_solve_event_type_clingo_solve_event_type_finish => Ok(SolveEventType::Finish),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_solve_event_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_solve_event_type.",
                })
            }
        }
    }
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
impl StatisticsType {
    fn try_from(code: i32) -> Result<StatisticsType, ClingoError> {
        match code as u32 {
            clingo_statistics_type_clingo_statistics_type_empty => Ok(StatisticsType::Empty),
            clingo_statistics_type_clingo_statistics_type_value => Ok(StatisticsType::Value),
            clingo_statistics_type_clingo_statistics_type_array => Ok(StatisticsType::Array),
            clingo_statistics_type_clingo_statistics_type_map => Ok(StatisticsType::Map),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_statistics_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_statistics_type.",
                })
            }
        }
    }
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
impl SymbolType {
    fn try_from(code: i32) -> Result<SymbolType, ClingoError> {
        match code as u32 {
            clingo_symbol_type_clingo_symbol_type_infimum => Ok(SymbolType::Infimum),
            clingo_symbol_type_clingo_symbol_type_number => Ok(SymbolType::Number),
            clingo_symbol_type_clingo_symbol_type_string => Ok(SymbolType::String),
            clingo_symbol_type_clingo_symbol_type_function => Ok(SymbolType::Function),
            clingo_symbol_type_clingo_symbol_type_supremum => Ok(SymbolType::Supremum),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_symbol_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_symbol_type.",
                })
            }
        }
    }
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
impl Warning {
    fn try_from(code: i32) -> Result<Warning, ClingoError> {
        match code as u32 {
            clingo_warning_clingo_warning_atom_undefined => Ok(Warning::AtomUndefined),
            clingo_warning_clingo_warning_file_included => Ok(Warning::FileIncluded),
            clingo_warning_clingo_warning_global_variable => Ok(Warning::GlobalVariable),
            clingo_warning_clingo_warning_operation_undefined => Ok(Warning::OperationUndefined),
            clingo_warning_clingo_warning_other => Ok(Warning::Other),
            clingo_warning_clingo_warning_runtime_error => Ok(Warning::RuntimeError),
            clingo_warning_clingo_warning_variable_unbounded => Ok(Warning::VariableUnbound),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_warning {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_warning.",
                })
            }
        }
    }
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
impl ExternalType {
    fn try_from(code: i32) -> Result<ExternalType, ClingoError> {
        match code as u32 {
            clingo_external_type_clingo_external_type_false => Ok(ExternalType::False),
            clingo_external_type_clingo_external_type_free => Ok(ExternalType::Free),
            clingo_external_type_clingo_external_type_release => Ok(ExternalType::Release),
            clingo_external_type_clingo_external_type_true => Ok(ExternalType::True),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_external_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_external_type.",
                })
            }
        }
    }
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
impl HeuristicType {
    fn try_from(code: i32) -> Result<HeuristicType, ClingoError> {
        match code as u32 {
            clingo_heuristic_type_clingo_heuristic_type_factor => Ok(HeuristicType::Factor),
            clingo_heuristic_type_clingo_heuristic_type_false => Ok(HeuristicType::False),
            clingo_heuristic_type_clingo_heuristic_type_init => Ok(HeuristicType::Init),
            clingo_heuristic_type_clingo_heuristic_type_level => Ok(HeuristicType::Level),
            clingo_heuristic_type_clingo_heuristic_type_sign => Ok(HeuristicType::Sign),
            clingo_heuristic_type_clingo_heuristic_type_true => Ok(HeuristicType::True),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_heuristic_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_heuristic_type.",
                })
            }
        }
    }
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
impl TheoryTermType {
    fn try_from(code: i32) -> Result<TheoryTermType, ClingoError> {
        match code as u32 {
            clingo_theory_term_type_clingo_theory_term_type_tuple => Ok(TheoryTermType::Tuple),
            clingo_theory_term_type_clingo_theory_term_type_list => Ok(TheoryTermType::List),
            clingo_theory_term_type_clingo_theory_term_type_set => Ok(TheoryTermType::Set),
            clingo_theory_term_type_clingo_theory_term_type_function => {
                Ok(TheoryTermType::Function)
            }
            clingo_theory_term_type_clingo_theory_term_type_number => Ok(TheoryTermType::Number),
            clingo_theory_term_type_clingo_theory_term_type_symbol => Ok(TheoryTermType::Symbol),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_theory_term_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_theory_term_type.",
                })
            }
        }
    }
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
impl ModelType {
    fn try_from(code: i32) -> Result<ModelType, ClingoError> {
        match code as u32 {
            clingo_model_type_clingo_model_type_stable_model => Ok(ModelType::StableModel),
            clingo_model_type_clingo_model_type_brave_consequences => {
                Ok(ModelType::BraveConsequences)
            }
            clingo_model_type_clingo_model_type_cautious_consequences => {
                Ok(ModelType::CautiousConsequences)
            }
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_model_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_model_type.",
                })
            }
        }
    }
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
impl PropagatorCheckMode {
    fn try_from(code: i32) -> Result<PropagatorCheckMode, ClingoError> {
        match code as u32 {
            clingo_propagator_check_mode_clingo_propagator_check_mode_fixpoint => {
                Ok(PropagatorCheckMode::Fixpoint)
            }
            clingo_propagator_check_mode_clingo_propagator_check_mode_total => {
                Ok(PropagatorCheckMode::Total)
            }
            clingo_propagator_check_mode_clingo_propagator_check_mode_none => {
                Ok(PropagatorCheckMode::None)
            }
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_propagator_check_mode {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_propagator_check_mode.",
                })
            }
        }
    }
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
    event: *mut c_void,
    event_handler: *mut c_void,
    goon: *mut bool,
) -> bool;
pub trait SolveEventHandler {
    /// Callback function called during search to notify when the search is finished or a model is ready
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
        _event: *mut c_void,
        event_handler: *mut c_void,
        goon: *mut bool,
    ) -> bool {
        // check for null pointers
        if event_handler.is_null() | goon.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_solve_callback() got a null pointer.",
            );
            return false;
        }
        let event_handler = &mut *(event_handler as *mut T);
        let goon = &mut *goon;

        match SolveEventType::try_from(etype) {
            Err(_) => {
                // from the libclingo docs:
                // If a (non-recoverable) clingo API function fails in this callback, it must return false.
                // In case of errors not related to clingo, set error code ErrorType::Unknown and return false to stop solving with an error.
                set_internal_error(ErrorType::Runtime, "Error in unsafe_solve_callback().");
                false
            }
            Ok(etype) => event_handler.on_solve_event(etype, goon),
        }
    }
}

type AstCallback =
    unsafe extern "C" fn(arg1: *const clingo_ast_statement_t, arg2: *mut c_void) -> bool;
pub trait StatementHandler {
    /// Callback function called on an ast statement while traversing the ast.
    ///
    /// **Returns** whether the call was successful
    fn on_statement(&mut self, arg1: &ast::Statement) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_ast_callback<T: StatementHandler>(
        stm: *const clingo_ast_statement_t,
        event_handler: *mut c_void,
    ) -> bool {
        // check for null pointers
        if stm.is_null() | event_handler.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_ast_callback() got a null pointer.",
            );
            return false;
        }
        let stm = &*(stm as *const ast::Statement);
        let event_handler = &mut *(event_handler as *mut T);

        event_handler.on_statement(stm)
    }
}

type LoggingCallback =
    unsafe extern "C" fn(code: clingo_warning_t, message: *const c_char, logger: *mut c_void);
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
        message: *const c_char,
        logger: *mut c_void,
    ) {
        // check for null pointers
        if message.is_null() | logger.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_logging_callback() got a null pointer.",
            );
            return;
        }
        let message = CStr::from_ptr(message);
        let logger = &mut *(logger as *mut L);

        if let Err(e) = logger.try_logging_callback(code, message) {
            eprintln!("Error in unsafe_logging_callback(): {}.", e);
            set_internal_error(ErrorType::Runtime, "Error in unsafe_logging_callback().");
        }
    }
    #[doc(hidden)]
    fn try_logging_callback(
        &mut self,
        code: clingo_warning_t,
        message: &CStr,
    ) -> Result<(), ClingoError> {
        let code = Warning::try_from(code)?;
        let message = message.to_str()?;
        self.log(code, message);
        Ok(())
    }
}

type GroundCallback = unsafe extern "C" fn(
    location: *const clingo_location_t,
    name: *const c_char,
    arguments: *const clingo_symbol_t,
    arguments_size: usize,
    event_handler: *mut c_void,
    symbol_callback: clingo_symbol_callback_t,
    symbol_callback_data: *mut c_void,
) -> bool;
pub trait ExternalFunctionHandler {
    /// Callback function to implement external functions.
    ///
    /// If an external function of form `@name(parameters)` occurs in a logic program,
    /// then this function is called with its location, name, parameters, and a callback to inject symbols as arguments.
    /// The callback can be called multiple times; all symbols passed are injected.
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
    /// ) -> Result<Vec<Symbol>,Error> {
    ///     if name == "f" && arguments.len() == 0 {
    ///         let symbol = Symbol::create_number(42);
    ///         Ok(vec![symbol])
    ///     } else {
    ///        Err(MyError{ msg: "function not found"})?
    ///    }
    /// }
    /// ```
    fn on_external_function(
        &mut self,
        location: &Location,
        name: &str,
        arguments: &[Symbol],
    ) -> Result<Vec<Symbol>, ExternalError>;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_ground_callback<T: ExternalFunctionHandler>(
        location: *const clingo_location_t,
        name: *const c_char,
        arguments: *const clingo_symbol_t,
        arguments_size: usize,
        event_handler: *mut c_void,
        symbol_callback: clingo_symbol_callback_t,
        symbol_callback_data: *mut c_void,
    ) -> bool {
        // check for null pointers
        if location.is_null()
            | name.is_null()
            | (arguments_size > 0 && arguments.is_null())
            | event_handler.is_null()
        {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_ground_callback() got a null pointer.",
            );
            return false;
        }
        let location = &*(location as *const Location);
        let name = CStr::from_ptr(name);
        let arguments = std::slice::from_raw_parts(arguments as *const Symbol, arguments_size);
        let event_handler = &mut *(event_handler as *mut T);

        match event_handler.try_symbol_callback(
            location,
            name,
            arguments,
            symbol_callback,
            symbol_callback_data,
        ) {
            Ok(x) => x,
            Err(e) => {
                // from the libclingo docs:
                // If a (non-recoverable) clingo API function fails in this callback, it must return false.
                // In case of errors not related to clingo, set error code ErrorType::Unknown and return false to stop solving with an error.
                eprintln!("Error in unsafe_ground_callback(): {}.", e);
                set_internal_error(ErrorType::Runtime, "Error in unsafe_ground_callback().");
                false
            }
        }
    }

    #[doc(hidden)]
    fn try_symbol_callback(
        &mut self,
        location: &Location,
        name: &CStr,
        arguments: &[Symbol],
        symbol_callback: clingo_symbol_callback_t,
        symbol_callback_data: *mut c_void,
    ) -> Result<bool, ClingoError> {
        let name = name.to_str()?;
        let symbols = self.on_external_function(location, name, arguments)?;
        if let Some(symbol_callback) = symbol_callback {
            let v: Vec<clingo_symbol_t> = symbols.iter().map(|symbol| (*symbol).0).collect();
            Ok(unsafe { symbol_callback(v.as_slice().as_ptr(), v.len(), symbol_callback_data) })
        } else {
            // no symbol callback
            Ok(true)
        }
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
    /// Create a default location.
    fn default() -> clingo_location {
        let file = CString::new("").unwrap();
        clingo_location {
            begin_line: 0,
            end_line: 0,
            begin_column: 0,
            end_column: 0,
            begin_file: file.as_ptr(),
            end_file: file.as_ptr(),
        }
    }
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
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `begin_file` `end_file` or contain a nul byte
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
            return Some(Ordering::Less);
        }
        if unsafe { clingo_signature_is_less_than(other.0, self.0) } {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Equal)
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
    /// * `name` - name of the signature
    /// * `arity` - arity of the signature
    /// * `positive` - false if the signature has a classical negation sign
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    pub fn new(name: &str, arity: u32, positive: bool) -> Result<Signature, ClingoError> {
        let name = CString::new(name)?;
        let mut signature = 0;
        if !unsafe { clingo_signature_create(name.as_ptr(), arity, positive, &mut signature) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_signature_create() failed",
            ));
        }
        Ok(Signature(signature))
    }

    // TODO: should i return empty string vs Error
    /// Get the name of a signature.
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
            return Some(Ordering::Less);
        }
        if unsafe { clingo_symbol_is_less_than(other.0, self.0) } {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Equal)
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
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `string` contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn create_string(string: &str) -> Result<Symbol, ClingoError> {
        let mut symbol = 0 as clingo_symbol_t;
        let c_str = CString::new(string)?;
        if !unsafe { clingo_symbol_create_string(c_str.as_ptr(), &mut symbol) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_create_string() failed",
            ));
        }
        Ok(Symbol(symbol))
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
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn create_id(name: &str, positive: bool) -> Result<Symbol, ClingoError> {
        let mut symbol = 0 as clingo_symbol_t;
        let name = CString::new(name)?;
        if !unsafe { clingo_symbol_create_id(name.as_ptr(), positive, &mut symbol) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_create_id() failed",
            ));
        }
        Ok(Symbol(symbol))
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
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn create_function(
        name: &str,
        arguments: &[Symbol],
        positive: bool,
    ) -> Result<Symbol, ClingoError> {
        let mut symbol = 0 as clingo_symbol_t;
        let name = CString::new(name)?;
        if !unsafe {
            clingo_symbol_create_function(
                name.as_ptr(),
                arguments.as_ptr() as *const clingo_symbol_t,
                arguments.len(),
                positive,
                &mut symbol,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_create_function() failed",
            ));
        }
        Ok(Symbol(symbol))
    }
    /// Get the number of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if symbol is not of type [`SymbolType::Number`](enum.SymbolType.html#variant.Number)
    pub fn number(self) -> Result<i32, ClingoError> {
        let mut number = 0;
        if !unsafe { clingo_symbol_number(self.0, &mut number) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_number() failed",
            ));
        }
        Ok(number)
    }

    /// Get the name of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if symbol is not of type [`SymbolType::Function`](enum.SymbolType.html#variant.Function)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn name(&self) -> Result<&str, ClingoError> {
        let mut char_ptr = std::ptr::null();
        if !unsafe { clingo_symbol_name(self.0, &mut char_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_name() failed",
            ));
        }
        if char_ptr.is_null() {
            return Err(ClingoError::new_internal(
                "clingo_symbol_name() returned a null pointer.",
            ));
        }
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        Ok(c_str.to_str()?)
    }

    /// Get the string of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if symbol is not of type [`SymbolType::String`](enum.SymbolType.html#variant.String)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn string(&self) -> Result<&str, ClingoError> {
        let mut char_ptr = std::ptr::null();
        if !unsafe { clingo_symbol_string(self.0, &mut char_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_string() failed",
            ));
        }
        if char_ptr.is_null() {
            return Err(ClingoError::FFIError {
                msg: "clingo_symbol_string() returned a null pointer.",
            });
        }
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        Ok(c_str.to_str()?)
    }

    /// Check if a function is positive (does not have a sign).
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if symbol is not of type [`SymbolType::Function`](enum.SymbolType.html#variant.Function)
    pub fn is_positive(self) -> Result<bool, ClingoError> {
        let mut positive = false;
        if !unsafe { clingo_symbol_is_positive(self.0, &mut positive) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_is_positive() failed",
            ));
        }
        Ok(positive)
    }

    /// Check if a function is negative (has a sign).
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if symbol is not of type [`SymbolType::Function`](enum.SymbolType.html#variant.Function)
    pub fn is_negative(self) -> Result<bool, ClingoError> {
        let mut negative = false;
        if !unsafe { clingo_symbol_is_negative(self.0, &mut negative) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_is_negative() failed",
            ));
        }
        Ok(negative)
    }

    /// Get the arguments of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if symbol is not of type [`SymbolType::Function`](enum.SymbolType.html#variant.Function)
    pub fn arguments(self) -> Result<Vec<Symbol>, ClingoError> {
        let mut symbol_ptr = std::ptr::null();
        let mut size: usize = 0;
        if !unsafe { clingo_symbol_arguments(self.0, &mut symbol_ptr, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_arguments() failed",
            ));
        }
        let mut symbols = Vec::<Symbol>::with_capacity(size);

        // let symbols_ptr = symbols.as_mut_ptr();
        // let symbols = unsafe {std::slice::from_raw_parts(symbols_ptr, size)};

        for _ in 0..size {
            if symbol_ptr.is_null() {
                return Err(ClingoError::FFIError {
                    msg: "clingo_symbol_arguments() returned a null pointer.",
                });
            }
            let nsymbol = unsafe { *symbol_ptr };
            symbols.push(Symbol(nsymbol));
            symbol_ptr = unsafe { symbol_ptr.offset(1) };
        }
        Ok(symbols)
    }

    /// Get the type of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) - may failed to match clingo symbol type
    pub fn symbol_type(self) -> Result<SymbolType, ClingoError> {
        SymbolType::try_from(unsafe { clingo_symbol_type(self.0) })
    }

    /// Get the string representation of a symbol.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn to_string(self) -> Result<String, ClingoError> {
        let mut size: usize = 0;
        if !unsafe { clingo_symbol_to_string_size(self.0, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_to_string_size() failed",
            ));
        }
        let mut string = Vec::with_capacity(size);
        let string_ptr = string.as_mut_ptr();
        if !unsafe { clingo_symbol_to_string(self.0, string_ptr, size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbol_to_string() failed",
            ));
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(string_ptr) };
        let str_slice: &str = c_str.to_str()?;
        Ok(str_slice.to_owned())
    }
}

/// Parse the given program and return an abstract syntax tree for each statement via a callback.
///
/// # Arguments
///
/// * `program` - the program in gringo syntax
/// * `handler` - implementing the trait [`StatementHandler`](trait.StatementHandler.html)
///
/// # Errors
///
/// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `program` contains a nul byte
/// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if parsing fails
///  or with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
pub fn parse_program<T: StatementHandler>(
    program: &str,
    handler: &mut T,
) -> Result<(), ClingoError> {
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let program = CString::new(program)?;
    let handler = handler as *mut T;
    if !unsafe {
        clingo_parse_program(
            program.as_ptr(),
            Some(T::unsafe_ast_callback::<T> as AstCallback),
            handler as *mut c_void,
            logger,
            logger_data,
            0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_parse_program() failed",
        ));
    }
    Ok(())
}

/// Parse the given program and return an abstract syntax tree for each statement via a callback.
///
/// # Arguments
///
/// * `program` - the program in gringo syntax
/// * `handler` - implementating the trait [`StatementHandler`](trait.StatementHandler.html)
/// * `logger` - implementing the trait [`Logger`](trait.Logger.html) to report messages during parsing
/// * `message_limit` - the maximum number of times the logger is called
///
/// # Errors
///
/// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `program` contains a nul byte
/// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if parsing fails
/// or [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
pub fn parse_program_with_logger<T: StatementHandler, L: Logger>(
    program: &str,
    handler: &mut T,
    logger: &mut L,
    message_limit: u32,
) -> Result<(), ClingoError> {
    let handler = handler as *mut T;
    let logger = logger as *mut L;
    let program = CString::new(program)?;
    if !unsafe {
        clingo_parse_program(
            program.as_ptr(),
            Some(T::unsafe_ast_callback::<T> as AstCallback),
            handler as *mut c_void,
            Some(L::unsafe_logging_callback::<L> as LoggingCallback),
            logger as *mut c_void,
            message_limit,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_parse_program() failed",
        ));
    }
    Ok(())
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
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if argument parsing fails
    pub fn new(name: &str, params: &'a [Symbol]) -> Result<Part<'a>, ClingoError> {
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
        propagator: *mut c_void,
    ) -> bool {
        // check for null pointers
        if init.is_null() | propagator.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_init() got a null pointer.");
            return false;
        }
        let init = &mut *(init as *mut PropagateInit);
        let propagator = &mut *(propagator as *mut T);

        propagator.init(init)
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
        propagator: *mut c_void,
    ) -> bool {
        // check for null pointers
        if control.is_null() | (size > 0 && changes.is_null()) | propagator.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_propagate() got a null pointer.");
            return false;
        }
        let control = &mut *(control as *mut PropagateControl);
        let changes = std::slice::from_raw_parts(changes as *const Literal, size);
        let propagator = &mut *(propagator as *mut T);

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
    ///
    /// **Returns** whether the call was successful
    fn undo(&mut self, _control: &mut PropagateControl, _changes: &[Literal]) -> bool {
        true
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_undo<T: Propagator>(
        control: *const clingo_propagate_control_t,
        changes: *const clingo_literal_t,
        size: usize,
        propagator: *mut c_void,
    ) -> bool {
        // check for null pointers
        if control.is_null() | (size > 0 && changes.is_null()) | propagator.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_undo() got a null pointer.");
            return false;
        }
        let control = &mut *(control as *mut PropagateControl);
        let changes = std::slice::from_raw_parts(changes as *const Literal, size);
        let propagator = &mut *(propagator as *mut T);

        propagator.undo(control, changes)
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
        propagator: *mut c_void,
    ) -> bool {
        // check for null pointers
        if control.is_null() | propagator.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_check() got a null pointer.");
            return false;
        }
        let control = &mut *(control as *mut PropagateControl);
        let propagator = &mut *(propagator as *mut T);

        propagator.check(control)
    }
    /// This function allows a propagator to implement domain-specific heuristics.
    ///
    /// It is called whenever propagation reaches a fixed point and
    /// should return a free solver literal that is to be assigned true.
    /// In case multiple propagators are registered,
    /// this function can return 0 to let a propagator registered later make a decision.
    /// If all propagators return 0, then the fallback literal is
    ///
    /// # Arguments
    ///
    /// * `thread_id` - the solver's thread id
    /// * `assignment` -  the assignment of the solver
    /// * `fallback` -  the literal choosen by the solver's heuristic
    /// * `decision` -  the literal to make true
    ///
    /// **Returns** whether the call was successful
    fn decide(
        &mut self,
        _thread_id: Id,
        _assignment: &Assignment,
        _fallback: Literal,
        _decision: &mut Literal,
    ) -> bool {
        true
    }

    #[doc(hidden)]
    unsafe extern "C" fn unsafe_decide<T: Propagator>(
        thread_id: clingo_id_t,
        assignment: *const clingo_assignment_t,
        fallback: clingo_literal_t,
        propagator: *mut ::std::os::raw::c_void,
        decision: *mut clingo_literal_t,
    ) -> bool {
        // check for null pointers
        if assignment.is_null() | propagator.is_null() | decision.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_decide() got a null pointer.");
            return false;
        }
        let assignment = &*(assignment as *const Assignment);
        let fallback = Literal(fallback);
        let propagator = &mut *(propagator as *mut T);
        let decision = &mut *(decision as *mut Literal);

        propagator.decide(Id(thread_id), assignment, fallback, decision)
    }
}

/// Control object holding grounding and solving state.
#[derive(Debug)]
pub struct Control {
    ctl: NonNull<clingo_control_t>,
}
impl Drop for Control {
    fn drop(&mut self) {
        // println!("drop Control");
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
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if an argument contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if argument parsing fails
    pub fn new(arguments: std::vec::Vec<String>) -> Result<Control, ClingoError> {
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

        let mut ctl_ptr = std::ptr::null_mut();

        if !unsafe {
            clingo_control_new(
                c_args.as_ptr(),
                c_args.len(),
                logger,
                logger_data,
                0,
                &mut ctl_ptr,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_new() failed",
            ));
        }
        match NonNull::new(ctl_ptr) {
            Some(ctl) => Ok(Control { ctl }),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
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
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if an argument contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if argument parsing fails
    pub fn new_with_logger<L: Logger>(
        arguments: Vec<String>,
        logger: &mut L,
        message_limit: u32,
    ) -> Result<Control, ClingoError> {
        let mut args = vec![];
        for arg in arguments {
            args.push(CString::new(arg)?);
        }

        // convert the strings to raw pointers
        let c_args = args
            .iter()
            .map(|arg| arg.as_ptr())
            .collect::<Vec<*const c_char>>();

        let mut ctl_ptr = std::ptr::null_mut();

        let logger = logger as *mut L;
        if !unsafe {
            clingo_control_new(
                c_args.as_ptr(),
                c_args.len(),
                Some(L::unsafe_logging_callback::<L> as LoggingCallback),
                logger as *mut c_void,
                message_limit,
                &mut ctl_ptr,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_new() failed",
            ));
        }
        match NonNull::new(ctl_ptr) {
            Some(ctl) => Ok(Control { ctl }),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
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
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if a any argument contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if parsing fails
    pub fn add(
        &mut self,
        name: &str,
        parameters: &[&str],
        program: &str,
    ) -> Result<(), ClingoError> {
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

        if !unsafe {
            clingo_control_add(
                self.ctl.as_ptr(),
                name.as_ptr(),
                c_parameters.as_ptr(),
                parameters_size,
                program_ptr,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_add() failed",
            ));
        }
        Ok(())
    }
    pub fn add_facts(&mut self, facts: &FactBase) {
        for sym in facts.iter() {
            // print!("{}",sym.to_string().unwrap());

            // initilize atom to add
            let atom = ast::Term::from(*sym);

            // create literal
            let lit = ast::Literal::from_term(ast::Sign::None, &atom);

            // create headliteral
            let hlit = ast::HeadLiteral::from(&lit);

            // create (fact) rule
            let rule = ast::Rule::new(hlit, &[]);

            // initialize the statement
            let stm = rule.ast_statement();

            // get the program builder
            let mut builder = ast::ProgramBuilder::from(self).unwrap();

            // add the rewritten statement to the program
            builder
                .add(&stm)
                .expect("Failed to add statement to ProgramBuilder.");

            builder.end().expect("Failed to finish building a program.");
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn ground(&mut self, parts: &[Part]) -> Result<(), ClingoError> {
        let parts_size = parts.len();
        let parts = parts
            .iter()
            .map(|arg| arg.from())
            .collect::<Vec<clingo_part>>();

        if !unsafe {
            clingo_control_ground(
                self.ctl.as_ptr(),
                parts.as_ptr(),
                parts_size,
                None,
                std::ptr::null_mut(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_ground() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
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

        let handler = handler as *mut T;
        if !unsafe {
            clingo_control_ground(
                self.ctl.as_ptr(),
                parts.as_ptr(),
                parts_size,
                Some(T::unsafe_ground_callback::<T> as GroundCallback),
                handler as *mut c_void,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_ground() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving could not be started
    pub fn solve(
        &mut self,
        mode: SolveMode,
        assumptions: &[Literal],
    ) -> Result<SolveHandle, ClingoError> {
        let mut handle = std::ptr::null_mut();
        if !unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode.bits(),
                assumptions.as_ptr() as *const clingo_literal_t,
                assumptions.len(),
                None,
                std::ptr::null_mut(),
                &mut handle,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_solve() failed",
            ));
        }
        match unsafe { handle.as_mut() } {
            Some(handle_ref) => Ok(SolveHandle { theref: handle_ref }),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mut clingo_solve_handle.",
            }),
        }
    }

    /// Covenience function that returns an iterator over the models.
    /// Uses [solve](struct.Control.html#method.solve) with [SolveMode::Yield](enum.SolveMode.html#variant.YIELD) and empty assumptions.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving could not be started
    pub fn all_models(&mut self) -> Result<AllModels, ClingoError> {
        let mut handle = std::ptr::null_mut();
        if !unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                SolveMode::YIELD.bits(),
                std::ptr::null() as *const clingo_literal_t,
                0,
                None,
                std::ptr::null_mut(),
                &mut handle,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_solve() failed",
            ));
        }
        match unsafe { handle.as_mut() } {
            Some(handle_ref) => Ok(AllModels(SolveHandle { theref: handle_ref })),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mut clingo_solve_handle.",
            }),
        }
    }

    /// Covenience function that returns an iterator over the optimal models.
    /// Uses [solve](struct.Control.html#method.solve) with [SolveMode::Yield](enum.SolveMode.html#variant.YIELD) and empty assumptions.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving could not be started
    pub fn optimal_models(&mut self) -> Result<OptimalModels, ClingoError> {
        let mut handle = std::ptr::null_mut();
        if !unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                SolveMode::YIELD.bits(),
                std::ptr::null() as *const clingo_literal_t,
                0,
                None,
                std::ptr::null_mut(),
                &mut handle,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_solve() failed",
            ));
        }
        match unsafe { handle.as_mut() } {
            Some(handle_ref) => Ok(OptimalModels(SolveHandle { theref: handle_ref })),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mut clingo_solve_handle.",
            }),
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving could not be started
    pub fn solve_with_event_handler<T: SolveEventHandler>(
        &mut self,
        mode: SolveMode,
        assumptions: &[Literal],
        event_handler: &mut T,
    ) -> Result<SolveHandle, ClingoError> {
        let mut handle = std::ptr::null_mut();
        let event_handler = event_handler as *mut T;
        if !unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode.bits(),
                assumptions.as_ptr() as *const clingo_literal_t,
                assumptions.len(),
                Some(T::unsafe_solve_callback::<T> as SolveEventCallback),
                event_handler as *mut c_void,
                &mut handle,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_solve() failed",
            ));
        }
        match unsafe { handle.as_mut() } {
            Some(handle_ref) => Ok(SolveHandle { theref: handle_ref }),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mut clingo_solve_handle.",
            }),
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn cleanup(&mut self) -> Result<(), ClingoError> {
        if !unsafe { clingo_control_cleanup(self.ctl.as_ptr()) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_cleanup() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn assign_external(
        &mut self,
        literal: Literal,
        value: TruthValue,
    ) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_control_assign_external(
                self.ctl.as_ptr(),
                literal.0,
                value as clingo_truth_value_t,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_assign_external() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn release_external(&mut self, Literal(literal): Literal) -> Result<(), ClingoError> {
        if !unsafe { clingo_control_release_external(self.ctl.as_ptr(), literal) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_release_external() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn register_propagator<T: Propagator>(
        &mut self,
        propagator: &mut T,
        sequential: bool,
    ) -> Result<(), ClingoError> {
        let propagator = propagator as *mut T;
        let clingo_propagator = clingo_propagator_t {
            init: Some(T::unsafe_init::<T>),
            propagate: Some(T::unsafe_propagate::<T>),
            undo: Some(T::unsafe_undo::<T>),
            check: Some(T::unsafe_check::<T>),
            decide: Some(T::unsafe_decide::<T>),
        };
        if !unsafe {
            clingo_control_register_propagator(
                self.ctl.as_ptr(),
                &clingo_propagator,
                propagator as *mut c_void,
                sequential,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_register_propagator() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn statistics<'a>(&'a self) -> Result<&'a Statistics, ClingoError> {
        let mut stat = std::ptr::null();
        if !unsafe { clingo_control_statistics(self.ctl.as_ptr(), &mut stat) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_statistics() failed",
            ));
        }
        match unsafe { (stat as *mut Statistics).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &Statistics.",
            }),
        }
    }

    /// Interrupt the active solve call (or the following solve call right at the beginning).
    pub fn interrupt(&mut self) {
        unsafe {
            clingo_control_interrupt(self.ctl.as_ptr());
        }
    }

    /// Get a configuration object to change the solver configuration.
    pub fn configuration_mut<'a>(&'a mut self) -> Result<&'a mut Configuration, ClingoError> {
        let mut conf = std::ptr::null_mut();
        if !unsafe { clingo_control_configuration(self.ctl.as_ptr(), &mut conf) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_configuration() failed",
            ));
        }
        match unsafe { (conf as *mut Configuration).as_mut() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mut Configuration.",
            }),
        }
    }

    /// Get a configuration object to change the solver configuration.
    pub fn configuration<'a>(&'a self) -> Result<&'a Configuration, ClingoError> {
        let mut conf = std::ptr::null_mut();
        if !unsafe { clingo_control_configuration(self.ctl.as_ptr(), &mut conf) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_configuration() failed",
            ));
        }
        match unsafe { (conf as *const Configuration).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &Configuration.",
            }),
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
    pub fn use_enumeration_assumption(&mut self, enable: bool) -> Result<(), ClingoError> {
        if !unsafe { clingo_control_use_enumeration_assumption(self.ctl.as_ptr(), enable) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_use_enumeration_assumption() failed",
            ));
        }
        Ok(())
    }

    /// Return the symbol for a constant definition of form: `#const name = symbol`.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the constant if it exists
    ///
    /// # Errors
    ///
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    pub fn get_const(&self, name: &str) -> Result<Symbol, ClingoError> {
        let name = CString::new(name)?;
        let mut symbol = 0 as clingo_symbol_t;
        if !unsafe { clingo_control_get_const(self.ctl.as_ptr(), name.as_ptr(), &mut symbol) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_get_const() failed",
            ));
        }
        Ok(Symbol(symbol))
    }

    /// Check if there is a constant definition for the given constant.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the constant
    ///
    /// # Errors
    ///
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    ///
    /// **See:** [`Part::get_const()`](struct.Part.html#method.get_const)
    pub fn has_const(&self, name: &str) -> Result<bool, ClingoError> {
        let name = CString::new(name)?;
        let mut exist = false;
        if !unsafe { clingo_control_has_const(self.ctl.as_ptr(), name.as_ptr(), &mut exist) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_has_const() failed",
            ));
        }
        Ok(exist)
    }

    /// Get an object to inspect symbolic atoms (the relevant Herbrand base) used
    pub fn symbolic_atoms<'a>(&self) -> Result<&'a SymbolicAtoms, ClingoError> {
        let mut atoms = std::ptr::null();
        if !unsafe { clingo_control_symbolic_atoms(self.ctl.as_ptr(), &mut atoms) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_symbolic_atoms() failed",
            ));
        }
        match unsafe { (atoms as *const SymbolicAtoms).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &SymbolicAtoms.",
            }),
        }
    }

    /// Get an object to inspect theory atoms that occur in the grounding.
    pub fn theory_atoms<'a>(&'a self) -> Result<&'a TheoryAtoms, ClingoError> {
        let mut atoms = std::ptr::null();
        if !unsafe { clingo_control_theory_atoms(self.ctl.as_ptr(), &mut atoms) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_theory_atoms() failed",
            ));
        }
        match unsafe { (atoms as *const TheoryAtoms).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &TheoryAtoms.",
            }),
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
        let observer = observer as *mut T;
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
                observer as *mut c_void,
            )
        }
    }

    /// Get an object to add ground directives to the program.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn backend<'a>(&'a mut self) -> Result<Backend<'a>, ClingoError> {
        let mut backend = std::ptr::null_mut();
        if !unsafe { clingo_control_backend(self.ctl.as_ptr(), &mut backend) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_backend() failed",
            ));
        }
        if !unsafe { clingo_backend_begin(backend) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_backend_begin() failed",
            ));
        }
        match unsafe { backend.as_mut() } {
            Some(backend_ref) => Ok(Backend {
                theref: backend_ref,
            }),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mut clingo_backend.",
            }),
        }
    }

    // NODO: pub fn clingo_control_clasp_facade()
}
/// Handle for the solver configuration.
#[derive(Debug)]
pub struct Configuration(clingo_configuration_t);
impl Configuration {
    /// Get the root key of the configuration.
    pub fn root(&self) -> Result<Id, ClingoError> {
        let mut root_key = 0 as clingo_id_t;
        if !unsafe { clingo_configuration_root(&self.0, &mut root_key) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_root() failed",
            ));
        }
        Ok(Id(root_key))
    }

    /// Get the type of a key.
    /// The type is a bitset, an entry can have multiple (but at least one) type.
    pub fn configuration_type(&self, Id(key): Id) -> Result<ConfigurationType, ClingoError> {
        let mut ctype = 0 as clingo_configuration_type_bitset_t;
        if !unsafe { clingo_configuration_type(&self.0, key, &mut ctype) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_type() failed",
            ));
        }
        match ConfigurationType::from_bits(ctype) {
            Some(x) => Ok(x),
            None => {
                eprintln!(
                    "Failed to match to clingo_configuration_type_bitset_t {}.",
                    ctype
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match to clingo_configuration_type_bitset_t.",
                })
            }
        }
    }

    /// Get the description of an entry.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn description<'a>(&'a self, Id(key): Id) -> Result<&'a str, ClingoError> {
        let mut description_ptr = std::ptr::null();
        if !unsafe { clingo_configuration_description(&self.0, key, &mut description_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_description() failed",
            ));
        }
        if description_ptr.is_null() {
            return Err(ClingoError::FFIError {
                msg: "clingo_configuration_description() returned a null pointer.",
            });
        }
        let cstr = unsafe { CStr::from_ptr(description_ptr) };
        Ok(cstr.to_str()?)
    }

    /// Get the size of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be  [`ConfigurationType::ARRAY`](struct.ConfigurationType.html#associatedconstant.ARRAY).
    pub fn array_size(&self, Id(key): Id) -> Result<usize, ClingoError> {
        let mut size = 0;
        if !unsafe { clingo_configuration_array_size(&self.0, key, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_array_size() failed",
            ));
        }
        Ok(size)
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
    pub fn array_at(&self, Id(key): Id, offset: usize) -> Result<Id, ClingoError> {
        let mut nkey = 0 as clingo_id_t;
        if !unsafe { clingo_configuration_array_at(&self.0, key, offset, &mut nkey) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_array_at() failed",
            ));
        }
        Ok(Id(nkey))
    }

    /// Get the number of subkeys of a map entry.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::MAP`](struct.ConfigurationType.html#associatedconstant.MAP).
    pub fn map_size(&self, Id(key): Id) -> Result<usize, ClingoError> {
        let mut size = 0;
        if !unsafe { clingo_configuration_map_size(&self.0, key, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_map_size() failed",
            ));
        }
        Ok(size)
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
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    ///
    /// **Returns** whether the key is in the map
    pub fn map_has_subkey(&self, Id(key): Id, name: &str) -> Result<bool, ClingoError> {
        let mut result = false;
        let name = CString::new(name)?;
        if !unsafe { clingo_configuration_map_has_subkey(&self.0, key, name.as_ptr(), &mut result) }
        {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_map_has_subkey() failed",
            ));
        }
        Ok(result)
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
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn map_subkey_name<'a>(
        &'a self,
        Id(key): Id,
        offset: usize,
    ) -> Result<&'a str, ClingoError> {
        let mut name_ptr = std::ptr::null();
        if !unsafe { clingo_configuration_map_subkey_name(&self.0, key, offset, &mut name_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_map_subkey_name() failed",
            ));
        }
        if name_ptr.is_null() {
            return Err(ClingoError::FFIError {
                msg: "clingo_configuration_map_subkey_name() returned a null pointer.",
            });
        }
        let cstr = unsafe { CStr::from_ptr(name_ptr) };
        Ok(cstr.to_str()?)
    }

    /// Lookup a subkey under the given name.
    ///
    /// # Pre-condition
    ///
    /// The [type](struct.Configuration.html#method.type) of the entry must be [`ConfigurationType::MAP`](struct.ConfigurationType.html#associatedconstant.MAP).
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    pub fn map_at(&self, Id(key): Id, name: &str) -> Result<Id, ClingoError> {
        let mut nkey = 0 as clingo_id_t;
        let name = CString::new(name)?;
        if !unsafe { clingo_configuration_map_at(&self.0, key, name.as_ptr(), &mut nkey) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_value_is_assigned() failed",
            ));
        }
        Ok(Id(nkey))
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
    pub fn value_is_assigned(&self, Id(key): Id) -> Result<bool, ClingoError> {
        let mut assigned = false;
        if !unsafe { clingo_configuration_value_is_assigned(&self.0, key, &mut assigned) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_value_is_assigned() failed",
            ));
        }
        Ok(assigned)
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn value_get<'a>(&'a self, Id(key): Id) -> Result<String, ClingoError> {
        let mut size = 0;
        if !unsafe { clingo_configuration_value_get_size(&self.0, key, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_value_get_size() failed",
            ));
        }
        let mut string = Vec::with_capacity(size);
        let string_ptr = string.as_mut_ptr();
        if !unsafe { clingo_configuration_value_get(&self.0, key, string_ptr, size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_value_get() failed",
            ));
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(string_ptr) };
        let str_slice: &str = c_str.to_str()?;
        Ok(str_slice.to_owned())
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
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `value` contains a nul byte
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    pub fn value_set(&mut self, Id(key): Id, value: &str) -> Result<(), ClingoError> {
        let value = CString::new(value)?;
        if !unsafe { clingo_configuration_value_set(&mut self.0, key, value.as_ptr()) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_configuration_value_set() failed",
            ));
        }
        Ok(())
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
            panic!("Call to clingo_backend_end() failed");
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn rule(
        &mut self,
        choice: bool,
        head: &[Atom],
        body: &[Literal],
    ) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_backend_rule(
                self.theref,
                choice,
                head.as_ptr() as *const clingo_atom_t,
                head.len(),
                body.as_ptr() as *const clingo_literal_t,
                body.len(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_backend_rule() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn weight_rule(
        &mut self,
        choice: bool,
        head: &[Atom],
        lower_bound: i32,
        body: &[WeightedLiteral],
    ) -> Result<(), ClingoError> {
        if !unsafe {
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
            return Err(ClingoError::new_internal(
                "Call to clingo_backend_weight_rule() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn minimize(
        &mut self,
        priority: i32,
        literals: &[WeightedLiteral],
    ) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_backend_minimize(
                self.theref,
                priority,
                literals.as_ptr() as *const clingo_weighted_literal_t,
                literals.len(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_backend_minimize() failed",
            ));
        }
        Ok(())
    }

    /// Add a projection directive.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the atoms to project on
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn project(&mut self, atoms: &[Atom]) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_backend_project(
                self.theref,
                atoms.as_ptr() as *const clingo_atom_t,
                atoms.len(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_backend_project() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn external(&mut self, atom: Atom, type_: ExternalType) -> Result<(), ClingoError> {
        if !unsafe { clingo_backend_external(self.theref, atom.0, type_ as clingo_external_type_t) }
        {
            return Err(ClingoError::new_internal(
                "Call to clingo_backend_external() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn assume(&mut self, literals: &[Literal]) -> Result<(), ClingoError> {
        let size = literals.len();
        if !unsafe {
            clingo_backend_assume(
                self.theref,
                literals.as_ptr() as *const clingo_literal_t,
                size,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_backend_assume() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn heuristic(
        &mut self,
        atom: Atom,
        htype: HeuristicType,
        bias: i32,
        priority: u32,
        condition: &[Literal],
    ) -> Result<(), ClingoError> {
        let size = condition.len();
        if !unsafe {
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
            return Err(ClingoError::new_internal(
                "Call to clingo_backend_heuristic() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn acyc_edge(
        &mut self,
        node_u: i32,
        node_v: i32,
        condition: &[Literal],
    ) -> Result<(), ClingoError> {
        let size = condition.len();
        if !unsafe {
            clingo_backend_acyc_edge(
                self.theref,
                node_u,
                node_v,
                condition.as_ptr() as *const clingo_literal_t,
                size,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_backend_acyc_edge() failed",
            ));
        }
        Ok(())
    }

    /// Get a fresh atom to be used in aspif directives.
    /// # Arguments
    ///
    /// * `symbol` - optional symbol to associate the atom with
    pub fn add_atom(&mut self, symbol: Option<Symbol>) -> Result<Atom, ClingoError> {
        match symbol {
            Some(Symbol(mut symbol)) => {
                let mut atom = 0 as clingo_atom_t;
                if unsafe { clingo_backend_add_atom(self.theref, &mut symbol, &mut atom) } {
                    Ok(Atom(atom))
                } else {
                    Err(ClingoError::new_internal(
                        "Call to clingo_backend_add_atom() failed",
                    ))
                }
            }
            None => {
                let mut atom = 0 as clingo_atom_t;
                let null = std::ptr::null_mut();
                if unsafe { clingo_backend_add_atom(self.theref, null, &mut atom) } {
                    Ok(Atom(atom))
                } else {
                    Err(ClingoError::new_internal(
                        "Call to clingo_backend_add_atom() failed",
                    ))
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
    pub fn root(&self) -> Result<u64, ClingoError> {
        let mut root_key = 0 as u64;
        if !unsafe { clingo_statistics_root(&self.0, &mut root_key) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_root() failed",
            ));
        }
        Ok(root_key)
    }

    /// Get the type of a key.
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn statistics_type(&self, key: u64) -> Result<StatisticsType, ClingoError> {
        let mut stype = 0 as clingo_statistics_type_t;
        if !unsafe { clingo_statistics_type(&self.0, key, &mut stype) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_type() failed",
            ));
        }
        StatisticsType::try_from(stype)
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
    pub fn array_size(&self, key: u64) -> Result<usize, ClingoError> {
        let mut size = 0 as usize;
        if !unsafe { clingo_statistics_array_size(&self.0, key, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_array_size() failed",
            ));
        }
        Ok(size)
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
    pub fn array_at(&self, key: u64, offset: usize) -> Result<u64, ClingoError> {
        let mut subkey = 0 as u64;
        if !unsafe { clingo_statistics_array_at(&self.0, key, offset, &mut subkey) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_array_at() failed",
            ));
        }
        Ok(subkey)
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
    pub fn array_push(&mut self, key: u64, stype: StatisticsType) -> Result<u64, ClingoError> {
        let mut subkey = 0 as u64;
        if !unsafe {
            clingo_statistics_array_push(
                &mut self.0,
                key,
                stype as clingo_statistics_type_t,
                &mut subkey,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_array_push() failed",
            ));
        }
        Ok(subkey)
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
    pub fn map_size(&self, key: u64) -> Result<usize, ClingoError> {
        let mut size = 0 as usize;
        if !unsafe { clingo_statistics_map_size(&self.0, key, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_map_size() failed",
            ));
        }
        Ok(size)
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
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    pub fn map_has_subkey(&self, key: u64, name: &str) -> Result<bool, ClingoError> {
        let mut result = false;
        let name = CString::new(name)?;
        if !unsafe { clingo_statistics_map_has_subkey(&self.0, key, name.as_ptr(), &mut result) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_map_has_subkey() failed",
            ));
        }
        Ok(result)
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
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn map_subkey_name<'a>(&'a self, key: u64, offset: usize) -> Result<&'a str, ClingoError> {
        let mut name = std::ptr::null();
        if !unsafe { clingo_statistics_map_subkey_name(&self.0, key, offset, &mut name) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_map_subkey_name() failed",
            ));
        }
        if name.is_null() {
            return Err(ClingoError::FFIError {
                msg: "clingo_statistics_map_subkey_name() returned a null pointer.",
            });
        }
        Ok(unsafe { CStr::from_ptr(name) }.to_str()?)
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
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    pub fn map_at(&self, key: u64, name: &str) -> Result<u64, ClingoError> {
        let mut subkey = 0 as u64;
        let name = CString::new(name)?;
        if !unsafe { clingo_statistics_map_at(&self.0, key, name.as_ptr(), &mut subkey) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_map_at() failed",
            ));
        }
        Ok(subkey)
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
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `name` contains a nul byte
    ///
    /// **Returns** the index of the resulting subkey
    pub fn map_add_subkey(
        &mut self,
        key: u64,
        name: &str,
        stype: StatisticsType,
    ) -> Result<u64, ClingoError> {
        let mut subkey = 0 as u64;
        let name = CString::new(name)?;
        if !unsafe {
            clingo_statistics_map_add_subkey(
                &mut self.0,
                key,
                name.as_ptr(),
                stype as clingo_statistics_type_t,
                &mut subkey,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_map_add_subkey() failed",
            ));
        }
        Ok(subkey)
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
    pub fn value_get(&self, key: u64) -> Result<f64, ClingoError> {
        let mut value = 0.0 as f64;
        if !unsafe { clingo_statistics_value_get(&self.0, key, &mut value) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_size() failed",
            ));
        }
        Ok(value)
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
    pub fn size(&self) -> Result<usize, ClingoError> {
        let mut size = 0 as usize;
        if !unsafe { clingo_symbolic_atoms_size(&self.0, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_size() failed",
            ));
        }
        Ok(size)
    }

    /// Get a forward iterator of the sequence of all symbolic atoms.
    pub fn iter(&self) -> Result<SymbolicAtomsIterator, ClingoError> {
        let mut begin = 0 as clingo_symbolic_atom_iterator_t;
        if !unsafe { clingo_symbolic_atoms_begin(&self.0, std::ptr::null(), &mut begin) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_begin() failed",
            ));
        }
        let mut end = 0 as clingo_symbolic_atom_iterator_t;
        if !unsafe { clingo_symbolic_atoms_end(&self.0, &mut end) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_end() failed",
            ));
        }
        Ok(SymbolicAtomsIterator {
            cur: begin,
            end,
            atoms: &self.0,
        })
    }
    /// Get a forward iterator of the sequence of all symbolic atoms restricted to a given signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - the signature
    pub fn iter_with_signature(
        &self,
        sig: &Signature,
    ) -> Result<SymbolicAtomsIterator, ClingoError> {
        let mut begin = 0 as clingo_symbolic_atom_iterator_t;
        if !unsafe { clingo_symbolic_atoms_begin(&self.0, &sig.0, &mut begin) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_begin() failed",
            ));
        }
        let mut end = 0 as clingo_symbolic_atom_iterator_t;
        if !unsafe { clingo_symbolic_atoms_end(&self.0, &mut end) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_end() failed",
            ));
        }
        Ok(SymbolicAtomsIterator {
            cur: begin,
            end,
            atoms: &self.0,
        })
    }

    //NODO: fn clingo_symbolic_atoms_signatures_size()

    /// Get the predicate signatures occurring in a logic program.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if the size is too small
    pub fn signatures(&self) -> Result<Vec<Signature>, ClingoError> {
        let mut size = 0;
        if !unsafe { clingo_symbolic_atoms_signatures_size(&self.0, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_signatures_size() failed",
            ));
        }
        let mut signatures = vec![Signature(0); size];
        if !unsafe {
            clingo_symbolic_atoms_signatures(
                &self.0,
                signatures.as_mut_ptr() as *mut clingo_signature_t,
                size,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_signatures() failed",
            ));
        }
        Ok(signatures)
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
        if !unsafe {
            clingo_symbolic_atoms_iterator_is_equal_to(self.atoms, self.cur, self.end, &mut equal)
        } {
            return None;
        }
        if equal {
            return None;
        }
        let ret = SymbolicAtom {
            cur: self.cur,
            atoms: self.atoms,
        };
        if !unsafe { clingo_symbolic_atoms_next(self.atoms, self.cur, &mut self.cur) } {
            panic!(
                "Call clingo_symbolic_atoms_next() failed {}{}{}.",
                file!(),
                line!(),
                column!()
            );
        }
        Some(ret)
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
    pub fn is_fact(&self) -> Result<bool, ClingoError> {
        let mut fact = false;
        if !unsafe { clingo_symbolic_atoms_is_fact(self.atoms, self.cur, &mut fact) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_is_fact() failed",
            ));
        }
        Ok(fact)
    }

    /// Check whether an atom is external.
    ///
    /// An atom is external if it has been defined using an external directive and
    /// has not been released or defined by a rule.
    pub fn is_external(&self) -> Result<bool, ClingoError> {
        let mut external = false;
        if !unsafe { clingo_symbolic_atoms_is_external(self.atoms, self.cur, &mut external) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_is_external() failed",
            ));
        }
        Ok(external)
    }

    /// Get the symbolic representation of an atom.
    pub fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut symbol = 0 as clingo_symbol_t;
        if !unsafe { clingo_symbolic_atoms_symbol(self.atoms, self.cur, &mut symbol) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_symbol() failed",
            ));
        }
        Ok(Symbol(symbol))
    }

    /// Returns the (numeric) aspif literal corresponding to the given symbolic atom.
    ///
    /// Such a literal can be mapped to a solver literal (see [`Propagator`](struct.Propagator)).
    /// or be used in rules in aspif format (see [`ProgramBuilder`](struct.ProgramBuilder.html)).
    pub fn literal(&self) -> Result<Literal, ClingoError> {
        let mut literal = 0 as clingo_literal_t;
        if !unsafe { clingo_symbolic_atoms_literal(self.atoms, self.cur, &mut literal) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_literal() failed",
            ));
        }
        Ok(Literal(literal))
    }
}

/// Container that stores theory atoms, elements, and terms of a program.
///
/// **See:** [`Control::theory_atoms()`](struct.Control.html#method.theory_atoms)
#[derive(Debug)]
pub struct TheoryAtoms(clingo_theory_atoms_t);
impl TheoryAtoms {
    /// Get the total number of theory atoms.
    pub fn size(&self) -> Result<usize, ClingoError> {
        let mut size = 0 as usize;
        if !unsafe { clingo_theory_atoms_size(&self.0, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_size() failed",
            ));
        }
        Ok(size)
    }

    ///  Returns an iterator over the theory atoms.
    pub fn iter(&self) -> TheoryAtomsIterator {
        TheoryAtomsIterator {
            count: 0,
            atoms: &self,
        }
    }

    /// Get the type of the given theory term.
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    pub fn term_type(&self, Id(term): Id) -> Result<TheoryTermType, ClingoError> {
        let mut ttype = 0 as clingo_theory_term_type_t;
        if !unsafe { clingo_theory_atoms_term_type(&self.0, term, &mut ttype) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_term_type() failed",
            ));
        }
        TheoryTermType::try_from(ttype)
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
    pub fn term_number(&self, Id(term): Id) -> Result<i32, ClingoError> {
        let mut number = 0;
        if !unsafe { clingo_theory_atoms_term_number(&self.0, term, &mut number) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_term_number() failed",
            ));
        }
        Ok(number)
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
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn term_name<'a>(&self, Id(term): Id) -> Result<&'a str, ClingoError> {
        let mut char_ptr = std::ptr::null();
        if !unsafe { clingo_theory_atoms_term_name(&self.0, term, &mut char_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_term_name() failed",
            ));
        }
        if char_ptr.is_null() {
            return Err(ClingoError::FFIError {
                msg: "clingo_theory_atoms_term_name() returned a null pointer.",
            });
        }
        let c_str = unsafe { CStr::from_ptr(char_ptr) };
        Ok(c_str.to_str()?)
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
    pub fn term_arguments<'a>(&'a self, Id(term): Id) -> Result<&'a [Id], ClingoError> {
        let mut size = 0;
        let mut c_ptr = std::ptr::null();
        if !unsafe { clingo_theory_atoms_term_arguments(&self.0, term, &mut c_ptr, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_term_arguments() failed",
            ));
        }
        let arguments_ref = unsafe { std::slice::from_raw_parts(c_ptr as *const Id, size) };
        Ok(arguments_ref)
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if the size is too small
    /// or [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn term_to_string(&self, Id(term): Id) -> Result<String, ClingoError> {
        let mut size = 0;
        if !unsafe { clingo_theory_atoms_term_to_string_size(&self.0, term, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_term_to_string_size() failed",
            ));
        }
        let mut string = Vec::with_capacity(size);
        let string_ptr = string.as_mut_ptr();
        if !unsafe { clingo_theory_atoms_term_to_string(&self.0, term, string_ptr, size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_term_to_string() failed",
            ));
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(string_ptr) };
        let str_slice: &str = c_str.to_str()?;
        Ok(str_slice.to_owned())
    }

    /// Get the tuple (array of theory terms) of the given theory element.
    ///
    /// # Arguments
    ///
    /// * `element` - id of the element
    pub fn element_tuple(&self, Id(element): Id) -> Result<&[Id], ClingoError> {
        let mut size = 0;
        let mut tuple_ptr = std::ptr::null();
        if !unsafe {
            clingo_theory_atoms_element_tuple(&self.0, element, &mut tuple_ptr, &mut size)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_element_tuple() failed",
            ));
        }
        let tuple_ref = unsafe { std::slice::from_raw_parts(tuple_ptr as *const Id, size) };
        Ok(tuple_ref)
    }

    /// Get the condition (array of aspif literals) of the given theory element.
    ///
    /// # Arguments
    ///
    /// * `element` - id of the element
    pub fn element_condition(&self, Id(element): Id) -> Result<&[Literal], ClingoError> {
        let mut size = 0;
        let mut condition_ptr = std::ptr::null();
        if !unsafe {
            clingo_theory_atoms_element_condition(&self.0, element, &mut condition_ptr, &mut size)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_element_condition() failed",
            ));
        }
        let condition_ref =
            unsafe { std::slice::from_raw_parts(condition_ptr as *const Literal, size) };
        Ok(condition_ref)
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
    pub fn element_condition_id(&self, Id(element): Id) -> Result<Literal, ClingoError> {
        let condition_ptr = std::ptr::null_mut();
        if !unsafe { clingo_theory_atoms_element_condition_id(&self.0, element, condition_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_element_condition_id() failed",
            ));
        }
        if condition_ptr.is_null() {
            return Err(ClingoError::FFIError {
                msg: "clingo_theory_atoms_element_condition_id() returned a null pointer.",
            });
        }
        Ok(Literal(unsafe { *condition_ptr }))
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if the size is too small
    /// or [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn element_to_string(&self, Id(element): Id) -> Result<String, ClingoError> {
        let mut size = 0;
        if !unsafe { clingo_theory_atoms_element_to_string_size(&self.0, element, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_element_to_string_size() failed",
            ));
        }
        let mut string = Vec::with_capacity(size);
        let string_ptr = string.as_mut_ptr();
        if !unsafe { clingo_theory_atoms_element_to_string(&self.0, element, string_ptr, size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_element_to_string() failed",
            ));
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(string_ptr) };
        let str_slice: &str = c_str.to_str()?;
        Ok(str_slice.to_owned())
    }

    /// Get the theory term associated with the theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` - id of the atom
    pub fn atom_term(&self, Id(atom): Id) -> Result<Id, ClingoError> {
        let mut term = 0 as clingo_id_t;
        if !unsafe { clingo_theory_atoms_atom_term(&self.0, atom, &mut term) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_atom_term() failed",
            ));
        }
        Ok(Id(term))
    }

    /// Get the theory elements associated with the theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` - id of the atom
    pub fn atom_elements(&self, Id(atom): Id) -> Result<&[Id], ClingoError> {
        let mut size = 0;
        let mut elements_ptr = std::ptr::null() as *const clingo_id_t;
        if !unsafe {
            clingo_theory_atoms_atom_elements(&self.0, atom, &mut elements_ptr, &mut size)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_atom_elements() failed",
            ));
        }
        let elements = unsafe { std::slice::from_raw_parts(elements_ptr as *const Id, size) };
        Ok(elements)
    }

    /// Whether the theory atom has a guard.
    ///
    /// # Arguments
    ///
    /// * `atom` - id of the atom
    pub fn atom_has_guard(&self, Id(atom): Id) -> Result<bool, ClingoError> {
        let mut has_guard = false;
        if !unsafe { clingo_theory_atoms_atom_has_guard(&self.0, atom, &mut has_guard) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_atom_has_guard() failed",
            ));
        }
        Ok(has_guard)
    }

    /// Get the guard consisting of a theory operator and a theory term of the given theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` - id of the atom
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError)
    pub fn atom_guard(&self, Id(atom): Id) -> Result<(&str, Id), ClingoError> {
        let mut c_ptr = std::ptr::null() as *const c_char;
        let mut term = 0 as clingo_id_t;
        if !unsafe { clingo_theory_atoms_atom_guard(&self.0, atom, &mut c_ptr, &mut term) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_atom_guard() failed",
            ));
        }
        if c_ptr.is_null() {
            return Err(ClingoError::FFIError {
                msg: "clingo_theory_atoms_atom_guard() returned a null pointer.",
            });
        }
        let cstr = unsafe { CStr::from_ptr(c_ptr) };
        Ok((cstr.to_str()?, Id(term)))
    }

    /// Get the aspif literal associated with the given theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` id of the atom
    pub fn atom_literal(&self, Id(atom): Id) -> Result<Literal, ClingoError> {
        let mut literal = 0 as clingo_literal_t;
        if !unsafe { clingo_theory_atoms_atom_literal(&self.0, atom, &mut literal) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_atom_literal() failed",
            ));
        }
        Ok(Literal(literal))
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if the size is too small
    /// or [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
    pub fn atom_to_string(&self, Id(atom): Id) -> Result<String, ClingoError> {
        let mut size = 0;
        if !unsafe { clingo_theory_atoms_atom_to_string_size(&self.0, atom, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_atom_to_string_size() failed",
            ));
        }
        let mut string = Vec::with_capacity(size);
        let string_ptr = string.as_mut_ptr();
        if !unsafe { clingo_theory_atoms_atom_to_string(&self.0, atom, string_ptr, size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_atom_to_string() failed",
            ));
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(string_ptr) };
        let str_slice: &str = c_str.to_str()?;
        Ok(str_slice.to_owned())
    }
}

/// Iterator over theory atoms.
pub struct TheoryAtomsIterator<'a> {
    count: usize,
    atoms: &'a TheoryAtoms,
}
impl<'a> Iterator for TheoryAtomsIterator<'a> {
    type Item = Id;

    fn next(&mut self) -> Option<Id> {
        // check to see if we've finished counting or not.
        if self.count < self.atoms.size().unwrap() {
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
    pub fn model_type(&self) -> Result<ModelType, ClingoError> {
        let mut mtype = 0 as clingo_model_type_t;
        if !unsafe { clingo_model_type(&self.0, &mut mtype) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_type() failed",
            ));
        }
        ModelType::try_from(mtype)
    }

    /// Get the running number of the model.
    pub fn number(&self) -> Result<u64, ClingoError> {
        let mut number = 0;
        if !unsafe { clingo_model_number(&self.0, &mut number) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_number() failed",
            ));
        }
        Ok(number)
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if the size is too small
    pub fn symbols(&self, show: ShowType) -> Result<Vec<Symbol>, ClingoError> {
        let mut size: usize = 0;
        if !unsafe { clingo_model_symbols_size(&self.0, show.bits(), &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_symbols_size() failed",
            ));
        }
        let symbols = vec![Symbol(0); size];
        if !unsafe {
            clingo_model_symbols(
                &self.0,
                show.bits(),
                symbols.as_ptr() as *mut clingo_symbol_t,
                size,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_symbols() failed",
            ));
        }
        Ok(symbols)
    }

    /// Constant time lookup to test whether an atom is in a model.
    ///
    /// # Arguments
    ///
    /// * `atom` - the atom to lookup
    pub fn contains(&self, Symbol(atom): Symbol) -> Result<bool, ClingoError> {
        let mut contained = false;
        if !unsafe { clingo_model_contains(&self.0, atom, &mut contained) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_contains() failed",
            ));
        }
        Ok(contained)
    }

    /// Check whether a program literal is true in a model.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal to lookup
    pub fn is_true(&self, literal: Literal) -> Result<bool, ClingoError> {
        let mut is_true = false;
        if !unsafe { clingo_model_is_true(&self.0, literal.0, &mut is_true) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_is_true() failed",
            ));
        }
        Ok(is_true)
    }

    //NODO: pub fn clingo_model_cost_size(model: *mut Model, size: *mut size_t) -> u8;

    /// Get the cost vector of a model.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if the size is too small
    ///
    /// **See:** [`Model::optimality_proven()`](struct.Model.html#method.optimality_proven)
    pub fn cost(&self) -> Result<Vec<i64>, ClingoError> {
        let mut size: usize = 0;
        if !unsafe { clingo_model_cost_size(&self.0, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_cost_size() failed",
            ));
        }
        let mut cost = vec![0; size];
        if !unsafe { clingo_model_cost(&self.0, cost.as_mut_ptr(), size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_cost() failed",
            ));
        }
        Ok(cost)
    }

    /// Whether the optimality of a model has been proven.
    ///
    /// **See:** [`Model::cost()`](struct.Model.html#method.cost)
    pub fn optimality_proven(&self) -> Result<bool, ClingoError> {
        let mut proven = false;
        if !unsafe { clingo_model_optimality_proven(&self.0, &mut proven) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_optimality_proven() failed",
            ));
        }
        Ok(proven)
    }

    /// Get the id of the solver thread that found the model.
    pub fn thread_id(&self) -> Result<Id, ClingoError> {
        let mut id = 0 as clingo_id_t;
        if !unsafe { clingo_model_thread_id(&self.0, &mut id) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_thread_id() failed",
            ));
        }
        Ok(Id(id))
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
    pub fn context<'a>(&'a self) -> Result<&'a mut SolveControl, ClingoError> {
        let control_ptr = std::ptr::null_mut();
        if !unsafe { clingo_model_context(&self.0, control_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_context() failed",
            ));
        }
        if control_ptr.is_null() {
            return Err(ClingoError::FFIError {
                msg: "clingo_model_context() returned a null pointer.",
            });
        }
        unsafe { Ok(&mut *(control_ptr as *mut SolveControl)) }
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if adding the clause fails
    pub fn add_clause(&mut self, clause: &[Literal]) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_solve_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                clause.len(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_control_add_clause() failed",
            ));
        }
        Ok(())
    }

    /// Get an object to inspect the symbolic atoms.
    pub fn symbolic_atoms(&self) -> Result<&SymbolicAtoms, ClingoError> {
        let mut atoms = std::ptr::null() as *const clingo_symbolic_atoms_t;
        if !unsafe { clingo_solve_control_symbolic_atoms(&self.0, &mut atoms) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_level() failed",
            ));
        }
        match unsafe { (atoms as *const SymbolicAtoms).as_ref() } {
            Some(stm) => Ok(stm),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &SymbolicAtoms.",
            }),
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
    /// Get the current root level.
    ///
    /// Decisions levels smaller or equal to the root level are not backtracked during solving.
    pub fn root_level(&self) -> u32 {
        unsafe { clingo_assignment_root_level(&self.0) }
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
    pub fn level(&self, literal: Literal) -> Result<u32, ClingoError> {
        let mut level = 0;
        if !unsafe { clingo_assignment_level(&self.0, literal.0, &mut level) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_level() failed",
            ));
        }
        Ok(level)
    }

    /// Determine the decision literal given a decision level.
    ///
    /// # Arguments
    ///
    /// * `level` - the level
    ///
    /// **Returns** the decision literal for the given decision level
    pub fn decision(&self, level: u32) -> Result<Literal, ClingoError> {
        let mut lit = 0 as clingo_literal_t;
        if !unsafe { clingo_assignment_decision(&self.0, level, &mut lit) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_decision() failed",
            ));
        }
        Ok(Literal(lit))
    }

    /// Check if a literal has a fixed truth value.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    ///
    /// **Returns** whether the literal is fixed
    pub fn is_fixed(&self, literal: Literal) -> Result<bool, ClingoError> {
        let mut is_fixed = false;
        if !unsafe { clingo_assignment_is_fixed(&self.0, literal.0, &mut is_fixed) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_is_fixed() failed",
            ));
        }
        Ok(is_fixed)
    }

    /// Check if a literal is true.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    /// **Returns** whether the literal is true (see [`Assignment::truth_value()`](struct.Assignment.html#method.truth_value))
    pub fn is_true(&self, literal: Literal) -> Result<bool, ClingoError> {
        let mut is_true = false;
        if !unsafe { clingo_assignment_is_true(&self.0, literal.0, &mut is_true) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_is_true() failed",
            ));
        }
        Ok(is_true)
    }

    /// Check if a literal has a fixed truth value.
    ///
    /// # Arguments
    /// * `literal` - the literal
    ///
    /// **Returns** whether the literal is false (see [`Assignment::truth_value()`](struct.Assignment.html#method.truth_value))
    pub fn is_false(&self, literal: Literal) -> Result<bool, ClingoError> {
        let mut is_false = false;
        if !unsafe { clingo_assignment_is_false(&self.0, literal.0, &mut is_false) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_is_false() failed",
            ));
        }
        Ok(is_false)
    }

    /// Determine the truth value of a given literal.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    /// * `value` - the resulting truth value
    ///
    /// **Returns** whether the call was successful
    pub fn truth_value(&self, literal: Literal) -> Result<TruthValue, ClingoError> {
        let mut value = 0;
        if !unsafe { clingo_assignment_truth_value(&self.0, literal.0, &mut value) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_truth_value() failed",
            ));
        }
        TruthValue::try_from(value)
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
    pub fn thread_id(&self) -> u32 {
        unsafe { clingo_propagate_control_thread_id(&self.0) }
    }

    /// Get the assignment associated with the underlying solver.
    pub fn assignment(&self) -> Result<&Assignment, ClingoError> {
        match unsafe {
            (clingo_propagate_control_assignment(&self.0) as *const Assignment).as_ref()
        } {
            Some(stm) => Ok(stm),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &Assignment.",
            }),
        }
    }

    /// Get the assignment associated with the underlying solver.
    pub fn assignment_mut(&self) -> Result<&mut Assignment, ClingoError> {
        match unsafe { (clingo_propagate_control_assignment(&self.0) as *mut Assignment).as_mut() }
        {
            Some(stm) => Ok(stm),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mut Assignment.",
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Logic`](enum.ErrorCode.html#variant.Logic) if the assignment is conflicting
    pub fn add_literal(&mut self, result: &mut Literal) -> Result<(), ClingoError> {
        if !unsafe { clingo_propagate_control_add_literal(&mut self.0, &mut result.0) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_control_add_literal() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Logic`](enum.ErrorCode.html#variant.Logic) if the literal is invalid
    ///
    /// **See:** [`PropagateControl::remove_watch()`](struct.PropagateControl.html#method.remove_watch)
    pub fn add_watch(&mut self, literal: Literal) -> Result<(), ClingoError> {
        if !unsafe { clingo_propagate_control_add_watch(&mut self.0, literal.0) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_control_add_watch() failed",
            ));
        }
        Ok(())
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn add_clause(
        &mut self,
        clause: &[Literal],
        ctype: ClauseType,
    ) -> Result<bool, ClingoError> {
        let mut result = false;
        if !unsafe {
            clingo_propagate_control_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                clause.len(),
                ctype as clingo_clause_type_t,
                &mut result,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_control_add_clause() failed",
            ));
        }
        Ok(result)
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn propagate(&mut self) -> Result<bool, ClingoError> {
        let mut result = false;
        if !unsafe { clingo_propagate_control_propagate(&mut self.0, &mut result) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_control_propagate() failed",
            ));
        }
        Ok(result)
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
    pub fn solver_literal(&self, Literal(aspif_literal): Literal) -> Result<Literal, ClingoError> {
        let mut solver_literal = 0 as clingo_literal_t;
        if !unsafe {
            clingo_propagate_init_solver_literal(&self.0, aspif_literal, &mut solver_literal)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_solver_literal() failed",
            ));
        }
        Ok(Literal(solver_literal))
    }

    /// Add a watch for the solver literal in the given phase.
    ///
    /// # Arguments
    ///
    /// * `solver_literal` - the solver literal
    pub fn add_watch(&mut self, Literal(solver_literal): Literal) -> Result<(), ClingoError> {
        if !unsafe { clingo_propagate_init_add_watch(&mut self.0, solver_literal) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_add_watch() failed",
            ));
        }
        Ok(())
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
    ) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_propagate_init_add_watch_to_thread(&mut self.0, solver_literal, thread_id)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_add_watch_to_thread() failed",
            ));
        }
        Ok(())
    }

    /// Get an object to inspect the symbolic atoms.
    pub fn symbolic_atoms(&self) -> Result<&SymbolicAtoms, ClingoError> {
        let mut atoms_ptr = std::ptr::null() as *const clingo_symbolic_atoms_t;
        if !unsafe { clingo_propagate_init_symbolic_atoms(&self.0, &mut atoms_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_symbolic_atoms() failed",
            ));
        }
        match unsafe { (atoms_ptr as *const SymbolicAtoms).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &SymbolicAtoms.",
            }),
        }
    }

    /// Get an object to inspect the theory atoms.
    pub fn theory_atoms(&self) -> Result<&TheoryAtoms, ClingoError> {
        let mut atoms_ptr = std::ptr::null() as *const clingo_theory_atoms_t;
        if !unsafe { clingo_propagate_init_theory_atoms(&self.0, &mut atoms_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_symbolic_atoms() failed",
            ));
        }
        match unsafe { (atoms_ptr as *const TheoryAtoms).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &TheoryAtoms.",
            }),
        }
    }

    /// Get the number of threads used in subsequent solving.
    ///
    /// **See:** [`PropagateControl::thread_id()`](struct.PropagateControl.html#method.thread_id)
    pub fn number_of_threads(&self) -> usize {
        (unsafe { clingo_propagate_init_number_of_threads(&self.0) }) as usize
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
    pub fn get_check_mode(&self) -> Result<PropagatorCheckMode, ClingoError> {
        PropagatorCheckMode::try_from(unsafe { clingo_propagate_init_get_check_mode(&self.0) })
    }

    /// Get the top level assignment solver.
    ///
    /// **Returns** the assignment
    pub fn assignment(&self) -> Result<&Assignment, ClingoError> {
        match unsafe { (clingo_propagate_init_assignment(&self.0) as *const Assignment).as_ref() } {
            Some(stm) => Ok(stm),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mut Assignment.",
            }),
        }
    }

    /// Add the given clause to the solver.
    ///
    /// This method sets its result to false if the clause is causing a conflict.
    ///
    /// # Arguments
    ///
    /// * `clause` - the clause to add
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn add_clause(&mut self, clause: &[Literal]) -> Result<bool, ClingoError> {
        let mut result = false;
        if !unsafe {
            clingo_propagate_init_add_clause(
                &mut self.0,
                clause.as_ptr() as *const clingo_literal_t,
                clause.len(),
                &mut result,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_add_clause() failed",
            ));
        }
        Ok(result)
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving fails
    pub fn get(&mut self) -> Result<SolveResult, ClingoError> {
        let mut result = 0;
        if !unsafe { clingo_solve_handle_get(self.theref, &mut result) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_get() failed",
            ));
        }
        if let Some(res) = SolveResult::from_bits(result) {
            Ok(res)
        } else {
            eprintln!("Unknown bitflag in clingo_solve_result: {}.", result);
            Err(ClingoError::FFIError {
                msg: "Unknown bitflag in clingo_solve_result.",
            })
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
    pub fn wait(&mut self, timeout: Duration) -> bool {
        let mut result = false;
        let timeout_secs = timeout.as_secs_f64();
        unsafe { clingo_solve_handle_wait(self.theref, timeout_secs, &mut result) }
        result
    }

    /// Get the next model or None if there are no more models.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving fails
    pub fn model(&mut self) -> Result<Option<&Model>, ClingoError> {
        let mut model = std::ptr::null_mut() as *const clingo_model_t;
        if !unsafe { clingo_solve_handle_model(self.theref, &mut model) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_model() failed",
            ));
        }
        Ok(unsafe { (model as *const Model).as_ref() })
    }

    /// Get the next model or None if there are no more models.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving fails
    pub fn model_mut(&mut self) -> Result<&mut Model, ClingoError> {
        let mut model = std::ptr::null_mut() as *const clingo_model_t;
        if !unsafe { clingo_solve_handle_model(self.theref, &mut model) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_model() failed",
            ));
        }
        match unsafe { (model as *mut Model).as_mut() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mut Model.",
            }),
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
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving fails
    pub fn resume(&mut self) -> Result<(), ClingoError> {
        if !unsafe { clingo_solve_handle_resume(self.theref) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_resume() failed",
            ));
        }
        Ok(())
    }

    /// Stop the running search and block until done.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving fails
    pub fn cancel(&mut self) -> Result<(), ClingoError> {
        if !unsafe { clingo_solve_handle_cancel(self.theref) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_cancel() failed",
            ));
        }
        Ok(())
    }

    /// Stops the running search and releases the handle.
    ///
    /// Blocks until the search is stopped (as if an implicit cancel was called before the handle is
    /// released).
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    /// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if solving fails
    pub fn close(self) -> Result<(), ClingoError> {
        if !unsafe { clingo_solve_handle_close(self.theref) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_close() failed",
            ));
        }
        Ok(())
    }
}

pub struct OptimalModels<'a>(SolveHandle<'a>);

impl<'a, 'b> Iterator for OptimalModels<'a> {
    type Item = MModel;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.0.resume().expect("Failed resume on solve handle.");
            match self.0.model() {
                Ok(Some(model)) => {
                    if model.optimality_proven().unwrap() {
                        let symbols = model.symbols(ShowType::SHOWN).unwrap();
                        let cost = model.cost().unwrap();
                        return Some(MModel {
                            symbols,
                            cost,
                            model_type: model.model_type().unwrap(),
                            number: model.number().unwrap(),
                        });
                    }
                }
                Ok(None) => {
                    return None;
                }
                Err(e) => panic!(e),
            }
        }
    }
}
pub struct AllModels<'a>(SolveHandle<'a>);
impl<'a, 'b> Iterator for AllModels<'a> {
    type Item = MModel;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.0.resume().expect("Failed resume on solve handle.");
            match self.0.model() {
                Ok(Some(model)) => {
                    let symbols = model.symbols(ShowType::SHOWN).unwrap();
                    let cost = model.cost().unwrap();
                    return Some(MModel {
                        symbols,
                        cost,
                        model_type: model.model_type().unwrap(),
                        number: model.number().unwrap(),
                    });
                }
                Ok(None) => {
                    return None;
                }
                Err(e) => panic!(e),
            }
        }
    }
}

pub struct MModel {
    pub symbols: Vec<Symbol>,
    pub cost: Vec<i64>,
    pub model_type: ModelType,
    pub number: u64,
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
/// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
/// - [`ClingoError::Utf8Error`](enum.ClingoError.html#variant.Utf8Error)
pub fn add_string(string: &str) -> Result<&'static str, ClingoError> {
    let in_cstr = CString::new(string)?;
    let mut out_ptr = std::ptr::null() as *const c_char;
    if !unsafe { clingo_add_string(in_cstr.as_ptr(), &mut out_ptr) } {
        return Err(ClingoError::new_internal(
            "Call to clingo_add_string() failed",
        ));
    }
    if out_ptr.is_null() {
        return Err(ClingoError::FFIError {
            msg: "clingo_add_string returned a null pointer.",
        });
    }
    let out_cstr = unsafe { CStr::from_ptr(out_ptr) };
    Ok(out_cstr.to_str()?)
}
fn internalize_string(string: &str) -> Result<*const c_char, ClingoError> {
    let in_cstr = CString::new(string)?;
    let mut out_ptr = std::ptr::null() as *const c_char;
    if !unsafe { clingo_add_string(in_cstr.as_ptr(), &mut out_ptr) } {
        return Err(ClingoError::new_internal(
            "Call to clingo_add_string() failed",
        ));
    }
    if out_ptr.is_null() {
        Err(ClingoError::FFIError {
            msg: "clingo_add_string returned a null pointer.",
        })?
    }
    Ok(out_ptr)
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
/// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `string` contains a nul byte
/// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
/// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if parsing fails
pub fn parse_term(string: &str) -> Result<Symbol, ClingoError> {
    let c_str = CString::new(string)?;
    let mut symbol = 0 as clingo_symbol_t;
    if !unsafe { clingo_parse_term(c_str.as_ptr(), None, std::ptr::null_mut(), 0, &mut symbol) } {
        return Err(ClingoError::new_internal(
            "Call to clingo_parse_term() failed",
        ));
    }
    Ok(Symbol(symbol))
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
/// - [`ClingoError::NulError`](enum.ClingoError.html#variant.NulError) - if `string` contains a nul byte
/// - [`ClingoError::InternalError`](enum.ClingoError.html#variant.InternalError) with [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
/// or [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) if parsing fails
pub fn parse_term_with_logger<L: Logger>(
    string: &str,
    logger: &mut L,
    message_limit: u32,
) -> Result<Symbol, ClingoError> {
    let c_str = CString::new(string)?;
    let logger = logger as *mut L;
    let mut symbol = 0 as clingo_symbol_t;
    if !unsafe {
        clingo_parse_term(
            c_str.as_ptr(),
            Some(L::unsafe_logging_callback::<L> as LoggingCallback),
            logger as *mut c_void,
            message_limit,
            &mut symbol,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_parse_term() failed",
        ));
    }
    Ok(Symbol(symbol))
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
        gpo: *mut c_void,
    ) -> bool {
        if let Some(gpo) = (gpo as *mut T).as_mut() {
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
    unsafe extern "C" fn unsafe_begin_step<T: GroundProgramObserver>(gpo: *mut c_void) -> bool {
        if let Some(gpo) = (gpo as *mut T).as_mut() {
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
    unsafe extern "C" fn unsafe_end_step<T: GroundProgramObserver>(gpo: *mut c_void) -> bool {
        if let Some(gpo) = (gpo as *mut T).as_mut() {
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (head_size > 0 && head.is_null()) | (body_size > 0 && body.is_null()) | gpo.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_rule() got a null pointer.");
            return false;
        }
        let head = std::slice::from_raw_parts(head as *const Atom, head_size);
        let body = std::slice::from_raw_parts(body as *const Literal, body_size);
        let gpo = &mut *(gpo as *mut T);

        gpo.rule(choice, head, body)
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (head_size > 0 && head.is_null()) | (body_size > 0 && body.is_null()) | gpo.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_weight_rule() got a null pointer.",
            );
            return false;
        }
        let head = std::slice::from_raw_parts(head as *const Atom, head_size);
        let body = std::slice::from_raw_parts(body as *const WeightedLiteral, body_size);
        let gpo = &mut *(gpo as *mut T);

        gpo.weight_rule(choice, head, lower_bound, body)
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && literals.is_null()) | gpo.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_minimize() got a null pointer.");
            return false;
        }
        let literals = std::slice::from_raw_parts(literals as *const WeightedLiteral, size);
        let gpo = &mut *(gpo as *mut T);

        gpo.minimize(priority, literals)
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && atoms.is_null()) | gpo.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_project() got a null pointer.");
            return false;
        }
        let atoms = std::slice::from_raw_parts(atoms as *const Atom, size);
        let gpo = &mut *(gpo as *mut T);

        gpo.project(atoms)
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
        gpo: *mut c_void,
    ) -> bool {
        if let Some(gpo) = (gpo as *mut T).as_mut() {
            gpo.output_atom(Symbol(symbol), Atom(atom))
        } else {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_output_atom() tried casting a null pointer to &mut GroundProgramObserver.",
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && condition.is_null()) | gpo.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_output_term() got a null pointer.",
            );
            return false;
        }
        let condition = std::slice::from_raw_parts(condition as *const Literal, size);
        let gpo = &mut *(gpo as *mut T);

        gpo.output_term(Symbol(symbol), condition)
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && condition.is_null()) | gpo.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_output_csp() got a null pointer.",
            );
            return false;
        }
        let condition = std::slice::from_raw_parts(condition as *const Literal, size);
        let gpo = &mut *(gpo as *mut T);

        gpo.output_csp(Symbol(symbol), value, condition)
    }

    /// Observe external statements passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `atom` - the external atom
    /// * `etype` - the type of the external statement
    ///
    /// **Returns** whether the call was successful
    fn external(&mut self, atom: Atom, type_: ExternalType) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_external<T: GroundProgramObserver>(
        atom: clingo_atom_t,
        etype: clingo_external_type_t,
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if gpo.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_external() got a null pointer.");
            return false;
        }
        let gpo = &mut *(gpo as *mut T);

        match ExternalType::try_from(etype) {
            Err(e) => {
                eprintln!("Error in unsafe_external(): {}.", e);
                set_internal_error(ErrorType::Runtime, "Error in unsafe_external().");
                false
            }
            Ok(etype) => gpo.external(Atom(atom), etype),
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && literals.is_null()) | gpo.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_assume() got a null pointer.");
            return false;
        }
        let literals = std::slice::from_raw_parts(literals as *const Literal, size);
        let gpo = &mut *(gpo as *mut T);

        gpo.assume(literals)
    }

    /// Observe heuristic directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `atom` - the target atom
    /// * `htype` - the type of the heuristic modification
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
        htype: clingo_heuristic_type_t,
        bias: ::std::os::raw::c_int,
        priority: ::std::os::raw::c_uint,
        condition: *const clingo_literal_t,
        size: usize,
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && condition.is_null()) | gpo.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_heuristic() got a null pointer.");
            return false;
        }
        let condition = std::slice::from_raw_parts(condition as *const Literal, size);
        let gpo = &mut *(gpo as *mut T);

        match HeuristicType::try_from(htype) {
            Err(e) => {
                eprintln!("Error in unsafe_heuristic(): {}.", e);
                set_internal_error(ErrorType::Runtime, "Error in unsafe_heuristic().");
                false
            }
            Ok(htype) => gpo.heuristic(Atom(atom), htype, bias, priority, condition),
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && condition.is_null()) | gpo.is_null() {
            set_internal_error(ErrorType::Runtime, "unsafe_heuristic() got a null pointer.");
            return false;
        }
        let condition = std::slice::from_raw_parts(condition as *const Literal, size);
        let gpo = &mut *(gpo as *mut T);

        gpo.acyc_edge(node_u, node_v, condition)
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
        gpo: *mut c_void,
    ) -> bool {
        if let Some(gpo) = (gpo as *mut T).as_mut() {
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
        name: *const c_char,
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if name.is_null() | gpo.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_theory_term_string() got a null pointer.",
            );
            return false;
        }
        let name = CStr::from_ptr(name);
        let gpo = &mut *(gpo as *mut T);

        match name.to_str() {
            Ok(name) => gpo.theory_term_string(Id(term_id), name),
            Err(e) => {
                eprintln!("Utf8Error in unsafe theory_term_string: {}", e);
                set_internal_error(
                    ErrorType::Runtime,
                    "Utf8Error in unsafe theory_term_string.",
                );
                false
            }
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && arguments.is_null()) | gpo.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_theory_term_compound() got a null pointer.",
            );
            return false;
        }
        let arguments = std::slice::from_raw_parts(arguments as *const Id, size);
        let gpo = &mut *(gpo as *mut T);

        gpo.theory_term_compound(Id(term_id), name_id_or_type, arguments)
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (terms_size > 0 && terms.is_null())
            | (condition_size > 0 && condition.is_null())
            | gpo.is_null()
        {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_theory_element() got a null pointer.",
            );
            return false;
        }
        let terms = std::slice::from_raw_parts(terms as *const Id, terms_size);
        let condition = std::slice::from_raw_parts(condition as *const Literal, condition_size);
        let gpo = &mut *(gpo as *mut T);

        gpo.theory_element(Id(element_id), terms, condition)
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && elements.is_null()) | gpo.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_theory_atom() got a null pointer.",
            );
            return false;
        }
        let elements = std::slice::from_raw_parts(elements as *const Id, size);
        let gpo = &mut *(gpo as *mut T);

        gpo.theory_atom(Id(atom_id_or_zero), Id(term_id), elements)
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
        gpo: *mut c_void,
    ) -> bool {
        // check for null pointers
        if (size > 0 && elements.is_null()) | gpo.is_null() {
            set_internal_error(
                ErrorType::Runtime,
                "unsafe_theory_atom_with_guard() got a null pointer.",
            );
            return false;
        }
        let elements = std::slice::from_raw_parts(elements as *const Id, size);
        let gpo = &mut *(gpo as *mut T);

        gpo.theory_atom_with_guard(
            Id(atom_id_or_zero),
            Id(term_id),
            elements,
            Id(operator_id),
            Id(right_hand_side_id),
        )
    }
}

/// helper types and traits to simplify conversion from structs to clingo symbols

pub trait ToSymbol {
    fn symbol(&self) -> Result<Symbol, ClingoError>;
}

impl ToSymbol for Symbol {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(*self)
    }
}

// Due to a temporary restriction in Rust's type system, these function are only implemented on tuples of arity 12 or less.
// In the future, this may change.
impl ToSymbol for () {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Symbol::create_function("", &[], true)
    }
}
impl<A: ToSymbol, B: ToSymbol> ToSymbol for (A, B) {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Symbol::create_function("", &[self.0.symbol()?, self.1.symbol()?], true)
    }
}
impl<A: ToSymbol, B: ToSymbol, C: ToSymbol> ToSymbol for (A, B, C) {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}
impl<A: ToSymbol, B: ToSymbol, C: ToSymbol, D: ToSymbol> ToSymbol for (A, B, C, D) {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}

impl<A: ToSymbol, B: ToSymbol, C: ToSymbol, D: ToSymbol, E: ToSymbol> ToSymbol for (A, B, C, D, E) {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}
impl<A: ToSymbol, B: ToSymbol, C: ToSymbol, D: ToSymbol, E: ToSymbol, F: ToSymbol> ToSymbol
    for (A, B, C, D, E, F)
{
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}
impl<A: ToSymbol, B: ToSymbol, C: ToSymbol, D: ToSymbol, E: ToSymbol, F: ToSymbol, G: ToSymbol>
    ToSymbol for (A, B, C, D, E, F, G)
{
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}
impl<
        A: ToSymbol,
        B: ToSymbol,
        C: ToSymbol,
        D: ToSymbol,
        E: ToSymbol,
        F: ToSymbol,
        G: ToSymbol,
        H: ToSymbol,
    > ToSymbol for (A, B, C, D, E, F, G, H)
{
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}

impl<
        A: ToSymbol,
        B: ToSymbol,
        C: ToSymbol,
        D: ToSymbol,
        E: ToSymbol,
        F: ToSymbol,
        G: ToSymbol,
        H: ToSymbol,
        I: ToSymbol,
    > ToSymbol for (A, B, C, D, E, F, G, H, I)
{
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        tempvec.push(self.8.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}
impl<
        A: ToSymbol,
        B: ToSymbol,
        C: ToSymbol,
        D: ToSymbol,
        E: ToSymbol,
        F: ToSymbol,
        G: ToSymbol,
        H: ToSymbol,
        I: ToSymbol,
        J: ToSymbol,
    > ToSymbol for (A, B, C, D, E, F, G, H, I, J)
{
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        tempvec.push(self.8.symbol()?);
        tempvec.push(self.9.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}
impl<
        A: ToSymbol,
        B: ToSymbol,
        C: ToSymbol,
        D: ToSymbol,
        E: ToSymbol,
        F: ToSymbol,
        G: ToSymbol,
        H: ToSymbol,
        I: ToSymbol,
        J: ToSymbol,
        K: ToSymbol,
    > ToSymbol for (A, B, C, D, E, F, G, H, I, J, K)
{
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        tempvec.push(self.8.symbol()?);
        tempvec.push(self.9.symbol()?);
        tempvec.push(self.10.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}
impl<
        A: ToSymbol,
        B: ToSymbol,
        C: ToSymbol,
        D: ToSymbol,
        E: ToSymbol,
        F: ToSymbol,
        G: ToSymbol,
        H: ToSymbol,
        I: ToSymbol,
        J: ToSymbol,
        K: ToSymbol,
        L: ToSymbol,
    > ToSymbol for (A, B, C, D, E, F, G, H, I, J, K, L)
{
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        tempvec.push(self.8.symbol()?);
        tempvec.push(self.9.symbol()?);
        tempvec.push(self.10.symbol()?);
        tempvec.push(self.11.symbol()?);
        Symbol::create_function("", &tempvec, true)
    }
}
impl ToSymbol for bool {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        if *self {
            Symbol::create_id("true", true)
        } else {
            Symbol::create_id("false", true)
        }
    }
}
impl ToSymbol for u8 {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(Symbol::create_number(i32::from(*self)))
    }
}
impl ToSymbol for i8 {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(Symbol::create_number(i32::from(*self)))
    }
}
impl ToSymbol for u16 {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(Symbol::create_number(i32::from(*self)))
    }
}
impl ToSymbol for i16 {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(Symbol::create_number(i32::from(*self)))
    }
}
impl ToSymbol for u32 {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(Symbol::create_number(*self as i32))
    }
}
impl ToSymbol for i32 {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(Symbol::create_number(*self))
    }
}
impl ToSymbol for String {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Symbol::create_string(self)
    }
}
impl ToSymbol for str {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Symbol::create_string(self)
    }
}
impl<T: ToSymbol> ToSymbol for &T {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        (*self).symbol()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FactBase {
    facts: HashSet<Symbol>,
}
impl FactBase {
    pub fn new() -> FactBase {
        FactBase {
            facts: HashSet::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.facts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }
    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, Symbol> {
        self.facts.iter()
    }
    pub fn insert(&mut self, fact: &dyn ToSymbol) {
        self.facts.insert(fact.symbol().unwrap());
        // self.facts.sort();
    }
    pub fn union(&mut self, facts: &FactBase) {
        for s in &facts.facts {
            self.facts.insert(s.clone());
        }
    }
    pub fn print(&self) {
        for fact in &self.facts {
            print!("{}.", fact.to_string().unwrap());
        }
        println!();
    }
}

// Re-export #[derive(ToSymbol)].
#[cfg(feature = "derive")]
#[allow(unused_imports)]
pub use clingo_derive::*;
