use crate::{
    internalize_string, set_internal_error, ClingoError, ControlCtx, ErrorType, ExternalType,
    GenericControl, Symbol,
};

use crate::ast_internals::Body;
use crate::ast_internals::{ASTType, AST};
use clingo_sys::*;
use std::ffi::CStr;
use std::ffi::NulError;
use std::str::Utf8Error;
use std::{
    ffi::CString,
    marker::PhantomData,
    os::raw::{c_char, c_void},
    ptr::NonNull,
};
use vec1::Vec1;

/// Represents a source code location marking its beginning and end.
///
/// **Note:** Not all locations refer to physical files.
/// By convention, such locations use a name put in angular brackets as filename.
/// The string members of a location object are internalized and valid for the duration of the process.
#[derive(Debug, Copy, Clone)]
pub struct Location(pub(crate) clingo_location);
impl Default for Location {
    /// Create a default location.
    fn default() -> Self {
        let file = CString::new("").unwrap();
        Location(clingo_location {
            begin_line: 0,
            end_line: 0,
            begin_column: 0,
            end_column: 0,
            begin_file: file.as_ptr(),
            end_file: file.as_ptr(),
        })
    }
}
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
    pub fn begin_file(&self) -> Result<&'static str, Utf8Error> {
        if self.0.begin_file.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.begin_file) };
            c_str.to_str()
        }
    }
    /// the file where the location ends
    pub fn end_file(&self) -> Result<&'static str, Utf8Error> {
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

/// Object to build non-ground programs.
pub struct ProgramBuilder<'a> {
    pub(crate) theref: &'a mut clingo_program_builder_t,
}
impl<'a> ProgramBuilder<'a> {
    /// Get an object to add non-ground directives to the program.
    pub fn from<C: ControlCtx>(
        ctl: &'a mut GenericControl<C>,
    ) -> Result<ProgramBuilder<'a>, ClingoError> {
        let mut builder = std::ptr::null_mut();
        if !unsafe { clingo_program_builder_init(ctl.ctl.as_mut(), &mut builder) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_control_program_builder() failed.",
            ));
        }
        // begin building the program
        if !unsafe { clingo_program_builder_begin(builder) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_program_builder_begin() failed",
            ));
        }
        match unsafe { builder.as_mut() } {
            Some(builder_ref) => Ok(ProgramBuilder {
                theref: builder_ref,
            }),
            None => Err(ClingoError::FFIError {
                msg: "tried casting a null pointer to &mut clingo_program_builder.",
            }),
        }
    }
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
    /// - [`ClingoError`](struct.ClingoError.html) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) for statements of invalid form
    /// or [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
    pub fn add(&mut self, stm: &Statement) -> Result<(), ClingoError> {
        if !unsafe { clingo_program_builder_add(self.theref, stm.ast.ptr.as_ptr()) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_program_builder_add() failed",
            ));
        }
        Ok(())
    }

    /// End building a program.
    /// The method consumes the program builder.
    pub fn end(self) -> Result<(), ClingoError> {
        if !unsafe { clingo_program_builder_end(self.theref) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_program_builder_end() failed",
            ));
        }
        Ok(())
    }
}

// #[doc = "! Callback function to intercept AST nodes."]
// #[doc = "!"]
// #[doc = "! @param[in] ast the AST"]
// #[doc = "! @param[in] data a user data pointer"]
// #[doc = "! @return whether the call was successful"]
// pub type clingo_ast_callback_v2_t = ::std::option::Option<
//     unsafe extern "C" fn(ast: *mut clingo_ast_t, data: *mut ::std::os::raw::c_void) -> bool,
// >;

type ASTCallback = unsafe extern "C" fn(ast: *mut clingo_ast_t, data: *mut c_void) -> bool;

// Parse the given program and return an abstract syntax tree for each statement via a callback.
//
// **Note** The control object can be set to a NULL to disable reading input in aspif format.
//
// - program: the program in gringo syntax
// - callback: the callback reporting statements
// - callback_data: user data for the callback
// - control: object to add ground statements to
// - logger: callback to report messages during parsing
// - logger_data: user data for the logger
// - message_limit: the maximum number of times the logger is called
// *Returns* whether the call was successful
//
// # Errors
//
// - [`ClingoError`](struct.ClingoError.html) with [`ErrorCode::Runtime`](enum.ErrorCode.html#variant.Runtime) for statements of invalid form
// or [`ErrorCode::BadAlloc`](enum.ErrorCode.html#variant.BadAlloc)
pub fn parse_string_with_statement_handler<
    // C: ControlCtx,
    T: StatementHandler,
>(
    // ctl: &mut GenericControl<C>,
    program: &str,
    handler: &mut T,
) -> Result<(), ClingoError> {
    let logger = None;
    let logger_data = std::ptr::null_mut();
    let program = CString::new(program)?;
    let handler = handler as *mut T;
    if !unsafe {
        clingo_ast_parse_string(
            program.as_ptr(),
            Some(unsafe_ast_callback::<T> as ASTCallback),
            handler as *mut c_void,
            // ctl.ctl.as_mut(),
            std::ptr::null_mut(),
            logger,
            logger_data,
            0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_parse_string() failed",
        ));
    }
    Ok(())
}
pub trait StatementHandler {
    /// Callback function called on an ast statement while traversing the ast.
    ///
    /// **Returns** whether the call was successful
    fn on_statement(&mut self, ast: &Statement) -> bool;
}
unsafe extern "C" fn unsafe_ast_callback<T: StatementHandler>(
    ast: *mut clingo_ast_t,
    event_handler: *mut c_void,
) -> bool {
    // check for null pointers
    if ast.is_null() | event_handler.is_null() {
        set_internal_error(
            ErrorType::Runtime,
            "unsafe_ast_callback() got a null pointer.",
        );
        return false;
    }

    let event_handler = &mut *(event_handler as *mut T);

    let ast = match NonNull::new(ast) {
        Some(x) => AST {
            ptr: x,
            _lifetime: PhantomData,
        },
        None => panic!("NonNull::new(ast) returned None"),
    };
    ast.acquire();
    let stm = match ast.get_type() {
        Ok(ASTType::Rule) => Statement { ast },
        Ok(ASTType::Definition) => Statement { ast },
        Ok(ASTType::ShowSignature) => Statement { ast },
        Ok(ASTType::Defined) => Statement { ast },
        Ok(ASTType::ShowTerm) => Statement { ast },
        Ok(ASTType::Minimize) => Statement { ast },
        Ok(ASTType::Script) => Statement { ast },
        Ok(ASTType::Program) => Statement { ast },
        Ok(ASTType::External) => Statement { ast },
        Ok(ASTType::Edge) => Statement { ast },
        Ok(ASTType::Heuristic) => Statement { ast },
        Ok(ASTType::ProjectAtom) => Statement { ast },
        Ok(ASTType::ProjectSignature) => Statement { ast },
        x => panic!("unexpected ASTType: {:?}", x),
    };
    event_handler.on_statement(&stm)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sign {
    /// For positive literals.
    NoSign = clingo_ast_sign_e_clingo_ast_sign_no_sign as isize,
    ///  For negative literals (prefix `not`s).
    Negation = clingo_ast_sign_e_clingo_ast_sign_negation as isize,
    /// For double negated literals (prefix `not not`).
    DoubleNegation = clingo_ast_sign_e_clingo_ast_sign_double_negation as isize,
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of comparison relations
pub enum ComparisonOperator {
    /// Operator `>`.
    GreaterThan =
        clingo_ast_comparison_operator_e_clingo_ast_comparison_operator_greater_than as isize,
    /// Operator `<`.
    LessThan = clingo_ast_comparison_operator_e_clingo_ast_comparison_operator_less_than as isize,
    /// Operator `<=`.
    LessEqual = clingo_ast_comparison_operator_e_clingo_ast_comparison_operator_less_equal as isize,
    /// Operator `>=`.
    GreaterEqual =
        clingo_ast_comparison_operator_e_clingo_ast_comparison_operator_greater_equal as isize,
    /// Operator `!=`.
    NotEqual = clingo_ast_comparison_operator_e_clingo_ast_comparison_operator_not_equal as isize,
    /// Operator `==`.
    Equal = clingo_ast_comparison_operator_e_clingo_ast_comparison_operator_equal as isize,
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of unary operators.
pub enum UnaryOperator {
    /// Operator `-`.
    Minus = clingo_ast_unary_operator_e_clingo_ast_unary_operator_minus as isize,
    /// Operator `~`.
    Negation = clingo_ast_unary_operator_e_clingo_ast_unary_operator_negation as isize,
    /// Operator `|.|`.
    Absolute = clingo_ast_unary_operator_e_clingo_ast_unary_operator_absolute as isize,
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of binary operators.
pub enum BinaryOperator {
    /// Operator `^`.
    Xor = clingo_ast_binary_operator_e_clingo_ast_binary_operator_xor as isize,
    /// Operator `?`.
    Or = clingo_ast_binary_operator_e_clingo_ast_binary_operator_or as isize,
    /// Operator `&`.
    And = clingo_ast_binary_operator_e_clingo_ast_binary_operator_and as isize,
    /// Operator `+`.
    Plus = clingo_ast_binary_operator_e_clingo_ast_binary_operator_plus as isize,
    /// Operator `-`.
    Minus = clingo_ast_binary_operator_e_clingo_ast_binary_operator_minus as isize,
    /// Operator `*`.
    Multiplication =
        clingo_ast_binary_operator_e_clingo_ast_binary_operator_multiplication as isize,
    /// Operator `/`.
    Division = clingo_ast_binary_operator_e_clingo_ast_binary_operator_division as isize,
    /// Operator `\`.
    Modulo = clingo_ast_binary_operator_e_clingo_ast_binary_operator_modulo as isize,
    /// Operator `**`.
    Power = clingo_ast_binary_operator_e_clingo_ast_binary_operator_power as isize,
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of aggregate functions.
pub enum AggregateFunction {
    Count = clingo_ast_aggregate_function_e_clingo_ast_aggregate_function_count as isize,
    Sum = clingo_ast_aggregate_function_e_clingo_ast_aggregate_function_sum as isize,
    Sump = clingo_ast_aggregate_function_e_clingo_ast_aggregate_function_sump as isize,
    Min = clingo_ast_aggregate_function_e_clingo_ast_aggregate_function_min as isize,
    Max = clingo_ast_aggregate_function_e_clingo_ast_aggregate_function_max as isize,
}

#[derive(Debug, Copy, Clone)]
pub enum TheoryTermSequenceType {
    /// For theory tuples `(t1,...,tn)`.
    Tuple = clingo_ast_theory_sequence_type_e_clingo_ast_theory_sequence_type_tuple as isize,
    /// For theory lists `[t1,...,tn]`.
    List = clingo_ast_theory_sequence_type_e_clingo_ast_theory_sequence_type_list as isize,
    /// for theory sets `{t1,...,tn}`.
    Set = clingo_ast_theory_sequence_type_e_clingo_ast_theory_sequence_type_set as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum TheoryOperatorType {
    /// A left associative binary operator.
    BinaryLeft =
        clingo_ast_theory_operator_type_e_clingo_ast_theory_operator_type_binary_left as isize,
    /// A right associative binary operator.
    BinaryRight =
        clingo_ast_theory_operator_type_e_clingo_ast_theory_operator_type_binary_right as isize,
    /// An unary theory operator.
    Unary = clingo_ast_theory_operator_type_e_clingo_ast_theory_operator_type_unary as isize,
}

#[derive(Debug, Copy, Clone)]
/// Enumeration of the theory atom types.
pub enum TheoryAtomType {
    /// For theory atoms that can appear in the head.
    Head = clingo_ast_theory_atom_definition_type_e_clingo_ast_theory_atom_definition_type_head
        as isize,
    /// For theory atoms that can appear in the body.
    Body = clingo_ast_theory_atom_definition_type_e_clingo_ast_theory_atom_definition_type_body
        as isize,
    /// For theory atoms that can appear in both head and body.
    Any = clingo_ast_theory_atom_definition_type_e_clingo_ast_theory_atom_definition_type_any
        as isize,
    /// For theory atoms that must not have a body.
    Directive =
        clingo_ast_theory_atom_definition_type_e_clingo_ast_theory_atom_definition_type_directive
            as isize,
}

// Here start the ASTTypes

#[derive(Debug, Clone)]
pub struct Term<'a> {
    pub(crate) ast: AST<'a>,
}
impl<'a> Term<'a> {
    pub fn is_a(self) -> Result<TermIsA<'a>, ClingoError> {
        match self.ast.get_type()? {
            ASTType::Variable => Ok(TermIsA::Variable(Variable { ast: self.ast })),
            ASTType::SymbolicTerm => Ok(TermIsA::SymbolicTerm(SymbolicTerm { ast: self.ast })),
            ASTType::UnaryOperation => {
                Ok(TermIsA::UnaryOperation(UnaryOperation { ast: self.ast }))
            }
            ASTType::BinaryOperation => {
                Ok(TermIsA::BinaryOperation(BinaryOperation { ast: self.ast }))
            }
            ASTType::Interval => Ok(TermIsA::Interval(Interval { ast: self.ast })),
            ASTType::Function => Ok(TermIsA::Function(Function { ast: self.ast })),
            ASTType::Pool => Ok(TermIsA::Pool(Pool { ast: self.ast })),
            x => panic!("unexpected ASTType: {:?}", x),
        }
    }
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
    pub fn location(&self) -> Location {
        self.ast.location().unwrap()
    }
}
impl<'a> From<Variable<'a>> for Term<'a> {
    fn from(x: Variable<'a>) -> Self {
        Term { ast: x.ast }
    }
}
impl<'a> From<SymbolicTerm<'a>> for Term<'a> {
    fn from(x: SymbolicTerm<'a>) -> Self {
        Term { ast: x.ast }
    }
}
impl<'a> From<Function<'a>> for Term<'a> {
    fn from(x: Function<'a>) -> Self {
        Term { ast: x.ast }
    }
}
impl<'a> From<UnaryOperation<'a>> for Term<'a> {
    fn from(x: UnaryOperation<'a>) -> Self {
        Term { ast: x.ast }
    }
}
impl<'a> From<BinaryOperation<'a>> for Term<'a> {
    fn from(x: BinaryOperation<'a>) -> Self {
        Term { ast: x.ast }
    }
}
impl<'a> From<Interval<'a>> for Term<'a> {
    fn from(x: Interval<'a>) -> Self {
        Term { ast: x.ast }
    }
}
impl<'a> From<Pool<'a>> for Term<'a> {
    fn from(x: Pool<'a>) -> Self {
        Term { ast: x.ast }
    }
}
#[derive(Debug, Clone)]
pub enum TermIsA<'a> {
    Variable(Variable<'a>),
    SymbolicTerm(SymbolicTerm<'a>),
    UnaryOperation(UnaryOperation<'a>),
    BinaryOperation(BinaryOperation<'a>),
    Interval(Interval<'a>),
    Function(Function<'a>),
    Pool(Pool<'a>),
}
#[derive(Debug, Clone)]
pub struct Literal<'a> {
    ast: AST<'a>,
}
impl<'a> Literal<'a> {
    pub fn is_a(self) -> Result<BasicLiteral<'a>, ClingoError> {
        match self.ast.get_type()? {
            ASTType::Literal => Ok(BasicLiteral { ast: self.ast }),
            x => panic!("unexpected ASTType: {:?}", x),
        }
    }
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}
use std::fmt;
impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for Variable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for BasicLiteral<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for TheoryTerm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for Rule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for Head<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for BodyLiteral<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for AtomicLiteral<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for TheorySequence<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ast.fmt(f)
    }
}
impl<'a> From<BasicLiteral<'a>> for Literal<'a> {
    fn from(x: BasicLiteral<'a>) -> Self {
        Literal { ast: x.ast }
    }
}
impl<'a> From<AtomicLiteral<'a>> for Literal<'a> {
    fn from(x: AtomicLiteral<'a>) -> Self {
        Literal { ast: x.ast }
    }
}
#[derive(Debug, Clone)]
pub struct Head<'a> {
    pub(crate) ast: AST<'a>,
}
impl<'a> From<BasicLiteral<'a>> for Head<'a> {
    fn from(x: BasicLiteral<'a>) -> Self {
        Head { ast: x.ast }
    }
}
impl<'a> From<AtomicLiteral<'a>> for Head<'a> {
    fn from(x: AtomicLiteral<'a>) -> Self {
        Head { ast: x.ast }
    }
}
impl<'a> From<Literal<'a>> for Head<'a> {
    fn from(x: Literal<'a>) -> Self {
        Head { ast: x.ast }
    }
}
impl<'a> From<Aggregate<'a>> for Head<'a> {
    fn from(x: Aggregate<'a>) -> Self {
        Head { ast: x.ast }
    }
}
impl<'a> From<HeadAggregate<'a>> for Head<'a> {
    fn from(x: HeadAggregate<'a>) -> Self {
        Head { ast: x.ast }
    }
}
impl<'a> From<Disjunction<'a>> for Head<'a> {
    fn from(x: Disjunction<'a>) -> Self {
        Head { ast: x.ast }
    }
}
impl<'a> From<TheoryAtom<'a>> for Head<'a> {
    fn from(x: TheoryAtom<'a>) -> Self {
        Head { ast: x.ast }
    }
}
impl<'a> Head<'a> {
    pub fn is_a(self) -> Result<HeadIsA<'a>, ClingoError> {
        match self.ast.get_type()? {
            ASTType::Literal => Ok(HeadIsA::Literal(Literal { ast: self.ast })),
            ASTType::HeadAggregate => Ok(HeadIsA::HeadAggregate(HeadAggregate { ast: self.ast })),
            ASTType::Disjunction => Ok(HeadIsA::Disjunction(Disjunction { ast: self.ast })),
            ASTType::TheoryAtom => Ok(HeadIsA::TheoryAtom(TheoryAtom { ast: self.ast })),
            ASTType::Aggregate => Ok(HeadIsA::Aggregate(Aggregate { ast: self.ast })),
            x => panic!("unexpected ASTType for Head: {:?}", x),
        }
    }
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}
#[derive(Debug, Clone)]
pub enum HeadIsA<'a> {
    Literal(Literal<'a>),
    Aggregate(Aggregate<'a>),
    HeadAggregate(HeadAggregate<'a>),
    Disjunction(Disjunction<'a>),
    TheoryAtom(TheoryAtom<'a>),
}
#[derive(Debug, Clone)]
pub struct BodyLiteral<'a> {
    pub(crate) ast: AST<'a>,
}
impl<'a> From<BasicLiteral<'a>> for BodyLiteral<'a> {
    fn from(x: BasicLiteral<'a>) -> Self {
        BodyLiteral { ast: x.ast }
    }
}
impl<'a> From<Literal<'a>> for BodyLiteral<'a> {
    fn from(x: Literal<'a>) -> Self {
        BodyLiteral { ast: x.ast }
    }
}
impl<'a> From<ConditionalLiteral<'a>> for BodyLiteral<'a> {
    fn from(x: ConditionalLiteral<'a>) -> Self {
        BodyLiteral { ast: x.ast }
    }
}
impl<'a> From<AtomicLiteral<'a>> for BodyLiteral<'a> {
    fn from(x: AtomicLiteral<'a>) -> Self {
        BodyLiteral { ast: x.ast }
    }
}
impl<'a> From<TheoryAtom<'a>> for BodyLiteral<'a> {
    fn from(x: TheoryAtom<'a>) -> Self {
        BodyLiteral { ast: x.ast }
    }
}
#[derive(Debug, Clone)]
pub enum BodyLiteralIsA<'a> {
    Literal(Literal<'a>),
    ConditionalLiteral(ConditionalLiteral<'a>),
    TheoryAtom(TheoryAtom<'a>),
}
impl<'a> BodyLiteral<'a> {
    pub fn is_a(self) -> Result<BodyLiteralIsA<'a>, ClingoError> {
        match self.ast.get_type()? {
            ASTType::Literal => Ok(BodyLiteralIsA::Literal(Literal { ast: self.ast })),
            ASTType::ConditionalLiteral => {
                Ok(BodyLiteralIsA::ConditionalLiteral(ConditionalLiteral {
                    ast: self.ast,
                }))
            }
            ASTType::TheoryAtom => Ok(BodyLiteralIsA::TheoryAtom(TheoryAtom { ast: self.ast })),
            x => panic!("unexpected ASTType for BodyLiteral: {:?}", x),
        }
    }
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct BodyAtom<'a> {
    ast: AST<'a>,
}
impl<'a> From<Aggregate<'a>> for BodyAtom<'a> {
    fn from(x: Aggregate<'a>) -> Self {
        BodyAtom { ast: x.ast }
    }
}
impl<'a> From<BodyAggregate<'a>> for BodyAtom<'a> {
    fn from(x: BodyAggregate<'a>) -> Self {
        BodyAtom { ast: x.ast }
    }
}
impl<'a> From<Disjoint<'a>> for BodyAtom<'a> {
    fn from(x: Disjoint<'a>) -> Self {
        BodyAtom { ast: x.ast }
    }
}
impl<'a> From<TheoryAtom<'a>> for BodyAtom<'a> {
    fn from(x: TheoryAtom<'a>) -> Self {
        BodyAtom { ast: x.ast }
    }
}

#[derive(Debug, Clone)]
pub struct TheoryTerm<'a> {
    ast: AST<'a>,
}
impl<'a> From<SymbolicTerm<'a>> for TheoryTerm<'a> {
    fn from(x: SymbolicTerm<'a>) -> Self {
        TheoryTerm { ast: x.ast }
    }
}
impl<'a> From<Variable<'a>> for TheoryTerm<'a> {
    fn from(x: Variable<'a>) -> Self {
        TheoryTerm { ast: x.ast }
    }
}
impl<'a> From<TheorySequence<'a>> for TheoryTerm<'a> {
    fn from(x: TheorySequence<'a>) -> Self {
        TheoryTerm { ast: x.ast }
    }
}
impl<'a> From<TheoryFunction<'a>> for TheoryTerm<'a> {
    fn from(x: TheoryFunction<'a>) -> Self {
        TheoryTerm { ast: x.ast }
    }
}
impl<'a> From<TheoryUnparsedTerm<'a>> for TheoryTerm<'a> {
    fn from(x: TheoryUnparsedTerm<'a>) -> Self {
        TheoryTerm { ast: x.ast }
    }
}

#[derive(Debug, Clone)]
pub struct Statement<'a> {
    pub(crate) ast: AST<'a>,
}
impl<'a> Statement<'a> {
    pub fn is_a(self) -> Result<StatementIsA<'a>, ClingoError> {
        match self.ast.get_type()? {
            ASTType::Rule => Ok(StatementIsA::Rule(Rule { ast: self.ast })),
            ASTType::Definition => Ok(StatementIsA::Definition(Definition { ast: self.ast })),
            ASTType::ShowSignature => {
                Ok(StatementIsA::ShowSignature(ShowSignature { ast: self.ast }))
            }
            ASTType::Defined => Ok(StatementIsA::Defined(Defined { ast: self.ast })),
            ASTType::ShowTerm => Ok(StatementIsA::ShowTerm(ShowTerm { ast: self.ast })),
            ASTType::Minimize => Ok(StatementIsA::Minimize(Minimize { ast: self.ast })),
            ASTType::Script => Ok(StatementIsA::Script(Script { ast: self.ast })),
            ASTType::Program => Ok(StatementIsA::Program(Program { ast: self.ast })),
            ASTType::External => Ok(StatementIsA::External(External { ast: self.ast })),
            ASTType::Edge => Ok(StatementIsA::Edge(Edge { ast: self.ast })),
            ASTType::Heuristic => Ok(StatementIsA::Heuristic(Heuristic { ast: self.ast })),
            ASTType::ProjectAtom => Ok(StatementIsA::ProjectAtom(ProjectAtom { ast: self.ast })),
            ASTType::ProjectSignature => Ok(StatementIsA::ProjectSignature(ProjectSignature {
                ast: self.ast,
            })),
            x => panic!("unexpected ASTType: {:?}", x),
        }
    }
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}
#[derive(Debug, Clone)]
pub enum StatementIsA<'a> {
    Rule(Rule<'a>),
    Definition(Definition<'a>),
    ShowSignature(ShowSignature<'a>),
    Defined(Defined<'a>),
    ShowTerm(ShowTerm<'a>),
    Minimize(Minimize<'a>),
    Script(Script<'a>),
    Program(Program<'a>),
    External(External<'a>),
    Edge(Edge<'a>),
    Heuristic(Heuristic<'a>),
    ProjectAtom(ProjectAtom<'a>),
    ProjectSignature(ProjectSignature<'a>),
    TheoryDefinition(TheoryDefinition<'a>),
}
impl<'a> From<Rule<'a>> for Statement<'a> {
    fn from(x: Rule<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<Definition<'a>> for Statement<'a> {
    fn from(x: Definition<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<ShowSignature<'a>> for Statement<'a> {
    fn from(x: ShowSignature<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<Defined<'a>> for Statement<'a> {
    fn from(x: Defined<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<ShowTerm<'a>> for Statement<'a> {
    fn from(x: ShowTerm<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<Minimize<'a>> for Statement<'a> {
    fn from(x: Minimize<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<Script<'a>> for Statement<'a> {
    fn from(x: Script<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<Program<'a>> for Statement<'a> {
    fn from(x: Program<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<External<'a>> for Statement<'a> {
    fn from(x: External<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<Edge<'a>> for Statement<'a> {
    fn from(x: Edge<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<Heuristic<'a>> for Statement<'a> {
    fn from(x: Heuristic<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<ProjectAtom<'a>> for Statement<'a> {
    fn from(x: ProjectAtom<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<ProjectSignature<'a>> for Statement<'a> {
    fn from(x: ProjectSignature<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
impl<'a> From<TheoryDefinition<'a>> for Statement<'a> {
    fn from(x: TheoryDefinition<'a>) -> Self {
        Statement { ast: x.ast }
    }
}
#[derive(Debug, Clone)]
pub struct Id<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Variable<'a> {
    ast: AST<'a>,
}
impl<'a> Variable<'a> {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}
#[derive(Debug, Clone)]
pub struct SymbolicTerm<'a> {
    ast: AST<'a>,
}

impl<'a> SymbolicTerm<'a> {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub(crate) ast: AST<'a>,
}
impl<'a> Function<'a> {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }

    pub fn name(&self) -> String {
        self.ast.name().unwrap()
    }
}
#[derive(Debug, Clone)]
pub struct UnaryOperation<'a> {
    ast: AST<'a>,
}
impl<'a> UnaryOperation<'a> {}
#[derive(Debug, Clone)]
pub struct BinaryOperation<'a> {
    ast: AST<'a>,
}
impl<'a> BinaryOperation<'a> {
    // pub fn operator_type(&self) -> BinaryOperator {
    //     self.ast.operator_type()
    // }
    pub fn left(&self) -> Term {
        self.ast.left()
    }
    pub fn right(&self) -> Term {
        self.ast.right()
    }
}
#[derive(Debug, Clone)]
pub struct Interval<'a> {
    ast: AST<'a>,
}
impl<'a> Interval<'a> {
    pub fn left(&self) -> Term {
        self.ast.left()
    }
    pub fn right(&self) -> Term {
        self.ast.right()
    }
}
#[derive(Debug, Clone)]
pub struct Pool<'a> {
    ast: AST<'a>,
}

#[derive(Debug, Clone)]
struct BooleanConstant<'a> {
    ast: AST<'a>,
}

#[derive(Debug, Clone)]
pub struct SymbolicAtom<'a> {
    ast: AST<'a>,
}
impl<'a> SymbolicAtom<'a> {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}
#[derive(Debug, Clone)]
pub struct Guard<'a> {
    ast: AST<'a>,
}
impl<'a> Guard<'a> {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}
#[derive(Debug, Clone)]
pub struct Comparison<'a> {
    ast: AST<'a>,
}
impl<'a> Comparison<'a> {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}
#[derive(Debug, Clone)]
pub struct ConditionalLiteral<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Aggregate<'a> {
    ast: AST<'a>,
}
impl<'a> Aggregate<'a> {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}
#[derive(Debug, Clone)]
pub struct BodyAggregateElement<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct BodyAggregate<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct HeadAggregateElement<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct HeadAggregate<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Disjunction<'a> {
    ast: AST<'a>,
}

#[derive(Debug, Clone)]
pub struct DisjointElement<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Disjoint<'a> {
    ast: AST<'a>,
}

#[derive(Debug, Clone)]
pub struct TheorySequence<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryFunction<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryUnparsedTermElement<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryUnparsedTerm<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryGuard<'a> {
    pub(crate) ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryAtomElement<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryAtom<'a> {
    ast: AST<'a>,
}
impl<'a> TheoryAtom<'a> {
    pub fn location(&self) -> Location {
        self.ast.location().unwrap()
    }
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
    pub fn term(&self) -> Term {
        self.ast.term()
    }
    pub fn set_term(&mut self, term: Term) {
        self.ast.set_term(term)
    }
    pub fn guard(&self) -> TheoryGuard {
        self.ast.guard()
    }
    pub fn elements(&self) -> &[TheoryAtomElement] {
        self.ast.elements()
    }
}
#[derive(Debug, Clone)]
pub struct AtomicLiteral<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct BasicLiteral<'a> {
    ast: AST<'a>,
}
impl<'a> BasicLiteral<'a> {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.ast.to_string()
    }
}
#[derive(Debug, Clone)]
pub struct TheoryOperatorDefinition<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryTermDefinition<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryGuardDefinition<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryAtomDefinition<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Rule<'a> {
    ast: AST<'a>,
}
impl<'a> Rule<'a> {
    pub fn location(&self) -> Location {
        self.ast.location().unwrap()
    }
    pub fn body(&self) -> Body {
        self.ast.body()
    }
    pub fn head(&self) -> Head {
        self.ast.head()
    }
}
#[derive(Debug, Clone)]
pub struct Definition<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct ShowSignature<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct ShowTerm<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Minimize<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Script<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Program<'a> {
    pub(crate) ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct External<'a> {
    pub(crate) ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Edge<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Heuristic<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct ProjectAtom<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct ProjectSignature<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct Defined<'a> {
    ast: AST<'a>,
}
#[derive(Debug, Clone)]
pub struct TheoryDefinition<'a> {
    ast: AST<'a>,
}

// extern "C" {
//     #[doc = "! Construct an AST of the given type."]
//     #[doc = "!"]
//     #[doc = "! @note The arguments corresponding to the given type can be inspected using \"g_clingo_ast_constructors.constructors[type]\"."]
//     #[doc = "!"]
//     #[doc = "! @param[in] type the type of AST to construct"]
//     #[doc = "! @param[out] ast the resulting AST"]
//     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
//     #[doc = "! - ::clingo_error_bad_alloc"]
//     #[doc = "! - ::clingo_error_runtime if one of the arguments is incompatible with the type"]
//     pub fn clingo_ast_build(type_: clingo_ast_type_t, ast: *mut *mut clingo_ast_t, ...) -> bool;
// }

/// Construct an AST node of type `ASTType.Id`.
pub fn id<'a>(location: &Location, name: &str) -> Result<Id<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let variable = internalize_string(name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_id as i32,
            &mut ast,
            location,
            variable,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Id {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
/// Construct an AST node of type `ASTType.Variable`.
pub fn variable<'a>(location: &Location, name: &str) -> Result<Variable<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    let variable = internalize_string(name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_variable as i32,
            &mut ast,
            location,
            variable,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Variable {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
/// Construct an AST node of type `ASTType.SymbolicTerm`.
pub fn symbolic_term<'a>(
    location: &Location,
    symbol: &Symbol,
) -> Result<SymbolicTerm<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_symbolic_term as i32,
            &mut ast,
            location,
            symbol.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(SymbolicTerm {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),

        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

/// Construct an AST node of type `ASTType.Function`.
pub fn function<'a>(
    location: &Location,
    name: &str,
    arguments: &'a [Term],
    external: bool,
) -> Result<Function<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_function as i32,
            &mut ast,
            location,
            name,
            arguments.as_ptr(),
            arguments.len(),
            external as i32,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    let mut ast_type = 0;
    if !unsafe { clingo_ast_get_type(ast, &mut ast_type) } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_get_type() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Function {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

/// Construct an AST node of type `ASTType.UnaryOperation`.
pub fn unary_operation<'a, T>(
    location: &Location,
    operator_type: UnaryOperator,
    argument: T,
) -> Result<UnaryOperation<'a>, ClingoError>
where
    T: Into<Term<'a>>,
{
    let argument: Term = argument.into();
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_unary_operation as i32,
            &mut ast,
            location,
            operator_type as i32,
            argument.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(UnaryOperation {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.BinaryOperation`.
pub fn binary_operation<'a, T1, T2>(
    location: &Location,
    operator_type: BinaryOperator,
    left: T1,
    right: T2,
) -> Result<BinaryOperation<'a>, ClingoError>
where
    T1: Into<Term<'a>>,
    T2: Into<Term<'a>>,
{
    let left: Term = left.into();
    let right: Term = right.into();
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_binary_operation as i32,
            &mut ast,
            location,
            operator_type as i32,
            left.ast,
            right.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(BinaryOperation {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Interval`.
pub fn interval<'a, T1, T2>(
    location: &Location,
    left: T1,
    right: T2,
) -> Result<Interval, ClingoError>
where
    T1: Into<Term<'a>>,
    T2: Into<Term<'a>>,
{
    let left: Term = left.into();
    let right: Term = right.into();
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_interval as i32,
            &mut ast,
            location,
            left.ast,
            right.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Interval {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
/// Construct an AST node of type `ASTType.Pool`.
pub fn pool<'a>(location: &Location, arguments: &'a [Term]) -> Result<Pool<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_pool as i32,
            &mut ast,
            location,
            arguments.as_ptr() as *const clingo_ast_t,
            arguments.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Pool {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.BooleanConstant`.
fn boolean_constant<'a>(value: bool) -> Result<BooleanConstant<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_boolean_constant as i32,
            &mut ast,
            value as i32,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(BooleanConstant {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.SymbolicAtom`.
pub fn symbolic_atom<'a, T>(symbol: T) -> Result<SymbolicAtom<'a>, ClingoError>
where
    T: Into<Term<'a>>,
{
    let symbol: Term = symbol.into();
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_symbolic_atom as i32,
            &mut ast,
            symbol.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(SymbolicAtom {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Guard`.
pub fn guard<'a, T>(operator: ComparisonOperator, term: T) -> Result<Guard<'a>, ClingoError>
where
    T: Into<Term<'a>>,
{
    let term: Term = term.into();
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_guard as i32,
            &mut ast,
            operator as isize,
            term.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Guard {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Comparison`.
pub fn comparison<'a, T1>(term: T1, guards: &'a [Guard]) -> Result<Comparison<'a>, ClingoError>
where
    T1: Into<Term<'a>>,
{
    let term: Term = term.into();
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_comparison as i32,
            &mut ast,
            term.ast,
            guards.as_ptr() as *const clingo_ast_t,
            guards.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Comparison {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ConditionalLiteral`.
pub fn conditional_literal<'a, L>(
    location: &Location,
    literal: L,
    condition: &'a [Literal],
) -> Result<ConditionalLiteral<'a>, ClingoError>
where
    L: Into<Literal<'a>>,
{
    let literal: Literal = literal.into();
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_conditional_literal as i32,
            &mut ast,
            location,
            literal.ast,
            condition.as_ptr() as *const clingo_ast_t,
            condition.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(ConditionalLiteral {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Aggregate`.
pub fn aggregate<'a>(
    location: &Location,
    left_guard: Option<Guard>,
    elements: &'a [ConditionalLiteral],
    right_guard: Option<Guard>,
) -> Result<Aggregate<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let left_guard = match &left_guard {
        Some(left_guard) => left_guard.ast.ptr.as_ptr(),
        None => std::ptr::null(),
    };
    let right_guard = match &right_guard {
        Some(right_guard) => right_guard.ast.ptr.as_ptr(),
        None => std::ptr::null(),
    };

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_aggregate as i32,
            &mut ast,
            location,
            left_guard,
            elements.as_ptr() as *const clingo_ast_t,
            elements.len(),
            right_guard,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Aggregate {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.BodyAggregateElement`.
pub fn body_aggregate_element<'a>(
    terms: &'a [Term],
    condition: &'a [Literal],
) -> Result<BodyAggregateElement<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_body_aggregate_element as i32,
            &mut ast,
            terms.as_ptr() as *const clingo_ast_t,
            terms.len(),
            condition.as_ptr() as *const clingo_ast_t,
            condition.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(BodyAggregateElement {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.BodyAggregate`.
pub fn body_aggregate<'a>(
    location: &Location,
    left_guard: Option<Guard<'a>>,
    function: AggregateFunction,
    elements: &'a [BodyAggregateElement],
    right_guard: Option<Guard<'a>>,
) -> Result<BodyAggregate<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    let left_guard = match &left_guard {
        Some(left_guard) => left_guard.ast.ptr.as_ptr(),
        None => std::ptr::null(),
    };
    let right_guard = match &right_guard {
        Some(right_guard) => right_guard.ast.ptr.as_ptr(),
        None => std::ptr::null(),
    };
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_body_aggregate as i32,
            &mut ast,
            location,
            left_guard,
            function as i32,
            elements.as_ptr(),
            elements.len(),
            right_guard,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(BodyAggregate {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

/// Construct an AST node of type `ASTType.HeadAggregateElement`.
pub fn head_aggregate_element<'a>(
    terms: &'a [Term],
    condition: ConditionalLiteral<'a>,
) -> Result<HeadAggregateElement<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_head_aggregate_element as i32,
            &mut ast,
            terms.as_ptr() as *const clingo_ast_t,
            terms.len(),
            condition.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(HeadAggregateElement {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

/// Construct an AST node of type `ASTType.HeadAggregate`.
pub fn head_aggregate<'a>(
    location: &Location,
    left_guard: Option<Guard<'a>>,
    function: AggregateFunction,
    elements: &'a [HeadAggregateElement],
    right_guard: Option<Guard<'a>>,
) -> Result<HeadAggregate<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let left_guard = match &left_guard {
        Some(left_guard) => left_guard.ast.ptr.as_ptr(),
        None => std::ptr::null(),
    };
    let right_guard = match &right_guard {
        Some(right_guard) => right_guard.ast.ptr.as_ptr(),
        None => std::ptr::null(),
    };

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_head_aggregate as i32,
            &mut ast,
            location,
            left_guard,
            function as i32,
            elements.as_ptr(),
            elements.len(),
            right_guard,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(HeadAggregate {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

/// Construct an AST node of type `ASTType.Disjunction`.
pub fn disjunction<'a>(
    location: &Location,
    elements: &'a [ConditionalLiteral],
) -> Result<Disjunction<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_disjunction as i32,
            &mut ast,
            location,
            elements.as_ptr(),
            elements.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Disjunction {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

/// Construct an AST node of type `ASTType.TheorySequence`.
pub fn theory_sequence<'a>(
    location: &Location,
    sequence_type: TheoryTermSequenceType,
    terms: &'a [TheoryTerm],
) -> Result<TheorySequence<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_sequence as i32,
            &mut ast,
            location,
            sequence_type as i32,
            terms.as_ptr() as *const clingo_ast_t,
            terms.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheorySequence {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryFunction`.
pub fn theory_function<'a>(
    location: &Location,
    name: &str,
    arguments: &'a [TheoryTerm],
) -> Result<TheoryFunction<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_function as i32,
            &mut ast,
            location,
            name,
            arguments.as_ptr() as *const clingo_ast_t,
            arguments.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryFunction {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryUnparsedTermElement`.
pub fn theory_unparsed_term_element<'a>(
    operators: &[&str],
    term: TheoryTerm,
) -> Result<TheoryUnparsedTermElement<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    // c_operators = [ _ffi.new('char[]', x.encode()) for x in operators ]
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_unparsed_term_element as i32,
            &mut ast,
            operators.as_ptr(),
            operators.len(),
            term.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryUnparsedTermElement {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

/// Construct an AST node of type `ASTType.TheoryUnparsedTerm`.
pub fn theory_unparsed_term<'a>(
    location: &Location,
    elements: Vec1<TheoryUnparsedTermElement>,
) -> Result<TheoryUnparsedTerm<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_unparsed_term as i32,
            &mut ast,
            location,
            elements.as_ptr() as *const clingo_ast_t,
            elements.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryUnparsedTerm {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryGuard`.
pub fn theory_guard<'a, T>(operator_name: &str, term: T) -> Result<TheoryGuard, ClingoError>
where
    T: Into<TheoryTerm<'a>>,
{
    let term: TheoryTerm = term.into();
    let mut ast = std::ptr::null_mut();
    let operator_name = internalize_string(operator_name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_guard as i32,
            &mut ast,
            operator_name,
            term.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryGuard {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryAtomElement`.
pub fn theory_atom_element<'a>(
    terms: &'a [TheoryTerm],
    condition: &'a [Literal],
) -> Result<TheoryAtomElement<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_atom_element as i32,
            &mut ast,
            terms.as_ptr() as *const clingo_ast_t,
            terms.len(),
            condition.as_ptr() as *const clingo_ast_t,
            condition.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryAtomElement {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryAtom`.
pub fn theory_atom<'a, T>(
    location: &Location,
    term: T,
    elements: &'a [TheoryAtomElement],
    guard: Option<TheoryGuard<'a>>,
) -> Result<TheoryAtom<'a>, ClingoError>
where
    T: Into<Term<'a>>,
{
    let term: Term = term.into();
    let mut ast = std::ptr::null_mut();

    let guard = match &guard {
        Some(guard) => guard.ast.ptr.as_ptr(),
        None => std::ptr::null(),
    };
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_atom as i32,
            &mut ast,
            location,
            term.ast,
            elements.as_ptr() as *const clingo_ast_t,
            elements.len(),
            guard,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryAtom {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
/// Construct an AST node of type `ASTType.Literal`.
pub fn atomic_literal_from_body_atom<'a, BA>(
    location: &Location,
    sign: Sign,
    atom: BA,
) -> Result<AtomicLiteral<'a>, ClingoError>
where
    BA: Into<BodyAtom<'a>>,
{
    let atom: BodyAtom = atom.into();
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_literal as i32,
            &mut ast,
            location,
            sign as i32,
            atom.ast.ptr,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(AtomicLiteral {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Literal`.
pub fn basic_literal_from_symbolic_atom<'a>(
    location: &Location,
    sign: Sign,
    atom: SymbolicAtom<'a>,
) -> Result<BasicLiteral<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_literal as i32,
            &mut ast,
            location,
            sign as i32,
            atom.ast.ptr,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(BasicLiteral {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Literal`.
pub fn basic_literal_from_boolean_constant(
    location: &Location,
    sign: Sign,
    value: bool,
) -> Result<BasicLiteral, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let atom = boolean_constant(value)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_literal as i32,
            &mut ast,
            location,
            sign as i32,
            atom.ast.ptr,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(BasicLiteral {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
/// Construct an AST node of type `ASTType.Literal`.
pub fn basic_literal_from_comparison<'a>(
    location: &Location,
    sign: Sign,
    atom: Comparison<'a>,
) -> Result<BasicLiteral<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_literal as i32,
            &mut ast,
            location,
            sign as i32,
            atom.ast.ptr,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(BasicLiteral {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryOperatorDefinition`.
pub fn theory_operator_definition<'a>(
    location: &Location,
    name: &str,
    priority: u32,
    operator_type: TheoryOperatorType,
) -> Result<TheoryOperatorDefinition<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    let name = internalize_string(name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_operator_definition as i32,
            &mut ast,
            location,
            name,
            priority,
            operator_type as i32,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryOperatorDefinition {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryTermDefinition`.
pub fn theory_term_definition<'a>(
    location: &Location,
    name: &str,
    operators: &'a [TheoryOperatorDefinition],
) -> Result<TheoryTermDefinition<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_term_definition as i32,
            &mut ast,
            location,
            name,
            operators.as_ptr() as *const clingo_ast_t,
            operators.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryTermDefinition {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryGuardDefinition`.
pub fn theory_guard_definition<'a>(
    operators: &[&str],
    term: &str,
) -> Result<TheoryGuardDefinition<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let term = internalize_string(term)?;
    let mut args = vec![];
    for arg in operators {
        args.push(CString::new(*arg)?);
    }
    // convert the strings to raw pointers
    let c_operators = args
        .iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_guard_definition as i32,
            &mut ast,
            c_operators.as_ptr(),
            c_operators.len(),
            term,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryGuardDefinition {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryAtomDefinition`.
pub fn theory_atom_definition<'a>(
    location: &Location,
    atom_type: TheoryAtomType,
    name: &str,
    arity: u32,
    term: &str,
    guard: Option<TheoryGuardDefinition<'a>>,
) -> Result<TheoryAtomDefinition<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;
    let term = internalize_string(term)?;
    let guard = match &guard {
        Some(guard) => guard.ast.ptr.as_ptr(),
        None => std::ptr::null(),
    };

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_atom_definition as i32,
            &mut ast,
            location,
            atom_type as i32,
            name,
            arity,
            term,
            guard,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryAtomDefinition {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Rule`.
pub fn rule<'a, H>(
    location: &Location,
    head: H,
    body: &'a [BodyLiteral<'a>],
) -> Result<Rule<'a>, ClingoError>
where
    H: Into<Head<'a>>,
{
    let head: Head = head.into();
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_rule as i32,
            &mut ast,
            location,
            head.ast,
            body.as_ptr(),
            body.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }

    match NonNull::new(ast) {
        Some(ast) => Ok(Rule {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Definition`.
pub fn definition<'a, T>(
    location: &Location,
    name: &str,
    value: T,
    is_default: bool,
) -> Result<Definition<'a>, ClingoError>
where
    T: Into<Term<'a>>,
{
    let value: Term = value.into();
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_definition as i32,
            &mut ast,
            location,
            name,
            value.ast,
            is_default as i32,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Definition {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ShowSignature`.
pub fn show_signature<'a>(
    location: &Location,
    name: &str,
    arity: u32,
    positive: bool,
    csp: bool,
) -> Result<ShowSignature<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_show_signature as i32,
            &mut ast,
            location,
            name,
            arity,
            positive as i32,
            csp as i32,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(ShowSignature {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ShowTerm`.
pub fn show_term<'a, T>(
    location: &Location,
    term: T,
    body: &'a [BodyLiteral],
    csp: bool,
) -> Result<ShowTerm<'a>, ClingoError>
where
    T: Into<Term<'a>>,
{
    let term: Term = term.into();
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_show_term as i32,
            &mut ast,
            location,
            term.ast,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
            csp as i32,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(ShowTerm {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Minimize`.
pub fn minimize<'a, T1, T2>(
    location: &Location,
    weight: T1,
    priority: T2,
    terms: &'a [Term],
    body: &'a [BodyLiteral],
) -> Result<Minimize<'a>, ClingoError>
where
    T1: Into<Term<'a>>,
    T2: Into<Term<'a>>,
{
    let weight: Term = weight.into();
    let priority: Term = priority.into();
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_minimize as i32,
            &mut ast,
            location,
            weight.ast,
            priority.ast,
            terms.as_ptr() as *const clingo_ast_t,
            terms.len(),
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Minimize {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Script`.
pub fn script<'a>(location: &Location, name: &str, code: &str) -> Result<Script<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let code = internalize_string(code);

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_script as i32,
            &mut ast,
            location,
            name,
            code,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Script {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Program`.
pub fn program<'a>(
    location: &Location,
    name: &str,
    parameters: &'a [Id],
) -> Result<Program<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_program as i32,
            &mut ast,
            location,
            name,
            parameters.as_ptr() as *const clingo_ast_t,
            parameters.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Program {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

pub fn external<'a>(
    location: &Location,
    atom: SymbolicAtom<'a>,
    body: &'a [BodyLiteral],
    external_type: ExternalType,
) -> Result<External<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let external_type = match external_type {
        ExternalType::False => {
            let symbol = Symbol::create_id("false", true)?;
            symbolic_term(&Location::default(), &symbol)
        }
        ExternalType::True => {
            let symbol = Symbol::create_id("true", true)?;
            symbolic_term(&Location::default(), &symbol)
        }
        ExternalType::Free => {
            let symbol = Symbol::create_id("free", true)?;
            symbolic_term(&Location::default(), &symbol)
        }
        ExternalType::Release => {
            let symbol = Symbol::create_id("release", true)?;
            symbolic_term(&Location::default(), &symbol)
        }
    }
    .unwrap();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_external as i32,
            &mut ast,
            location,
            atom.ast,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
            external_type.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(External {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Edge`.
pub fn edge<'a, T1, T2>(
    location: &Location,
    node_u: T1,
    node_v: T2,
    body: &'a [BodyLiteral],
) -> Result<Edge<'a>, ClingoError>
where
    T1: Into<Term<'a>>,
    T2: Into<Term<'a>>,
{
    let node_u: Term = node_u.into();
    let node_v: Term = node_v.into();
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_edge as i32,
            &mut ast,
            location,
            node_u.ast,
            node_v.ast,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Edge {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Heuristic`.
pub fn heuristic<'a, T1, T2, T3>(
    location: &Location,
    atom: SymbolicAtom<'a>,
    body: &'a [BodyLiteral],
    bias: T1,
    priority: T2,
    modifier: T3,
) -> Result<Heuristic<'a>, ClingoError>
where
    T1: Into<Term<'a>>,
    T2: Into<Term<'a>>,
    T3: Into<Term<'a>>,
{
    let bias: Term = bias.into();
    let priority: Term = priority.into();
    let modifier: Term = modifier.into();
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_heuristic as i32,
            &mut ast,
            location,
            atom.ast,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
            bias.ast,
            priority.ast,
            modifier.ast,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Heuristic {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ProjectAtom`.
pub fn project_atom<'a>(
    location: &Location,
    atom: SymbolicAtom<'a>,
    body: &'a [BodyLiteral],
) -> Result<ProjectAtom<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_project_atom as i32,
            &mut ast,
            location,
            atom.ast,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(ProjectAtom {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ProjectSignature`.
pub fn project_signature<'a>(
    location: &Location,
    name: &str,
    arity: u32,
    positive: bool,
) -> Result<ProjectSignature<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_project_signature as i32,
            &mut ast,
            location,
            name,
            arity,
            positive as i32,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(ProjectSignature {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Defined`.
pub fn defined<'a>(
    location: &Location,
    name: &str,
    arity: u32,
    positive: bool,
) -> Result<Defined<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_defined as i32,
            &mut ast,
            location,
            name,
            arity,
            positive as i32,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Defined {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryDefinition`.
pub fn theory_definition<'a>(
    location: &Location,
    name: &str,
    terms: &'a [TheoryTermDefinition],
    atoms: &'a [TheoryAtomDefinition],
) -> Result<TheoryDefinition<'a>, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_definition as i32,
            &mut ast,
            location,
            name,
            terms.as_ptr() as *const clingo_ast_t,
            terms.len(),
            atoms.as_ptr() as *const clingo_ast_t,
            atoms.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryDefinition {
            ast: AST {
                ptr: ast,
                _lifetime: PhantomData,
            },
        }),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

impl<'a> From<ProgramBuilder<'a>> for &'a mut clingo_program_builder_t {
    fn from(pb: ProgramBuilder<'a>) -> Self {
        pb.theref
    }
}
impl<'a> From<&mut ProgramBuilder<'a>> for *mut clingo_program_builder_t {
    fn from(pb: &mut ProgramBuilder<'a>) -> Self {
        pb.theref
    }
}
impl<'a> From<Statement<'a>> for NonNull<clingo_ast> {
    fn from(stmt: Statement) -> Self {
        stmt.ast.ptr
    }
}
impl<'a> From<&Statement<'a>> for NonNull<clingo_ast> {
    fn from(stmt: &Statement<'a>) -> Self {
        stmt.ast.ptr
    }
}
