#![allow(clippy::needless_lifetimes)]
use crate::{
    internalize_string, set_internal_error, ClingoError, ControlLPOF, FunctionHandler,
    GroundProgramObserver, Location, Logger, Propagator, Symbol,
};
use crate::ErrorType;
use clingo_sys::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr::NonNull;

pub struct AstArray<'a> {
    ast: &'a Ast,
    attribute: AstAttribute,
}
impl<'a> AstArray<'a> {
    /// Get the size of an AstArray
    ///
    /// @param[in] ast the target AstArray
    /// @param[in] attribute the target attribute"]
    /// @param[out] size the resulting size"]
    /// @return whether the call was successful; might set one of the following error codes:"]
    /// - ::clingo_error_runtime"]

    pub fn size(&self) -> Result<usize, ClingoError> {
        let mut size: usize = 0;
        if !unsafe {
            clingo_ast_attribute_size_ast_array(
                self.ast.0.as_ptr(),
                self.attribute as i32,
                &mut size,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_size_ast_array() failed.",
            ));
        }
        Ok(size)
    }

    ///  Returns an iterator over the theory atoms.
    pub fn iter(&self) -> AstArrayIterator {
        AstArrayIterator {
            ast_array: self,
            index: 0,
        }
    }
}
pub struct AstArrayIterator<'a> {
    ast_array: &'a AstArray<'a>,
    index: usize,
}
impl<'a> Iterator for AstArrayIterator<'a> {
    type Item = Ast;

    fn next(&mut self) -> Option<Ast> {
        let size = self.ast_array.size().unwrap(); //Err->None

        if size == self.index {
            return None;
        }

        let mut ast = std::ptr::null_mut();
        if !unsafe {
            clingo_ast_attribute_get_ast_at(
                self.ast_array.ast.0.as_ptr(),
                self.ast_array.attribute as i32,
                self.index,
                &mut ast,
            )
        } {
            return None;
        }
        self.index += 1;
        match NonNull::new(ast) {
            Some(x) => Some(Ast(x)),
            None => None,
        }
    }
}
type AstCallback = unsafe extern "C" fn(ast: *mut clingo_ast_t, data: *mut c_void) -> bool;
pub trait StatementHandler {
    /// Callback function called on an ast statement while traversing the ast.
    ///
    /// **Returns** whether the call was successful
    fn on_statement(&mut self, ast: &mut Ast) -> bool;
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
    let ast2 = NonNull::new(ast);

    let event_handler = &mut *(event_handler as *mut T);

    // println!("ast2: {:?}",ast);
    match ast2 {
        Some(x) => event_handler.on_statement(&mut Ast(x)),
        None => panic!("ast.as_mut() returned None"),
    }

    // event_handler.on_statement()
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
/// Enumeration of script types.
pub enum ScriptType {
    /// For Lua scripts.
    Lua = clingo_ast_script_type_e_clingo_ast_script_type_lua as isize,
    /// For Python scripts.
    Python = clingo_ast_script_type_e_clingo_ast_script_type_python as isize,
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
/// Object to build non-ground programs.
pub struct ProgramBuilder<'a> {
    pub(crate) theref: &'a mut clingo_program_builder_t,
}
impl<'a> ProgramBuilder<'a> {
    /// Get an object to add non-ground directives to the program.
    pub fn from<L: Logger, P: Propagator, O: GroundProgramObserver, F: FunctionHandler>(
        ctl: &'a mut ControlLPOF<L, P, O, F>,
    ) -> Result<ProgramBuilder<'a>, ClingoError> {
        let mut builder = std::ptr::null_mut();
        if !unsafe { clingo_control_program_builder(ctl.ctl.as_mut(), &mut builder) } {
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
    pub fn add(&mut self, stm: &Ast) -> Result<(), ClingoError> {
        println!("add stm {:?}", stm);
        let bla = stm.to_string();
        println!("add stm.to_string {:?}", bla);
        let bla = stm.get_type();
        println!("add stm.get_type {:?}", bla);
        match bla {
            Ok(AstType::Program) => {
                let attribute = AstAttribute::Parameters;
                let blub = stm.get_attribute_type(&attribute);
                println!("stm.get_attribute_type {:?} {:?}", attribute, blub);
            }
            Ok(AstType::Rule) => {
                let attribute = AstAttribute::Head;
                let blub = stm.get_attribute_type(&attribute);
                println!("stm.get_attribute_type {:?} {:?}", attribute, blub);
                let blub = stm.get_attribute_ast(&attribute).unwrap();
                println!("stm.get_attribute_ast {:?} {:?}", attribute, blub);
                let blub = blub.to_string();
                println!("stm.get_attribute_ast_to_string {:?} {:?}", attribute, blub);
                let attribute = AstAttribute::Body;
                let blub = stm.get_attribute_type(&attribute);
                println!("stm.get_attribute_type {:?} {:?}", attribute, blub);
            }
            Ok(AstType::External) => {
                let attribute = AstAttribute::Atom;
                let blub = stm.get_attribute_type(&attribute);
                println!("stm.get_attribute_type {:?} {:?}", attribute, blub);
                let ast = stm.get_attribute_ast(&attribute).unwrap();
                println!("   stm.get_attribute_ast {:?} {:?}", attribute, ast);
                let string = ast.to_string();
                println!("   ast.to_string {:?}", string);
                let bla = ast.get_type();
                println!("   ast.get_type {:?}", bla);
                let attribute = AstAttribute::Symbol;
                let blub = ast.get_attribute_type(&attribute);
                println!("   ast.get_attribute_type {:?} {:?}", attribute, blub);

                let ast2 = ast.get_attribute_ast(&attribute).unwrap();
                println!("        ast.get_attribute_ast {:?} {:?}", attribute, ast2);
                let string = ast2.to_string();
                println!("        ast2.to_string {:?}", string);
                let ast_type = ast2.get_type();
                println!("        ast2.get_type {:?}", ast_type);

                let attribute = AstAttribute::Body;
                let blub = stm.get_attribute_type(&attribute);
                println!("stm.get_attribute_type {:?} {:?}", attribute, blub);
                // let ast = stm.get_attribute_ast_at(&attribute,0).unwrap();
                // println!("stm.get_attribute_ast_ast {:?} {:?}", attribute, ast);
                // let string = ast.to_string();
                // println!("ast.to_string {:?} {:?}", attribute, string);

                let attribute = AstAttribute::ExternalType;
                let blub = stm.get_attribute_type(&attribute);
                println!("stm.get_attribute_type {:?} {:?}", attribute, blub);
                let ast = stm.get_attribute_ast(&attribute).unwrap();
                println!("    stm.get_attribute_ast {:?} {:?}", attribute, ast);
                let string = ast.to_string();
                println!("    ast.to_string {:?}", string);
                let bla = ast.get_type();
                println!("    ast.get_type {:?}", bla);

                let attribute = AstAttribute::Symbol;
                let blub = ast.get_attribute_type(&attribute);
                println!("    ast.get_attribute_type {:?} {:?}", attribute, blub);
                // let sym = ast.get_symbol().unwrap();
                // println!("        ast.get_symbol() {:?}", sym);
                // let string = sym.to_string();
                // println!("        sym.to_string {:?}", string);
            }
            _ => println!("unmatched ast_type {:?}", bla),
        }

        if !unsafe { clingo_program_builder_add(self.theref, stm.0.as_ptr()) } {
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

#[doc(hidden)]
#[cfg(feature = "dl_theory")]
pub(crate) unsafe extern "C" fn unsafe_program_builder_add(
    statement: *const clingo_ast_statement_t,
    data: *mut ::std::os::raw::c_void,
) -> bool {
    let builder = data as *mut clingo_program_builder;
    clingo_program_builder_add(builder, statement)
}

// Here starts AST2

#[derive(Debug, Copy, Clone)]
/// Enumeration of AST types.
pub enum AstType {
    Id = clingo_ast_type_e_clingo_ast_type_id as isize,
    Variable = clingo_ast_type_e_clingo_ast_type_variable as isize,
    SymbolicTerm = clingo_ast_type_e_clingo_ast_type_symbolic_term as isize,
    UnaryOperation = clingo_ast_type_e_clingo_ast_type_unary_operation as isize,
    BinaryOperation = clingo_ast_type_e_clingo_ast_type_binary_operation as isize,
    Interval = clingo_ast_type_e_clingo_ast_type_interval as isize,
    Function = clingo_ast_type_e_clingo_ast_type_function as isize,
    Pool = clingo_ast_type_e_clingo_ast_type_pool as isize,
    CspProduct = clingo_ast_type_e_clingo_ast_type_csp_product as isize,
    CspSum = clingo_ast_type_e_clingo_ast_type_csp_sum as isize,
    CspGuard = clingo_ast_type_e_clingo_ast_type_csp_guard as isize,
    BooleanConstant = clingo_ast_type_e_clingo_ast_type_boolean_constant as isize,
    SymbolicAtom = clingo_ast_type_e_clingo_ast_type_symbolic_atom as isize,
    Comparison = clingo_ast_type_e_clingo_ast_type_comparison as isize,
    CspLiteral = clingo_ast_type_e_clingo_ast_type_csp_literal as isize,
    AggregateGuard = clingo_ast_type_e_clingo_ast_type_aggregate_guard as isize,
    ConditionalLiteral = clingo_ast_type_e_clingo_ast_type_conditional_literal as isize,
    Aggregate = clingo_ast_type_e_clingo_ast_type_aggregate as isize,
    BodyAggregateElement = clingo_ast_type_e_clingo_ast_type_body_aggregate_element as isize,
    BodyAggregate = clingo_ast_type_e_clingo_ast_type_body_aggregate as isize,
    HeadAggregateElement = clingo_ast_type_e_clingo_ast_type_head_aggregate_element as isize,
    HeadAggregate = clingo_ast_type_e_clingo_ast_type_head_aggregate as isize,
    Disjunction = clingo_ast_type_e_clingo_ast_type_disjunction as isize,
    DisjointElement = clingo_ast_type_e_clingo_ast_type_disjoint_element as isize,
    Disjoint = clingo_ast_type_e_clingo_ast_type_disjoint as isize,
    TheorySequence = clingo_ast_type_e_clingo_ast_type_theory_sequence as isize,
    TheoryFunction = clingo_ast_type_e_clingo_ast_type_theory_function as isize,
    TheoryUnparsedTermElement =
        clingo_ast_type_e_clingo_ast_type_theory_unparsed_term_element as isize,
    TheoryUnparsedTerm = clingo_ast_type_e_clingo_ast_type_theory_unparsed_term as isize,
    TheoryGuard = clingo_ast_type_e_clingo_ast_type_theory_guard as isize,
    TheoryAtomElement = clingo_ast_type_e_clingo_ast_type_theory_atom_element as isize,
    TheoryAtom = clingo_ast_type_e_clingo_ast_type_theory_atom as isize,
    Literal = clingo_ast_type_e_clingo_ast_type_literal as isize,
    TheoryOperatorDefinition =
        clingo_ast_type_e_clingo_ast_type_theory_operator_definition as isize,
    TheoryTermDefinition = clingo_ast_type_e_clingo_ast_type_theory_term_definition as isize,
    TheoryGuardDefinition = clingo_ast_type_e_clingo_ast_type_theory_guard_definition as isize,
    TheoryAtomDefinition = clingo_ast_type_e_clingo_ast_type_theory_atom_definition as isize,
    Rule = clingo_ast_type_e_clingo_ast_type_rule as isize,
    Definition = clingo_ast_type_e_clingo_ast_type_definition as isize,
    ShowSignature = clingo_ast_type_e_clingo_ast_type_show_signature as isize,
    ShowTerm = clingo_ast_type_e_clingo_ast_type_show_term as isize,
    Minimize = clingo_ast_type_e_clingo_ast_type_minimize as isize,
    Script = clingo_ast_type_e_clingo_ast_type_script as isize,
    Program = clingo_ast_type_e_clingo_ast_type_program as isize,
    External = clingo_ast_type_e_clingo_ast_type_external as isize,
    Edge = clingo_ast_type_e_clingo_ast_type_edge as isize,
    Heuristic = clingo_ast_type_e_clingo_ast_type_heuristic as isize,
    ProjectAtom = clingo_ast_type_e_clingo_ast_type_project_atom as isize,
    ProjectAtomSignature = clingo_ast_type_e_clingo_ast_type_project_signature as isize,
    Defined = clingo_ast_type_e_clingo_ast_type_defined as isize,
    TheoryDefinition = clingo_ast_type_e_clingo_ast_type_theory_definition as isize,
}
impl AstType {
    fn try_from(code: u32) -> Result<AstType, ClingoError> {
        // println!("in try_from");
        match code {
            clingo_ast_type_e_clingo_ast_type_id => Ok(AstType::Id),
            clingo_ast_type_e_clingo_ast_type_variable => Ok(AstType::Variable),
            clingo_ast_type_e_clingo_ast_type_symbolic_term => Ok(AstType::SymbolicTerm),
            clingo_ast_type_e_clingo_ast_type_unary_operation => Ok(AstType::UnaryOperation),
            clingo_ast_type_e_clingo_ast_type_binary_operation => Ok(AstType::BinaryOperation),
            clingo_ast_type_e_clingo_ast_type_interval => Ok(AstType::Interval),
            clingo_ast_type_e_clingo_ast_type_function => Ok(AstType::Function),
            clingo_ast_type_e_clingo_ast_type_pool => Ok(AstType::Pool),
            clingo_ast_type_e_clingo_ast_type_csp_product => Ok(AstType::CspProduct),
            clingo_ast_type_e_clingo_ast_type_csp_sum => Ok(AstType::CspSum),
            clingo_ast_type_e_clingo_ast_type_csp_guard => Ok(AstType::CspGuard),
            clingo_ast_type_e_clingo_ast_type_boolean_constant => Ok(AstType::BooleanConstant),
            clingo_ast_type_e_clingo_ast_type_symbolic_atom => Ok(AstType::SymbolicAtom),
            clingo_ast_type_e_clingo_ast_type_comparison => Ok(AstType::Comparison),
            clingo_ast_type_e_clingo_ast_type_csp_literal => Ok(AstType::CspLiteral),
            clingo_ast_type_e_clingo_ast_type_aggregate_guard => Ok(AstType::AggregateGuard),
            clingo_ast_type_e_clingo_ast_type_conditional_literal => {
                Ok(AstType::ConditionalLiteral)
            }
            clingo_ast_type_e_clingo_ast_type_aggregate => Ok(AstType::Aggregate),
            clingo_ast_type_e_clingo_ast_type_body_aggregate_element => {
                Ok(AstType::BodyAggregateElement)
            }
            clingo_ast_type_e_clingo_ast_type_body_aggregate => Ok(AstType::BodyAggregate),
            clingo_ast_type_e_clingo_ast_type_head_aggregate_element => {
                Ok(AstType::HeadAggregateElement)
            }
            clingo_ast_type_e_clingo_ast_type_head_aggregate => Ok(AstType::HeadAggregate),
            clingo_ast_type_e_clingo_ast_type_disjunction => Ok(AstType::Disjunction),
            clingo_ast_type_e_clingo_ast_type_disjoint_element => Ok(AstType::DisjointElement),
            clingo_ast_type_e_clingo_ast_type_disjoint => Ok(AstType::Disjoint),
            clingo_ast_type_e_clingo_ast_type_theory_sequence => Ok(AstType::TheorySequence),
            clingo_ast_type_e_clingo_ast_type_theory_function => Ok(AstType::TheoryFunction),
            clingo_ast_type_e_clingo_ast_type_theory_unparsed_term_element => {
                Ok(AstType::TheoryUnparsedTermElement)
            }
            clingo_ast_type_e_clingo_ast_type_theory_unparsed_term => {
                Ok(AstType::TheoryUnparsedTerm)
            }
            clingo_ast_type_e_clingo_ast_type_theory_guard => Ok(AstType::TheoryGuard),
            clingo_ast_type_e_clingo_ast_type_theory_atom_element => Ok(AstType::TheoryAtomElement),
            clingo_ast_type_e_clingo_ast_type_theory_atom => Ok(AstType::TheoryAtom),
            clingo_ast_type_e_clingo_ast_type_literal => Ok(AstType::Literal),
            clingo_ast_type_e_clingo_ast_type_theory_operator_definition => {
                Ok(AstType::TheoryOperatorDefinition)
            }
            clingo_ast_type_e_clingo_ast_type_theory_term_definition => {
                Ok(AstType::TheoryTermDefinition)
            }
            clingo_ast_type_e_clingo_ast_type_theory_guard_definition => {
                Ok(AstType::TheoryGuardDefinition)
            }
            clingo_ast_type_e_clingo_ast_type_theory_atom_definition => {
                Ok(AstType::TheoryAtomDefinition)
            }
            clingo_ast_type_e_clingo_ast_type_rule => Ok(AstType::Rule),
            clingo_ast_type_e_clingo_ast_type_definition => Ok(AstType::Definition),
            clingo_ast_type_e_clingo_ast_type_show_signature => Ok(AstType::ShowSignature),
            clingo_ast_type_e_clingo_ast_type_show_term => Ok(AstType::ShowTerm),
            clingo_ast_type_e_clingo_ast_type_minimize => Ok(AstType::Minimize),
            clingo_ast_type_e_clingo_ast_type_script => Ok(AstType::Script),
            clingo_ast_type_e_clingo_ast_type_program => Ok(AstType::Program),
            clingo_ast_type_e_clingo_ast_type_external => Ok(AstType::External),
            clingo_ast_type_e_clingo_ast_type_edge => Ok(AstType::Edge),
            clingo_ast_type_e_clingo_ast_type_heuristic => Ok(AstType::Heuristic),
            clingo_ast_type_e_clingo_ast_type_project_atom => Ok(AstType::ProjectAtom),
            clingo_ast_type_e_clingo_ast_type_project_signature => {
                Ok(AstType::ProjectAtomSignature)
            }
            clingo_ast_type_e_clingo_ast_type_defined => Ok(AstType::Defined),
            clingo_ast_type_e_clingo_ast_type_theory_definition => Ok(AstType::TheoryDefinition),
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_ast_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_ast_type.",
                })
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Enumeration of attributes types used by the AST.
pub enum AstAttributeType {
    /// For an attribute of type `int`.
    Number = clingo_ast_attribute_type_e_clingo_ast_attribute_type_number as isize,
    /// For an attribute of type `clingo_ast_symbol_t`.
    Symbol = clingo_ast_attribute_type_e_clingo_ast_attribute_type_symbol as isize,
    /// For an attribute of type `clingo_location_t`.
    Location = clingo_ast_attribute_type_e_clingo_ast_attribute_type_location as isize,
    /// For an attribute of type `char const *`.
    String = clingo_ast_attribute_type_e_clingo_ast_attribute_type_string as isize,
    /// For an attribute of type `clingo_ast_t *`.
    Ast = clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast as isize,
    /// For an attribute of type `clingo_ast_t *` that can be NULL.
    OptionalAst = clingo_ast_attribute_type_e_clingo_ast_attribute_type_optional_ast as isize,
    /// For an attribute of type `char const **`.
    StringArray = clingo_ast_attribute_type_e_clingo_ast_attribute_type_string_array as isize,
    /// For an attribute of type `clingo_ast_t **`.
    AstArray = clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast_array as isize,
}
impl AstAttributeType {
    fn try_from(code: u32) -> Result<AstAttributeType, ClingoError> {
        // println!("in try_from");
        match code {
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_number => {
                Ok(AstAttributeType::Number)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_symbol => {
                Ok(AstAttributeType::Symbol)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_location => {
                Ok(AstAttributeType::Location)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_string => {
                Ok(AstAttributeType::String)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast => Ok(AstAttributeType::Ast),
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_optional_ast => {
                Ok(AstAttributeType::OptionalAst)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_string_array => {
                Ok(AstAttributeType::StringArray)
            }
            clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast_array => {
                Ok(AstAttributeType::AstArray)
            }
            x => {
                eprintln!(
                    "FFIError in {} {}, {} : Failed to match clingo_ast_type {}",
                    file!(),
                    line!(),
                    column!(),
                    x
                );
                Err(ClingoError::FFIError {
                    msg: "Failed to match clingo_ast_type.",
                })
            }
        }
    }
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of attributes used by the AST.
pub enum AstAttribute {
    Argument = clingo_ast_attribute_e_clingo_ast_attribute_argument as isize,
    Arguments = clingo_ast_attribute_e_clingo_ast_attribute_arguments as isize,
    Arity = clingo_ast_attribute_e_clingo_ast_attribute_arity as isize,
    Atom = clingo_ast_attribute_e_clingo_ast_attribute_atom as isize,
    Atoms = clingo_ast_attribute_e_clingo_ast_attribute_atoms as isize,
    AtomType = clingo_ast_attribute_e_clingo_ast_attribute_atom_type as isize,
    Bias = clingo_ast_attribute_e_clingo_ast_attribute_bias as isize,
    Body = clingo_ast_attribute_e_clingo_ast_attribute_body as isize,
    Code = clingo_ast_attribute_e_clingo_ast_attribute_code as isize,
    Coefficient = clingo_ast_attribute_e_clingo_ast_attribute_coefficient as isize,
    Comparison = clingo_ast_attribute_e_clingo_ast_attribute_comparison as isize,
    Condition = clingo_ast_attribute_e_clingo_ast_attribute_condition as isize,
    Csp = clingo_ast_attribute_e_clingo_ast_attribute_csp as isize,
    Elements = clingo_ast_attribute_e_clingo_ast_attribute_elements as isize,
    External = clingo_ast_attribute_e_clingo_ast_attribute_external as isize,
    ExternalType = clingo_ast_attribute_e_clingo_ast_attribute_external_type as isize,
    Function = clingo_ast_attribute_e_clingo_ast_attribute_function as isize,
    Guard = clingo_ast_attribute_e_clingo_ast_attribute_guard as isize,
    Guards = clingo_ast_attribute_e_clingo_ast_attribute_guards as isize,
    Head = clingo_ast_attribute_e_clingo_ast_attribute_head as isize,
    IsDefault = clingo_ast_attribute_e_clingo_ast_attribute_is_default as isize,
    Left = clingo_ast_attribute_e_clingo_ast_attribute_left as isize,
    LeftGuard = clingo_ast_attribute_e_clingo_ast_attribute_left_guard as isize,
    Literal = clingo_ast_attribute_e_clingo_ast_attribute_literal as isize,
    Location = clingo_ast_attribute_e_clingo_ast_attribute_location as isize,
    Modifier = clingo_ast_attribute_e_clingo_ast_attribute_modifier as isize,
    Name = clingo_ast_attribute_e_clingo_ast_attribute_name as isize,
    NodeU = clingo_ast_attribute_e_clingo_ast_attribute_node_u as isize,
    NodeV = clingo_ast_attribute_e_clingo_ast_attribute_node_v as isize,
    OperatorName = clingo_ast_attribute_e_clingo_ast_attribute_operator_name as isize,
    OperatorType = clingo_ast_attribute_e_clingo_ast_attribute_operator_type as isize,
    Operators = clingo_ast_attribute_e_clingo_ast_attribute_operators as isize,
    Parameters = clingo_ast_attribute_e_clingo_ast_attribute_parameters as isize,
    Positive = clingo_ast_attribute_e_clingo_ast_attribute_positive as isize,
    Priority = clingo_ast_attribute_e_clingo_ast_attribute_priority as isize,
    Right = clingo_ast_attribute_e_clingo_ast_attribute_right as isize,
    RightGuard = clingo_ast_attribute_e_clingo_ast_attribute_right_guard as isize,
    ScriptType = clingo_ast_attribute_e_clingo_ast_attribute_script_type as isize,
    SequenceType = clingo_ast_attribute_e_clingo_ast_attribute_sequence_type as isize,
    Sign = clingo_ast_attribute_e_clingo_ast_attribute_sign as isize,
    Symbol = clingo_ast_attribute_e_clingo_ast_attribute_symbol as isize,
    Term = clingo_ast_attribute_e_clingo_ast_attribute_term as isize,
    Terms = clingo_ast_attribute_e_clingo_ast_attribute_terms as isize,
    Value = clingo_ast_attribute_e_clingo_ast_attribute_value as isize,
    Variable = clingo_ast_attribute_e_clingo_ast_attribute_variable as isize,
    Weight = clingo_ast_attribute_e_clingo_ast_attribute_weight as isize,
}

/// Struct to map attributes to their string representation.
#[derive(Debug, Copy, Clone)]
pub struct AttributeNames(clingo_ast_attribute_names);
// pub struct clingo_ast_attribute_names {
//     pub names: *const *const ::std::os::raw::c_char,
//     pub size: usize,
// }

/// Struct to define an argument that consists of a name and a type.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Argument(clingo_ast_argument);
// pub struct clingo_ast_argument {
//     pub attribute: clingo_ast_attribute_t,
//     pub type_: clingo_ast_attribute_type_t,
// }

/// A lists of required attributes to construct an AST.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Constructor(clingo_ast_constructor);
// pub struct clingo_ast_constructor {
//     pub name: *const ::std::os::raw::c_char,
//     pub arguments: *const clingo_ast_argument_t,
//     pub size: usize,

/// Struct to map AST types to lists of required attributes to construct ASTs.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Constructors(clingo_ast_constructors);
// pub struct clingo_ast_constructors {
//     pub constructors: *const clingo_ast_constructor_t,
//     pub size: usize,
// }

/// This struct provides a view to nodes in the AST.
#[derive(Debug, Copy, Clone)]
pub struct Ast(NonNull<clingo_ast_t>);

// TODO
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
pub fn Id(location: Location, name: &str) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let variable = internalize_string(name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_id as i32,
            &mut ast,
            &location,
            variable,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Variable`.
pub fn Variable(location: &Location, name: &str) -> Result<Ast, ClingoError> {
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.SymbolicTerm`.
pub fn SymbolicTerm(location: &Location, symbol: &Symbol) -> Result<Ast, ClingoError> {
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
    println!("in SymbolicTerm");
    let x = match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),

        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    };
    x
}

/// Construct an AST node of type `ASTType.UnaryOperation`.
pub fn UnaryOperation(
    location: Location,
    operator_type: UnaryOperator,
    argument: Ast,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_unary_operation as i32,
            &mut ast,
            &location,
            operator_type as i32,
            argument,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.BinaryOperation`.
pub fn BinaryOperation(
    location: Location,
    operator_type: BinaryOperator,
    left: Ast,
    right: Ast,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_binary_operation as i32,
            &mut ast,
            &location,
            operator_type as i32,
            left.0,
            right.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Interval`.
pub fn Interval(location: Location, left: Ast, right: Ast) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_interval as i32,
            &mut ast,
            &location,
            left.0,
            right.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Function`.
pub fn Function(
    location: &Location,
    name: &str,
    arguments: &[Ast],
    external: bool,
) -> Result<Ast, ClingoError> {
    println!("in function");
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_function as i32,
            &mut ast,
            &location,
            name,
            arguments.as_ptr() as *const clingo_ast_t,
            0, //arguments.len(),
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
    println!("A {}", ast_type);
    match NonNull::new(ast) {
        Some(ast_ref) => {
            let x = Ast(ast_ref);
            println!("B {:?}", x.get_type());
            Ok(x)
        }
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

/// Construct an AST node of type `ASTType.Pool`.
pub fn Pool(location: &Location, arguments: &[Ast]) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_pool as i32,
            &mut ast,
            &location,
            arguments.as_ptr() as *const clingo_ast_t,
            arguments.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.CspProduct`.
pub fn CspProduct(location: Location, coefficient: Ast, variable: Ast) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_csp_product as i32,
            &mut ast,
            &location,
            coefficient.0,
            variable.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.CspSum`.
pub fn CspSum(location: Location, coefficient: Ast, variable: Ast) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_csp_sum as i32,
            &mut ast,
            &location,
            coefficient.0,
            variable.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.CspGuard`.
pub fn CspGuard(
    location: Location,
    comparison: ComparisonOperator,
    term: Ast,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_csp_guard as i32,
            &mut ast,
            &location,
            comparison as i32,
            term.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.BooleanConstant`.
pub fn BooleanConstant(value: bool) -> Result<Ast, ClingoError> {
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.SymbolicAtom`.
pub fn SymbolicAtom(symbol: Ast) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_symbolic_atom as i32,
            &mut ast,
            symbol.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Comparison`.
pub fn Comparison(
    comparison: ComparisonOperator,
    left: Ast,
    right: Ast,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_comparison as i32,
            &mut ast,
            comparison as i32,
            left.0,
            right.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.CspLiteral`.
pub fn CspLiteral(location: Location, term: Ast, guards: &[Ast]) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_csp_literal as i32,
            &mut ast,
            &location,
            term.0,
            guards.as_ptr() as *const clingo_ast_t,
            guards.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.AggregateGuard`.
pub fn AggregateGuard(comparison: ComparisonOperator, term: Ast) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_aggregate_guard as i32,
            &mut ast,
            comparison as i32,
            term.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ConditionalLiteral`.
pub fn ConditionalLiteral(
    location: &Location,
    literal: &Ast,
    condition: &[Ast],
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_conditional_literal as i32,
            &mut ast,
            location,
            literal.0,
            condition.as_ptr() as *const clingo_ast_t,
            condition.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Aggregate`.
pub fn Aggregate(
    location: Location,
    left_guard: Option<Ast>,
    elements: &[Ast],
    right_guard: Option<Ast>,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let left_guard = match &left_guard {
        Some(left_guard) => &left_guard.0,
        None => std::ptr::null(),
    };
    let right_guard = match &right_guard {
        Some(right_guard) => &right_guard.0,
        None => std::ptr::null(),
    };
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_aggregate as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.BodyAggregateElement`.
pub fn BodyAggregateElement(terms: &[Ast], condition: &[Ast]) -> Result<Ast, ClingoError> {
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

// pub fn BodyAggregate(location: Location, left_guard: Option<Ast>, function: int, elements: &[Ast],
//                   right_guard: Option<Ast>) -> Result<Ast,ClingoError> {
//     /// Construct an AST node of type `ASTType.BodyAggregate`.

//     let mut ast = std::ptr::null_mut();

//     if !unsafe { clingo_ast_build(
//         clingo_ast_type_e_clingo_ast_type_body_aggregate as i32, &mut ast,
//         &location,
//         _ffi.NULL if left_guard is None else left_guard.0,
//         _ffi.cast('int', function),
//         _ffi.new('clingo_ast_t*[]', [ x.0 for x in elements ]),
//         _ffi.cast('size_t', len(elements)),
//         _ffi.NULL if right_guard is None else right_guard.0) } {
//             return Err(ClingoError::new_internal(
//                 "Call to clingo_ast_build() failed.",
//             ));
//         }
//         match unsafe { ast.as_mut() } {
//             Some( ast_ref) => Ok(Ast(ast_ref)),
//             None => Err(ClingoError::FFIError {
//                 msg: "tried casting a null pointer to &mut clingo_ast.",
//             }),
//         }
//     }

// pub fn HeadAggregateElement(terms: &[Ast], condition: Ast) -> Result<Ast,ClingoError> {
//     /// Construct an AST node of type `ASTType.HeadAggregateElement`.

//     let mut ast = std::ptr::null_mut();
//     if !unsafe { clingo_ast_build(
//         clingo_ast_type_e_clingo_ast_type_head_aggregate_element as i32, &mut ast,
//         terms.as_ptr() as *const clingo_ast_t,
//         terms.len(),
//         condition.0) } {
//             return Err(ClingoError::new_internal(
//                 "Call to clingo_ast_build() failed.",
//             ));
//         }
//         match unsafe { ast.as_mut() } {
//             Some( ast_ref) => Ok(Ast(ast_ref)),
//             None => Err(ClingoError::FFIError {
//                 msg: "tried casting a null pointer to &mut clingo_ast.",
//             }),
//         }
//     }

// pub fn HeadAggregate(location: Location, left_guard: Option<Ast>, function: int, elements: &[Ast],
//                   right_guard: Option<Ast>) -> Result<Ast,ClingoError> {
//     /// Construct an AST node of type `ASTType.HeadAggregate`.

//     let mut ast = std::ptr::null_mut();

//     if !unsafe { clingo_ast_build(
//         clingo_ast_type_e_clingo_ast_type_head_aggregate as i32, &mut ast,
//         &location,
//         _ffi.NULL if left_guard is None else left_guard.0,
//         _ffi.cast('int', function),
//         _ffi.new('clingo_ast_t*[]', [ x.0 for x in elements ]),
//         _ffi.cast('size_t', len(elements)),
//         _ffi.NULL if right_guard is None else right_guard.0) } {
//             return Err(ClingoError::new_internal(
//                 "Call to clingo_ast_build() failed.",
//             ));
//         }
//         match unsafe { ast.as_mut() } {
//             Some( ast_ref) => Ok(Ast(ast_ref)),
//             None => Err(ClingoError::FFIError {
//                 msg: "tried casting a null pointer to &mut clingo_ast.",
//             }),
//         }
//     }

// pub fn Disjunction(location: Location, elements: &[Ast]) -> Result<Ast,ClingoError> {
//     /// Construct an AST node of type `ASTType.Disjunction`.

//     let mut ast = std::ptr::null_mut();

//     if !unsafe { clingo_ast_build(
//         clingo_ast_type_e_clingo_ast_type_disjunction as i32, &mut ast,
//         &location,
//         _ffi.new('clingo_ast_t*[]', [ x.0 for x in elements ]),
//         _ffi.cast('size_t', len(elements))) } {
//             return Err(ClingoError::new_internal(
//                 "Call to clingo_ast_build() failed.",
//             ));
//         }
//         match unsafe { ast.as_mut() } {
//             Some( ast_ref) => Ok(Ast(ast_ref)),
//             None => Err(ClingoError::FFIError {
//                 msg: "tried casting a null pointer to &mut clingo_ast.",
//             }),
//         }
//     }

/// Construct an AST node of type `ASTType.DisjointElement`.
pub fn DisjointElement(
    location: Location,
    terms: &[Ast],
    term: Ast,
    condition: &[Ast],
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_disjoint_element as i32,
            &mut ast,
            &location,
            terms.as_ptr() as *const clingo_ast_t,
            terms.len(),
            term.0,
            condition.as_ptr() as *const clingo_ast_t,
            condition.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
/// Construct an AST node of type `ASTType.Disjoint`.
pub fn Disjoint(location: Location, elements: &[Ast]) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_disjoint as i32,
            &mut ast,
            &location,
            elements.as_ptr() as *const clingo_ast_t,
            elements.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
/// Construct an AST node of type `ASTType.TheorySequence`.

pub fn TheorySequence(
    location: Location,
    sequence_type: TheoryTermSequenceType,
    terms: &[Ast],
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_sequence as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryFunction`.
pub fn TheoryFunction(
    location: Location,
    name: &str,
    arguments: &[Ast],
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_function as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

//    /// Construct an AST node of type `ASTType.TheoryUnparsedTermElement`.
// pub fn TheoryUnparsedTermElement(operators: Sequence[str], term: &Ast) -> Result<Ast,ClingoError> {
//
// let mut ast = std::ptr::null_mut();
// c_operators = [ _ffi.new('char[]', x.encode()) for x in operators ]
// if !unsafe { clingo_ast_build(
// clingo_ast_type_e_clingo_ast_type_theory_unparsed_term_element as i32, &mut ast,
// _ffi.new('char*[]', c_operators),
// _ffi.cast('size_t', len(operators)),
// &term.0) } {
// return Err(ClingoError::new_internal(
// "Call to clingo_ast_build() failed.",
// ));
// }
// match unsafe { ast.as_mut() } {
// Some( ast_ref) => Ok(Ast(ast_ref)),
// None => Err(ClingoError::FFIError {
// msg: "tried casting a null pointer to &mut clingo_ast.",
// }),
// }
// }

/// Construct an AST node of type `ASTType.TheoryUnparsedTerm`.
pub fn TheoryUnparsedTerm(location: Location, elements: &[Ast]) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_unparsed_term as i32,
            &mut ast,
            &location,
            elements.as_ptr() as *const clingo_ast_t,
            elements.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryGuard`.
pub fn TheoryGuard(operator_name: &str, term: Ast) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let operator_name = internalize_string(operator_name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_guard as i32,
            &mut ast,
            operator_name,
            term.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryAtomElement`.
pub fn TheoryAtomElement(terms: &[Ast], condition: &[Ast]) -> Result<Ast, ClingoError> {
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryAtom`.
pub fn TheoryAtom(
    location: Location,
    term: Ast,
    elements: &[Ast],
    guard: Option<Ast>,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    let guard = match &guard {
        Some(guard) => &guard.0,
        None => std::ptr::null(),
    };
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_atom as i32,
            &mut ast,
            &location,
            term.0,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Literal`.
pub fn Literal(location: &Location, sign: Sign, atom: &Ast) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_literal as i32,
            &mut ast,
            location,
            sign as i32,
            atom.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryOperatorDefinition`.
pub fn TheoryOperatorDefinition(
    location: Location,
    name: &str,
    priority: u32,
    operator_type: u32,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    let name = internalize_string(name)?;
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_operator_definition as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryTermDefinition`.
pub fn TheoryTermDefinition(
    location: Location,
    name: &str,
    operators: &[Ast],
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_term_definition as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryGuardDefinition`.
pub fn TheoryGuardDefinition(operators: &[&str], term: &str) -> Result<Ast, ClingoError> {
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
            c_operators.as_ptr() as *const *const c_char,
            c_operators.len(),
            term,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryAtomDefinition`.
pub fn TheoryAtomDefinition(
    location: Location,
    atom_type: TheoryAtomType,
    name: &str,
    arity: u32,
    term: &str,
    guard: Option<Ast>,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;
    let term = internalize_string(term)?;
    let guard = match &guard {
        Some(guard) => &guard.0,
        None => std::ptr::null(),
    };

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_atom_definition as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Rule`.
pub fn Rule(location: &Location, head: Ast, body: &[Ast]) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_rule as i32,
            &mut ast,
            location,
            head.0,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Definition`.
pub fn Definition(
    location: Location,
    name: &str,
    value: Ast,
    is_default: bool,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_definition as i32,
            &mut ast,
            &location,
            name,
            value.0,
            is_default as i32,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ShowSignature`.
pub fn ShowSignature(
    location: Location,
    name: &str,
    arity: u32,
    positive: bool,
    csp: bool,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_show_signature as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ShowTerm`.
pub fn ShowTerm(
    location: Location,
    term: Ast,
    body: &[Ast],
    csp: bool,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_show_term as i32,
            &mut ast,
            &location,
            term.0,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Minimize`.
pub fn Minimize(
    location: Location,
    weight: Ast,
    priority: Ast,
    terms: &[Ast],
    body: &[Ast],
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_minimize as i32,
            &mut ast,
            &location,
            weight.0,
            priority.0,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Script`.
pub fn Script(location: Location, script_type: ScriptType, code: &str) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let code = internalize_string(code);

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_script as i32,
            &mut ast,
            &location,
            script_type as i32,
            code,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Program`.
pub fn Program(location: Location, name: &str, parameters: &[Ast]) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_program as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.External`.
pub fn External(
    location: &Location,
    atom: &Ast,
    body: &[Ast],
    external_type: &Ast,
) -> Result<Ast, ClingoError> {
    println!("in External");
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_external as i32,
            &mut ast,
            &location,
            atom.0,
            body.as_ptr() as *const clingo_ast_t,
            0,
            external_type.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    println!("out External");
    println!("External pointer: {:?}", ast);
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Edge`.
pub fn Edge(
    location: Location,
    node_u: Ast,
    node_v: Ast,
    body: &[Ast],
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_edge as i32,
            &mut ast,
            &location,
            node_u.0,
            node_v.0,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Heuristic`.
pub fn Heuristic(
    location: Location,
    atom: Ast,
    body: &[Ast],
    bias: Ast,
    priority: Ast,
    modifier: Ast,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_heuristic as i32,
            &mut ast,
            &location,
            atom.0,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
            bias.0,
            priority.0,
            modifier.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ProjectAtom`.
pub fn ProjectAtom(location: Location, atom: Ast, body: &[Ast]) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_project_atom as i32,
            &mut ast,
            &location,
            atom.0,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.ProjectSignature`.
pub fn ProjectSignature(
    location: Location,
    name: &str,
    arity: u32,
    positive: bool,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_project_signature as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.Defined`.
pub fn Defined(
    location: Location,
    name: &str,
    arity: u32,
    positive: bool,
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_defined as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

/// Construct an AST node of type `ASTType.TheoryDefinition`.
pub fn TheoryDefinition(
    location: Location,
    name: &str,
    terms: &[Ast],
    atoms: &[Ast],
) -> Result<Ast, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_definition as i32,
            &mut ast,
            &location,
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
        Some(ast) => Ok(Ast(ast)),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
impl Ast {
    // pub fn build_symbolic_term<'a>(sym: &Symbol) -> Result<Ast, ClingoError> {
    //     let mut ast = std::ptr::null_mut();
    //     let loc = Location::default();
    //     if !unsafe { clingo_ast_build(AstType::SymbolicTerm as i32, &mut ast, &loc, *sym) } {
    //         return Err(ClingoError::new_internal(
    //             "Call to clingo_ast_build() failed.",
    //         ));
    //     }
    //     match NonNull::new(ast) {
    //         Some(ast) => Ok(Ast(ast)),
    //         None => Err(ClingoError::FFIError {
    //             msg: "Tried creating NonNull from a null pointer.",
    //         })?,
    //     }
    // }
    // pub fn build_symbolic_atom<'a>(sym_term: Ast) -> Result<Ast, ClingoError> {
    //     let mut ast = std::ptr::null_mut();
    //     let loc = Location::default();
    //     if !unsafe { clingo_ast_build(AstType::SymbolicAtom as i32, &mut ast, &loc, sym_term.0) } {
    //         return Err(ClingoError::new_internal(
    //             "Call to clingo_ast_build() failed.",
    //         ));
    //     }
    //     match NonNull::new(ast) {
    //         Some(ast) => Ok(Ast(ast)),
    //         None => Err(ClingoError::FFIError {
    //             msg: "Tried creating NonNull from a null pointer.",
    //         })?,
    //     }
    // }

    // extern "C" {
    //     #[doc = "! Increment the reference count of an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @note All functions that return AST nodes already increment the reference count."]
    //     #[doc = "! The reference count of callback arguments is not incremented."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     pub fn clingo_ast_acquire(ast: *mut clingo_ast_t);
    // }
    // extern "C" {
    //     #[doc = "! Decrement the reference count of an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @note The node is deleted if the reference count reaches zero."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     pub fn clingo_ast_release(ast: *mut clingo_ast_t);
    // }
    // extern "C" {
    //     #[doc = "! Deep copy an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the AST to copy"]
    //     #[doc = "! @param[out] copy the resulting AST"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_bad_alloc"]
    //     pub fn clingo_ast_copy(ast: *mut clingo_ast_t, copy: *mut *mut clingo_ast_t) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Create a shallow copy of an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the AST to copy"]
    //     #[doc = "! @param[out] copy the resulting AST"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_bad_alloc"]
    //     pub fn clingo_ast_deep_copy(ast: *mut clingo_ast_t, copy: *mut *mut clingo_ast_t) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Less than compare two AST nodes."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] a the left-hand-side AST"]
    //     #[doc = "! @param[in] b the right-hand-side AST"]
    //     #[doc = "! @return the result of the comparison"]
    //     pub fn clingo_ast_less_than(a: *mut clingo_ast_t, b: *mut clingo_ast_t) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Equality compare two AST nodes."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] a the left-hand-side AST"]
    //     #[doc = "! @param[in] b the right-hand-side AST"]
    //     #[doc = "! @return the result of the comparison"]
    //     pub fn clingo_ast_equal(a: *mut clingo_ast_t, b: *mut clingo_ast_t) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Compute a hash for an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @return the resulting hash code"]
    //     pub fn clingo_ast_hash(ast: *mut clingo_ast_t) -> usize;
    // }
    // extern "C" {
    //     #[doc = "! Get the size of the string representation of an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[out] size the size of the string representation"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_to_string_size(ast: *mut clingo_ast_t, size: *mut usize) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the string representation of an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[out] string the string representation"]
    //     #[doc = "! @param[out] size the size of the string representation"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_to_string(
    //         ast: *mut clingo_ast_t,
    //         string: *mut ::std::os::raw::c_char,
    //         size: usize,
    //     ) -> bool;
    // }
    pub fn to_string(&self) -> Result<String, ClingoError> {
        let mut size: usize = 0;
        if !unsafe { clingo_ast_to_string_size(self.0.as_ptr(), &mut size) } {
            eprintln!("Call to clingo_ast_to_string_size() failed");
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_to_string_size() failed.",
            ));
        }
        // println!("size: {}", size);
        let mut string = Vec::with_capacity(size);
        let string_ptr = string.as_mut_ptr();
        if !unsafe { clingo_ast_to_string(self.0.as_ptr(), string_ptr, size) } {
            eprintln!("Call to clingo_ast_to_string() failed");
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_to_string() failed.",
            ));
        }
        let c_str: &CStr = unsafe { CStr::from_ptr(string_ptr) };
        let str_slice: &str = match c_str.to_str() {
            Ok(slice) => slice,
            Err(e) => {
                eprintln!("{:?}", e);
                return Err(ClingoError::new_internal("Call to c_str.to_str() failed."));
            }
        };
        // println!("xxxx : {}", str_slice);
        Ok(str_slice.to_string())
    }

    /// Get the type of an AST node.
    ///
    /// @param[in] ast the target AST
    /// @param[out] type the resulting type
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub fn get_type(&self) -> Result<AstType, ClingoError> {
        let mut ast_type = 0;
        if !unsafe { clingo_ast_get_type(self.0.as_ptr(), &mut ast_type) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_get_type() failed.",
            ));
        }
        AstType::try_from(ast_type as u32)
    }

    // extern "C" {
    //     #[doc = "! Check if an AST has the given attribute."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the attribute to check"]
    //     #[doc = "! @param[out] has_attribute the result"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_has_attribute(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         has_attribute: *mut bool,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the type of the given AST."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[out] type the resulting type"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_type(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         type_: *mut clingo_ast_attribute_type_t,
    //     ) -> bool;
    // }
    // Get the type of the given AST node.
    //
    // #[doc = "! @param[in] ast the target AST"]
    // #[doc = "! @param[in] attribute the target attribute"]
    // #[doc = "! @param[out] type the resulting type"]
    // #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    // #[doc = "! - ::clingo_error_runtime"]
    pub fn get_attribute_type(
        &self,
        attribute: &AstAttribute,
    ) -> Result<AstAttributeType, ClingoError> {
        let mut attribute_type = 0;
        if !unsafe {
            clingo_ast_attribute_type(self.0.as_ptr(), *attribute as i32, &mut attribute_type)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_type() failed.",
            ));
        }
        AstAttributeType::try_from(attribute_type as u32)
    }
    // extern "C" {
    //     #[doc = "! Get the value of an attribute of type \"clingo_ast_attribute_type_number\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[out] value the resulting value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_get_number(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *mut ::std::os::raw::c_int,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Set the value of an attribute of type \"clingo_ast_attribute_type_number\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] value the value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_set_number(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: ::std::os::raw::c_int,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the value of an attribute of type \"clingo_ast_attribute_type_symbol\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[out] value the resulting value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_get_symbol(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *mut clingo_symbol_t,
    //     ) -> bool;
    // }
    //  Get the value of an attribute of type AstAttributeType::Symbol
    //
    // #[doc = "! @param[in] ast the target AST"]
    // #[doc = "! @param[in] attribute the target attribute"]
    // #[doc = "! @param[out] type the resulting type"]
    // #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    // #[doc = "! - ::clingo_error_runtime"]
    pub fn get_symbol(&self) -> Result<Symbol, ClingoError> {
        let mut sym = 0;
        let attribute = AstAttributeType::Symbol;
        if !unsafe { clingo_ast_attribute_get_symbol(self.0.as_ptr(), attribute as i32, &mut sym) }
        {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_symbol() failed.",
            ));
        }
        Ok(Symbol(sym))
    }
    // extern "C" {
    //     #[doc = "! Set the value of an attribute of type \"clingo_ast_attribute_type_symbol\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] value the value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_set_symbol(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: clingo_symbol_t,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the value of an attribute of type \"clingo_ast_attribute_type_location\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[out] value the resulting value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_get_location(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *mut clingo_location_t,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Set the value of an attribute of type \"clingo_ast_attribute_type_location\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] value the value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_set_location(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *const clingo_location_t,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the value of an attribute of type \"clingo_ast_attribute_type_string\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[out] value the resulting value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_get_string(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *mut *const ::std::os::raw::c_char,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Set the value of an attribute of type \"clingo_ast_attribute_type_string\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] value the value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_set_string(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *const ::std::os::raw::c_char,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the value of an attribute of type \"clingo_ast_attribute_type_ast\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[out] value the resulting value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_get_ast(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *mut *mut clingo_ast_t,
    //     ) -> bool;
    // }
    // Get the value of an attribute of type AstAttributeType
    //
    // #[doc = "! @param[in] ast the target AST"]
    // #[doc = "! @param[in] attribute the target attribute"]
    // #[doc = "! @param[out] type the resulting type"]
    // #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    // #[doc = "! - ::clingo_error_runtime"]
    pub fn get_attribute_ast(&self, attribute: &AstAttribute) -> Result<Ast, ClingoError> {
        let mut ast = std::ptr::null_mut();
        if !unsafe { clingo_ast_attribute_get_ast(self.0.as_ptr(), *attribute as i32, &mut ast) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_ast() failed.",
            ));
        }
        match NonNull::new(ast) {
            Some(x) => Ok(Ast(x)),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }

    // extern "C" {
    //     #[doc = "! Set the value of an attribute of type \"clingo_ast_attribute_type_ast\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] value the value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_set_ast(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *mut clingo_ast_t,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the value of an attribute of type \"clingo_ast_attribute_type_optional_ast\"."]
    //     #[doc = "!"]
    //     #[doc = "! @note The value might be \"NULL\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[out] value the resulting value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_get_optional_ast(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *mut *mut clingo_ast_t,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Set the value of an attribute of type \"clingo_ast_attribute_type_optional_ast\"."]
    //     #[doc = "!"]
    //     #[doc = "! @note The value might be \"NULL\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] value the value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_set_optional_ast(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         value: *mut clingo_ast_t,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the value of an attribute of type \"clingo_ast_attribute_type_string_array\" at the given index."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] index the target index"]
    //     #[doc = "! @param[out] value the resulting value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_get_string_at(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         index: usize,
    //         value: *mut *const ::std::os::raw::c_char,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Set the value of an attribute of type \"clingo_ast_attribute_type_string_array\" at the given index."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] index the target index"]
    //     #[doc = "! @param[in] value the value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     #[doc = "! - ::clingo_error_bad_alloc"]
    //     pub fn clingo_ast_attribute_set_string_at(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         index: usize,
    //         value: *const ::std::os::raw::c_char,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Remove an element from an attribute of type \"clingo_ast_attribute_type_string_array\" at the given index."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] index the target index"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_delete_string_at(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         index: usize,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the size of an attribute of type \"clingo_ast_attribute_type_string_array\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[out] size the resulting size"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_size_string_array(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         size: *mut usize,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Insert a value into an attribute of type \"clingo_ast_attribute_type_string_array\" at the given index."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] index the target index"]
    //     #[doc = "! @param[in] value the value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     #[doc = "! - ::clingo_error_bad_alloc"]
    //     pub fn clingo_ast_attribute_insert_string_at(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         index: usize,
    //         value: *const ::std::os::raw::c_char,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the value of an attribute of type \"clingo_ast_attribute_type_ast_array\" at the given index."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] index the target index"]
    //     #[doc = "! @param[out] value the resulting value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_get_ast_at(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         index: usize,
    //         value: *mut *mut clingo_ast_t,
    //     ) -> bool;
    // }
    // Get the value of an attribute of type AstAttributeType::AstArray at the given index."]
    //
    // #[doc = "! @param[in] ast the target AST"]
    // #[doc = "! @param[in] attribute the target attribute"]
    // #[doc = "! @param[out] type the resulting type"]
    // #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    // #[doc = "! - ::clingo_error_runtime"]
    pub fn get_attribute_ast_at(
        &self,
        attribute: &AstAttribute,
        index: usize,
    ) -> Result<Ast, ClingoError> {
        let mut ast = std::ptr::null_mut();
        if !unsafe {
            clingo_ast_attribute_get_ast_at(self.0.as_ptr(), *attribute as i32, index, &mut ast)
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_get_ast_at() failed.",
            ));
        }
        match NonNull::new(ast) {
            Some(x) => Ok(Ast(x)),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }

    // extern "C" {
    //     #[doc = "! Set the value of an attribute of type \"clingo_ast_attribute_type_ast_array\" at the given index."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] index the target index"]
    //     #[doc = "! @param[in] value the value"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     #[doc = "! - ::clingo_error_bad_alloc"]
    //     pub fn clingo_ast_attribute_set_ast_at(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         index: usize,
    //         value: *mut clingo_ast_t,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Remove an element from an attribute of type \"clingo_ast_attribute_type_ast_array\" at the given index."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[in] index the target index"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_delete_ast_at(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         index: usize,
    //     ) -> bool;
    // }
    // extern "C" {
    //     #[doc = "! Get the size of an attribute of type \"clingo_ast_attribute_type_ast_array\"."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     #[doc = "! @param[in] attribute the target attribute"]
    //     #[doc = "! @param[out] size the resulting size"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime"]
    //     pub fn clingo_ast_attribute_size_ast_array(
    //         ast: *mut clingo_ast_t,
    //         attribute: clingo_ast_attribute_t,
    //         size: *mut usize,
    //     ) -> bool;
    // }
    pub fn ast_array(&self, attribute: AstAttribute) -> AstArray {
        AstArray {
            ast: &self,
            attribute,
        }
    }
}
// extern "C" {
//     #[doc = "! Insert a value into an attribute of type \"clingo_ast_attribute_type_ast_array\" at the given index."]
//     #[doc = "!"]
//     #[doc = "! @param[in] ast the target AST"]
//     #[doc = "! @param[in] attribute the target attribute"]
//     #[doc = "! @param[in] index the target index"]
//     #[doc = "! @param[in] value the value"]
//     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
//     #[doc = "! - ::clingo_error_runtime"]
//     #[doc = "! - ::clingo_error_bad_alloc"]
//     pub fn clingo_ast_attribute_insert_ast_at(
//         ast: *mut clingo_ast_t,
//         attribute: clingo_ast_attribute_t,
//         index: usize,
//         value: *mut clingo_ast_t,
//     ) -> bool;
// }
// #[doc = "! Callback function to intercept AST nodes."]
// #[doc = "!"]
// #[doc = "! @param[in] ast the AST"]
// #[doc = "! @param[in] data a user data pointer"]
// #[doc = "! @return whether the call was successful"]
// pub type clingo_ast_callback_v2_t = ::std::option::Option<
//     unsafe extern "C" fn(ast: *mut clingo_ast_t, data: *mut ::std::os::raw::c_void) -> bool,
// >;
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

pub fn parse_string(program: &str) -> Result<(), ClingoError> {
    let program = CString::new(program)?;
    println!("in parse_string");
    println!("{:?}", program);
    if !unsafe {
        clingo_ast_parse_string(
            program.as_ptr(),
            None,
            std::ptr::null_mut() as *mut c_void,
            None,
            std::ptr::null_mut(),
            0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_parse_string() failed",
        ));
    }
    println!("out parse_string");
    Ok(())
}
pub fn parse_string_with_statement_handler<T: StatementHandler>(
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
            Some(unsafe_ast_callback::<T> as AstCallback),
            handler as *mut c_void,
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
// extern "C" {
//     #[doc = "! Parse the programs in the given list of files and return an abstract syntax tree for each statement via a callback."]
//     #[doc = "!"]
//     #[doc = "! The function follows clingo's handling of files on the command line."]
//     #[doc = "! Filename \"-\" is treated as \"STDIN\" and if an empty list is given, then the parser will read from \"STDIN\"."]
//     #[doc = "!"]
//     #[doc = "! @param[in] files the beginning of the file name array"]
//     #[doc = "! @param[in] size the number of file names"]
//     #[doc = "! @param[in] callback the callback reporting statements"]
//     #[doc = "! @param[in] callback_data user data for the callback"]
//     #[doc = "! @param[in] logger callback to report messages during parsing"]
//     #[doc = "! @param[in] logger_data user data for the logger"]
//     #[doc = "! @param[in] message_limit the maximum number of times the logger is called"]
//     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
//     #[doc = "! - ::clingo_error_runtime if parsing fails"]
//     #[doc = "! - ::clingo_error_bad_alloc"]
//     pub fn clingo_ast_parse_files(
//         files: *const *const ::std::os::raw::c_char,
//         size: usize,
//         callback: clingo_ast_callback_v2_t,
//         callback_data: *mut ::std::os::raw::c_void,
//         logger: clingo_logger_t,
//         logger_data: *mut ::std::os::raw::c_void,
//         message_limit: ::std::os::raw::c_uint,
//     ) -> bool;
// }

#[derive(Debug, Copy, Clone)]
/// Enum to configure unpooling.
pub enum Unpooling {
    /// To only unpool conditions of conditional literals.
    Condition = clingo_ast_unpool_type_e_clingo_ast_unpool_type_condition as isize,
    /// To unpool everything except conditions of conditional literals.
    Other = clingo_ast_unpool_type_e_clingo_ast_unpool_type_other as isize,
    /// To unpool everything.
    All = clingo_ast_unpool_type_e_clingo_ast_unpool_type_all as isize,
}

// TODO
// extern "C" {
//     #[doc = "! Unpool the given AST."]
//     #[doc = "!"]
//     #[doc = "! @param[in] ast the target AST"]
//     #[doc = "! @param[in] unpool_type what to unpool"]
//     #[doc = "! @param[in] callback the callback to report ASTs"]
//     #[doc = "! @param[in] callback_data user data for the callback"]
//     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
//     #[doc = "! - ::clingo_error_bad_alloc"]
//     pub fn clingo_ast_unpool(
//         ast: *mut clingo_ast_t,
//         unpool_type: clingo_ast_unpool_type_bitset_t,
//         callback: clingo_ast_callback_v2_t,
//         callback_data: *mut ::std::os::raw::c_void,
//     ) -> bool;
// }
