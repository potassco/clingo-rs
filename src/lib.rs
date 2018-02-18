#![feature(ptr_internals)]
#![allow(non_upper_case_globals)]
extern crate clingo_sys;
extern crate libc;

use std::mem;
use std::ptr::Unique;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ffi::CStr;
use std::ffi::CString;
use libc::c_char;
use clingo_sys::*;

pub mod ast;
#[derive(Debug, Copy, Clone)]
pub enum Error {
    Success = clingo_error_clingo_error_success as isize,
    Runtime = clingo_error_clingo_error_runtime as isize,
    Logic = clingo_error_clingo_error_logic as isize,
    BadAlloc = clingo_error_clingo_error_bad_alloc as isize,
    Unknown = clingo_error_clingo_error_unknown as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum TruthValue {
    Free = clingo_truth_value_clingo_truth_value_free as isize,
    True = clingo_truth_value_clingo_truth_value_true as isize,
    False = clingo_truth_value_clingo_truth_value_false as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ClauseType {
    Learnt = clingo_clause_type_clingo_clause_type_learnt as isize,
    Static = clingo_clause_type_clingo_clause_type_static as isize,
    Volatile = clingo_clause_type_clingo_clause_type_volatile as isize,
    VolatileStatic = clingo_clause_type_clingo_clause_type_volatile_static as isize,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SolveEventType {
    Model = clingo_solve_event_type_clingo_solve_event_type_model as isize,
    Finish = clingo_solve_event_type_clingo_solve_event_type_finish as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum StatisticsType {
    Empty = clingo_statistics_type_clingo_statistics_type_empty as isize,
    Value = clingo_statistics_type_clingo_statistics_type_value as isize,
    Array = clingo_statistics_type_clingo_statistics_type_array as isize,
    Map = clingo_statistics_type_clingo_statistics_type_map as isize,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SymbolType {
    Infimum = clingo_symbol_type_clingo_symbol_type_infimum as isize,
    Number = clingo_symbol_type_clingo_symbol_type_number as isize,
    String = clingo_symbol_type_clingo_symbol_type_string as isize,
    Function = clingo_symbol_type_clingo_symbol_type_function as isize,
    Supremum = clingo_symbol_type_clingo_symbol_type_supremum as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum Warning {
    OperationUndefined = clingo_warning_clingo_warning_operation_undefined as isize,
    RuntimeError = clingo_warning_clingo_warning_runtime_error as isize,
    AtomUndefined = clingo_warning_clingo_warning_atom_undefined as isize,
    FileIncluded = clingo_warning_clingo_warning_file_included as isize,
    VariableUnbound = clingo_warning_clingo_warning_variable_unbounded as isize,
    GlobalVariable = clingo_warning_clingo_warning_global_variable as isize,
    Other = clingo_warning_clingo_warning_other as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ExternalType {
    Free = clingo_external_type_clingo_external_type_free as isize,
    True = clingo_external_type_clingo_external_type_true as isize,
    False = clingo_external_type_clingo_external_type_false as isize,
    Release = clingo_external_type_clingo_external_type_release as isize,
}

#[derive(Debug, Copy, Clone)]
pub enum HeuristicType {
    Level = clingo_heuristic_type_clingo_heuristic_type_level as isize,
    Sign = clingo_heuristic_type_clingo_heuristic_type_sign as isize,
    Factor = clingo_heuristic_type_clingo_heuristic_type_factor as isize,
    Init = clingo_heuristic_type_clingo_heuristic_type_init as isize,
    True = clingo_heuristic_type_clingo_heuristic_type_true as isize,
    False = clingo_heuristic_type_clingo_heuristic_type_false as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum TermType {
    Tuple = clingo_theory_term_type_clingo_theory_term_type_tuple as isize,
    List = clingo_theory_term_type_clingo_theory_term_type_list as isize,
    Set = clingo_theory_term_type_clingo_theory_term_type_set as isize,
    Function = clingo_theory_term_type_clingo_theory_term_type_function as isize,
    Number = clingo_theory_term_type_clingo_theory_term_type_number as isize,
    Symbol = clingo_theory_term_type_clingo_theory_term_type_symbol as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ModelType {
    StableModel = clingo_model_type_clingo_model_type_stable_model as isize,
    BraveConsequences = clingo_model_type_clingo_model_type_brave_consequences as isize,
    CautiousConsequences = clingo_model_type_clingo_model_type_cautious_consequences as isize,
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

// TODO: check documentation
/// Callback function called during search to notify when the search is finished or a model is ready.
///
/// If a (non-recoverable) clingo API function fails in this callback, it must return false.
/// In case of errors not related to clingo, set error code ::clingo_error_unknown and return false to stop solving with an error.
///
/// **Attention:** If the search is finished, the model is NULL.
///
/// # Arguments
///
/// * `model` - the current model
/// * `data` - user data of the callback
/// * `goon` - can be set to false to stop solving
///
/// **Returns** whether the call was successful
///
/// @see clingo_control_solve()
type SolveEventCallback = unsafe extern "C" fn(
    type_: clingo_solve_event_type_t,
    event: *mut ::std::os::raw::c_void,
    data: *mut ::std::os::raw::c_void,
    goon: *mut bool,
) -> bool;
pub trait SolveEventHandler<T> {
    fn on_solve_event(type_: SolveEventType, data: &mut T, goon: &mut bool) -> bool;
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
            clingo_solve_event_type_clingo_solve_event_type_model => SolveEventType::Model,
            clingo_solve_event_type_clingo_solve_event_type_finish => SolveEventType::Finish,
            _ => panic!("Rust binding failed to match clingo solve event type"),
        };
        let data = (data_ as *mut T).as_mut().unwrap();
        let goon = goon_.as_mut().unwrap();
        Self::on_solve_event(event_type, data, goon)
    }
}

type AstCallback =
    unsafe extern "C" fn(arg1: *const clingo_ast_statement_t, arg2: *mut ::std::os::raw::c_void)
        -> bool;
pub trait AstStatementHandler<T> {
    fn on_statement(arg1: &AstStatement, arg2: &mut T) -> bool;
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_ast_callback(
        stm_: *const clingo_ast_statement_t,
        data_: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!stm_.is_null());
        assert!(!data_.is_null());
        let stm = (stm_ as *const AstStatement).as_ref().unwrap();
        let data = (data_ as *mut T).as_mut().unwrap();
        Self::on_statement(stm, data)
    }
}

//TODO: check documentation
/// Callback to intercept warning messages.
///
/// **Parameters:**
///
/// * `code` - associated warning code
/// * `message` - warning message
/// * `data` - user data for callback
///
/// @see clingo_control_new()
/// @see clingo_parse_term()
/// @see clingo_parse_program()
type LoggingCallback = unsafe extern "C" fn(
    code: clingo_warning_t,
    message: *const ::std::os::raw::c_char,
    data: *mut ::std::os::raw::c_void,
);
pub trait Logger<T> {
    fn log(code: Warning, message: &str, data: &mut T);
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_logging_callback(
        code_: clingo_warning_t,
        message_: *const ::std::os::raw::c_char,
        data_: *mut ::std::os::raw::c_void,
    ) {
        assert!(!message_.is_null());
        assert!(!data_.is_null());
        let warning = match code_ as u32 {
            clingo_warning_clingo_warning_atom_undefined => Warning::AtomUndefined,
            clingo_warning_clingo_warning_file_included => Warning::FileIncluded,
            clingo_warning_clingo_warning_global_variable => Warning::GlobalVariable,
            clingo_warning_clingo_warning_operation_undefined => Warning::OperationUndefined,
            clingo_warning_clingo_warning_other => Warning::Other,
            clingo_warning_clingo_warning_runtime_error => Warning::RuntimeError,
            clingo_warning_clingo_warning_variable_unbounded => Warning::VariableUnbound,
            _ => panic!("Rust binding failed to match clingo warning"),
        };
        let c_str = CStr::from_ptr(message_);
        let message = c_str.to_str().unwrap();
        let data = (data_ as *mut T).as_mut().unwrap();
        Self::log(warning, message, data)
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
pub trait GroundEventHandler<T> {
    fn on_ground_event(
        location: &Location,
        name: &str,
        arguments: &[Symbol],
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

        let location = (location_ as *const Location).as_ref().unwrap();
        let c_str = CStr::from_ptr(name_);
        let name = c_str.to_str().unwrap();
        let arguments = std::slice::from_raw_parts(arguments_ as *const Symbol, arguments_size);
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

/// Signed integer type for weights in sum aggregates and minimize constraints.
pub type Weight = clingo_weight_t;

/// A Literal with an associated weight.
/// @ingroup ProgramInspection
#[derive(Debug, Copy, Clone)]
pub struct WeightedLiteral(clingo_weighted_literal);
impl WeightedLiteral {
    pub fn literal(&self) -> Literal {
        Literal(self.0.literal)
    }
    pub fn weight(&self) -> Weight {
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
    //TODO  /// < the file where the location begins
    //     pub begin_file: *const ::std::os::raw::c_char,
    //TODO  /// < the file where the location ends
    //     pub end_file: *const ::std::os::raw::c_char,
    /// < the line where the location begins
    pub fn begin_line(&self) -> usize {
        self.0.begin_line
    }
    /// < the line where the location ends
    pub fn end_line(&self) -> usize {
        self.0.end_line
    }
    /// < the column where the location begins
    pub fn begin_column(&self) -> usize {
        self.0.begin_column
    }
    /// < the column where the location ends
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
        //         self.isbn == other.isbn
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn create(name_: &str, arity: u32, positive: bool) -> Result<Signature, Error> {
        let name_c_str = CString::new(name_).unwrap();
        let mut signature = 0;
        if unsafe { clingo_signature_create(name_c_str.as_ptr(), arity, positive, &mut signature) }
        {
            Ok(Signature(signature))
        } else {
            Err(error())
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn create_string(string: &str) -> Result<Symbol, Error> {
        let mut symbol = 0 as clingo_symbol_t;
        let c_str = CString::new(string).unwrap();
        if unsafe { clingo_symbol_create_string(c_str.as_ptr(), &mut symbol) } {
            Ok(Symbol(symbol))
        } else {
            Err(error())
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn create_id(name: &str, positive: bool) -> Result<Symbol, Error> {
        let mut symbol = 0 as clingo_symbol_t;
        let name_c_str = CString::new(name).unwrap();
        if unsafe { clingo_symbol_create_id(name_c_str.as_ptr(), positive, &mut symbol) } {
            //             println!("create Symbol! sym {} {:?}", symbol, name_c_str);
            Ok(Symbol(symbol))
        } else {
            Err(error())
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
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
            Err(error())
        }
    }
    /// Get the number of a symbol.
    ///
    /// # Errors
    ///
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if symbol is not of type `SymbolType::Number`
    pub fn number(&self) -> Result<i32, Error> {
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
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if symbol is not of type `SymbolType::Function`
    pub fn name(&self) -> Result<&str, Error> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_symbol_name(self.0, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Ok(c_str.to_str().unwrap())
        } else {
            Err(error())
        }
    }

    /// Get the string of a symbol.
    ///
    /// # Errors
    ///
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if symbol is not of type `SymbolType::String`
    pub fn string(&self) -> Result<&str, Error> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_symbol_string(self.0, &mut char_ptr) } {
            let c_str = unsafe { CStr::from_ptr(char_ptr) };
            Ok(c_str.to_str().unwrap())
        } else {
            Err(error())
        }
    }

    /// Check if a function is positive (does not have a sign).
    ///
    /// # Errors
    ///
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if symbol is not of type `SymbolType::Function`
    pub fn is_positive(&self) -> Result<bool, Error> {
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
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if symbol is not of type `SymbolType::Function`
    pub fn is_negative(&self) -> Result<bool, Error> {
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
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if symbol is not of type `SymbolType::Function`
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
            Err(error())
        }
    }

    /// Get the type of a symbol.
    ///
    /// # Errors
    ///
    /// - may failed to match clingo symbol type
    //TODO maybe unnecesary function in Rust API?
    pub fn get_type(&self) -> Result<SymbolType, &'static str> {
        let stype = unsafe { clingo_symbol_type(self.0) } as u32;
        match stype {
            clingo_symbol_type_clingo_symbol_type_infimum => Ok(SymbolType::Infimum),
            clingo_symbol_type_clingo_symbol_type_number => Ok(SymbolType::Number),
            clingo_symbol_type_clingo_symbol_type_string => Ok(SymbolType::String),
            clingo_symbol_type_clingo_symbol_type_function => Ok(SymbolType::Function),
            clingo_symbol_type_clingo_symbol_type_supremum => Ok(SymbolType::Supremum),
            _ => Err("Rust binding failed to match clingo symbol type"),
        }
    }

    /// Get the string representation of a symbol.
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    ///
    pub fn to_string(&self) -> Option<String> {
        let mut size: usize = 0;
        if unsafe { clingo_symbol_to_string_size(self.0, &mut size) } {
            let a1 = vec![1; size];
            let cstring = unsafe { CString::from_vec_unchecked(a1) };
            let err =
                unsafe { clingo_symbol_to_string(self.0, cstring.as_ptr() as *mut c_char, size) };
            if !err {
                None
            } else {
                cstring.into_string().ok()
            }
        } else {
            None
        }
    }
}

// struct MaLogger;
// impl Logger<u32> for MaLogger {
//
//     fn log(code: Warning, message: &str, data: &mut u32){
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
/// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if parsing fails
/// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
pub fn parse_program<D, T: AstStatementHandler<D>>(
    program_: &str,
    _callback: &T,
    callback_data: &mut D,
) -> Result<(), Error> {
    let logger = None;
    //         let logger = Some(MaLogger::unsafe_logging_callback as LoggingCallback);
    let logger_data = std::ptr::null_mut();
    let program = CString::new(program_).unwrap();
    let data = callback_data as *mut D;
    if unsafe {
        clingo_parse_program(
            program.as_ptr(),
            Some(T::unsafe_ast_callback as AstCallback),
            data as *mut ::std::os::raw::c_void,
            logger,
            logger_data,
            0,
        )
    } {
        Ok(())
    } else {
        Err(error())
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
/// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if parsing fails
/// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
pub fn parse_program_with_logger<CD, C: AstStatementHandler<CD>, LD, L: Logger<LD>>(
    program_: &str,
    _callback: &C,
    cdata_: &mut CD,
    _logger: &L,
    ldata_: &mut LD,
    message_limit: u32,
) -> Result<(), Error> {
    let callback_data = cdata_ as *mut CD;
    let logger_data = ldata_ as *mut LD;
    let program = CString::new(program_).unwrap();
    if unsafe {
        clingo_parse_program(
            program.as_ptr(),
            Some(C::unsafe_ast_callback as AstCallback),
            callback_data as *mut ::std::os::raw::c_void,
            Some(L::unsafe_logging_callback as LoggingCallback),
            logger_data as *mut ::std::os::raw::c_void,
            message_limit,
        )
    } {
        Ok(())
    } else {
        Err(error())
    }
}
pub fn create_location(
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
pub fn error() -> Error {
    let code = unsafe { clingo_error_code() };
    match code as u32 {
        clingo_error_clingo_error_success => Error::Success,
        clingo_error_clingo_error_runtime => Error::Runtime,
        clingo_error_clingo_error_logic => Error::Logic,
        clingo_error_clingo_error_bad_alloc => Error::BadAlloc,
        clingo_error_clingo_error_unknown => Error::Unknown,
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
pub fn set_error(code: Error, message: &str) {
    let message_c_str = CString::new(message).unwrap();
    unsafe { clingo_set_error(code as clingo_error_t, message_c_str.as_ptr()) }
}

/// An instance of this trait has to be registered with a solver to implement a custom propagator.
///
/// For all functions exist default implementations and they must not be implemented manually.
pub trait Propagator<T> {
    //TODO
    /// This function is called once before each solving step.
    /// It is used to map relevant program literals to solver literals, add watches for solver
    /// literals, and initialize the data structures used during propagation.
    ///
    /// **Note:** This is the last point to access symbolic and theory atoms.
    /// Once the search has started, they are no longer accessible.
    ///
    /// **Parameters:**
    ///
    /// * `init` - initizialization object
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    /// @see ::clingo_propagator_init_callback_t
    fn init(_init: &mut PropagateInit, _data: &mut T) -> bool {
        true
    }

    //TODO
    /// Can be used to propagate solver literals given a @link clingo_assignment_t partial assignment@endlink.
    ///
    /// Called during propagation with a non-empty array of @link clingo_propagate_init_add_watch() watched solver literals@endlink
    /// that have been assigned to true since the last call to either propagate, undo, (or the start of the search) - the change set.
    /// Only watched solver literals are contained in the change set.
    /// Each literal in the change set is true w.r.t. the current @link clingo_assignment_t assignment@endlink.
    /// @ref clingo_propagate_control_add_clause() can be used to add clauses.
    /// If a clause is unit resulting, it can be propagated using @ref clingo_propagate_control_propagate().
    /// If the result of either of the two methods is false, the propagate function must return
    /// immediately.
    ///
    /// The following snippet shows how to use the methods to add clauses and propagate consequences
    /// within the callback.
    /// The important point is to return true (true to indicate there was no error) if the result of
    /// either of the methods is false.
    /// ~~~~~~~~~~~~~~~{.c}
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
    /// ~~~~~~~~~~~~~~~
    ///
    /// **Note:**
    /// This function can be called from different solving threads.
    /// Each thread has its own assignment and id, which can be obtained using @ref clingo_propagate_control_thread_id().
    ///
    /// **Parameters:**
    ///
    /// * `control` - control object for the target solver
    /// * `changes` - the change set
    /// * `size` - the size of the change set
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    /// @see ::clingo_propagator_propagate_callback_t
    fn propagate(_control: &mut PropagateControl, _changes: &[Literal], _data: &mut T) -> bool {
        true
    }

    //TODO: check documentation
    /// Called whenever a solver undoes assignments to watched solver literals.
    ///
    /// This callback is meant to update assignment dependent state in the propagator.
    ///
    /// **Note:** No clauses must be propagated in this callback.
    ///
    /// **Parameters:**
    ///
    /// * `control` - control object for the target solver
    /// * `changes` - the change set
    /// * `size` - the size of the change set
    /// * `data` - user data for the callback
    /// @see ::clingo_propagator_undo_callback_t
    fn undo(_control: &mut PropagateControl, _changes: &[Literal], _data: &mut T) -> bool {
        true
    }

    //TODO: check documentation
    /// This function is similar to @ref clingo_propagate_control_propagate() but is only called on total assignments without a change set.
    ///
    /// When exactly this function is called, can be configured using the @ref clingo_propagate_init_set_check_mode() function.
    ///
    /// **Note:** This function is called even if no watches have been added.
    ///
    /// **Parameters:**
    ///
    /// * `control` - control object for the target solver
    /// * `data` - user data for the callback
    ///
    /// **Returns** whether the call was successful
    /// @see ::clingo_propagator_check_callback_t
    fn check(_control: &mut PropagateControl, _data: &mut T) -> bool {
        true
    }
    #[doc(hidden)]
    unsafe extern "C" fn unsafe_init(
        init_: *mut clingo_propagate_init_t,
        data: *mut ::std::os::raw::c_void,
    ) -> bool {
        assert!(!init_.is_null());
        assert!(!data.is_null());
        let init = (init_ as *mut PropagateInit).as_mut().unwrap();
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
        let control = (control_ as *mut PropagateControl).as_mut().unwrap();
        let changes = std::slice::from_raw_parts(changes_ as *const Literal, size);
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
        let control = (control_ as *mut PropagateControl).as_mut().unwrap();
        let changes = std::slice::from_raw_parts(changes_ as *const Literal, size);
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
        let control = (control_ as *mut PropagateControl).as_mut().unwrap();
        let propagator = (data as *mut T).as_mut().unwrap();
        Self::check(control, propagator)
    }
}

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
    /// **Note:** Only gringo options (without `--output`) and clasp`s options are supported as
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if argument parsing fails
    pub fn new(arguments: std::vec::Vec<String>, message_limit: u32) -> Result<Control, Error> {
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
            Ok(Control {
                ctl: Unique::new(ctl).unwrap(),
            })
        } else {
            Err(error())
        }
    }

    /// Create a new control object.
    ///
    /// **Note:** Only gringo options (without `--output`) and clasp`s options are supported as arguments,
    /// except basic options such as `--help`.
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if argument parsing fails
    pub fn new_with_logger<D, T: Logger<D>>(
        arguments: std::vec::Vec<String>,
        _logger: &T,
        logger_data: &mut D,
        message_limit: u32,
    ) -> Result<Control, Error> {
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
                Some(T::unsafe_logging_callback as LoggingCallback),
                data as *mut ::std::os::raw::c_void,
                message_limit,
                &mut ctl,
            )
        } {
            Ok(Control {
                ctl: Unique::new(ctl).unwrap(),
            })
        } else {
            Err(error())
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
    /// * `name` name of the program block
    /// * `parameters` string array of parameters of the program block
    /// * `program` string representation of the program
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if parsing fails
    pub fn add(&mut self, name_: &str, parameters: Vec<&str>, program_: &str) -> Result<(), Error> {
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
            Err(error())
        }
    }

    /// Ground the selected [`Part`](struct.Part.html) parts of the current (non-ground) logic program.
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    ///
    /// **See:** [`Part`](struct.Part.html)
    pub fn ground(&mut self, sparts: &[Part]) -> Result<(), Error> {
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
            Err(error())
        }
    }

    /// Ground the selected [`Part`](struct.Part.html) parts of the current (non-ground) logic
    /// program.
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - error code of ground callback
    ///
    /// **See:** [`Part`](struct.Part.html)
    pub fn ground_with_event_handler<D, T: GroundEventHandler<D>>(
        &mut self,
        sparts: &[Part],
        _ground_callback: &T,
        ground_callback_data: &mut D,
    ) -> Result<(), Error> {
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
                Some(T::unsafe_ground_callback as GroundCallback),
                data as *mut ::std::os::raw::c_void,
            )
        } {
            Ok(())
        } else {
            Err(error())
        }
    }

    /// Solve the currently [`ground()`](struct.Control.html#method.ground) grounded logic program enumerating its models.
    ///
    /// See the [`SolveHandle`](struct.SolveHandle.html) module for more information.
    ///
    /// # Arguments
    ///
    /// * `mode` - configures the search mode
    /// * `assumptions` - array of assumptions to solve under
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if solving could not be started
    pub fn solve(
        &mut self,
        mode: &SolveMode,
        assumptions: &[SymbolicLiteral],
    ) -> Result<&mut SolveHandle, &'static str> {
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
            unsafe { (handle as *mut SolveHandle).as_mut() }
                .ok_or("Rust binding failed to dereference pointer to clingo solve handle")
        } else {
            Err(error_message())
        }
    }

    /// Solve the currently [`ground()`](struct.Control.html#method.ground) grounded logic program
    /// enumerating its models.
    ///
    /// See the [`SolveHandle`](struct.SolveHandle.html) module for more information.
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if solving could not be started
    pub fn solve_with_event_handler<D, T: SolveEventHandler<D>>(
        &mut self,
        mode: &SolveMode,
        assumptions: &[SymbolicLiteral],
        _notify: &T,
        data_: &mut D,
    ) -> Result<&mut SolveHandle, &'static str> {
        let mut handle = std::ptr::null_mut() as *mut clingo_solve_handle_t;
        let data = data_ as *mut D;
        if unsafe {
            clingo_control_solve(
                self.ctl.as_ptr(),
                mode.0,
                assumptions.as_ptr() as *const clingo_symbolic_literal_t,
                assumptions.len(),
                Some(T::unsafe_solve_callback as SolveEventCallback),
                data as *mut ::std::os::raw::c_void,
                &mut handle,
            )
        } {
            unsafe { (handle as *mut SolveHandle).as_mut() }
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn cleanup(&mut self) -> Result<(), Error> {
        if unsafe { clingo_control_cleanup(self.ctl.as_ptr()) } {
            Ok(())
        } else {
            Err(error())
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
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
            Err(error())
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn release_external(&mut self, Symbol(atom): Symbol) -> Result<(), Error> {
        if unsafe { clingo_control_release_external(self.ctl.as_ptr(), atom) } {
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
    /// See the [`Propagator`](struct.Propagator) module for more information.
    ///
    /// # Arguments
    ///
    /// * `propagator` - the propagator
    /// * `data` user data passed to the propagator functions
    /// * `sequential` - whether the propagator should be called sequentially
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn register_propagator<D, T: Propagator<D>>(
        &mut self,
        _propagator_builder: &T,
        data: &mut D,
        sequential: bool,
    ) -> Result<(), Error> {
        let propagator = clingo_propagator_t {
            init: Some(T::unsafe_init),
            propagate: Some(T::unsafe_propagate),
            undo: Some(T::unsafe_undo),
            check: Some(T::unsafe_check),
        };
        let data_ptr = data as *mut D;
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

    /// Get a statistics object to inspect solver statistics.
    ///
    /// Statistics are updated after a solve call.
    ///
    /// See the [`Statistics`](struct.Statistics.html) module for more information.
    ///
    /// **Attention:**
    /// The level of detail of the statistics depends on the stats option
    /// (which can be set using [`Configuration`](struct.Configuration.html) module or passed as an
    /// option when [`new()`](struct.Control.html#method.new)  creating the control object).
    /// The default level zero only provides basic statistics,
    /// level one provides extended and accumulated statistics,
    /// and level two provides per-thread statistics.
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn statistics(&mut self) -> Result<&mut Statistics, &'static str> {
        let mut stat = std::ptr::null_mut() as *mut clingo_statistics_t;
        if unsafe { clingo_control_statistics(self.ctl.as_ptr(), &mut stat) } {
            unsafe { (stat as *mut Statistics).as_mut() }
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
    /// See the [`Configuration`](struct.Configuration.html) module for more information.
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
    /// the solver`s various enumeration modes is removed after a solve call. This
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
    pub fn get_const(&mut self, name: &str) -> Option<Symbol> {
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
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if constant definition does not exist
    ///
    /// **See:** [`Part::get_const()`](struct.Part.html#method.get_const)
    pub fn has_const(&mut self, name: &str) -> Result<bool, Error> {
        let c_str_name = CString::new(name).unwrap();
        let mut exist = false;
        if unsafe { clingo_control_has_const(self.ctl.as_ptr(), c_str_name.as_ptr(), &mut exist) } {
            Ok(exist)
        } else {
            Err(error())
        }
    }

    /// Get an object to inspect symbolic atoms (the relevant Herbrand base) used
    /// for grounding.
    ///
    /// See the [`SymbolicAtoms`](struct.SymbolicAtoms.html) module for more information.
    pub fn symbolic_atoms(&mut self) -> Option<&mut SymbolicAtoms> {
        let mut atoms = std::ptr::null_mut() as *mut clingo_symbolic_atoms_t;
        if unsafe { clingo_control_symbolic_atoms(self.ctl.as_ptr(), &mut atoms) } {
            unsafe { (atoms as *mut SymbolicAtoms).as_mut() }
        } else {
            None
        }
    }

    /// Get an object to inspect theory atoms that occur in the grounding.
    ///
    /// See the [`TheoryAtoms`](struct.TheoryAtoms.html) module for more information.
    pub fn theory_atoms(&mut self) -> Option<&mut TheoryAtoms> {
        let mut atoms = std::ptr::null_mut() as *mut clingo_theory_atoms_t;
        if unsafe { clingo_control_theory_atoms(self.ctl.as_ptr(), &mut atoms) } {
            unsafe { (atoms as *mut TheoryAtoms).as_mut() }
        } else {
            None
        }
    }

    // TODO
    //     /// Register a program observer with the control object.
    //     ///
    //     /// **Parameters:**
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
    /// See the [`ProgramBuilder`](struct.ProgramBuilder.html) module for more information.
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn backend(&mut self) -> Option<&mut Backend> {
        let mut backend = std::ptr::null_mut();
        if unsafe { clingo_control_backend(self.ctl.as_ptr(), &mut backend) } {
            unsafe { (backend as *mut Backend).as_mut() }
        } else {
            None
        }
    }

    /// Get an object to add non-ground directives to the program.
    ///
    /// See the [`ProgramBuilder`](struct.ProgramBuilder.html) module for more information.
    pub fn program_builder(&mut self) -> Option<&mut ProgramBuilder> {
        let mut builder = std::ptr::null_mut() as *mut clingo_program_builder_t;
        if unsafe { clingo_control_program_builder(self.ctl.as_ptr(), &mut builder) } {
            unsafe { (builder as *mut ProgramBuilder).as_mut() }
        } else {
            None
        }
    }

    // NOTTODO: pub fn clingo_control_clasp_facade()
}

pub struct ProgramBuilder(clingo_program_builder_t);
impl ProgramBuilder {
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
    /// **Attention:** [`begin()`](struct.ProgramBuilder.html#method.begin) must be called before
    /// adding statements and [`end()`](struct.ProgramBuilder.html#method.end) must be called after
    /// all statements have been added.
    ///
    /// # Arguments
    ///
    /// * `statement` - the statement to add
    ///
    /// # Errors
    ///
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) for statements of invalid form
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn add(&mut self, statement: &AstStatement) -> Result<(), Error> {
        let AstStatement(ref stm) = *statement;
        if unsafe { clingo_program_builder_add(&mut self.0, stm) } {
            Ok(())
        } else {
            Err(error())
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
        (ast_rule_ptr as *const ast::Rule).as_ref().unwrap()
    }
}

    }

    pub fn location(&self) -> Location {
        Location(self.0.location)
    }

    }
}

pub struct Configuration(clingo_configuration_t);
impl Configuration {
    /// Get the root key of the configuration.
    pub fn root(&mut self) -> Option<Id> {
        let mut root_key = 0 as clingo_id_t;
        if unsafe { clingo_configuration_root(&mut self.0, &mut root_key) } {
            Some(Id(root_key))
        } else {
            None
        }
    }

    /// Get the type of a key.
    /// The type is bitset, an entry can have multiple (but at least one) type.
    pub fn configuration_type(&mut self, Id(key): Id) -> Option<ConfigurationType> {
        let mut ctype = 0 as clingo_configuration_type_bitset_t;
        if unsafe { clingo_configuration_type(&mut self.0, key, &mut ctype) } {
            Some(ConfigurationType(ctype))
        } else {
            None
        }
    }

    /// Get the description of an entry.
    pub fn description(&mut self, Id(key): Id) -> Option<&str> {
        let mut description_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_configuration_description(
                &mut self.0,
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
    /// The [`configuration_type()`](struct.Configuration.html#method.configuration_type) type of
    /// the entry must be  [`ConfigurationType::ARRAY`](struct.ConfigurationType.html#associatedconstant.ARRAY).
    pub fn array_size(&mut self, Id(key): Id) -> Option<usize> {
        let mut size = 0;
        if unsafe { clingo_configuration_array_size(&mut self.0, key, &mut size) } {
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
    /// The [`configuration_type()`](struct.Configuration.html#method.configuration_type) type of
    /// the entry must be [`ConfigurationType::ARRAY`](struct.ConfigurationType.html#associatedconstant.ARRAY).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset in the array
    pub fn array_at(&mut self, Id(key): Id, offset: usize) -> Option<Id> {
        let mut nkey = 0 as clingo_id_t;
        if unsafe { clingo_configuration_array_at(&mut self.0, key, offset, &mut nkey) } {
            Some(Id(nkey))
        } else {
            None
        }
    }

    /// Get the number of subkeys of a map entry.
    ///
    /// # Pre-condition
    ///
    /// The [`configuration_type()`](struct.Configuration.html#method.configuration_type) type of
    /// the entry must be [`ConfigurationType::MAP`](struct.ConfigurationType.html#associatedconstant.MAP).
    pub fn map_size(&mut self, Id(key): Id) -> Option<usize> {
        let mut size = 0;
        if unsafe { clingo_configuration_map_size(&mut self.0, key, &mut size) } {
            Some(size)
        } else {
            None
        }
    }

    /// Get the name associated with the offset-th subkey.
    ///
    /// # Pre-condition
    ///
    /// The [`configuration_type()`](struct.Configuration.html#method.configuration_type) type of
    /// the entry must be [`ConfigurationType::MAP`](struct.ConfigurationType.html#associatedconstant.MAP).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset of the name
    pub fn map_subkey_name(&mut self, Id(key): Id, offset: usize) -> Option<&str> {
        let mut name_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_configuration_map_subkey_name(
                &mut self.0,
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
    /// The [`configuration_type()`](struct.Configuration.html#method.configuration_type) type of
    /// the entry must be [`ConfigurationType::MAP`](struct.ConfigurationType.html#associatedconstant.MAP).
    ///
    /// **Note:** Multiple levels can be looked up by concatenating keys with a period.
    pub fn map_at(&mut self, Id(key): Id, name: &str) -> Option<Id> {
        let mut nkey = 0 as clingo_id_t;
        let name_c_str = CString::new(name).unwrap();
        if unsafe { clingo_configuration_map_at(&mut self.0, key, name_c_str.as_ptr(), &mut nkey) }
        {
            Some(Id(nkey))
        } else {
            None
        }
    }

    /// Check whether a entry has a value.
    ///
    /// # Pre-condition
    ///
    /// The [`configuration_type()`](struct.Configuration.html#method.configuration_type) type of
    /// the entry must be [`ConfigurationType::VALUE`](struct.ConfigurationType.html#associatedconstant.VALUE).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn value_is_assigned(&mut self, Id(key): Id) -> Option<bool> {
        let mut assigned = false;
        if unsafe { clingo_configuration_value_is_assigned(&mut self.0, key, &mut assigned) } {
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
    /// The [`configuration_type()`](struct.Configuration.html#method.configuration_type) type of
    /// the entry must be [`ConfigurationType::VALUE`](struct.ConfigurationType.html#associatedconstant.VALUE).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    pub fn value_get(&mut self, Id(key): Id) -> Option<&str> {
        let mut size = 0;
        if unsafe { clingo_configuration_value_get_size(&mut self.0, key, &mut size) } {
            let mut value_ptr = unsafe { mem::uninitialized() };
            if unsafe { clingo_configuration_value_get(&mut self.0, key, &mut value_ptr, size) } {
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
    /// The [`configuration_type()`](struct.Configuration.html#method.configuration_type) type of
    /// the entry must be [`ConfigurationType::VALUE`](struct.ConfigurationType.html#associatedconstant.VALUE).
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn external(&mut self, atom: &Atom, type_: ExternalType) -> Result<(), Error> {
        if unsafe { clingo_backend_external(&mut self.0, atom.0, type_ as clingo_external_type_t) }
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
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
            Err(error())
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
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
            Err(error())
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

pub struct Statistics(clingo_statistics_t);
impl Statistics {
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
    pub fn statistics_type(&mut self, key: u64) -> Option<StatisticsType> {
        let mut stype = 0 as clingo_statistics_type_t;
        if unsafe { clingo_statistics_type(&mut self.0, key, &mut stype) } {
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
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Array`](enum.StatisticsType.html#variant.Array).
    ///
    /// # Arguments
    ///
    /// * `key` - the key
    /// * `offset` - the offset in the array
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
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must
    /// be [`StatisticsType::Map`](enum.StatisticsType.html#variant.Map).
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
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Map`](enum.StatisticsType.html#variant.Map).
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
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Map`](enum.StatisticsType.html#variant.Map).
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
    /// The [statistics type](struct.Statistics.html#method.statistics_type) of the entry must be
    /// [`StatisticsType::Value`](enum.StatisticsType.html#variant.Value).
    pub fn value_get(&mut self, key: u64) -> Option<f64> {
        let mut value = 0.0 as f64;
        if unsafe { clingo_statistics_value_get(&mut self.0, key, &mut value) } {
            Some(value)
        } else {
            None
        }
    }
}

/// Object to inspect symbolic atoms in a program---the relevant Herbrand base
/// gringo uses to instantiate programs.
///
/// @see clingo_control_symbolic_atoms()
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
    /// * `signature` optional signature
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
    /// * `iterator` iterator pointing to the symbolic atom or to the end
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
    /// * `iterator` iterator to the atom
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
    /// grounding or solving component`s simplifications can only detect this in
    /// some cases.
    ///
    /// # Arguments
    ///
    /// * `iterator` iterator to the atom
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
    /// * `iterator` iterator to the atom
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
    /// Such a literal can be mapped to a solver literal (see the \ref Propagator
    /// module) or be used in rules in aspif format (see the \ref ProgramBuilder
    /// module).
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if the size is too small
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
                Err(error())
            }
        } else {
            Err(error())
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

pub struct TheoryAtoms(clingo_theory_atoms_t);
impl TheoryAtoms {
    /// Get the type of the given theory term.
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    // TODO ? is this needed in an Rust API
    pub fn term_type(&mut self, Id(term): Id) -> Option<TermType> {
        let mut ttype = 0 as clingo_theory_term_type_t;
        if unsafe { clingo_theory_atoms_term_type(&mut self.0, term, &mut ttype) } {
            match ttype as u32 {
                clingo_theory_term_type_clingo_theory_term_type_tuple => Some(TermType::Tuple),
                clingo_theory_term_type_clingo_theory_term_type_list => Some(TermType::List),
                clingo_theory_term_type_clingo_theory_term_type_set => Some(TermType::Set),
                clingo_theory_term_type_clingo_theory_term_type_function => {
                    Some(TermType::Function)
                }
                clingo_theory_term_type_clingo_theory_term_type_number => Some(TermType::Number),
                clingo_theory_term_type_clingo_theory_term_type_symbol => Some(TermType::Symbol),
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
    /// The term must be of type [`TermType::Number](enum.TermType.html#variant.Number) .
    ///
    /// # Arguments
    ///
    /// * `term` - id of the term
    pub fn term_number(&mut self, Id(term): Id) -> Option<i32> {
        let mut number = 0;
        if unsafe { clingo_theory_atoms_term_number(&mut self.0, term, &mut number) } {
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
    pub fn term_name<'a>(&mut self, Id(term): Id) -> Option<&'a str> {
        let mut char_ptr = std::ptr::null() as *const c_char;
        if unsafe { clingo_theory_atoms_term_name(&mut self.0, term, &mut char_ptr) } {
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
    pub fn term_arguments(&mut self, Id(term): Id) -> Option<Vec<Id>> {
        let mut size = 0;
        let mut c_ptr = unsafe { mem::uninitialized() };
        if unsafe { clingo_theory_atoms_term_arguments(&mut self.0, term, &mut c_ptr, &mut size) } {
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
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if the size is too small
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn term_to_string(&mut self, Id(term): Id) -> Result<&str, Error> {
        let mut size = 0;
        if unsafe { clingo_theory_atoms_term_to_string_size(&mut self.0, term, &mut size) } {
            let mut c_ptr = unsafe { mem::uninitialized() };
            if unsafe { clingo_theory_atoms_term_to_string(&mut self.0, term, &mut c_ptr, size) } {
                let cstr = unsafe { CStr::from_ptr(&c_ptr) };
                Ok(cstr.to_str().unwrap())
            } else {
                Err(error())
            }
        } else {
            Err(error())
        }
    }

    /// Get the tuple (array of theory terms) of the given theory element.
    ///
    /// # Arguments
    ///
    /// * `element` - id of the element
    pub fn element_tuple(&mut self, Id(element): Id) -> Option<Vec<Id>> {
        let mut size = 0;
        let mut tuple_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_theory_atoms_element_tuple(&mut self.0, element, &mut tuple_ptr, &mut size)
        } {
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
    pub fn element_condition(&mut self, Id(element): Id) -> Option<Vec<Literal>> {
        let mut size = 0;
        let mut condition_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_theory_atoms_element_condition(
                &mut self.0,
                element,
                &mut condition_ptr,
                &mut size,
            )
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
    pub fn element_condition_id(&mut self, Id(element): Id) -> Option<Literal> {
        let mut condition = unsafe { mem::uninitialized() };
        if unsafe { clingo_theory_atoms_element_condition_id(&mut self.0, element, &mut condition) }
        {
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
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if the size is too small
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn element_to_string(&mut self, Id(element): Id) -> Result<&str, Error> {
        let mut size = 0;
        if unsafe { clingo_theory_atoms_element_to_string_size(&mut self.0, element, &mut size) } {
            let mut c_ptr = unsafe { mem::uninitialized() };
            if unsafe {
                clingo_theory_atoms_element_to_string(&mut self.0, element, &mut c_ptr, size)
            } {
                let cstr = unsafe { CStr::from_ptr(&c_ptr) };
                Ok(cstr.to_str().unwrap())
            } else {
                Err(error())
            }
        } else {
            Err(error())
        }
    }

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
    pub fn atom_term(&mut self, Id(atom): Id) -> Option<Id> {
        let mut term = 0 as clingo_id_t;
        if unsafe { clingo_theory_atoms_atom_term(&mut self.0, atom, &mut term) } {
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
    pub fn atom_elements(&mut self, Id(atom): Id) -> Option<Vec<Id>> {
        let mut size = 0;
        let mut elements_ptr = unsafe { mem::uninitialized() };
        if unsafe {
            clingo_theory_atoms_atom_elements(&mut self.0, atom, &mut elements_ptr, &mut size)
        } {
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
    pub fn atom_has_guard(&mut self, Id(atom): Id) -> Option<bool> {
        let mut has_guard = false;
        if unsafe { clingo_theory_atoms_atom_has_guard(&mut self.0, atom, &mut has_guard) } {
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
    pub fn atom_guard(&mut self, Id(atom): Id) -> Option<(&str, Id)> {
        let mut c_ptr = unsafe { mem::uninitialized() };
        let mut term = 0 as clingo_id_t;
        if unsafe { clingo_theory_atoms_atom_guard(&mut self.0, atom, &mut c_ptr, &mut term) } {
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
    pub fn atom_literal(&mut self, Id(atom): Id) -> Option<Literal> {
        let mut literal = 0 as clingo_literal_t;
        if unsafe { clingo_theory_atoms_atom_literal(&mut self.0, atom, &mut literal) } {
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
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if the size is too small
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn atom_to_string(&mut self, Id(atom): Id) -> Result<&str, Error> {
        let mut size = 0;
        if unsafe { clingo_theory_atoms_atom_to_string_size(&mut self.0, atom, &mut size) } {
            let mut c_ptr = unsafe { mem::uninitialized() };
            if unsafe { clingo_theory_atoms_atom_to_string(&mut self.0, atom, &mut c_ptr, size) } {
                let cstr = unsafe { CStr::from_ptr(&c_ptr) };
                Ok(cstr.to_str().unwrap())
            } else {
                Err(error())
            }
        } else {
            Err(error())
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
    pub fn from(cta: &mut TheoryAtoms) -> UNSAFE_TheoryAtomsIterator {
        UNSAFE_TheoryAtomsIterator {
            count: 0,
            size: cta.size().unwrap(),
        }
    }
}

pub struct Model(clingo_model_t);
impl Model {
    /// Get the type of the model.
    pub fn model_type(&mut self) -> Option<ModelType> {
        let mut mtype = 0 as clingo_model_type_t;
        if unsafe { clingo_model_type(&mut self.0, &mut mtype) } {
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
    pub fn number(&mut self) -> Option<u64> {
        let mut number = 0;
        if unsafe { clingo_model_number(&mut self.0, &mut number) } {
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if the size is too small
    pub fn symbols(&mut self, show: &ShowType) -> Result<Vec<Symbol>, Error> {
        let Model(ref mut model) = *self;
        let mut size: usize = 0;
        if unsafe { clingo_model_symbols_size(model, show.0, &mut size) } {
            let symbols = Vec::<Symbol>::with_capacity(size);
            let symbols_ptr = symbols.as_ptr();
            if unsafe {
                clingo_model_symbols(model, show.0, symbols_ptr as *mut clingo_symbol_t, size)
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
    pub fn contains(&mut self, Symbol(atom): Symbol) -> Option<bool> {
        let mut contained = false;
        if unsafe { clingo_model_contains(&mut self.0, atom, &mut contained) } {
            Some(contained)
        } else {
            None
        }
    }

    //NOTTODO: pub fn clingo_model_cost_size(model: *mut Model, size: *mut size_t) -> u8;

    /// Get the cost vector of a model.
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if the size is too small
    ///
    /// **See:** [`Model::optimality_proven()`](struct.Model.html#method.optimality_proven)
    pub fn cost(&mut self) -> Result<Vec<i64>, Error> {
        let mut size: usize = 0;
        if unsafe { clingo_model_cost_size(&mut self.0, &mut size) } {
            let cost = Vec::<i64>::with_capacity(size);
            let cost_ptr = cost.as_ptr();
            if unsafe { clingo_model_cost(&mut self.0, cost_ptr as *mut i64, size) } {
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
    pub fn optimality_proven(&mut self) -> Option<bool> {
        let mut proven = false;
        if unsafe { clingo_model_optimality_proven(&mut self.0, &mut proven) } {
            Some(proven)
        } else {
            None
        }
    }

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

pub struct SolveControl(clingo_solve_control_t);
impl SolveControl {
    /// Add a clause that applies to the current solving step during model
    /// enumeration.
    ///
    /// **Note:** The [`Propagator`](enum.Propagator.html) module provides a more sophisticated
    /// interface to add clauses - even on partial assignments.
    ///
    /// # Arguments
    ///
    /// * `clause` array of literals representing the clause
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if adding the clause fails
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
            Err(error())
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

pub struct Assignment(clingo_assignment_t);

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

    /// Add the given clause to the solver.
    ///
    /// This method sets its result to false if the current propagation must be stopped for the solver to backtrack.
    ///
    /// **Attention:** No further calls on the control object or functions on the assignment should be called when the result of this method is false.
    ///
    /// # Arguments
    ///
    /// * `clause` - the clause to add
    /// * `type` - the clause type determining its lifetime
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn add_clause(&mut self, clause: &[Literal], type_: ClauseType) -> Result<bool, Error> {
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    pub fn propagate(&mut self) -> Result<bool, Error> {
        let mut result = false;
        if unsafe { clingo_propagate_control_propagate(&mut self.0, &mut result) } {
            Ok(result)
        } else {
            Err(error())
        }
    }
}

pub struct PropagateInit(clingo_propagate_init_t);
impl PropagateInit {
    /// Map the given program literal or condition id to its solver literal.
    ///
    /// # Arguments
    ///
    /// * `aspif_literal` - the aspif literal to map
    ///
    /// **Returns** the corresponding solver literal
    pub fn solver_literal(&mut self, Literal(aspif_literal): Literal) -> Option<Literal> {
        let mut solver_literal = 0 as clingo_literal_t;
        if unsafe {
            clingo_propagate_init_solver_literal(&mut self.0, aspif_literal, &mut solver_literal)
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
    pub fn symbolic_atoms<'a>(&mut self) -> Option<&'a mut SymbolicAtoms> {
        let mut atoms_ptr = unsafe { std::mem::uninitialized() };
        if unsafe { clingo_propagate_init_symbolic_atoms(&mut self.0, &mut atoms_ptr) } {
            unsafe { (atoms_ptr as *mut SymbolicAtoms).as_mut() }
        } else {
            None
        }
    }

    /// Get an object to inspect the theory atoms.
    pub fn theory_atoms(&mut self) -> Option<&mut TheoryAtoms> {
        let mut atoms_ptr = unsafe { std::mem::uninitialized() };
        if unsafe { clingo_propagate_init_theory_atoms(&mut self.0, &mut atoms_ptr) } {
            unsafe { (atoms_ptr as *mut TheoryAtoms).as_mut() }
        } else {
            None
        }
    }

    /// Get the number of threads used in subsequent solving.
    /// **See:** [`PropagateControl::thread_id()`](struct.PropagateControl.html#method.thread_id)
    pub fn number_of_threads(&mut self) -> usize {
        (unsafe { clingo_propagate_init_number_of_threads(&mut self.0) } as usize)
    }
}

pub struct SolveHandle(clingo_solve_handle);
impl SolveHandle {
    /// Get the next solve result.
    ///
    /// Blocks until the result is ready.
    /// When yielding partial solve results can be obtained, i.e.,
    /// when a model is ready, the result will be satisfiable but neither the search exhausted nor the optimality proven.
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if solving fails
    pub fn get(&mut self) -> Result<SolveResult, Error> {
        let mut result = 0;
        if unsafe { clingo_solve_handle_get(&mut self.0, &mut result) } {
            Ok(SolveResult(result))
        } else {
            Err(error())
        }
    }

    /// Get the next model (or zero if there are no more models).
    /// (it is NULL if there are no more models)
    ///
    /// # Errors
    ///
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if solving fails
    pub fn model(&mut self) -> Result<&mut Model, &'static str> {
        let SolveHandle(ref mut handle) = *self;
        let mut model = std::ptr::null_mut() as *mut clingo_model_t;
        if unsafe { clingo_solve_handle_model(handle, &mut model) } {
            unsafe { (model as *mut Model).as_mut() }
                .ok_or("Rust binding failed to dereference pointer to clingo model")
        } else {
            Err(error_message())
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if solving fails
    pub fn resume(&mut self) -> Result<(), Error> {
        let SolveHandle(ref mut handle) = *self;
        if unsafe { clingo_solve_handle_resume(handle) } {
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
    /// - [`Error::BadAlloc`](enum.Error.html#variant.BadAlloc)
    /// - [`Error::Runtime`](enum.Error.html#variant.Runtime) if solving fails
    pub fn close(&mut self) -> Result<(), Error> {
        let SolveHandle(ref mut handle) = *self;
        if unsafe { clingo_solve_handle_close(handle) } {
            Ok(())
        } else {
            Err(error())
        }
    }
}

//TODO     /// Internalize a string.
//     ///
//     /// This functions takes a string as input and returns an equal unique string
//     /// that is (at the moment) not freed until the program is closed.  All strings
//     /// returned from clingo API functions are internalized and must not be freed.
//     ///
//     /// **Parameters:**
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
//     /// **Parameters:**
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

// TODO clingo_ground_program_observer

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
        assert!(SymbolType::Infimum == sym.get_type().unwrap());
    }
}
