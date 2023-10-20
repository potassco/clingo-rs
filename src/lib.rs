#![doc(html_root_url = "https://docs.rs/clingo/0.8.0")]
#![doc = include_str!("../README.md")]
#![allow(non_upper_case_globals)]
#![allow(clippy::try_err)]
use bitflags::bitflags;
use clingo_sys::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::Infallible;
use std::convert::TryInto;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::NulError;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::str::Utf8Error;
use std::time::Duration;
use thiserror::Error;

/// Functions and data structures to work with program ASTs.
pub mod ast;
mod ast_internals;

pub mod theory;

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

    fn new_external(msg: &'static str) -> ClingoError {
        ExternalError { msg }.into()
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
    Success = clingo_error_e_clingo_error_success as isize,
    /// Errors only detectable at runtime like invalid input
    Runtime = clingo_error_e_clingo_error_runtime as isize,
    /// Wrong usage of the clingo API
    Logic = clingo_error_e_clingo_error_logic as isize,
    /// Memory could not be allocated
    BadAlloc = clingo_error_e_clingo_error_bad_alloc as isize,
    /// Errors unrelated to clingo
    Unknown = clingo_error_e_clingo_error_unknown as isize,
}
/// Enumeration of clingo error codes for [`ClingoError::InternalError`].
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
            clingo_error_e_clingo_error_success => ErrorCode::Success,
            clingo_error_e_clingo_error_runtime => ErrorCode::Runtime,
            clingo_error_e_clingo_error_logic => ErrorCode::Logic,
            clingo_error_e_clingo_error_bad_alloc => ErrorCode::BadAlloc,
            clingo_error_e_clingo_error_unknown => ErrorCode::Unknown,
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
/// - [`ClingoError::NulError`] - if `message` contains a nul byte
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

/// Object to add command-line options.
pub struct Options(clingo_options_t);

/// Represents three-valued truth values.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TruthValue {
    /// No truth value
    Free = clingo_truth_value_e_clingo_truth_value_free as isize,
    /// True
    True = clingo_truth_value_e_clingo_truth_value_true as isize,
    /// False
    False = clingo_truth_value_e_clingo_truth_value_false as isize,
}
impl TruthValue {
    fn try_from(code: u32) -> Result<TruthValue, ClingoError> {
        match code {
            clingo_truth_value_e_clingo_truth_value_false => Ok(TruthValue::False),
            clingo_truth_value_e_clingo_truth_value_true => Ok(TruthValue::True),
            clingo_truth_value_e_clingo_truth_value_free => Ok(TruthValue::Free),
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
    Learnt = clingo_clause_type_e_clingo_clause_type_learnt as isize,
    /// The clause is not subject to the solvers deletion policy
    Static = clingo_clause_type_e_clingo_clause_type_static as isize,
    /// Like `Learnt` but the clause is deleted after a solving step
    Volatile = clingo_clause_type_e_clingo_clause_type_volatile as isize,
    /// Like `Static` but the clause is deleted after a solving step
    VolatileStatic = clingo_clause_type_e_clingo_clause_type_volatile_static as isize,
}
/// Enumeration of solve events.
#[derive(Debug)]
enum SolveEventType {
    /// Issued if a model is found.
    Model = clingo_solve_event_type_e_clingo_solve_event_type_model as isize,
    /// Issued if an optimization problem is found unsatisfiable.
    Unsat = clingo_solve_event_type_e_clingo_solve_event_type_unsat as isize,
    /// Issued when the statistics can be updated.
    Statistics = clingo_solve_event_type_e_clingo_solve_event_type_statistics as isize,
    /// Issued if the search has completed.
    Finish = clingo_solve_event_type_e_clingo_solve_event_type_finish as isize,
}
impl SolveEventType {
    fn try_from(code: u32) -> Result<SolveEventType, ClingoError> {
        match code {
            clingo_solve_event_type_e_clingo_solve_event_type_model => Ok(SolveEventType::Model),
            clingo_solve_event_type_e_clingo_solve_event_type_unsat => Ok(SolveEventType::Unsat),
            clingo_solve_event_type_e_clingo_solve_event_type_statistics => {
                Ok(SolveEventType::Statistics)
            }
            clingo_solve_event_type_e_clingo_solve_event_type_finish => Ok(SolveEventType::Finish),
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
/// Enumeration of solve events.
#[derive(Debug)]
pub enum SolveEvent<'a> {
    /// Issued if a model is found.
    Model(&'a mut Model),
    /// Issued if an optimization problem is found unsatisfiable.
    Unsat,
    /// Issued when the statistics can be updated.
    Statistics {
        step: &'a mut Statistics,
        akku: &'a mut Statistics,
    },
    /// Issued if the search has completed.
    Finish(&'a mut SolveResult),
}

/// Enumeration for entries of the statistics.
#[derive(Debug, Copy, Clone)]
pub enum StatisticsType {
    /// The entry is invalid (has neither of the types below)
    Empty = clingo_statistics_type_e_clingo_statistics_type_empty as isize,
    /// The entry is a (double) value
    Value = clingo_statistics_type_e_clingo_statistics_type_value as isize,
    /// The entry is an array
    Array = clingo_statistics_type_e_clingo_statistics_type_array as isize,
    /// The entry is a map
    Map = clingo_statistics_type_e_clingo_statistics_type_map as isize,
}
impl StatisticsType {
    fn try_from(code: u32) -> Result<StatisticsType, ClingoError> {
        match code {
            clingo_statistics_type_e_clingo_statistics_type_empty => Ok(StatisticsType::Empty),
            clingo_statistics_type_e_clingo_statistics_type_value => Ok(StatisticsType::Value),
            clingo_statistics_type_e_clingo_statistics_type_array => Ok(StatisticsType::Array),
            clingo_statistics_type_e_clingo_statistics_type_map => Ok(StatisticsType::Map),
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
    Infimum = clingo_symbol_type_e_clingo_symbol_type_infimum as isize,
    /// A numeric symbol, e.g., `1`
    Number = clingo_symbol_type_e_clingo_symbol_type_number as isize,
    /// A string symbol, e.g., `"a"`
    String = clingo_symbol_type_e_clingo_symbol_type_string as isize,
    /// A numeric symbol, e.g., `c`, `(1, "a")`, or `f(1,"a")`
    Function = clingo_symbol_type_e_clingo_symbol_type_function as isize,
    /// The `#sup` symbol
    Supremum = clingo_symbol_type_e_clingo_symbol_type_supremum as isize,
}
impl SymbolType {
    fn try_from(code: u32) -> Result<SymbolType, ClingoError> {
        match code {
            clingo_symbol_type_e_clingo_symbol_type_infimum => Ok(SymbolType::Infimum),
            clingo_symbol_type_e_clingo_symbol_type_number => Ok(SymbolType::Number),
            clingo_symbol_type_e_clingo_symbol_type_string => Ok(SymbolType::String),
            clingo_symbol_type_e_clingo_symbol_type_function => Ok(SymbolType::Function),
            clingo_symbol_type_e_clingo_symbol_type_supremum => Ok(SymbolType::Supremum),
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
    OperationUndefined = clingo_warning_e_clingo_warning_operation_undefined as isize,
    /// To report multiple errors; a corresponding runtime error is raised later
    RuntimeError = clingo_warning_e_clingo_warning_runtime_error as isize,
    /// An undefined atom in program
    AtomUndefined = clingo_warning_e_clingo_warning_atom_undefined as isize,
    /// The Same file included multiple times
    FileIncluded = clingo_warning_e_clingo_warning_file_included as isize,
    /// CSP variable with unbounded domain
    VariableUnbound = clingo_warning_e_clingo_warning_variable_unbounded as isize,
    /// A global variable in tuple of aggregate element
    GlobalVariable = clingo_warning_e_clingo_warning_global_variable as isize,
    /// Other kinds of warnings
    Other = clingo_warning_e_clingo_warning_other as isize,
}
impl Warning {
    fn try_from(code: u32) -> Result<Warning, ClingoError> {
        match code {
            clingo_warning_e_clingo_warning_atom_undefined => Ok(Warning::AtomUndefined),
            clingo_warning_e_clingo_warning_file_included => Ok(Warning::FileIncluded),
            clingo_warning_e_clingo_warning_global_variable => Ok(Warning::GlobalVariable),
            clingo_warning_e_clingo_warning_operation_undefined => Ok(Warning::OperationUndefined),
            clingo_warning_e_clingo_warning_other => Ok(Warning::Other),
            clingo_warning_e_clingo_warning_runtime_error => Ok(Warning::RuntimeError),
            clingo_warning_e_clingo_warning_variable_unbounded => Ok(Warning::VariableUnbound),
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
    Free = clingo_external_type_e_clingo_external_type_free as isize,
    /// Assign an external to true
    True = clingo_external_type_e_clingo_external_type_true as isize,
    /// Assign an external to false
    False = clingo_external_type_e_clingo_external_type_false as isize,
    /// No longer treat an atom as external
    Release = clingo_external_type_e_clingo_external_type_release as isize,
}
impl ExternalType {
    fn try_from(code: u32) -> Result<ExternalType, ClingoError> {
        match code {
            clingo_external_type_e_clingo_external_type_false => Ok(ExternalType::False),
            clingo_external_type_e_clingo_external_type_free => Ok(ExternalType::Free),
            clingo_external_type_e_clingo_external_type_release => Ok(ExternalType::Release),
            clingo_external_type_e_clingo_external_type_true => Ok(ExternalType::True),
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
    Level = clingo_heuristic_type_e_clingo_heuristic_type_level as isize,
    /// Configure which sign to chose for an atom
    Sign = clingo_heuristic_type_e_clingo_heuristic_type_sign as isize,
    /// Modify VSIDS factor of an atom
    Factor = clingo_heuristic_type_e_clingo_heuristic_type_factor as isize,
    /// Modify the initial VSIDS score of an atom
    Init = clingo_heuristic_type_e_clingo_heuristic_type_init as isize,
    /// Set the level of an atom and choose a positive sign
    True = clingo_heuristic_type_e_clingo_heuristic_type_true as isize,
    /// Set the level of an atom and choose a negative sign
    False = clingo_heuristic_type_e_clingo_heuristic_type_false as isize,
}
impl HeuristicType {
    fn try_from(code: u32) -> Result<HeuristicType, ClingoError> {
        match code {
            clingo_heuristic_type_e_clingo_heuristic_type_factor => Ok(HeuristicType::Factor),
            clingo_heuristic_type_e_clingo_heuristic_type_false => Ok(HeuristicType::False),
            clingo_heuristic_type_e_clingo_heuristic_type_init => Ok(HeuristicType::Init),
            clingo_heuristic_type_e_clingo_heuristic_type_level => Ok(HeuristicType::Level),
            clingo_heuristic_type_e_clingo_heuristic_type_sign => Ok(HeuristicType::Sign),
            clingo_heuristic_type_e_clingo_heuristic_type_true => Ok(HeuristicType::True),
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
    Tuple = clingo_theory_term_type_e_clingo_theory_term_type_tuple as isize,
    /// A list term, e.g., `[1,2,3]`
    List = clingo_theory_term_type_e_clingo_theory_term_type_list as isize,
    /// A set term, e.g., `{1,2,3}`
    Set = clingo_theory_term_type_e_clingo_theory_term_type_set as isize,
    /// A function term, e.g., `f(1,2,3)`
    Function = clingo_theory_term_type_e_clingo_theory_term_type_function as isize,
    /// A number term, e.g., `42`
    Number = clingo_theory_term_type_e_clingo_theory_term_type_number as isize,
    /// A symbol term, e.g., `c`
    Symbol = clingo_theory_term_type_e_clingo_theory_term_type_symbol as isize,
}
impl TheoryTermType {
    fn try_from(code: u32) -> Result<TheoryTermType, ClingoError> {
        match code {
            clingo_theory_term_type_e_clingo_theory_term_type_tuple => Ok(TheoryTermType::Tuple),
            clingo_theory_term_type_e_clingo_theory_term_type_list => Ok(TheoryTermType::List),
            clingo_theory_term_type_e_clingo_theory_term_type_set => Ok(TheoryTermType::Set),
            clingo_theory_term_type_e_clingo_theory_term_type_function => {
                Ok(TheoryTermType::Function)
            }
            clingo_theory_term_type_e_clingo_theory_term_type_number => Ok(TheoryTermType::Number),
            clingo_theory_term_type_e_clingo_theory_term_type_symbol => Ok(TheoryTermType::Symbol),
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
    StableModel = clingo_model_type_e_clingo_model_type_stable_model as isize,
    /// The model represents a set of brave consequences.
    BraveConsequences = clingo_model_type_e_clingo_model_type_brave_consequences as isize,
    /// The model represents a set of cautious consequences.
    CautiousConsequences = clingo_model_type_e_clingo_model_type_cautious_consequences as isize,
}
impl ModelType {
    fn try_from(code: u32) -> Result<ModelType, ClingoError> {
        match code {
            clingo_model_type_e_clingo_model_type_stable_model => Ok(ModelType::StableModel),
            clingo_model_type_e_clingo_model_type_brave_consequences => {
                Ok(ModelType::BraveConsequences)
            }
            clingo_model_type_e_clingo_model_type_cautious_consequences => {
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
/// Note that total checks are subject to the lock when a model is found.
/// This means that information from previously found models can be used to discard assignments in check calls.
#[derive(Debug, Copy, Clone)]
pub enum PropagatorCheckMode {
    /// Do not call [`Propagator::check()`] at all
    None = clingo_propagator_check_mode_e_clingo_propagator_check_mode_none as isize,
    /// Call [`Propagator::check()`] on total assignments
    Total = clingo_propagator_check_mode_e_clingo_propagator_check_mode_total as isize,
    /// Call [`Propagator::check()`] on propagation fixpoints
    Fixpoint = clingo_propagator_check_mode_e_clingo_propagator_check_mode_fixpoint as isize,
    /// Call [`Propagator::check()`] on propagation fixpoints and total assignments
    Both = clingo_propagator_check_mode_e_clingo_propagator_check_mode_both as isize,
}
impl PropagatorCheckMode {
    fn try_from(code: u32) -> Result<PropagatorCheckMode, ClingoError> {
        match code {
            clingo_propagator_check_mode_e_clingo_propagator_check_mode_fixpoint => {
                Ok(PropagatorCheckMode::Fixpoint)
            }
            clingo_propagator_check_mode_e_clingo_propagator_check_mode_total => {
                Ok(PropagatorCheckMode::Total)
            }
            clingo_propagator_check_mode_e_clingo_propagator_check_mode_none => {
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
/// Enumeration of weight_constraint_types.
#[derive(Debug, Copy, Clone)]
pub enum WeigthConstraintType {
    /// The weight constraint implies the literal
    ImplicationLeft =
        clingo_weight_constraint_type_e_clingo_weight_constraint_type_implication_left as isize,
    /// The literal implies the weight constraint
    ImplicationRight =
        clingo_weight_constraint_type_e_clingo_weight_constraint_type_implication_right as isize,
    /// The weight constraint is equivalent to the literal
    Equivalence =
        clingo_weight_constraint_type_e_clingo_weight_constraint_type_equivalence as isize,
}

bitflags! {
    /// Bit flags describing the entries of a configuration.
    pub struct ConfigurationType: u32 {
        /// The entry is a (string) value.
        const VALUE =
            clingo_configuration_type_e_clingo_configuration_type_value;
        /// The entry is an array.
        const ARRAY =
            clingo_configuration_type_e_clingo_configuration_type_array;
        /// The entry is a map.
        const MAP =
            clingo_configuration_type_e_clingo_configuration_type_map;
    }
}
bitflags! {
    /// Bit flags describing solve modes.
    pub struct SolveMode: u32 {
        /// Enable non-blocking search.
        const ASYNC = clingo_solve_mode_e_clingo_solve_mode_async;
        /// Yield models in calls to clingo_solve_handle_model.
        const YIELD = clingo_solve_mode_e_clingo_solve_mode_yield;
    }
}
bitflags! {
    /// Bit flags to select symbols in models.
    pub struct ShowType: u32 {
        /// Select symbols added by theory.
        const THEORY = clingo_show_type_e_clingo_show_type_theory;
        /// Select shown atoms and terms.
        const SHOWN = clingo_show_type_e_clingo_show_type_shown;
        /// Select all atoms.
        const ATOMS = clingo_show_type_e_clingo_show_type_atoms;
        /// Select all terms.
        const TERMS = clingo_show_type_e_clingo_show_type_terms;
        /// Select everything.
        const ALL = clingo_show_type_e_clingo_show_type_all;
        /// Select false instead of true atoms (Atoms) or terms (Terms)."
        const COMPLEMENT = clingo_show_type_e_clingo_show_type_complement;
    }
}
bitflags! {
    #[derive(Debug)]
    /// Bit flags that describes the result of a solve call.
    pub struct SolveResult: u32 {
        /// The problem is satisfiable.
        const SATISFIABLE = clingo_solve_result_e_clingo_solve_result_satisfiable;
        /// The problem is unsatisfiable.
        const UNSATISFIABLE =
            clingo_solve_result_e_clingo_solve_result_unsatisfiable;
        /// The search space was exhausted.
        const EXHAUSTED = clingo_solve_result_e_clingo_solve_result_exhausted;
        /// The search was interupted.
        const INTERRUPTED = clingo_solve_result_e_clingo_solve_result_interrupted;
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
    /// * `event` - the solve event
    /// * `goon` - can be set to false to stop solving
    ///
    /// **Returns** whether the call was successful
    ///
    /// **See:** [`Control::solve()`]
    fn on_solve_event(&mut self, _event: SolveEvent, _goon: &mut bool) -> bool {
        true
    }
}
unsafe extern "C" fn unsafe_solve_callback<T: SolveEventHandler>(
    event_type: clingo_solve_event_type_t,
    event_data: *mut c_void,
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

    match SolveEventType::try_from(event_type) {
        Ok(SolveEventType::Model) => {
            let model = &mut *(event_data as *mut Model);

            let event = SolveEvent::Model(model);

            event_handler.on_solve_event(event, goon)
        }
        Ok(SolveEventType::Unsat) => {
            let event = SolveEvent::Unsat;

            event_handler.on_solve_event(event, goon)
        }
        Ok(SolveEventType::Statistics) => {
            // check for null pointers
            if event_data.is_null() {
                set_internal_error(
                    ErrorType::Runtime,
                    "unsafe_solve_callback() got a null pointer event_data.",
                );
                return false;
            }
            let stats: &mut [&mut Statistics] =
                std::slice::from_raw_parts_mut(event_data as *mut &mut Statistics, 2);
            let stats: &mut [&mut Statistics; 2] =
                stats.try_into().expect("slice has more than two items");
            let stats = stats.split_at_mut(1);
            let event = SolveEvent::Statistics {
                step: stats.0[0],
                akku: stats.1[0],
            };
            event_handler.on_solve_event(event, goon)
        }
        Ok(SolveEventType::Finish) => {
            // check for null pointers
            if event_data.is_null() {
                set_internal_error(
                    ErrorType::Runtime,
                    "unsafe_solve_callback() got a null pointer event_data.",
                );
                return false;
            }
            let solve_result = &mut *(event_data as *mut SolveResult);
            let event = SolveEvent::Finish(solve_result);
            event_handler.on_solve_event(event, goon)
        }
        Err(e) => {
            eprintln!("{}", e);
            // from the libclingo docs:
            // If a (non-recoverable) clingo API function fails in this callback, it must return false.
            // In case of errors not related to clingo, set error code ErrorType::Unknown and return false to stop solving with an error.
            set_internal_error(
                ErrorType::Runtime,
                "Error in unsafe_solve_callback(): unknown event_type.",
            );
            false
        }
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
    /// * [`ControlCtx::L`]
    /// * [`parse_term_with_logger()`]
    fn log(&mut self, code: Warning, message: &str) {
        eprintln!("warn {:?}: {}", code, message);
    }
}

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

    if let Err(e) = try_logging_callback(logger, code, message) {
        eprintln!("Error in unsafe_logging_callback(): {}.", e);
        set_internal_error(ErrorType::Runtime, "Error in unsafe_logging_callback().");
    }
}
fn try_logging_callback<L: Logger>(
    logger: &mut L,
    code: clingo_warning_t,
    message: &CStr,
) -> Result<(), ClingoError> {
    let code = Warning::try_from(code as u32)?;
    let message = message.to_str()?;
    logger.log(code, message);
    Ok(())
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
pub trait FunctionHandler {
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
    /// **See:** [`ControlCtx::F`]
    ///
    /// The following example implements the external function `@f()` returning 42.
    /// ```ignore
    /// fn on_external_function(
    ///     &mut self,
    ///     _location: &ast::Location,
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
        location: &ast::Location,
        name: &str,
        arguments: &[Symbol],
    ) -> Result<Vec<Symbol>, ExternalError>;
}
unsafe extern "C" fn unsafe_ground_callback<T: FunctionHandler>(
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
    let location = &*(location as *const ast::Location);
    let name = CStr::from_ptr(name);
    let arguments = std::slice::from_raw_parts(arguments as *const Symbol, arguments_size);
    let event_handler = &mut *(event_handler as *mut T);

    match try_symbol_callback(
        event_handler,
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
unsafe fn try_symbol_callback(
    efh: &mut dyn FunctionHandler,
    location: &ast::Location,
    name: &CStr,
    arguments: &[Symbol],
    symbol_callback: clingo_symbol_callback_t,
    symbol_callback_data: *mut c_void,
) -> Result<bool, ClingoError> {
    let name = name.to_str()?;
    let symbols = efh.on_external_function(location, name, arguments)?;
    if let Some(symbol_callback) = symbol_callback {
        let v: Vec<clingo_symbol_t> = symbols.iter().map(|symbol| symbol.0).collect();
        Ok(symbol_callback(
            v.as_slice().as_ptr(),
            v.len(),
            symbol_callback_data,
        ))
    } else {
        // no symbol callback
        Ok(true)
    }
}
// /// Signed integer type used for aspif.
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub struct Literal(clingo_literal_t);
// impl Literal {
//     pub fn negate(self) -> Literal {
//         Literal(-(self.0))
//     }
//     pub fn from(Atom(atom): Atom) -> Literal {
//         Literal(atom as clingo_literal_t)
//     }
//     pub fn get_integer(self) -> i32 {
//         self.0
//     }
// }
/// Signed integer type used for solver literals.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SolverLiteral(clingo_literal_t);
impl SolverLiteral {
    pub fn negate(self) -> SolverLiteral {
        SolverLiteral(-(self.0))
    }
    // TODO: remove get_integer ?
    pub fn get_integer(self) -> i32 {
        self.0
    }
}
impl From<Atom> for SolverLiteral {
    fn from(atom: Atom) -> Self {
        SolverLiteral(atom.0 as i32)
    }
}
/// Unsigned integer type used for aspif atoms.
#[derive(Debug, Copy, Clone)]
pub struct Atom(clingo_atom_t);
impl From<SolverLiteral> for Atom {
    fn from(literal: SolverLiteral) -> Self {
        Atom(literal.0 as u32)
    }
}
/// Unsigned integer type used in various places.
#[derive(Debug, Copy, Clone)]
pub struct Id(clingo_id_t);
impl Id {
    pub fn get_integer(self) -> u32 {
        self.0
    }
}
impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}
/// A Literal with an associated weight.
#[derive(Debug, Copy, Clone)]
pub struct WeightedLiteral(clingo_weighted_literal);
impl WeightedLiteral {
    pub fn literal(self) -> SolverLiteral {
        SolverLiteral(self.0.literal)
    }
    pub fn weight(self) -> i32 {
        self.0.weight
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
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
    /// **Note:**
    /// The string is internalized and valid for the duration of the process.
    pub fn name(&self) -> Result<&'static str, Utf8Error> {
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
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut size: usize = 0;
        if !unsafe { clingo_symbol_to_string_size(self.0, &mut size) } {
            eprintln!("Call to clingo_symbol_to_string_size() failed");
            return Err(fmt::Error);
        }
        let mut string = Vec::with_capacity(size);
        let string_ptr = string.as_mut_ptr();
        if !unsafe { clingo_symbol_to_string(self.0, string_ptr, size) } {
            eprintln!("Call to clingo_symbol_to_string() failed");
            return Err(fmt::Error);
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(string_ptr) };
        let str_slice: &str = match c_str.to_str() {
            Ok(slice) => slice,
            Err(e) => {
                eprintln!("{}", e);
                return Err(fmt::Error);
            }
        };
        std::fmt::Display::fmt(str_slice, f)
    }
}
impl Symbol {
    /// Construct a symbol representing a number.
    pub fn create_number(number: i32) -> Symbol {
        let mut symbol = 0;
        unsafe { clingo_symbol_create_number(number, &mut symbol) };
        Symbol(symbol)
    }

    /// Construct a symbol representing \#sup.
    pub fn create_supremum() -> Symbol {
        let mut symbol = 0;
        unsafe { clingo_symbol_create_supremum(&mut symbol) };
        Symbol(symbol)
    }

    /// Construct a symbol representing \#inf
    pub fn create_infimum() -> Symbol {
        let mut symbol = 0;
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
    /// - [`ClingoError::NulError`] - if `string` contains a nul byte
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn create_string(string: &str) -> Result<Symbol, ClingoError> {
        let mut symbol = 0;
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
    /// **Note:** This is just a shortcut for [`Symbol::create_function()`] with
    /// empty arguments.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the symbol
    /// * `positive` - whether the symbol has a classical negation sign
    ///
    /// # Errors
    ///
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn create_id(name: &str, positive: bool) -> Result<Symbol, ClingoError> {
        let mut symbol = 0;
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
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn create_function(
        name: &str,
        arguments: &[Symbol],
        positive: bool,
    ) -> Result<Symbol, ClingoError> {
        let mut symbol = 0;
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::Runtime`] if symbol is not of type [`SymbolType::Number`]
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
    /// **Note:**
    /// The string is internalized and valid for the duration of the process.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::Runtime`]
    /// - [`ClingoError::Utf8Error`]
    pub fn name(&self) -> Result<&'static str, ClingoError> {
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
    /// **Note:**
    /// The string is internalized and valid for the duration of the process.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::Runtime`] if symbol is not of type [`SymbolType::String`]
    /// - [`ClingoError::Utf8Error`]
    pub fn string(&self) -> Result<&'static str, ClingoError> {
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::Runtime`]
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::Runtime`]
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::Runtime`]
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
    /// - [`ClingoError::InternalError`] - may failed to match clingo symbol type
    pub fn symbol_type(self) -> Result<SymbolType, ClingoError> {
        SymbolType::try_from(unsafe { clingo_symbol_type(self.0) } as u32)
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
/// Programs may be structured into parts, which can be grounded independently with [`Control::ground()`].
/// Program parts are mainly interesting for incremental grounding and multi-shot solving.
/// For single-shot solving, program parts are not needed.
///
/// **Note:** Parts of a logic program without an explicit `#program`
/// specification are by default put into a program called `base` - without
/// arguments.
///
/// **See:** [`Control::ground()`]
#[derive(Debug, Clone)]
pub struct Part {
    part: clingo_part,
    _params: Vec<Symbol>,
}
impl Part {
    /// Create a new program part object.
    ///
    /// # Arguments
    ///
    /// * `name` - the identifier of the program
    /// * `params` - the parameter of the program
    ///
    /// # Errors
    ///
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if argument parsing fails
    pub fn new(name: &str, params: Vec<Symbol>) -> Result<Part, ClingoError> {
        let name = internalize_string(name)?;

        Ok(Part {
            part: clingo_part {
                name: name as *const c_char,
                params: params.as_ptr() as *const clingo_symbol_t,
                size: params.len(),
            },
            _params: params,
        })
    }
}

/// An instance of this trait has to be registered with a solver to implement a custom propagator.
///
/// Not all functions have to be implemented, there exist default implementations.
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

    /// Can be used to propagate solver literals given a
    /// partial assignment [`Assignment`].
    ///
    /// Called during propagation with a non-empty array of
    /// watched solver literals ([`PropagateInit::add_watch()`])
    /// that have been assigned to true since the last call to either propagate, undo, (or the start
    /// of the search) - the change set.
    /// Only watched solver literals are contained in the change set.
    /// Each literal in the change set is true w.r.t. the current
    /// assignment [`Assignment`].
    /// [`PropagateControl::add_clause()`] can be
    /// used to add clauses.
    /// If a clause is unit resulting, it can be propagated using
    /// [`PropagateControl::propagate()`].
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
    /// [`PropagateControl::thread_id()`].
    ///
    /// # Arguments
    ///
    /// * `control` - control object for the target solver
    /// * `changes` - the change set
    ///
    /// **Returns** whether the call was successful
    fn propagate(&mut self, _control: &mut PropagateControl, _changes: &[SolverLiteral]) -> bool {
        true
    }
    /// Called whenever a solver undoes assignments to watched solver literals.
    ///
    /// This callback is meant to update assignment dependent state in the propagator.
    ///
    /// **Note:** No clauses must be propagated in this callback and no errors should be set.
    ///
    /// # Arguments
    ///
    /// * `control` - control object for the target solver
    /// * `changes` - the change set
    ///
    /// **Returns** whether the call was successful
    fn undo(&mut self, _control: &mut PropagateControl, _changes: &[SolverLiteral]) {}

    /// This function is similar to
    /// [`PropagateControl::propagate()`] but is only
    /// called on total assignments without a change set.
    ///
    /// When exactly this function is called, can be configured using the
    /// [`PropagateInit::set_check_mode()`]
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
        _fallback: SolverLiteral,
        _decision: &mut SolverLiteral,
    ) -> bool {
        true
    }
}
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
    let changes = std::slice::from_raw_parts(changes as *const SolverLiteral, size);
    let propagator = &mut *(propagator as *mut T);

    propagator.propagate(control, changes)
}
unsafe extern "C" fn unsafe_undo<T: Propagator>(
    control: *const clingo_propagate_control_t,
    changes: *const clingo_literal_t,
    size: usize,
    propagator: *mut c_void,
) {
    // check for null pointers
    if control.is_null() | (size > 0 && changes.is_null()) | propagator.is_null() {
        set_internal_error(ErrorType::Runtime, "unsafe_undo() got a null pointer.");
        return;
    }
    let control = &mut *(control as *mut PropagateControl);
    let changes = std::slice::from_raw_parts(changes as *const SolverLiteral, size);
    let propagator = &mut *(propagator as *mut T);

    propagator.undo(control, changes)
}
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
    let fallback = SolverLiteral(fallback);
    let propagator = &mut *(propagator as *mut T);
    let decision = &mut *(decision as *mut SolverLiteral);

    propagator.decide(Id(thread_id), assignment, fallback, decision)
}
pub mod defaults {
    use crate::ast::Location;
    use crate::{
        ExternalError, FunctionHandler, GroundProgramObserver, Logger, Propagator,
        SolveEventHandler, Symbol,
    };
    /// Default implementation for Logger, Propagator, GroundProgramObserver, FunctionHandler and SolveEventHandler
    #[derive(Debug, Clone, Copy)]
    pub struct Non;
    impl Logger for Non {}
    impl Propagator for Non {}
    impl GroundProgramObserver for Non {}
    impl FunctionHandler for Non {
        fn on_external_function(
            &mut self,
            _location: &Location,
            _name: &str,
            _arguments: &[Symbol],
        ) -> Result<Vec<Symbol>, ExternalError> {
            Ok(vec![])
        }
    }
    impl SolveEventHandler for Non {}
}

/// An instance of this trait can be registered with a control object
/// to enable logging, custom propagation, observers and function handling.
///
/// See: [`control_with_context()`]
pub trait ControlCtx {
    type L: Logger;
    type P: Propagator;
    type O: GroundProgramObserver;
    type F: FunctionHandler;
    /// Return a logger and the maximum number of times the logger callback is called
    fn logger(&mut self) -> (&mut Self::L, u32);
    /// Return a propagator and boolean flag sequential
    ///
    /// If the sequential flag is true, the propagator is called
    /// sequentially when solving with multiple threads.
    fn propagator(&mut self) -> (&mut Self::P, bool);
    /// Return a program observer and boolean flag for replace
    ///
    /// If the replace flag is true, the grounding is passed to the observer but not the solver
    fn observer(&mut self) -> (&mut Self::O, bool);
    /// Return a function handler
    fn function_handler(&mut self) -> &mut Self::F;
}

/// Default context for the Control.
///
/// No logging, no additional propagation, no observer, no function handler
#[derive(Debug)]
pub struct DefaultCtx {
    non: defaults::Non,
}
impl ControlCtx for DefaultCtx {
    type L = defaults::Non;
    type P = defaults::Non;
    type O = defaults::Non;
    type F = defaults::Non;

    fn logger(&mut self) -> (&mut Self::L, u32) {
        (&mut self.non, 0)
    }
    fn propagator(&mut self) -> (&mut Self::P, bool) {
        (&mut self.non, false)
    }
    fn observer(&mut self) -> (&mut Self::O, bool) {
        (&mut self.non, false)
    }
    fn function_handler(&mut self) -> &mut Self::F {
        &mut self.non
    }
}

/// Control object holding grounding and solving state.
#[derive(Debug)]
pub struct GenericControl<C: ControlCtx> {
    ctl: NonNull<clingo_control_t>,
    copied: bool,
    context: Box<C>,
}
pub type Control = GenericControl<DefaultCtx>;
impl<C: ControlCtx> Drop for GenericControl<C> {
    fn drop(&mut self) {
        if !self.copied {
            unsafe { clingo_control_free(self.ctl.as_ptr()) }
        }
    }
}
impl<C: ControlCtx> GenericControl<C> {
    /// Ground the selected parts [`Part`] of the current (non-ground) logic
    /// program.
    ///
    /// After grounding, logic programs can be solved with [`Control::solve()`].
    ///
    /// **Note:** Parts of a logic program without an explicit `#program`
    /// specification are by default put into a program called `base` - without
    /// arguments.
    ///
    /// # Arguments
    ///
    /// * `parts` -  array of parts to ground
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn ground(&mut self, parts: &[Part]) -> Result<(), ClingoError> {
        let parts_size = parts.len();
        let parts = parts
            .iter()
            .map(|arg| arg.part)
            .collect::<Vec<clingo_part>>();
        let function_handler = self.context.function_handler();
        if !unsafe {
            clingo_control_ground(
                self.ctl.as_ptr(),
                parts.as_ptr(),
                parts_size,
                Some(unsafe_ground_callback::<C::F> as GroundCallback),
                function_handler as *mut C::F as *mut c_void,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_ground() failed",
            ));
        }
        Ok(())
    }
    /// Register a control context
    ///
    /// # Arguments
    ///
    /// * `context` - implementing the trait [`ControlCtx`]
    pub fn register_control_context<T: ControlCtx>(mut self, context: T) -> GenericControl<T> {
        let context = Box::new(context);
        self.copied = true;
        GenericControl {
            ctl: self.ctl,
            copied: false,
            context,
        }
    }
    /// Solve the currently grounded ([`Control::ground()`]) logic program
    /// enumerating its models.
    ///
    /// # Arguments
    ///
    /// * `mode` - configures the search mode
    /// * `assumptions` - array of assumptions to solve under
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving could not be started
    pub fn solve(
        self,
        mode: SolveMode,
        assumptions: &[SolverLiteral],
    ) -> Result<GenericSolveHandle<C, defaults::Non>, ClingoError> {
        let mut handle = std::ptr::null_mut();
        let event_handler = std::ptr::null_mut();
        if !unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode.bits(),
                assumptions.as_ptr() as *const clingo_literal_t,
                assumptions.len(),
                None,
                event_handler,
                &mut handle,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_solve() failed",
            ));
        }
        match NonNull::new(handle) {
            Some(handle) => Ok(GenericSolveHandle {
                handle,
                ctl: self,
                _event_handler: Box::new(defaults::Non),
            }),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
    /// Solve the currently grounded ([`Control::ground()`]) logic program
    /// enumerating its models.
    ///
    /// # Arguments
    ///
    /// * `mode` - configures the search mode
    /// * `assumptions` - array of assumptions to solve under
    /// * `handler` - implementing the trait [`SolveEventHandler`]
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving could not be started
    pub fn solve_with_event_handler<T: SolveEventHandler>(
        self,
        mode: SolveMode,
        assumptions: &[SolverLiteral],
        event_handler: T,
    ) -> Result<GenericSolveHandle<C, T>, ClingoError> {
        let mut handle = std::ptr::null_mut();
        let mut event_handler = Box::new(event_handler);
        if !unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode.bits(),
                assumptions.as_ptr() as *const clingo_literal_t,
                assumptions.len(),
                Some(unsafe_solve_callback::<T> as SolveEventCallback),
                event_handler.as_mut() as *mut T as *mut c_void,
                &mut handle,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_solve() failed",
            ));
        }
        match NonNull::new(handle) {
            Some(handle) => Ok(GenericSolveHandle {
                handle,
                ctl: self,
                _event_handler: event_handler,
            }),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }

    // NODO: pub fn clingo_control_load(control: *mut Control, file: *const c_char) -> bool;

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
    /// - [`ClingoError::NulError`] - if a any argument contains a nul byte
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if parsing fails
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
    /// **See:** [`Control::get_enable_cleanup()`] and [`Control::set_enable_cleanup()`]
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn assign_external(
        &mut self,
        literal: SolverLiteral,
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn release_external(
        &mut self,
        SolverLiteral(literal): SolverLiteral,
    ) -> Result<(), ClingoError> {
        if !unsafe { clingo_control_release_external(self.ctl.as_ptr(), literal) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_release_external() failed",
            ));
        }
        Ok(())
    }
    /// Register a custom propagator with the control object.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    fn register_propagator(&mut self) -> Result<(), ClingoError> {
        let (propagator, sequential) = self.context.propagator();
        let clingo_propagator = clingo_propagator_t {
            init: Some(unsafe_init::<C::P>),
            propagate: Some(unsafe_propagate::<C::P>),
            undo: Some(unsafe_undo::<C::P>),
            check: Some(unsafe_check::<C::P>),
            decide: Some(unsafe_decide::<C::P>),
        };
        if !unsafe {
            clingo_control_register_propagator(
                self.ctl.as_ptr(),
                &clingo_propagator,
                propagator as *mut C::P as *mut c_void,
                sequential,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_register_propagator() failed",
            ));
        }
        self.copied = true;
        Ok(())
    }

    /// Check if the solver has determined that the internal program representation is conflicting.
    ///
    /// If this function returns true, solve calls will return immediately with an unsatisfiable solve result.
    /// Note that conflicts first have to be detected, e.g. -
    /// initial unit propagation results in an empty clause,
    /// or later if an empty clause is resolved during solving.
    /// Hence, the function might return false even if the problem is unsatisfiable.
    pub fn is_conflicting(&self) -> bool {
        unsafe { clingo_control_is_conflicting(self.ctl.as_ptr()) }
    }
    /// Get a statistics object to inspect solver statistics.
    ///
    /// Statistics are updated after a solve call.
    ///
    /// **Attention:**
    /// The level of detail of the statistics depends on the stats option
    /// (which can be set using [`Configuration`] or passed as an
    /// option when creating the control object ([`control()`]).
    /// The default level zero only provides basic statistics,
    /// level one provides extended and accumulated statistics,
    /// and level two provides per-thread statistics.
    /// Furthermore, the statistics object is best accessed right after solving.
    /// Otherwise, not all of its entries have valid values.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn statistics(&self) -> Result<&Statistics, ClingoError> {
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
    pub fn configuration_mut(&mut self) -> Result<&mut Configuration, ClingoError> {
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
    pub fn configuration(&self) -> Result<&Configuration, ClingoError> {
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
    /// as clauses added with [`SolveControl::add_clause()`].
    ///
    /// **Attention:** For practical purposes, this option is only interesting for single-shot
    /// solving or before the last solve call to squeeze out a tiny bit of performance.
    /// Initially, the enumeration assumption is enabled.
    ///
    /// # Arguments
    ///
    /// * `enable` - whether to enable the assumption
    pub fn set_enable_enumeration_assumption(&mut self, enable: bool) -> Result<(), ClingoError> {
        if !unsafe { clingo_control_set_enable_enumeration_assumption(self.ctl.as_ptr(), enable) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_use_enumeration_assumption() failed",
            ));
        }
        Ok(())
    }
    /// Check whether the enumeration assumption is enabled.
    ///
    /// **See** [`Control::set_enable_enumeration_assumption()`]
    ///
    /// **Returns** using the enumeration assumption is enabled
    pub fn get_enable_enumeration_assumption(&self) -> bool {
        unsafe { clingo_control_get_enable_enumeration_assumption(self.ctl.as_ptr()) }
    }
    /// Enable automatic cleanup after solving.
    ///
    /// **Note:** Cleanup is enabled by default.
    ///
    /// # Arguments
    ///
    /// * `enable` - whether to enable cleanups
    ///
    /// **Returns** whether the call was successful
    ///
    /// **See** [`Control::cleanup()`] and [`Control::get_enable_cleanup()`]
    pub fn set_enable_cleanup(&mut self, enable: bool) -> bool {
        unsafe { clingo_control_set_enable_cleanup(self.ctl.as_ptr(), enable) }
    }
    /// Check whether automatic cleanup is enabled.
    ///
    /// **See** [`Control::cleanup()`] and [`Control::set_enable_cleanup()`]
    pub fn get_enable_cleanup(&self) -> bool {
        unsafe { clingo_control_get_enable_cleanup(self.ctl.as_ptr()) }
    }
    /// Return the symbol for a constant definition of form: `#const name = symbol`.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the constant if it exists
    ///
    /// # Errors
    ///
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`]
    pub fn get_const(&self, name: &str) -> Result<Symbol, ClingoError> {
        let name = CString::new(name)?;
        let mut symbol = 0;
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
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
    /// - [`ClingoError::InternalError`]
    ///
    /// **See:** [`Control::get_const()`]
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
    pub fn theory_atoms(&self) -> Result<&TheoryAtoms, ClingoError> {
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
    /// **Returns** whether the call was successful
    fn register_observer(&mut self) -> Result<(), ClingoError> {
        let (observer, replace) = self.context.observer();
        let gpo = clingo_ground_program_observer_t {
            init_program: Some(unsafe_init_program::<C::O>),
            begin_step: Some(unsafe_begin_step::<C::O>),
            end_step: Some(unsafe_end_step::<C::O>),
            rule: Some(unsafe_rule::<C::O>),
            weight_rule: Some(unsafe_weight_rule::<C::O>),
            minimize: Some(unsafe_minimize::<C::O>),
            project: Some(unsafe_project::<C::O>),
            output_atom: Some(unsafe_output_atom::<C::O>),
            output_term: Some(unsafe_output_term::<C::O>),
            external: Some(unsafe_external::<C::O>),
            assume: Some(unsafe_assume::<C::O>),
            heuristic: Some(unsafe_heuristic::<C::O>),
            acyc_edge: Some(unsafe_acyc_edge::<C::O>),
            theory_term_number: Some(unsafe_theory_term_number::<C::O>),
            theory_term_string: Some(unsafe_theory_term_string::<C::O>),
            theory_term_compound: Some(unsafe_theory_term_compound::<C::O>),
            theory_element: Some(unsafe_theory_element::<C::O>),
            theory_atom: Some(unsafe_theory_atom::<C::O>),
            theory_atom_with_guard: Some(unsafe_theory_atom_with_guard::<C::O>),
        };
        if !unsafe {
            clingo_control_register_observer(
                self.ctl.as_ptr(),
                &gpo,
                replace,
                observer as *mut C::O as *mut c_void,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_register_observer() failed",
            ));
        }
        self.copied = true;
        Ok(())
    }
    /// Get an object to add ground directives to the program.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn backend(&mut self) -> Result<Backend, ClingoError> {
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

    pub fn add_facts(&mut self, facts: &FactBase) -> Result<(), ClingoError> {
        for sym in facts.iter() {
            let loc = ast::Location::default();
            // initilize atom to add
            let symbolic_term = ast::symbolic_term(&loc, sym)?;
            let atom = ast::symbolic_atom(symbolic_term)?;
            // create literal
            let lit = ast::basic_literal_from_symbolic_atom(&loc, ast::Sign::NoSign, atom).unwrap();
            let head: ast::Literal = ast::Literal::from(lit);
            // create (fact) rule
            let fact = ast::rule(&loc, head, &[]).unwrap();
            // get the program builder
            let mut builder = ast::ProgramBuilder::from(self).unwrap();

            // add the rewritten statement to the program
            builder
                .add(&fact.into())
                .expect("Failed to add statement to ProgramBuilder.");

            builder.end().expect("Failed to finish building a program.");
        }
        Ok(())
    }

    /// Covenience function that returns an iterator over the models.
    /// Uses [`Control::solve()`] with [SolveMode::YIELD] and empty assumptions.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving could not be started
    pub fn all_models(self) -> Result<AllModels<C, defaults::Non>, ClingoError> {
        let mut handle = std::ptr::null_mut();
        let event_handler = std::ptr::null_mut();
        if !unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                SolveMode::YIELD.bits(),
                std::ptr::null() as *const clingo_literal_t,
                0,
                None,
                event_handler,
                &mut handle,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_solve() failed",
            ));
        }
        match NonNull::new(handle) {
            Some(handle) => Ok(AllModels(GenericSolveHandle {
                handle,
                ctl: self,
                _event_handler: Box::new(defaults::Non),
            })),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }

    /// Covenience function that returns an iterator over the optimal models.
    /// Uses [`Control::solve()`] with [SolveMode::YIELD] and empty assumptions.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving could not be started
    pub fn optimal_models(self) -> Result<OptimalModels<C, defaults::Non>, ClingoError> {
        let mut handle = std::ptr::null_mut();
        let event_handler = std::ptr::null_mut();
        if !unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                SolveMode::YIELD.bits(),
                std::ptr::null() as *const clingo_literal_t,
                0,
                None,
                event_handler,
                &mut handle,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_solve() failed",
            ));
        }
        match NonNull::new(handle) {
            Some(handle) => Ok(OptimalModels(GenericSolveHandle {
                handle,
                ctl: self,
                _event_handler: Box::new(defaults::Non),
            })),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
}
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
/// - [`ClingoError::NulError`] - if an argument contains a nul byte
/// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
/// or [`ErrorCode::Runtime`] if argument parsing fails
pub fn control(arguments: std::vec::Vec<String>) -> Result<Control, ClingoError> {
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
        Some(ctl) => Ok(GenericControl {
            ctl,
            copied: false,
            context: Box::new(DefaultCtx { non: defaults::Non }),
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
/// Create a new control object with a custom context.
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
/// * `context` - an implementation of [`ControlCtx`]
///
/// # Errors
///
/// - [`ClingoError::NulError`] - if an argument contains a nul byte
/// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
/// or [`ErrorCode::Runtime`] if argument parsing fails
pub fn control_with_context<C: ControlCtx>(
    arguments: Vec<String>,
    mut context: C,
) -> Result<GenericControl<C>, ClingoError> {
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
    let (logger, message_limit) = context.logger();

    if !unsafe {
        clingo_control_new(
            c_args.as_ptr(),
            c_args.len(),
            Some(unsafe_logging_callback::<C::L> as LoggingCallback),
            logger as *mut C::L as *mut c_void,
            message_limit,
            &mut ctl_ptr,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_control_new() failed",
        ));
    }
    match NonNull::new(ctl_ptr) {
        Some(ctl) => {
            let mut control = GenericControl {
                ctl,
                copied: false,
                context: Box::new(context),
            };
            control.register_observer()?;
            control.register_propagator()?;
            Ok(control)
        }
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
/// Handle for the solver configuration.
#[derive(Debug)]
pub struct Configuration(clingo_configuration_t);
impl Configuration {
    /// Get the root key of the configuration.
    pub fn root(&self) -> Result<Id, ClingoError> {
        let mut root_key = 0;
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
        let mut ctype = 0;
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
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::Utf8Error`]
    pub fn description(&self, Id(key): Id) -> Result<&str, ClingoError> {
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
    /// The [`Configuration::configuration_type()`] of the entry must be [`ConfigurationType::ARRAY`].
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
    /// The [`Configuration::configuration_type()`] of the entry must be [`ConfigurationType::ARRAY`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset in the array
    pub fn array_at(&self, Id(key): Id, offset: usize) -> Result<Id, ClingoError> {
        let mut nkey = 0;
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
    /// The [`Configuration::configuration_type()`] of the entry must be [`ConfigurationType::MAP`].
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
    /// The [`Configuration::configuration_type()`] of the entry must be [`ConfigurationType::MAP`].
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
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
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
    /// The [`Configuration::configuration_type()`] of the entry must be [`ConfigurationType::MAP`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset of the name
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::Utf8Error`]
    pub fn map_subkey_name(&self, Id(key): Id, offset: usize) -> Result<&str, ClingoError> {
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
    /// The [`Configuration::configuration_type()`] of the entry must be [`ConfigurationType::MAP`].
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    pub fn map_at(&self, Id(key): Id, name: &str) -> Result<Id, ClingoError> {
        let mut nkey = 0;
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
    /// The [`Configuration::configuration_type()`] of the entry must be [`ConfigurationType::VALUE`].
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

    // NODO: fn clingo_configuration_value_get_size(&mut self.0, key, &mut size)

    /// Get the string value of the given entry.
    ///
    /// # Pre-condition
    ///
    /// The [`Configuration::configuration_type()`] of the entry must be [`ConfigurationType::VALUE`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::Utf8Error`]
    pub fn value_get(&self, Id(key): Id) -> Result<String, ClingoError> {
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
    /// The [`Configuration::configuration_type()`] of the entry must be [`ConfigurationType::VALUE`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `value` - the value to set
    ///
    /// # Errors
    ///
    /// - [`ClingoError::NulError`] - if `value` contains a nul byte
    /// - [`ClingoError::InternalError`]
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn rule(
        &mut self,
        choice: bool,
        head: &[Atom],
        body: &[SolverLiteral],
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn assume(&mut self, literals: &[SolverLiteral]) -> Result<(), ClingoError> {
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn heuristic(
        &mut self,
        atom: Atom,
        htype: HeuristicType,
        bias: i32,
        priority: u32,
        condition: &[SolverLiteral],
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn acyc_edge(
        &mut self,
        node_u: i32,
        node_v: i32,
        condition: &[SolverLiteral],
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
                let mut atom = 0;
                if unsafe { clingo_backend_add_atom(self.theref, &mut symbol, &mut atom) } {
                    Ok(Atom(atom))
                } else {
                    Err(ClingoError::new_internal(
                        "Call to clingo_backend_add_atom() failed",
                    ))
                }
            }
            None => {
                let mut atom = 0;
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
        let mut root_key = 0;
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
        let mut stype = 0;
        if !unsafe { clingo_statistics_type(&self.0, key, &mut stype) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_statistics_type() failed",
            ));
        }
        StatisticsType::try_from(stype as u32)
    }

    /// Get the size of an array entry.
    ///
    /// # Pre-condition
    ///
    /// The [`Statistics::statistics_type()`] of the entry must be
    /// [`StatisticsType::Array`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn array_size(&self, key: u64) -> Result<usize, ClingoError> {
        let mut size = 0;
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
    /// The [`Statistics::statistics_type()`] of the entry must be
    /// [`StatisticsType::Array`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset in the array
    pub fn array_at(&self, key: u64, offset: usize) -> Result<u64, ClingoError> {
        let mut subkey = 0;
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
    /// The [`Statistics::statistics_type()`] of the entry must
    /// be [`StatisticsType::Array`]
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `stype` -  the type of the new subkey
    pub fn array_push(&mut self, key: u64, stype: StatisticsType) -> Result<u64, ClingoError> {
        let mut subkey = 0;
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
    /// The [`Statistics::statistics_type()`] of the entry must
    /// be [`StatisticsType::Map`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn map_size(&self, key: u64) -> Result<usize, ClingoError> {
        let mut size = 0;
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
    /// The [`Statistics::statistics_type()`] of the entry must
    /// be [`StatisticsType::Map`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `name` - name of the subkey
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
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
    /// The [`Statistics::statistics_type()`] of the entry must be
    /// [`StatisticsType::Map`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset of the name
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::Utf8Error`]
    pub fn map_subkey_name(&self, key: u64, offset: usize) -> Result<&str, ClingoError> {
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
    /// The [`Statistics::statistics_type()`] of the entry must be
    /// [`StatisticsType::Map`].
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
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
    pub fn map_at(&self, key: u64, name: &str) -> Result<u64, ClingoError> {
        let mut subkey = 0;
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
    /// The [`Statistics::statistics_type()`] of the entry must be
    /// [`StatisticsType::Map`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `name` - the name to lookup the subkey
    /// * `stype` - the type of the new subkey
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::NulError`] - if `name` contains a nul byte
    ///
    /// **Returns** the index of the resulting subkey
    pub fn map_add_subkey(
        &mut self,
        key: u64,
        name: &str,
        stype: StatisticsType,
    ) -> Result<u64, ClingoError> {
        let mut subkey = 0;
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
    /// The [`Statistics::statistics_type()`] of the entry must be
    /// [`StatisticsType::Value`].
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn value_get(&self, key: u64) -> Result<f64, ClingoError> {
        let mut value = 0.0;
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
    /// The [`Statistics::statistics_type()`] of the entry must be
    /// [`StatisticsType::Value`].
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
/// **See:** [`Control::symbolic_atoms()`][`Statistics::statistics_type()`]
#[derive(Debug)]
pub struct SymbolicAtoms(clingo_symbolic_atoms_t);
impl SymbolicAtoms {
    /// Get the number of different atoms occurring in a logic program.
    pub fn size(&self) -> Result<usize, ClingoError> {
        let mut size = 0;
        if !unsafe { clingo_symbolic_atoms_size(&self.0, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_size() failed",
            ));
        }
        Ok(size)
    }

    /// Get a forward iterator of the sequence of all symbolic atoms.
    pub fn iter(&self) -> Result<SymbolicAtomsIterator, ClingoError> {
        let mut begin = 0;
        if !unsafe { clingo_symbolic_atoms_begin(&self.0, std::ptr::null(), &mut begin) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_begin() failed",
            ));
        }
        let mut end = 0;
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
        sig: Signature,
    ) -> Result<SymbolicAtomsIterator, ClingoError> {
        let mut begin = 0;
        if !unsafe { clingo_symbolic_atoms_begin(&self.0, &sig.0, &mut begin) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_begin() failed",
            ));
        }
        let mut end = 0;
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

    // NODO: fn clingo_symbolic_atoms_signatures_size()

    /// Get the predicate signatures occurring in a logic program.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if the size is too small
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

    // NODO: fn clingo_symbolic_atoms_is_valid()
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
        let mut symbol = 0;
        if !unsafe { clingo_symbolic_atoms_symbol(self.atoms, self.cur, &mut symbol) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_symbol() failed",
            ));
        }
        Ok(Symbol(symbol))
    }

    /// Returns the (numeric) aspif literal corresponding to the given symbolic atom.
    ///
    /// Such a literal can be mapped to a solver literal (see [`Propagator`][`Statistics::statistics_type()`].
    /// or be used in rules in aspif format (see [`ProgramBuilder`][`Statistics::statistics_type()`].
    pub fn literal(&self) -> Result<SolverLiteral, ClingoError> {
        let mut literal = 0;
        if !unsafe { clingo_symbolic_atoms_literal(self.atoms, self.cur, &mut literal) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_symbolic_atoms_literal() failed",
            ));
        }
        Ok(SolverLiteral(literal))
    }
}

/// Container that stores theory atoms, elements, and terms of a program.
///
/// **See:** [`Control::theory_atoms()`][`Statistics::statistics_type()`]
#[derive(Debug)]
pub struct TheoryAtoms(clingo_theory_atoms_t);
impl TheoryAtoms {
    /// Get the total number of theory atoms.
    pub fn size(&self) -> Result<usize, ClingoError> {
        let mut size = 0;
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
            atoms: self,
        }
    }

    /// Get the type of the given theory term.
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    pub fn term_type(&self, Id(term): Id) -> Result<TheoryTermType, ClingoError> {
        let mut ttype = 0;
        if !unsafe { clingo_theory_atoms_term_type(&self.0, term, &mut ttype) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_term_type() failed",
            ));
        }
        TheoryTermType::try_from(ttype as u32)
    }

    /// Get the number of the given numeric theory term.
    ///
    /// # Pre-condition
    ///
    /// The term must be of type [`TheoryTermType::Number`].
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
    /// The term must be of type [`TheoryTermType::Function`] or
    /// [`TheoryTermType::Symbol`].
    ///
    /// **Note:**
    /// The lifetime of the string is tied to the current solve step.
    ///
    /// # Arguments
    ///
    /// * `term` id of the term
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`]
    /// - [`ClingoError::Utf8Error`]
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
    /// The term must be of type [`TheoryTermType::Function`].
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    pub fn term_arguments(&self, Id(term): Id) -> Result<&[Id], ClingoError> {
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

    // NODO: fn clingo_theory_atoms_term_to_string_size()

    /// Get the string representation of the given theory term.
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::Runtime`] if the size is too small
    /// or [`ErrorCode::BadAlloc`]
    /// - [`ClingoError::Utf8Error`]
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
    pub fn element_condition(&self, Id(element): Id) -> Result<&[SolverLiteral], ClingoError> {
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
            unsafe { std::slice::from_raw_parts(condition_ptr as *const SolverLiteral, size) };
        Ok(condition_ref)
    }

    /// Get the id of the condition of the given theory element.
    ///
    /// **Note:**
    /// This id can be mapped to a solver literal using [`PropagateInit::solver_literal()`][`Statistics::statistics_type()`].
    /// This id is not (necessarily) an aspif literal;
    /// to get aspif literals use [`TheoryAtoms::element_condition()`][`Statistics::statistics_type()`].
    ///
    /// # Arguments
    ///
    /// * `element` - id of the element
    pub fn element_condition_id(&self, Id(element): Id) -> Result<SolverLiteral, ClingoError> {
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
        Ok(SolverLiteral(unsafe { *condition_ptr }))
    }

    // NODO: fn clingo_theory_atoms_element_to_string_size()

    /// Get the string representation of the given theory element.
    ///
    /// # Arguments
    ///
    /// * `element` - id of the element
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::Runtime`] if the size is too small
    /// or [`ErrorCode::BadAlloc`]
    /// - [`ClingoError::Utf8Error`]
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
        let mut term = 0;
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
    /// **Note:**
    /// The lifetime of the string is tied to the current solve step.
    ///
    /// # Arguments
    ///
    /// * `atom` - id of the atom
    ///
    /// # Errors
    ///
    /// - [`ClingoError::Utf8Error`]
    /// - [`ClingoError::InternalError`]
    pub fn atom_guard(&self, Id(atom): Id) -> Result<(&str, Id), ClingoError> {
        let mut c_ptr = std::ptr::null() as *const c_char;
        let mut term = 0;
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
    pub fn atom_literal(&self, Id(atom): Id) -> Result<SolverLiteral, ClingoError> {
        let mut literal = 0;
        if !unsafe { clingo_theory_atoms_atom_literal(&self.0, atom, &mut literal) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_theory_atoms_atom_literal() failed",
            ));
        }
        Ok(SolverLiteral(literal))
    }

    // NODO: fn clingo_theory_atoms_atom_to_string_size()

    /// Get the string representation of the given theory atom.
    ///
    /// # Arguments
    ///
    /// * `atom` - id of the element
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::Runtime`] if the size is too small
    /// or [`ErrorCode::BadAlloc`]
    /// - [`ClingoError::Utf8Error`]
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
        let mut mtype = 0;
        if !unsafe { clingo_model_type(&self.0, &mut mtype) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_type() failed",
            ));
        }
        ModelType::try_from(mtype as u32)
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

    // NODO: fn clingo_model_symbols_size()

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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if the size is too small
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
    pub fn is_true(&self, literal: SolverLiteral) -> Result<bool, ClingoError> {
        let mut is_true = false;
        if !unsafe { clingo_model_is_true(&self.0, literal.0, &mut is_true) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_is_true() failed",
            ));
        }
        Ok(is_true)
    }

    // NODO: fn clingo_model_cost_size(model: *mut Model, size: *mut size_t) -> u8;

    /// Get the cost vector of a model.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if the size is too small
    ///
    /// **See:** [`Model::optimality_proven()`][`Statistics::statistics_type()`]
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
    /// **See:** [`Model::cost()`][`Statistics::statistics_type()`]
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
        let mut id = 0;
        if !unsafe { clingo_model_thread_id(&self.0, &mut id) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_thread_id() failed",
            ));
        }
        Ok(Id(id))
    }

    /// Add symbols to the model.
    ///
    /// These symbols will appear in clingo\'s output, which means that this
    /// function is only meaningful if there is an underlying clingo application.
    /// Only models passed to the ::clingo_solve_event_callback_t are extendable.
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
    pub fn context(&self) -> Result<&mut SolveControl, ClingoError> {
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
    /// **Note:** The [`Propagator`] trait provides a more sophisticated
    /// interface to add clauses - even on partial assignments.
    ///
    /// # Arguments
    ///
    /// * `clause` - array of literals representing the clause
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if adding the clause fails
    pub fn add_clause(&mut self, clause: &[SolverLiteral]) -> Result<(), ClingoError> {
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
/// A literal is assigned to either true or false, or is unassigned ([`Assignment::truth_value()`]).
/// Furthermore, each assigned literal is associated with a decision level ([`Assignment::level()`]).
/// There is exactly one decision literal ([`Assignment::decision()`]) for each decision level greater than zero.
/// Assignments to all other literals on the same level are consequences implied by the current and possibly previous decisions.
/// Assignments on level zero are immediate consequences of the current program.
/// Decision levels are consecutive numbers starting with zero up to and including the current decision level ([`Assignment::decision_level()`]).
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
    pub fn has_literal(&self, literal: SolverLiteral) -> bool {
        unsafe { clingo_assignment_has_literal(&self.0, literal.0) }
    }

    /// Determine the decision level of a given literal.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    ///
    /// **Returns** the decision level of the given literal
    pub fn level(&self, literal: SolverLiteral) -> Result<u32, ClingoError> {
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
    pub fn decision(&self, level: u32) -> Result<SolverLiteral, ClingoError> {
        let mut lit = 0;
        if !unsafe { clingo_assignment_decision(&self.0, level, &mut lit) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_decision() failed",
            ));
        }
        Ok(SolverLiteral(lit))
    }

    /// Check if a literal has a fixed truth value.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal
    ///
    /// **Returns** whether the literal is fixed
    pub fn is_fixed(&self, literal: SolverLiteral) -> Result<bool, ClingoError> {
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
    /// **Returns** whether the literal is true (see [`Assignment::truth_value()`][`Statistics::statistics_type()`]
    pub fn is_true(&self, literal: SolverLiteral) -> Result<bool, ClingoError> {
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
    /// **Returns** whether the literal is false (see [`Assignment::truth_value()`][`Statistics::statistics_type()`]
    pub fn is_false(&self, literal: SolverLiteral) -> Result<bool, ClingoError> {
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
    pub fn truth_value(&self, literal: SolverLiteral) -> Result<TruthValue, ClingoError> {
        let mut value = 0;
        if !unsafe { clingo_assignment_truth_value(&self.0, literal.0, &mut value) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_truth_value() failed",
            ));
        }
        TruthValue::try_from(value as u32)
    }

    /// The number of (positive) literals in the assignment.
    pub fn size(&self) -> usize {
        unsafe { clingo_assignment_size(&self.0) }
    }

    /// The (positive) literal at the given offset in the assignment.
    ///
    /// # Arguments
    ///
    /// * `offset` - the offset of the literal
    ///
    /// **Returns** the literal
    pub fn at(&self, offset: usize) -> Result<SolverLiteral, ClingoError> {
        let mut lit = 0;
        if !unsafe { clingo_assignment_at(&self.0, offset, &mut lit) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_at() failed",
            ));
        }
        Ok(SolverLiteral(lit))
    }

    /// Check if the assignmen is total, i.e. there are no free literal.
    pub fn is_total(&self) -> bool {
        unsafe { clingo_assignment_is_total(&self.0) }
    }

    /// Returns the number of literals in the trail, i.e., the number of assigned literals.
    pub fn trail_size(&self) -> Result<u32, ClingoError> {
        let mut size = 0;
        if !unsafe { clingo_assignment_trail_size(&self.0, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_trail_size() failed",
            ));
        }
        Ok(size)
    }

    /// Returns the offset of the decision literal with the given decision level in
    /// the trail.
    ///
    /// **Note:**SolverLiterals in the trail are ordered by decision levels, where the first
    /// literal with a larger level than the previous literals is a decision; the
    /// following literals with same level are implied by this decision literal.
    /// Each decision level up to and including the current decision level has a
    /// valid offset in the trail.
    ///
    /// # Arguments
    ///
    /// * `level` - the decision level
    ///
    /// **Returns** the offset of the decision literal
    pub fn trail_begin(&self, level: u32) -> Result<u32, ClingoError> {
        let mut offset = 0;
        if !unsafe { clingo_assignment_trail_begin(&self.0, level, &mut offset) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_trail_begin() failed",
            ));
        }
        Ok(offset)
    }

    /// Returns the offset following the last literal with the given decision level.
    ///
    /// **Note:** This function is the counter part to clingo_assignment_trail_begin().
    ///
    /// # Arguments
    ///
    /// * `level` - the decision level
    ///
    /// **Returns** the offset
    pub fn trail_end(&self, level: u32) -> Result<u32, ClingoError> {
        let mut offset = 0;
        if !unsafe { clingo_assignment_trail_end(&self.0, level, &mut offset) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_trail_end() failed",
            ));
        }
        Ok(offset)
    }

    /// Returns the literal at the given position in the trail.
    ///
    /// # Arguments
    ///
    /// * `offset` - the offset of the literal
    ///
    /// **Returns** the literal
    pub fn trail_at(&self, offset: u32) -> Result<SolverLiteral, ClingoError> {
        let mut lit = 0;
        if !unsafe { clingo_assignment_trail_at(&self.0, offset, &mut lit) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_assignment_trail_at() failed",
            ));
        }
        Ok(SolverLiteral(lit))
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Logic`] if the assignment is conflicting
    pub fn add_literal(&mut self, result: &mut SolverLiteral) -> Result<(), ClingoError> {
        if !unsafe { clingo_propagate_control_add_literal(&mut self.0, &mut result.0) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_control_add_literal() failed",
            ));
        }
        Ok(())
    }

    /// Add a watch for the solver literal in the given phase.
    ///
    /// **Note:** Unlike [`PropagateInit::add_watch()`] this does not add a watch to all solver threads but just the current one.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal to watch
    ///
    /// **Errors:**
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Logic`] if the literal is invalid
    ///
    /// **See:** [`PropagateControl::remove_watch()`]
    pub fn add_watch(&mut self, literal: SolverLiteral) -> Result<(), ClingoError> {
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
    pub fn has_watch(&self, literal: SolverLiteral) -> bool {
        unsafe { clingo_propagate_control_has_watch(&self.0, literal.0) }
    }

    /// Removes the watch (if any) for the given solver literal.
    ///
    /// **Note:** Similar to [`PropagateInit::add_watch()`] this just removes the watch in the current solver thread.
    ///
    /// # Arguments
    ///
    /// * `literal` - the literal to remove
    pub fn remove_watch(&mut self, literal: SolverLiteral) {
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
    /// **Returns** result indicating whether propagation has to be stopped
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn add_clause(
        &mut self,
        clause: &[SolverLiteral],
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
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
/// Each symbolic ([`SymbolicAtoms`]) or theory atom ([`TheoryAtoms`]) is uniquely associated with an aspif atom in form of a positive integer ([`SolverLiteral`]).
/// Aspif literals additionally are signed to represent default negation.
/// Furthermore, there are non-zero integer solver literals (also represented using [`SolverLiteral`].
/// There is a surjective mapping from program atoms to solver literals.
///
/// All methods called during propagation use solver literals whereas [`SymbolicAtom::literal()`] and [`TheoryAtoms::atom_literal()`] return program literals.
/// The function [`PropagateInit::solver_literal()`] can be used to map program literals or condition ids([`TheoryAtoms::element_condition_id()`]) to solver literals.
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
    pub fn solver_literal(
        &self,
        SolverLiteral(aspif_literal): SolverLiteral,
    ) -> Result<SolverLiteral, ClingoError> {
        let mut solver_literal = 0;
        if !unsafe {
            clingo_propagate_init_solver_literal(&self.0, aspif_literal, &mut solver_literal)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_solver_literal() failed",
            ));
        }
        Ok(SolverLiteral(solver_literal))
    }

    /// Add a watch for the solver literal in the given phase.
    ///
    /// # Arguments
    ///
    /// * `solver_literal` - the solver literal
    pub fn add_watch(
        &mut self,
        SolverLiteral(solver_literal): SolverLiteral,
    ) -> Result<(), ClingoError> {
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
        SolverLiteral(solver_literal): SolverLiteral,
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
    /// **See:** [`PropagateControl::thread_id()`][`Statistics::statistics_type()`]
    pub fn number_of_threads(&self) -> usize {
        (unsafe { clingo_propagate_init_number_of_threads(&self.0) }) as usize
    }

    /// Configure when to call the check method of the propagator.
    ///
    /// # Arguments
    ///
    /// * `mode` - bitmask when to call the propagator
    ///
    /// **See:** [`Propagator::check()`]
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
    /// **See:** [`PropagateInit::set_check_mode()`][`Statistics::statistics_type()`]
    pub fn get_check_mode(&self) -> Result<PropagatorCheckMode, ClingoError> {
        PropagatorCheckMode::try_from(
            unsafe { clingo_propagate_init_get_check_mode(&self.0) } as u32
        )
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

    /// Add a literal to the solver.
    ///
    /// To be able to use the variable in clauses during propagation or add watches to it, it has to be frozen.
    /// Otherwise, it might be removed during preprocessing.
    ///
    /// **Attention** If varibales were added, subsequent calls to functions adding constraints or ::clingo_propagate_init_propagate() are expensive.
    /// It is best to add varables in batches.
    ///
    /// # Arguments
    ///
    /// * `freeze` - whether to freeze the literal
    /// **Returns** the added literal
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn add_literal(&mut self, freeze: bool) -> Result<&mut SolverLiteral, ClingoError> {
        let literal_ptr = std::ptr::null_mut() as *mut clingo_literal_t;
        if !unsafe { clingo_propagate_init_add_literal(&mut self.0, freeze, literal_ptr) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_add_literal() failed",
            ));
        }
        match unsafe { (literal_ptr as *mut SolverLiteral).as_mut() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::FFIError {
                msg: "Tried casting a null pointer to &mutSolverLiteral.",
            }),
        }
    }

    /// Add the given clause to the solver.
    ///
    /// **Attention** No further calls on the init object or functions on the assignment should be called when the result of this method is false.
    ///
    /// # Arguments
    ///
    /// * `clause` - the clause to add
    ///
    ///  **Returns** whether the problem became unsatisfiable
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn add_clause(&mut self, clause: &[SolverLiteral]) -> Result<bool, ClingoError> {
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

    /// Add the given weight constraint to the solver.
    ///
    /// This function adds a constraint of form `literal <=> { lit=weight | (lit, weight) in literals } >= bound` to the solver.
    /// Depending on the type the `<=>` connective can be either a left implication, right implication, or equivalence.
    ///
    /// **Attention** No further calls on the init object or functions on the assignment should be called when the result of this method is false.
    ///
    /// * `literal` - the literal of the constraint
    /// * `literals` - the weighted literals
    /// * `bound` - the bound of the constraint
    /// * `wctype` - the type of the weight constraint
    /// * `compare_equal` - if true compare equal instead of less than equal
    ///
    /// **Returns** result indicating whether the problem became unsatisfiable
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn add_weight_constraint(
        &mut self,
        literal: SolverLiteral,
        literals: &[WeightedLiteral],
        bound: i32,
        wctype: WeigthConstraintType,
        compare_equal: bool,
    ) -> Result<bool, ClingoError> {
        let mut result = false;
        if !unsafe {
            clingo_propagate_init_add_weight_constraint(
                &mut self.0,
                literal.0,
                literals.as_ptr() as *const clingo_weighted_literal_t,
                literals.len(),
                bound,
                wctype as i32,
                compare_equal,
                &mut result,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_add_weight_constraint() failed",
            ));
        }
        Ok(result)
    }
    /// Add the given literal to minimize to the solver.
    ///
    /// This corresponds to a weak constraint of form `:~ literal. [weight@priority]`.
    ///
    /// * `literal` - the literal to minimize
    /// * `weight` - the weight of the literal
    /// * `priority` - the priority of the literal
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn add_minimize(
        &mut self,
        literal: SolverLiteral,
        weight: i32,
        priority: i32,
    ) -> Result<(), ClingoError> {
        if !unsafe { clingo_propagate_init_add_minimize(&mut self.0, literal.0, weight, priority) }
        {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_add_minimize() failed",
            ));
        }
        Ok(())
    }

    /// Propagates consequences of the underlying problem excluding registered propagators.
    ///
    /// **Note** The function has no effect if SAT-preprocessing is enabled.
    ///
    /// **Attention** No further calls on the init object or functions on the assignment should be called when the result of this method is false.
    ///
    /// **Returns** result indicating whether the problem became unsatisfiable
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn propagate(&mut self) -> Result<bool, ClingoError> {
        let mut result = false;
        if !unsafe { clingo_propagate_init_propagate(&mut self.0, &mut result) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_propagate() failed",
            ));
        }
        Ok(result)
    }

    /// Remove the watch for the solver literal in the given phase.
    ///
    /// # Arguments
    ///
    /// * `literal` - the solver literal
    pub fn remove_watch(&mut self, literal: &SolverLiteral) -> Result<(), ClingoError> {
        if !unsafe { clingo_propagate_init_remove_watch(&mut self.0, literal.0) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_remove_watch() failed",
            ));
        }
        Ok(())
    }

    /// Remove the watch for the solver literal in the given phase from the given solver thread.
    ///
    /// * `literal` - the solver literal
    /// * `thread_id`- the id of the solver thread
    pub fn remove_watch_from_thread(
        &mut self,
        literal: &SolverLiteral,
        thread_id: u32,
    ) -> Result<(), ClingoError> {
        if !unsafe {
            clingo_propagate_init_remove_watch_from_thread(&mut self.0, literal.0, thread_id)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_remove_watch_from_thread() failed",
            ));
        }
        Ok(())
    }

    /// Freeze the given solver literal.
    ///
    /// Any solver literal that is not frozen is subject to simplification and might be removed in a preprocessing step after propagator initialization.
    /// A propagator should freeze all literals over which it might add clauses during propagation.
    /// Note that any watched literal is automatically frozen and that it does not matter which phase of the literal is frozen.
    ///
    /// * `literal` - the solver literal
    pub fn freeze_literal(&mut self, literal: &SolverLiteral) -> Result<(), ClingoError> {
        if !unsafe { clingo_propagate_init_freeze_literal(&mut self.0, literal.0) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_propagate_init_freeze_literal() failed",
            ));
        }
        Ok(())
    }
}

/// Search handle to a solve call.
#[derive(Debug)]
pub struct GenericSolveHandle<C: ControlCtx, E: SolveEventHandler> {
    handle: NonNull<clingo_solve_handle_t>,
    ctl: GenericControl<C>,
    _event_handler: Box<E>,
}
pub type SolveHandle = GenericSolveHandle<DefaultCtx, defaults::Non>;
impl<C: ControlCtx, E: SolveEventHandler> GenericSolveHandle<C, E> {
    /// Get the next solve result.
    ///
    /// Blocks until the result is ready.
    /// When yielding partial solve results can be obtained, i.e.,
    /// when a model is ready, the result will be satisfiable but neither the search exhausted nor the optimality proven.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving fails
    pub fn get(&mut self) -> Result<SolveResult, ClingoError> {
        let mut result = 0;
        if !unsafe { clingo_solve_handle_get(self.handle.as_ptr(), &mut result) } {
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
        unsafe { clingo_solve_handle_wait(self.handle.as_ptr(), timeout_secs, &mut result) }

        result
    }
    /// Get the next model or None if there are no more models.
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving fails
    pub fn model(&mut self) -> Result<Option<&Model>, ClingoError> {
        let mut model = std::ptr::null_mut() as *const clingo_model_t;
        if !unsafe { clingo_solve_handle_model(self.handle.as_ptr(), &mut model) } {
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving fails
    pub fn model_mut(&mut self) -> Result<Option<&mut Model>, ClingoError> {
        let mut model = std::ptr::null_mut() as *const clingo_model_t;
        if !unsafe { clingo_solve_handle_model(self.handle.as_ptr(), &mut model) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_model() failed",
            ));
        }
        Ok(unsafe { (model as *mut Model).as_mut() })
    }
    /// When a problem is unsatisfiable, get a subset of the assumptions that made the problem unsatisfiable.
    ///
    /// If the program is not unsatisfiable, an empty vector is returned.
    ///
    /// **Returns** the unsatisfiable core
    ///
    /// # Errors
    ///
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    pub fn core(&mut self) -> Result<Vec<SolverLiteral>, ClingoError> {
        let mut literal_ptr = std::ptr::null();
        let mut size: usize = 0;
        if !unsafe { clingo_solve_handle_core(self.handle.as_ptr(), &mut literal_ptr, &mut size) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_core() failed",
            ));
        }
        // let literals = unsafe {std::slice::from_raw_parts(literal_ptr, size)};

        let mut literals = Vec::<SolverLiteral>::with_capacity(size);
        for _ in 0..size {
            if literal_ptr.is_null() {
                return Err(ClingoError::FFIError {
                    msg: "clingo_solve_handle_core() returned a null pointer.",
                });
            }
            let nliteral = unsafe { *literal_ptr };
            literals.push(SolverLiteral(nliteral));
            literal_ptr = unsafe { literal_ptr.offset(1) };
        }
        Ok(literals)
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving fails
    pub fn resume(&mut self) -> Result<(), ClingoError> {
        if !unsafe { clingo_solve_handle_resume(self.handle.as_ptr()) } {
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving fails
    pub fn cancel(&mut self) -> Result<(), ClingoError> {
        if !unsafe { clingo_solve_handle_cancel(self.handle.as_ptr()) } {
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
    /// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
    /// or [`ErrorCode::Runtime`] if solving fails
    pub fn close(self) -> Result<GenericControl<C>, ClingoError> {
        if !unsafe { clingo_solve_handle_close(self.handle.as_ptr()) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_close() failed",
            ));
        }
        Ok(self.ctl)
    }
}
pub struct OptimalModels<C: ControlCtx, E: SolveEventHandler>(GenericSolveHandle<C, E>);
impl<C: ControlCtx, E: SolveEventHandler> Iterator for OptimalModels<C, E> {
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
                Err(e) => panic!("{}", e),
            }
        }
    }
}
pub struct AllModels<C: ControlCtx, E: SolveEventHandler>(GenericSolveHandle<C, E>);
impl<C: ControlCtx, E: SolveEventHandler> Iterator for AllModels<C, E> {
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
                Err(e) => panic!("{}", e),
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
// #[doc = "! Callback to customize clingo main function."]
// #[doc = "!"]
// #[doc = "! @param[in] control corresponding control object"]
// #[doc = "! @param[in] files files passed via command line arguments"]
// #[doc = "! @param[in] size number of files"]
// #[doc = "! @param[in] data user data for the callback"]
// #[doc = "!"]
// #[doc = "! @return whether the call was successful"]
// pub type clingo_main_function_t = ::std::option::Option<
//     unsafe extern "C" fn(
//         control: *mut clingo_control_t,
//         files: *const *const ::std::os::raw::c_char,
//         size: usize,
//         data: *mut ::std::os::raw::c_void,
//     ) -> bool,
// >;
// #[doc = "! Callback to print a model in default format."]
// #[doc = "!"]
// #[doc = "! @param[in] data user data for the callback"]
// #[doc = "!"]
// #[doc = "! @return whether the call was successful"]
// pub type clingo_default_model_printer_t =
//     ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> bool>;
// #[doc = "! Callback to customize model printing."]
// #[doc = "!"]
// #[doc = "! @param[in] model the model"]
// #[doc = "! @param[in] printer the default model printer"]
// #[doc = "! @param[in] printer_data user data for the printer"]
// #[doc = "! @param[in] data user data for the callback"]
// #[doc = "!"]
// #[doc = "! @return whether the call was successful"]
// pub type clingo_model_printer_t = ::std::option::Option<
//     unsafe extern "C" fn(
//         model: *const clingo_model_t,
//         printer: clingo_default_model_printer_t,
//         printer_data: *mut ::std::os::raw::c_void,
//         data: *mut ::std::os::raw::c_void,
//     ) -> bool,
// >;
// #[doc = "! This struct contains a set of functions to customize the clingo application."]
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct clingo_application {
//     #[doc = "!< callback to obtain program name"]
//     pub program_name: ::std::option::Option<
//         unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char,
//     >,
//     #[doc = "!< callback to obtain version information"]
//     pub version: ::std::option::Option<
//         unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char,
//     >,
//     #[doc = "!< callback to obtain message limit"]
//     pub message_limit: ::std::option::Option<
//         unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_uint,
//     >,
//     #[doc = "!< callback to override clingo's main function"]
//     pub main: clingo_main_function_t,
//     #[doc = "!< callback to override default logger"]
//     pub logger: clingo_logger_t,
//     #[doc = "!< callback to override default model printing"]
//     pub printer: clingo_model_printer_t,
//     #[doc = "!< callback to register options"]
//     pub register_options: ::std::option::Option<
//         unsafe extern "C" fn(
//             options: *mut clingo_options_t,
//             data: *mut ::std::os::raw::c_void,
//         ) -> bool,
//     >,
//     #[doc = "!< callback validate options"]
//     pub validate_options:
//         ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> bool>,
// }
// #[doc = "! This struct contains a set of functions to customize the clingo application."]
// pub type clingo_application_t = clingo_application;
// pub trait Application {
//     fn program_name(&self) -> String;

// }
// unsafe extern "C" fn unsafe_program_name<T: Application>(data: *mut ::std::os::raw::c_void) -> *const ::std::os::raw::c_char {
//     // check for null pointers
//     if data.is_null()
//     {
//         set_internal_error(
//             ErrorType::Runtime,
//             "unsafe_program_name() got a null pointer.",
//         );
//         return std::ptr::null();
//     }
//     let data = &mut *(data as *mut T);
//     let name = data.program_name();
//     let c_name: CStr = name.into();
//     c_name.as_ptr()
// }
// extern "C" {
//     #[doc = "! Add an option that is processed with a custom parser."]
//     #[doc = "!"]
//     #[doc = "! Note that the parser also has to take care of storing the semantic value of"]
//     #[doc = "! the option somewhere."]
//     #[doc = "!"]
//     #[doc = "! Parameter option specifies the name(s) of the option."]
//     #[doc = "! For example, \"ping,p\" adds the short option \"-p\" and its long form \"--ping\"."]
//     #[doc = "! It is also possible to associate an option with a help level by adding \",@l\" to the option specification."]
//     #[doc = "! Options with a level greater than zero are only shown if the argument to help is greater or equal to l."]
//     #[doc = "!"]
//     #[doc = "! @param[in] options object to register the option with"]
//     #[doc = "! @param[in] group options are grouped into sections as given by this string"]
//     #[doc = "! @param[in] option specifies the command line option"]
//     #[doc = "! @param[in] description the description of the option"]
//     #[doc = "! @param[in] parse callback to parse the value of the option"]
//     #[doc = "! @param[in] data user data for the callback"]
//     #[doc = "! @param[in] multi whether the option can appear multiple times on the command-line"]
//     #[doc = "! @param[in] argument optional string to change the value name in the generated help output"]
//     #[doc = "! @return whether the call was successful"]
//     pub fn clingo_options_add(
//         options: *mut clingo_options_t,
//         group: *const ::std::os::raw::c_char,
//         option: *const ::std::os::raw::c_char,
//         description: *const ::std::os::raw::c_char,
//         parse: ::std::option::Option<
//             unsafe extern "C" fn(
//                 value: *const ::std::os::raw::c_char,
//                 data: *mut ::std::os::raw::c_void,
//             ) -> bool,
//         >,
//         data: *mut ::std::os::raw::c_void,
//         multi: bool,
//         argument: *const ::std::os::raw::c_char,
//     ) -> bool;
// }
// extern "C" {
//     #[doc = "! Add an option that is a simple flag."]
//     #[doc = "!"]
//     #[doc = "! This function is similar to @ref clingo_options_add() but simpler because it only supports flags, which do not have values."]
//     #[doc = "! If a flag is passed via the command-line the parameter target is set to true."]
//     #[doc = "!"]
//     #[doc = "! @param[in] options object to register the option with"]
//     #[doc = "! @param[in] group options are grouped into sections as given by this string"]
//     #[doc = "! @param[in] option specifies the command line option"]
//     #[doc = "! @param[in] description the description of the option"]
//     #[doc = "! @param[in] target boolean set to true if the flag is given on the command-line"]
//     #[doc = "! @return whether the call was successful"]
//     pub fn clingo_options_add_flag(
//         options: *mut clingo_options_t,
//         group: *const ::std::os::raw::c_char,
//         option: *const ::std::os::raw::c_char,
//         description: *const ::std::os::raw::c_char,
//         target: *mut bool,
//     ) -> bool;
// }

// extern "C" {
//     #[doc = "! Run clingo with a customized main function (similar to python and lua embedding)."]
//     #[doc = "!"]
//     #[doc = "! @param[in] application struct with callbacks to override default clingo functionality"]
//     #[doc = "! @param[in] arguments command line arguments"]
//     #[doc = "! @param[in] size number of arguments"]
//     #[doc = "! @param[in] data user data to pass to callbacks in application"]
//     #[doc = "! @return exit code to return from main function"]
// pub fn clingo_mai<T: Application>(app:&mut T, arguments: Vec<String>) -> Result<i32,ClingoError> {
//     let c_app = clingo_application{
//         program_name: Some(<T>::unsafe_program_name::<T>),
//         version: Some(<T>::unsafe_version::<T>),
//         message_limit: Some(<T>::unsafe_message_limit::<T>),
//         main: Some(<T>::unsafe_main::<T>),
//         logger: Some(<T>::unsafe_logger::<T>),
//         printer: Some(<T>::unsafe_printer::<T>),
//         register_options: Some(<T>::unsafe_register_options::<T>),
//         validate_options: Some(<T>::unsafe_validate_options::<T>)
//     };
//     let mut args = vec![];
//     for arg in arguments {
//         args.push(CString::new(arg)?);
//     }
//     // convert the strings to raw pointers
//     let c_args = args
//         .iter()
//         .map(|arg| arg.as_ptr())
//         .collect::<Vec<*const c_char>>();
//     Ok(unsafe { clingo_main(&mut c_app, c_args.as_ptr(), arguments.len(), app as *mut c_void) } )
// }
//     pub fn clingo_main(
//         application: *mut clingo_application_t,
//         arguments: *const *const ::std::os::raw::c_char,
//         size: usize,
//         data: *mut ::std::os::raw::c_void,
//     ) -> ::std::os::raw::c_int;
// }
/// Internalize a string.
///
/// This functions takes a string as input and returns an equal unique string
/// that is (at the moment) not freed until the program is closed.
///
/// # Arguments
///
/// * `string` - the string to internalize
/// * `result` - the internalized string
///
/// # Errors
///
/// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
/// - [`ClingoError::Utf8Error`]
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
/// - [`ClingoError::NulError`] - if `string` contains a nul byte
/// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
/// or [`ErrorCode::Runtime`] if parsing fails
pub fn parse_term(string: &str) -> Result<Symbol, ClingoError> {
    let c_str = CString::new(string)?;
    let mut symbol = 0;
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
/// - [`ClingoError::NulError`] - if `string` contains a nul byte
/// - [`ClingoError::InternalError`] with [`ErrorCode::BadAlloc`]
/// or [`ErrorCode::Runtime`] if parsing fails
pub fn parse_term_with_logger<L: Logger>(
    string: &str,
    logger: &mut L,
    message_limit: u32,
) -> Result<Symbol, ClingoError> {
    let c_str = CString::new(string)?;
    let logger = logger as *mut L;
    let mut symbol = 0;
    if !unsafe {
        clingo_parse_term(
            c_str.as_ptr(),
            Some(unsafe_logging_callback::<L> as LoggingCallback),
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
/// An instance of this struct has to be registered with a solver to observe ground directives as they are passed to the solver.
///
/// **Note:** This interface is closely modeled after the aspif format.
/// For more information please refer to the specification of the aspif format.
///
/// Not all callbacks have to be implemented and can be set to NULL if not needed.
/// If one of the callbacks in the struct fails, grounding is stopped.
/// If a non-recoverable clingo API call fails, a callback must return false.
/// Otherwise ::clingo_error_unknown should be set and false returned.
///
// See [`Control::register_observer`][`Statistics::statistics_type()`].
pub trait GroundProgramObserver {
    /// Called once in the beginning.
    ///
    /// If the incremental flag is true, there can be multiple calls to
    /// [`Control::solve()`][`Statistics::statistics_type()`].
    ///
    /// # Arguments
    ///
    /// * `incremental` - whether the program is incremental
    ///
    /// **Returns** whether the call was successful
    fn init_program(&mut self, _incremental: bool) -> bool {
        true
    }

    /// Marks the beginning of a block of directives passed to the solver.
    ///
    /// **See:** [`GroundProgramObserver::end_step()`]
    ///
    /// **Returns** whether the call was successful
    fn begin_step(&mut self) -> bool {
        true
    }

    /// Marks the end of a block of directives passed to the solver.
    ///
    /// This function is called before solving starts.
    ///
    /// **See:** [`GroundProgramObserver::begin_step()`]
    ///
    /// **Returns** whether the call was successful
    fn end_step(&mut self) -> bool {
        true
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
    fn rule(&mut self, _choice: bool, _head: &[Atom], _body: &[SolverLiteral]) -> bool {
        true
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
        _choice: bool,
        _head: &[Atom],
        _lower_bound: i32,
        _body: &[WeightedLiteral],
    ) -> bool {
        true
    }

    /// Observe minimize constraints (or weak constraints) passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `priority` - the priority of the constraint
    /// * `literals` - the weighted literals whose sum to minimize
    ///
    /// **Returns** whether the call was successful
    fn minimize(&mut self, _priority: i32, _literals: &[WeightedLiteral]) -> bool {
        true
    }

    /// Observe projection directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `atoms` - the atoms to project on
    ///
    /// **Returns** whether the call was successful
    fn project(&mut self, _atoms: &[Atom]) -> bool {
        true
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
    fn output_atom(&mut self, _symbol: Symbol, _atom: Atom) -> bool {
        true
    }
    /// Observe shown terms passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `symbol` - the symbolic representation of the term
    /// * `condition` - the literals of the condition
    ///
    /// **Returns** whether the call was successful
    fn output_term(&mut self, _symbol: Symbol, _condition: &[SolverLiteral]) -> bool {
        true
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
    fn output_csp(&mut self, _symbol: Symbol, _value: i32, _condition: &[SolverLiteral]) -> bool {
        true
    }

    /// Observe external statements passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `atom` - the external atom
    /// * `etype` - the type of the external statement
    ///
    /// **Returns** whether the call was successful
    fn external(&mut self, _atom: Atom, _type_: ExternalType) -> bool {
        true
    }

    /// Observe assumption directives passed to the solver.
    ///
    /// # Arguments
    ///
    /// * `literals` - the literals to assume (positive literals are true and negative literals
    /// false for the next solve call)
    ///
    /// **Returns** whether the call was successful
    fn assume(&mut self, _literals: &[SolverLiteral]) -> bool {
        true
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
        _atom: Atom,
        _type_: HeuristicType,
        _bias: i32,
        _priority: u32,
        _condition: &[SolverLiteral],
    ) -> bool {
        true
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
    fn acyc_edge(&mut self, _node_u: i32, _node_v: i32, _condition: &[SolverLiteral]) -> bool {
        true
    }
    /// Observe numeric theory terms.
    ///
    /// # Arguments
    ///
    /// * `term_id` - the id of the term
    /// * `number` - the value of the term
    ///
    /// **Returns** whether the call was successful
    fn theory_term_number(&mut self, _term_id: Id, _number: i32) -> bool {
        true
    }
    /// Observe string theory terms.
    ///
    /// # Arguments
    ///
    /// * `term_id` - the id of the term
    /// * `name` - the value of the term
    ///
    /// **Returns** whether the call was successful
    fn theory_term_string(&mut self, _term_id: Id, _name: &str) -> bool {
        true
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
    fn theory_term_compound(
        &mut self,
        _term_id: Id,
        _name_id_or_type: i32,
        _arguments: &[Id],
    ) -> bool {
        true
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
    fn theory_element(
        &mut self,
        _element_id: Id,
        _terms: &[Id],
        _condition: &[SolverLiteral],
    ) -> bool {
        true
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
    fn theory_atom(&mut self, _atom_id_or_zero: Id, _term_id: Id, _elements: &[Id]) -> bool {
        true
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
        _atom_id_or_zero: Id,
        _term_id: Id,
        _elements: &[Id],
        _operator_id: Id,
        _right_hand_side_id: Id,
    ) -> bool {
        true
    }
}
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
    let body = std::slice::from_raw_parts(body as *const SolverLiteral, body_size);
    let gpo = &mut *(gpo as *mut T);

    gpo.rule(choice, head, body)
}
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
    let condition = std::slice::from_raw_parts(condition as *const SolverLiteral, size);
    let gpo = &mut *(gpo as *mut T);

    gpo.output_term(Symbol(symbol), condition)
}
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

    match ExternalType::try_from(etype as u32) {
        Err(e) => {
            eprintln!("Error in unsafe_external(): {}.", e);
            set_internal_error(ErrorType::Runtime, "Error in unsafe_external().");
            false
        }
        Ok(etype) => gpo.external(Atom(atom), etype),
    }
}
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
    let literals = std::slice::from_raw_parts(literals as *const SolverLiteral, size);
    let gpo = &mut *(gpo as *mut T);

    gpo.assume(literals)
}
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
    let condition = std::slice::from_raw_parts(condition as *const SolverLiteral, size);
    let gpo = &mut *(gpo as *mut T);

    match HeuristicType::try_from(htype as u32) {
        Err(e) => {
            eprintln!("Error in unsafe_heuristic(): {}.", e);
            set_internal_error(ErrorType::Runtime, "Error in unsafe_heuristic().");
            false
        }
        Ok(htype) => gpo.heuristic(Atom(atom), htype, bias, priority, condition),
    }
}
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
    let condition = std::slice::from_raw_parts(condition as *const SolverLiteral, size);
    let gpo = &mut *(gpo as *mut T);

    gpo.acyc_edge(node_u, node_v, condition)
}
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
            "unsafe_theory_term_number tried casting a null pointer to &mut GroundProgramObserver.",
        );
        false
    }
}
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
    let condition = std::slice::from_raw_parts(condition as *const SolverLiteral, condition_size);
    let gpo = &mut *(gpo as *mut T);

    gpo.theory_element(Id(element_id), terms, condition)
}
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
        let tempvec = vec![self.0.symbol()?, self.1.symbol()?, self.2.symbol()?];
        Symbol::create_function("", &tempvec, true)
    }
}
impl<A: ToSymbol, B: ToSymbol, C: ToSymbol, D: ToSymbol> ToSymbol for (A, B, C, D) {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let tempvec = vec![
            self.0.symbol()?,
            self.1.symbol()?,
            self.2.symbol()?,
            self.3.symbol()?,
        ];
        Symbol::create_function("", &tempvec, true)
    }
}

impl<A: ToSymbol, B: ToSymbol, C: ToSymbol, D: ToSymbol, E: ToSymbol> ToSymbol for (A, B, C, D, E) {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let tempvec = vec![
            self.0.symbol()?,
            self.1.symbol()?,
            self.2.symbol()?,
            self.3.symbol()?,
            self.4.symbol()?,
        ];
        Symbol::create_function("", &tempvec, true)
    }
}
impl<A: ToSymbol, B: ToSymbol, C: ToSymbol, D: ToSymbol, E: ToSymbol, F: ToSymbol> ToSymbol
    for (A, B, C, D, E, F)
{
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let tempvec = vec![
            self.0.symbol()?,
            self.1.symbol()?,
            self.2.symbol()?,
            self.3.symbol()?,
            self.4.symbol()?,
            self.5.symbol()?,
        ];
        Symbol::create_function("", &tempvec, true)
    }
}
impl<A: ToSymbol, B: ToSymbol, C: ToSymbol, D: ToSymbol, E: ToSymbol, F: ToSymbol, G: ToSymbol>
    ToSymbol for (A, B, C, D, E, F, G)
{
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        let tempvec = vec![
            self.0.symbol()?,
            self.1.symbol()?,
            self.2.symbol()?,
            self.3.symbol()?,
            self.4.symbol()?,
            self.5.symbol()?,
            self.6.symbol()?,
        ];
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
        let tempvec = vec![
            self.0.symbol()?,
            self.1.symbol()?,
            self.2.symbol()?,
            self.3.symbol()?,
            self.4.symbol()?,
            self.5.symbol()?,
            self.6.symbol()?,
            self.7.symbol()?,
        ];
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
        let tempvec = vec![
            self.0.symbol()?,
            self.1.symbol()?,
            self.2.symbol()?,
            self.3.symbol()?,
            self.4.symbol()?,
            self.5.symbol()?,
            self.6.symbol()?,
            self.7.symbol()?,
            self.8.symbol()?,
        ];
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
        let tempvec = vec![
            self.0.symbol()?,
            self.1.symbol()?,
            self.2.symbol()?,
            self.3.symbol()?,
            self.4.symbol()?,
            self.5.symbol()?,
            self.6.symbol()?,
            self.7.symbol()?,
            self.8.symbol()?,
            self.9.symbol()?,
        ];
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
        let tempvec = vec![
            self.0.symbol()?,
            self.1.symbol()?,
            self.2.symbol()?,
            self.3.symbol()?,
            self.4.symbol()?,
            self.5.symbol()?,
            self.6.symbol()?,
            self.7.symbol()?,
            self.8.symbol()?,
            self.9.symbol()?,
            self.10.symbol()?,
        ];
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
        let tempvec = vec![
            self.0.symbol()?,
            self.1.symbol()?,
            self.2.symbol()?,
            self.3.symbol()?,
            self.4.symbol()?,
            self.5.symbol()?,
            self.6.symbol()?,
            self.7.symbol()?,
            self.8.symbol()?,
            self.9.symbol()?,
            self.10.symbol()?,
            self.11.symbol()?,
        ];
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

pub trait FromSymbol: Sized {
    type Error;

    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error>;
}

impl FromSymbol for Symbol {
    type Error = Infallible;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        Ok(symbol)
    }
}
impl FromSymbol for u8 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        symbol
            .number()?
            .try_into()
            .map_err(|_| ClingoError::new_external("Could not convert to u8"))
    }
}
impl FromSymbol for i8 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        symbol
            .number()?
            .try_into()
            .map_err(|_| ClingoError::new_external("Could not convert to i8"))
    }
}
impl FromSymbol for u16 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        symbol
            .number()?
            .try_into()
            .map_err(|_| ClingoError::new_external("Could not convert to u16"))
    }
}
impl FromSymbol for i16 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        symbol
            .number()?
            .try_into()
            .map_err(|_| ClingoError::new_external("Could not convert to i16"))
    }
}
impl FromSymbol for u32 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        symbol
            .number()?
            .try_into()
            .map_err(|_| ClingoError::new_external("Could not convert to u32"))
    }
}
impl FromSymbol for i32 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        symbol.number()
    }
}
impl FromSymbol for u64 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        symbol
            .number()?
            .try_into()
            .map_err(|_| ClingoError::new_external("Could not convert to u64"))
    }
}
impl FromSymbol for i64 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        Ok(symbol.number()? as i64)
    }
}
impl FromSymbol for u128 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        symbol
            .number()?
            .try_into()
            .map_err(|_| ClingoError::new_external("Could not convert to u128"))
    }
}
impl FromSymbol for i128 {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        Ok(symbol.number()? as i128)
    }
}
impl FromSymbol for String {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        Ok(symbol.string()?.into())
    }
}
impl FromSymbol for &'static str {
    type Error = ClingoError;
    fn from_symbol(symbol: Symbol) -> Result<Self, Self::Error> {
        symbol.string()
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
            self.facts.insert(*s);
        }
    }
    pub fn print(&self) {
        for fact in &self.facts {
            print!("{fact}.");
        }
        println!();
    }
}

// #[doc = "! Custom scripting language to run functions during grounding."]
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct clingo_script {
//     #[doc = "! Evaluate the given source code."]
//     #[doc = "! @param[in] location the location in the logic program of the source code"]
//     #[doc = "! @param[in] code the code to evaluate"]
//     #[doc = "! @param[in] data user data as given when registering the script"]
//     #[doc = "! @return whether the function call was successful"]
//     pub execute: ::std::option::Option<
//         unsafe extern "C" fn(
//             location: *const clingo_location_t,
//             code: *const ::std::os::raw::c_char,
//             data: *mut ::std::os::raw::c_void,
//         ) -> bool,
//     >,
//     #[doc = "! Call the function with the given name and arguments."]
//     #[doc = "! @param[in] location the location in the logic program of the function call"]
//     #[doc = "! @param[in] name the name of the function"]
//     #[doc = "! @param[in] arguments the arguments to the function"]
//     #[doc = "! @param[in] arguments_size the number of arguments"]
//     #[doc = "! @param[in] symbol_callback callback to return a pool of symbols"]
//     #[doc = "! @param[in] symbol_callback_data user data for the symbol callback"]
//     #[doc = "! @param[in] data user data as given when registering the script"]
//     #[doc = "! @return whether the function call was successful"]
//     pub call: ::std::option::Option<
//         unsafe extern "C" fn(
//             location: *const clingo_location_t,
//             name: *const ::std::os::raw::c_char,
//             arguments: *const clingo_symbol_t,
//             arguments_size: usize,
//             symbol_callback: clingo_symbol_callback_t,
//             symbol_callback_data: *mut ::std::os::raw::c_void,
//             data: *mut ::std::os::raw::c_void,
//         ) -> bool,
//     >,
//     #[doc = "! Check if the given function is callable."]
//     #[doc = "! @param[in] name the name of the function"]
//     #[doc = "! @param[out] result whether the function is callable"]
//     #[doc = "! @param[in] data user data as given when registering the script"]
//     #[doc = "! @return whether the function call was successful"]
//     pub callable: ::std::option::Option<
//         unsafe extern "C" fn(
//             name: *const ::std::os::raw::c_char,
//             result: *mut bool,
//             data: *mut ::std::os::raw::c_void,
//         ) -> bool,
//     >,
//     #[doc = "! Run the main function."]
//     #[doc = "! @param[in] control the control object to pass to the main function"]
//     #[doc = "! @param[in] data user data as given when registering the script"]
//     #[doc = "! @return whether the function call was successful"]
//     pub main: ::std::option::Option<
//         unsafe extern "C" fn(
//             control: *mut clingo_control_t,
//             data: *mut ::std::os::raw::c_void,
//         ) -> bool,
//     >,
//     #[doc = "! This function is called once when the script is deleted."]
//     #[doc = "! @param[in] data user data as given when registering the script"]
//     pub free: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>,
//     pub version: *const ::std::os::raw::c_char,
// }

// #[doc = "! Custom scripting language to run functions during grounding."]
// pub type clingo_script_t = clingo_script;
// extern "C" {
//     #[doc = "! Add a custom scripting language to clingo."]
//     #[doc = "!"]
//     #[doc = "! @param[in] name the name of the scripting language"]
//     #[doc = "! @param[in] script struct with functions implementing the language"]
//     #[doc = "! @param[in] data user data to pass to callbacks in the script"]
//     #[doc = "! @return whether the call was successful"]
//     pub fn clingo_register_script(
//         name: *const ::std::os::raw::c_char,
//         script: *const clingo_script_t,
//         data: *mut ::std::os::raw::c_void,
//     ) -> bool;
// }
// extern "C" {
//     #[doc = "! Get the version of the registered scripting language."]
//     #[doc = "!"]
//     #[doc = "! @param[in] name the name of the scripting language"]
//     #[doc = "! @return the version"]
//     pub fn clingo_script_version(
//         name: *const ::std::os::raw::c_char,
//     ) -> *const ::std::os::raw::c_char;
// }

// Re-export #[derive(ToSymbol)].
#[cfg(feature = "derive")]
#[allow(unused_imports)]
pub use clingo_derive::*;

impl From<Symbol> for clingo_symbol_t {
    fn from(symbol: Symbol) -> Self {
        symbol.0
    }
}
impl From<clingo_symbol_t> for Symbol {
    fn from(symbol: clingo_symbol_t) -> Self {
        Symbol(symbol)
    }
}
impl From<Id> for clingo_id_t {
    fn from(id: Id) -> Self {
        id.0
    }
}
impl From<Options> for clingo_options_t {
    fn from(options: Options) -> Self {
        options.0
    }
}
impl From<&mut Options> for *mut clingo_options_t {
    fn from(options: &mut Options) -> Self {
        &mut options.0
    }
}
impl From<Statistics> for clingo_statistic {
    fn from(stats: Statistics) -> Self {
        stats.0
    }
}
impl From<&mut Statistics> for *mut clingo_statistic {
    fn from(stats: &mut Statistics) -> Self {
        &mut stats.0
    }
}
impl From<Model> for clingo_model {
    fn from(model: Model) -> Self {
        model.0
    }
}
impl From<&mut Model> for *mut clingo_model {
    fn from(model: &mut Model) -> Self {
        &mut model.0
    }
}
impl<C: ControlCtx> From<GenericControl<C>> for NonNull<clingo_control> {
    fn from(control: GenericControl<C>) -> Self {
        control.ctl
    }
}
impl<C: ControlCtx> From<&mut GenericControl<C>> for NonNull<clingo_control> {
    fn from(control: &mut GenericControl<C>) -> Self {
        control.ctl
    }
}
