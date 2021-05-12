use crate::ast::{BodyLiteral, Head};
use crate::{ClingoError, Symbol};
use clingo_sys::*;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr::NonNull;
pub struct Body<'a> {
    ast: &'a Ast,
    index: usize,
}
impl<'a> Body<'a> {
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
                AstAttribute::Body as i32,
                &mut size,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_attribute_size_ast_array() failed.",
            ));
        }
        Ok(size)
    }
}

impl<'a> Iterator for Body<'a> {
    type Item = BodyLiteral<'a>;

    fn next(&mut self) -> Option<BodyLiteral<'a>> {
        let size = self.size().unwrap(); //Err->None

        if size == self.index {
            return None;
        }

        let mut ast = std::ptr::null_mut();
        if !unsafe {
            clingo_ast_attribute_get_ast_at(
                self.ast.0.as_ptr(),
                AstAttribute::Body as i32,
                self.index,
                &mut ast,
            )
        } {
            return None;
        }
        self.index += 1;
        NonNull::new(ast).map(|x| BodyLiteral {
            ast: Ast(x),
            _lifetime: PhantomData,
        })
    }
}

// struct AstArray<'a> {
//     ast: &'a Ast,
//     attribute: AstAttribute,
// }
// impl<'a> AstArray<'a> {
//     /// Get the size of an AstArray
//     ///
//     /// @param[in] ast the target AstArray
//     /// @param[in] attribute the target attribute"]
//     /// @param[out] size the resulting size"]
//     /// @return whether the call was successful; might set one of the following error codes:"]
//     /// - ::clingo_error_runtime"]

//     pub fn size(&self) -> Result<usize, ClingoError> {
//         let mut size: usize = 0;
//         if !unsafe {
//             clingo_ast_attribute_size_ast_array(
//                 self.ast.0.as_ptr(),
//                 self.attribute as i32,
//                 &mut size,
//             )
//         } {
//             return Err(ClingoError::new_internal(
//                 "Call to clingo_ast_attribute_size_ast_array() failed.",
//             ));
//         }
//         Ok(size)
//     }

//     ///  Returns an iterator over the theory atoms.
//     fn iter(&self) -> AstArrayIterator {
//         AstArrayIterator {
//             ast_array: self,
//             index: 0,
//         }
//     }
// }
// struct AstArrayIterator<'a> {
//     ast_array: &'a AstArray<'a>,
//     index: usize,
// }
// impl<'a> Iterator for AstArrayIterator<'a> {
//     type Item = Ast;

//     fn next(&mut self) -> Option<Ast> {
//         let size = self.ast_array.size().unwrap(); //Err->None

//         if size == self.index {
//             return None;
//         }

//         let mut ast = std::ptr::null_mut();
//         if !unsafe {
//             clingo_ast_attribute_get_ast_at(
//                 self.ast_array.ast.0.as_ptr(),
//                 self.ast_array.attribute as i32,
//                 self.index,
//                 &mut ast,
//             )
//         } {
//             return None;
//         }
//         self.index += 1;
//         match NonNull::new(ast) {
//             Some(x) => Some(Ast(x)),
//             None => None,
//         }
//     }
// }

// Here starts AST2

#[derive(Debug, Copy, Clone)]
/// Enumeration of AST types.
pub(crate) enum AstType {
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
    ProjectSignature = clingo_ast_type_e_clingo_ast_type_project_signature as isize,
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
            clingo_ast_type_e_clingo_ast_type_project_signature => Ok(AstType::ProjectSignature),
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
// impl AstAttributeType {
//     fn try_from(code: u32) -> Result<AstAttributeType, ClingoError> {
//         // println!("in try_from");
//         match code {
//             clingo_ast_attribute_type_e_clingo_ast_attribute_type_number => {
//                 Ok(AstAttributeType::Number)
//             }
//             clingo_ast_attribute_type_e_clingo_ast_attribute_type_symbol => {
//                 Ok(AstAttributeType::Symbol)
//             }
//             clingo_ast_attribute_type_e_clingo_ast_attribute_type_location => {
//                 Ok(AstAttributeType::Location)
//             }
//             clingo_ast_attribute_type_e_clingo_ast_attribute_type_string => {
//                 Ok(AstAttributeType::String)
//             }
//             clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast => Ok(AstAttributeType::Ast),
//             clingo_ast_attribute_type_e_clingo_ast_attribute_type_optional_ast => {
//                 Ok(AstAttributeType::OptionalAst)
//             }
//             clingo_ast_attribute_type_e_clingo_ast_attribute_type_string_array => {
//                 Ok(AstAttributeType::StringArray)
//             }
//             clingo_ast_attribute_type_e_clingo_ast_attribute_type_ast_array => {
//                 Ok(AstAttributeType::AstArray)
//             }
//             x => {
//                 eprintln!(
//                     "FFIError in {} {}, {} : Failed to match clingo_ast_type {}",
//                     file!(),
//                     line!(),
//                     column!(),
//                     x
//                 );
//                 Err(ClingoError::FFIError {
//                     msg: "Failed to match clingo_ast_type.",
//                 })
//             }
//         }
//     }
// }
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
    SequenceType = clingo_ast_attribute_e_clingo_ast_attribute_sequence_type as isize,
    Sign = clingo_ast_attribute_e_clingo_ast_attribute_sign as isize,
    Symbol = clingo_ast_attribute_e_clingo_ast_attribute_symbol as isize,
    Term = clingo_ast_attribute_e_clingo_ast_attribute_term as isize,
    Terms = clingo_ast_attribute_e_clingo_ast_attribute_terms as isize,
    Value = clingo_ast_attribute_e_clingo_ast_attribute_value as isize,
    Variable = clingo_ast_attribute_e_clingo_ast_attribute_variable as isize,
    Weight = clingo_ast_attribute_e_clingo_ast_attribute_weight as isize,
}

/// This struct provides a view to nodes in the AST.
#[derive(Debug)]
pub(crate) struct Ast(pub NonNull<clingo_ast_t>);

impl Clone for Ast {
    fn clone(&self) -> Ast {
        self.deep_copy().unwrap()
    }
}
use std::fmt;
impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = self.to_string();
        match x {
            Ok(string) => write!(f, "{}", string),
            Err(e) => {
                eprintln!("{}", e);
                Err(fmt::Error)
            }
        }
    }
}

impl Drop for Ast {
    fn drop(&mut self) {
        self.release()
    }
}
impl Ast {
    pub fn body(&self) -> Body {
        Body {
            ast: &self,
            index: 0,
        }
    }
    pub fn head<'a>(&self) -> Head<'a> {
        let ast = self.get_attribute_ast(AstAttribute::Head).unwrap();
        Head {
            ast,
            _lifetime: PhantomData,
        }
    }

    // extern "C" {
    //     #[doc = "! Increment the reference count of an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @note All functions that return AST nodes already increment the reference count."]
    //     #[doc = "! The reference count of callback arguments is not incremented."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     pub fn clingo_ast_acquire(ast: *mut clingo_ast_t);
    // }
    pub fn acquire(&self) {
        // println!("acquire");
        // println!("ast: {:?}", self);
        // println!("ast: {}", self.to_string().unwrap());
        unsafe { clingo_ast_acquire(self.0.as_ptr()) }
    }
    // extern "C" {
    //     #[doc = "! Decrement the reference count of an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @note The node is deleted if the reference count reaches zero."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the target AST"]
    //     pub fn clingo_ast_release(ast: *mut clingo_ast_t);
    // }
    fn release(&self) {
        // println!("release");
        // println!("ast: {:?}", self);
        // println!("ast: {}", self.to_string().unwrap());
        unsafe { clingo_ast_release(self.0.as_ptr()) }
    }
    // extern "C" {
    // #[doc = "! Create a shallow copy of an AST node."]
    // #[doc = "!"]
    // #[doc = "! @param[in] ast the AST to copy"]
    // #[doc = "! @param[out] copy the resulting AST"]
    // #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    // #[doc = "! - ::clingo_error_bad_alloc"]
    // pub fn clingo_ast_copy(ast: *mut clingo_ast_t, copy: *mut *mut clingo_ast_t) -> bool;
    // }
    fn copy(&self) -> Result<Ast, ClingoError> {
        let mut cpy = std::ptr::null_mut();
        if !unsafe { clingo_ast_copy(self.0.as_ptr(), &mut cpy) } {
            eprintln!("Call to clingo_ast_copy() failed");
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_copy() failed.",
            ));
        }
        match NonNull::new(cpy) {
            Some(cpy) => Ok(Ast(cpy)),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
    // extern "C" {
    //     #[doc = "! Create a deep copy of an AST node."]
    //     #[doc = "!"]
    //     #[doc = "! @param[in] ast the AST to copy"]
    //     #[doc = "! @param[out] copy the resulting AST"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_bad_alloc"]
    //     pub fn clingo_ast_deep_copy(ast: *mut clingo_ast_t, copy: *mut *mut clingo_ast_t) -> bool;
    // }
    fn deep_copy(&self) -> Result<Ast, ClingoError> {
        let mut cpy = std::ptr::null_mut();
        if !unsafe { clingo_ast_deep_copy(self.0.as_ptr(), &mut cpy) } {
            eprintln!("Call to clingo_ast_deep_copy() failed");
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_deep_copy() failed.",
            ));
        }
        match NonNull::new(cpy) {
            Some(cpy) => Ok(Ast(cpy)),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
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
        Ok(str_slice.to_string())
    }

    /// Get the type of an AST node.
    ///
    /// @param[in] ast the target AST
    /// @param[out] type the resulting type
    /// @return whether the call was successful; might set one of the following error codes:
    /// - ::clingo_error_runtime
    pub(crate) fn get_type(&self) -> Result<AstType, ClingoError> {
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

    // Get the type of the given AST node.
    //
    // #[doc = "! @param[in] ast the target AST"]
    // #[doc = "! @param[in] attribute the target attribute"]
    // #[doc = "! @param[out] type the resulting type"]
    // #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    // #[doc = "! - ::clingo_error_runtime"]
    // fn get_attribute_type(&self, attribute: AstAttribute) -> Result<AstAttributeType, ClingoError> {
    //     let mut attribute_type = 0;
    //     if !unsafe {
    //         clingo_ast_attribute_type(self.0.as_ptr(), attribute as i32, &mut attribute_type)
    //     } {
    //         return Err(ClingoError::new_internal(
    //             "Call to clingo_ast_attribute_type() failed.",
    //         ));
    //     }
    //     AstAttributeType::try_from(attribute_type as u32)
    // }
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

    //  Get the value of an attribute of type AstAttributeType::Symbol
    //
    // #[doc = "! @param[in] ast the target AST"]
    // #[doc = "! @param[in] attribute the target attribute"]
    // #[doc = "! @param[out] type the resulting type"]
    // #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    // #[doc = "! - ::clingo_error_runtime"]
    fn get_symbol(&self) -> Result<Symbol, ClingoError> {
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
    fn get_attribute_ast(&self, attribute: AstAttribute) -> Result<Ast, ClingoError> {
        let mut ast = std::ptr::null_mut();
        if !unsafe { clingo_ast_attribute_get_ast(self.0.as_ptr(), attribute as i32, &mut ast) } {
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
        attribute: AstAttribute,
        index: usize,
    ) -> Result<Ast, ClingoError> {
        let mut ast = std::ptr::null_mut();
        if !unsafe {
            clingo_ast_attribute_get_ast_at(self.0.as_ptr(), attribute as i32, index, &mut ast)
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
    // fn ast_array(&self, attribute: AstAttribute) -> AstArray {
    //     AstArray {
    //         ast: &self,
    //         attribute,
    //     }
    // }
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
