use crate::*;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StatementType {
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
    Defined = clingo_ast_statement_type_clingo_ast_statement_type_defined as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum Sign {
    None = clingo_ast_sign_clingo_ast_sign_none as isize,
    Negation = clingo_ast_sign_clingo_ast_sign_negation as isize,
    DoubleNegation = clingo_ast_sign_clingo_ast_sign_double_negation as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum LiteralType {
    Boolean = clingo_ast_literal_type_clingo_ast_literal_type_boolean as isize,
    Symbolic = clingo_ast_literal_type_clingo_ast_literal_type_symbolic as isize,
    Comparison = clingo_ast_literal_type_clingo_ast_literal_type_comparison as isize,
    CSP = clingo_ast_literal_type_clingo_ast_literal_type_csp as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum BodyLiteralType {
    Literal = clingo_ast_body_literal_type_clingo_ast_body_literal_type_literal as isize,
    Conditional = clingo_ast_body_literal_type_clingo_ast_body_literal_type_conditional as isize,
    Aggregate = clingo_ast_body_literal_type_clingo_ast_body_literal_type_aggregate as isize,
    BodyAggregate =
        clingo_ast_body_literal_type_clingo_ast_body_literal_type_body_aggregate as isize,
    TheoryAtom = clingo_ast_body_literal_type_clingo_ast_body_literal_type_theory_atom as isize,
    Disjoint = clingo_ast_body_literal_type_clingo_ast_body_literal_type_disjoint as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ComparisonOperator {
    GreaterThan =
        clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than as isize,
    LessThan = clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than as isize,
    LessEqual = clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal as isize,
    GreaterEqual =
        clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal as isize,
    NotEqual = clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal as isize,
    Equal = clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum UnaryOperator {
    Minus = clingo_ast_unary_operator_clingo_ast_unary_operator_minus as isize,
    Negation = clingo_ast_unary_operator_clingo_ast_unary_operator_negation as isize,
    Absolute = clingo_ast_unary_operator_clingo_ast_unary_operator_absolute as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum BinaryOperator {
    Xor = clingo_ast_binary_operator_clingo_ast_binary_operator_xor as isize,
    Or = clingo_ast_binary_operator_clingo_ast_binary_operator_or as isize,
    And = clingo_ast_binary_operator_clingo_ast_binary_operator_and as isize,
    Plus = clingo_ast_binary_operator_clingo_ast_binary_operator_plus as isize,
    Minus = clingo_ast_binary_operator_clingo_ast_binary_operator_minus as isize,
    Multiplication = clingo_ast_binary_operator_clingo_ast_binary_operator_multiplication as isize,
    Division = clingo_ast_binary_operator_clingo_ast_binary_operator_division as isize,
    Modulo = clingo_ast_binary_operator_clingo_ast_binary_operator_modulo as isize,
    Power = clingo_ast_binary_operator_clingo_ast_binary_operator_power as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum TermType {
    Symbol = clingo_ast_term_type_clingo_ast_term_type_symbol as isize,
    Variable = clingo_ast_term_type_clingo_ast_term_type_variable as isize,
    UnaryOperation = clingo_ast_term_type_clingo_ast_term_type_unary_operation as isize,
    BinaryOperation = clingo_ast_term_type_clingo_ast_term_type_binary_operation as isize,
    Interval = clingo_ast_term_type_clingo_ast_term_type_interval as isize,
    Function = clingo_ast_term_type_clingo_ast_term_type_function as isize,
    ExternalFunction = clingo_ast_term_type_clingo_ast_term_type_external_function as isize,
    Pool = clingo_ast_term_type_clingo_ast_term_type_pool as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum AggregateFunction {
    Count = clingo_ast_aggregate_function_clingo_ast_aggregate_function_count as isize,
    Sum = clingo_ast_aggregate_function_clingo_ast_aggregate_function_sum as isize,
    Sump = clingo_ast_aggregate_function_clingo_ast_aggregate_function_sump as isize,
    Min = clingo_ast_aggregate_function_clingo_ast_aggregate_function_min as isize,
    Max = clingo_ast_aggregate_function_clingo_ast_aggregate_function_max as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum TheoryTermType {
    Symbol = clingo_ast_theory_term_type_clingo_ast_theory_term_type_symbol as isize,
    Variable = clingo_ast_theory_term_type_clingo_ast_theory_term_type_variable as isize,
    Tuple = clingo_ast_theory_term_type_clingo_ast_theory_term_type_tuple as isize,
    List = clingo_ast_theory_term_type_clingo_ast_theory_term_type_list as isize,
    Set = clingo_ast_theory_term_type_clingo_ast_theory_term_type_set as isize,
    Function = clingo_ast_theory_term_type_clingo_ast_theory_term_type_function as isize,
    UnparsedTerm = clingo_ast_theory_term_type_clingo_ast_theory_term_type_unparsed_term as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum HeadLiteralType {
    Literal = clingo_ast_head_literal_type_clingo_ast_head_literal_type_literal as isize,
    Disjuction = clingo_ast_head_literal_type_clingo_ast_head_literal_type_disjunction as isize,
    Aggregate = clingo_ast_head_literal_type_clingo_ast_head_literal_type_aggregate as isize,
    HeadAggregate =
        clingo_ast_head_literal_type_clingo_ast_head_literal_type_head_aggregate as isize,
    TheoryAtom = clingo_ast_head_literal_type_clingo_ast_head_literal_type_theory_atom as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum TheoryOperatorType {
    Unary = clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_unary as isize,
    BinaryLeft =
        clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_left as isize,
    BinaryRight =
        clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_right as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum ScriptType {
    Lua = clingo_ast_script_type_clingo_ast_script_type_lua as isize,
    Python = clingo_ast_script_type_clingo_ast_script_type_python as isize,
}

/// Representation of a program statement.
pub struct AstStatement<'a> {
    data: clingo_ast_statement_t,
    _lifetime: PhantomData<&'a ()>,
}
pub fn get_data_ref<'a>(stm: &'a AstStatement) -> &'a clingo_ast_statement_t {
    &stm.data
}
impl<'a> AstStatement<'a> {
    /// Get the location of the statement.
    // pub fn location(&self) -> Location {
    //     Location(self.data.as_ref().location)
    // }

    /// Get the type of the statement.
    pub fn statement_type(&self) -> Result<ast::StatementType, ClingoError> {
        match self.data.type_ as u32 {
            clingo_ast_statement_type_clingo_ast_statement_type_rule => {
                Ok(ast::StatementType::Rule)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_const => {
                Ok(ast::StatementType::Const)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_show_signature => {
                Ok(ast::StatementType::ShowSignature)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_show_term => {
                Ok(ast::StatementType::ShowTerm)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_minimize => {
                Ok(ast::StatementType::Minimize)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_script => {
                Ok(ast::StatementType::Script)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_program => {
                Ok(ast::StatementType::Program)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_external => {
                Ok(ast::StatementType::External)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_edge => {
                Ok(ast::StatementType::Edge)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_heuristic => {
                Ok(ast::StatementType::Heuristic)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_project_atom => {
                Ok(ast::StatementType::ProjectAtom)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_project_atom_signature => {
                Ok(ast::StatementType::ProjectAtomSignature)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_theory_definition => {
                Ok(ast::StatementType::TheoryDefinition)
            }
            x => {
                eprintln!("Failed to match clingo_ast_statement_type: {}.", x);
                Err(ClingoError::new(
                    "Failed to match clingo_ast_statement_type.",
                ))
            }
        }
    }

    /// Get a reference to the rule if the statement is a rule.
    ///
    /// # Errors
    ///
    /// - [`LogicError`](struct.LogicError.html) if the [statement type](#method.statement_type) is not [`Rule`](ast/enum.StatementType.html#variant.Rule)
    /// - [`ClingoError`](struct.ClingoError.html)
    pub fn rule(&'a self) -> Result<&'a ast::Rule, Error> {
        match self.statement_type()? {
            ast::StatementType::Rule => {
                let rule_ptr = unsafe { self.data.__bindgen_anon_1.rule } as *const Rule;
                let rule_ref = unsafe { rule_ptr.as_ref() };
                Ok(rule_ref.unwrap())
            }
            x => {
                eprintln!("Wrong StatementType: {:?}, expected Rule.", x);
                Err(LogicError {
                    msg: "Wrong StatementType, expected Rule.",
                })?
            }
        }
    }

    /// Get a reference to the external if the [statement type](#method.statement_type) is [`External`](ast/enum.StatementType.html#variant.External).
    ///
    /// # Errors
    ///
    /// - [`LogicError`](struct.LogicError.html) if the [statement type](#method.statement_type) is not [`External`](ast/enum.StatementType.html#variant.External)
    /// - [`ClingoError`](struct.ClingoError.html)
    pub fn external(&self) -> Result<&ast::External, Error> {
        match self.statement_type()? {
            ast::StatementType::External => {
                let external = unsafe { self.data.__bindgen_anon_1.external };
                match unsafe { (external as *const ast::External).as_ref() } {
                    Some(reference) => Ok(reference),
                    None => panic!("failed to dereference *const clingo_ast_external_t"),
                }
            }
            x => {
                eprintln!("Wrong StatementType: {:?}, expected External.", x);
                Err(LogicError {
                    msg: "Wrong StatementType, expected External.",
                })?
            }
        }
    }

    /// Get project signature if the [statement type](#method.statement_type) is [`ProjectAtomSignature`](ast/enum.StatementType.html#variant.ProjectAtomSignature).
    ///
    /// # Errors
    ///
    /// - [`LogicError`](struct.LogicError.html) if the [statement type](#method.statement_type) is not [`ProjectAtomSignature`](ast/enum.StatementType.html#variant.ProjectAtomSignature)
    /// - [`ClingoError`](struct.ClingoError.html)
    pub fn project_signature(&self) -> Result<Signature, Error> {
        match self.statement_type()? {
            ast::StatementType::ProjectAtomSignature => {
                let project_signature = unsafe { self.data.__bindgen_anon_1.project_signature };
                Ok(Signature(project_signature))
            }
            x => {
                eprintln!(
                    "Wrong StatementType: {:?}, expected ProjectAtomSignature.",
                    x
                );
                Err(LogicError {
                    msg: "Wrong StatementType, expected ProjectAtomSignature.",
                })?
            }
        }
    }
}
// #[derive(Copy, Clone)]
// pub enum HeadLiteral {
//     Literal(clingo_ast_head_literal_t),
//     Disjunction(clingo_ast_head_literal_t),
//     Aggregate(clingo_ast_head_literal_t),
//     HeadAggregate(clingo_ast_head_literal_t),
//     TheoryAtom(clingo_ast_head_literal_t),
// }
pub struct HeadLiteral<'a> {
    data: clingo_ast_head_literal_t,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for HeadLiteral<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data.type_ as u32 {
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_literal => {
                let literal = unsafe { self.data.__bindgen_anon_1.literal } as *const Literal;
                let literal = unsafe { literal.as_ref() }.unwrap();
                write!(f, "HeadLiteral.sign: ,literal {:?}", literal)
            }
            _ => unimplemented!(),
        }
    }
}
impl<'a> From<&'a Literal<'_>> for HeadLiteral<'a> {
    fn from(lit: &'a Literal) -> HeadLiteral<'a> {
        HeadLiteral {
            data: clingo_ast_head_literal_t {
                location: Location::default(),
                type_: clingo_ast_head_literal_type_clingo_ast_head_literal_type_literal as i32,
                __bindgen_anon_1: clingo_ast_head_literal__bindgen_ty_1 { literal: &lit.data },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> HeadLiteral<'a> {
    pub fn location(&self) -> Location {
        Location(self.data.location)
    }
    pub fn print_lit(&self) -> Option<&clingo_sys::clingo_ast_literal> {
        unsafe { self.data.__bindgen_anon_1.literal.as_ref() }
    }
}
// impl From<Disjunction> for HeadLiteral {
//     fn from(dis: Disjunction) -> Self {
//         let dis = Disjunction::into(dis);
//         HeadLiteral::Literal(clingo_ast_head_literal_t {
//             location: Location::default(),
//             type_: clingo_ast_head_literal_type_clingo_ast_head_literal_type_disjunction as i32,
//             __bindgen_anon_1: clingo_ast_head_literal__bindgen_ty_1 {
//                 disjunction: &dis as *const clingo_ast_disjunction_t,
//             },
//         })
//     }
// }
// impl From<Aggregate> for HeadLiteral {
//     fn from(agg: Aggregate) -> Self {
//         let agg = Aggregate::into(agg);
//         HeadLiteral::Literal(clingo_ast_head_literal_t {
//             location: Location::default(),
//             type_: clingo_ast_head_literal_type_clingo_ast_head_literal_type_aggregate as i32,
//             __bindgen_anon_1: clingo_ast_head_literal__bindgen_ty_1 {
//                 aggregate: &agg as *const clingo_ast_aggregate,
//             },
//         })
//     }
// }
// impl From<HeadAggregate> for HeadLiteral {
//     fn from(agg: HeadAggregate) -> Self {
//         let agg = HeadAggregate::into(agg);
//         HeadLiteral::Literal(clingo_ast_head_literal_t {
//             location: Location::default(),
//             type_: clingo_ast_head_literal_type_clingo_ast_head_literal_type_head_aggregate as i32,
//             __bindgen_anon_1: clingo_ast_head_literal__bindgen_ty_1 {
//                 head_aggregate: &agg as *const clingo_ast_head_aggregate,
//             },
//         })
//     }
// }
// impl From<TheoryAtom> for HeadLiteral {
//     fn from(atom: TheoryAtom) -> Self {
//         let atom = TheoryAtom::into(atom);
//         HeadLiteral::Literal(clingo_ast_head_literal_t {
//             location: Location::default(),
//             type_: clingo_ast_head_literal_type_clingo_ast_head_literal_type_theory_atom as i32,
//             __bindgen_anon_1: clingo_ast_head_literal__bindgen_ty_1 {
//                 theory_atom: &atom as *const clingo_ast_theory_atom,
//             },
//         })
//     }
// }
// #[derive(Copy, Clone)]
pub struct Rule<'a> {
    data: clingo_ast_rule_t,
    _lifetime: PhantomData<&'a ()>,
}
impl<'a> Rule<'a> {
    pub fn new(head: HeadLiteral<'a>, body: &'_ [BodyLiteral]) -> Rule<'a> {
        Rule {
            data: clingo_ast_rule {
                head: head.data,
                body: body.as_ptr() as *const clingo_ast_body_literal_t,
                size: body.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn head(&'a self) -> HeadLiteral<'a> {
        // let head_ptr = &self.data.head as *const HeadLiteral<'a>;
        // head_ptr.as_ref().unwrap()
        HeadLiteral {
            data: self.data.head.clone(),
            _lifetime: PhantomData,
        }
    }
    pub fn body(&self) -> &[BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.size) }
    }
    /// Create a statement for the rule.
    pub fn ast_statement(&'a self) -> Option<AstStatement<'a>> {
        Some(AstStatement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_rule as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    rule: &self.data as *const clingo_ast_rule,
                },
            },
            _lifetime: PhantomData,
        })
    }
}
pub struct Definition(clingo_ast_definition);
impl Definition {
    // pub fn new(name: &str, value: &Term, is_default: bool) -> Result<Definition, NulError> {
    //     let name = CString::new(name)?;
    //     let value = Term::into(*value);
    //     Ok(Definition(clingo_ast_definition {
    //         name: name.as_ptr(),
    //         value,
    //         is_default,
    //     }))
    // }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.0.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.name) };
            c_str.to_str()
        }
    }
    // pub fn value(&self) -> Term {
    //     Term::from(self.0.value)
    // }
    pub fn is_default(&self) -> bool {
        self.0.is_default
    }
}
#[derive(Debug, Copy, Clone)]
pub struct ShowSignature(clingo_ast_show_signature);
impl ShowSignature {
    pub fn signature(&self) -> Signature {
        Signature(self.0.signature)
    }
    pub fn csp(&self) -> bool {
        self.0.csp
    }
}
#[derive(Copy, Clone)]
pub struct ShowTerm(clingo_ast_show_term);
impl ShowTerm {
    // pub fn new(term: &Term, body: &[BodyLiteral], csp: bool) -> ShowTerm {
    //     let term = Term::into(*term);
    //     ShowTerm(clingo_ast_show_term {
    //         term,
    //         body: body.as_ptr() as *const clingo_ast_body_literal_t,
    //         size: body.len(),
    //         csp,
    //     })
    // }
    // pub fn term(&self) -> Term {
    //     Term::from(self.0.term)
    // }
    // pub fn body(&self) -> &[BodyLiteral] {
    //     unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    // }
    pub fn csp(&self) -> bool {
        self.0.csp
    }
}

#[derive(Copy, Clone)]
pub struct Defined(clingo_ast_defined);
impl Defined {
    pub fn new(signature: &Signature) -> Defined {
        let signature = Signature::into(*signature);
        Defined(clingo_ast_defined { signature })
    }
    pub fn signature(self) -> Signature {
        Signature(self.0.signature)
    }
}
#[derive(Copy, Clone)]
pub struct Minimize(clingo_ast_minimize);
impl Minimize {
    // pub fn new(weight: &Term, priority: &Term, tuple: &[Term], body: &[BodyLiteral]) -> Minimize {
    //     let weight = Term::into(*weight);
    //     let priority = Term::into(*priority);
    //     Minimize(clingo_ast_minimize {
    //         weight,
    //         priority,
    //         tuple: tuple.as_ptr() as *const clingo_ast_term_t,
    //         tuple_size: tuple.len(),
    //         body: body.as_ptr() as *const clingo_ast_body_literal_t,
    //         body_size: body.len(),
    //     })
    // }
    // pub fn weight(&self) -> Term {
    //     Term::from(self.0.weight)
    // }
    // pub fn priority(&self) -> Term {
    //     Term::from(self.0.priority)
    // }
    // pub fn tuple(&self) -> &[Term] {
    //     unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    // }
    // pub fn body(&self) -> &[BodyLiteral] {
    //     unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.body_size) }
    // }
}
#[derive(Debug, Copy, Clone)]
pub enum Script {
    Lua(clingo_ast_script),
    Python(clingo_ast_script),
}

impl Script {
    // fn from(script: clingo_ast_script) -> Script {
    //     match script.type_ as u32 {
    //         clingo_ast_script_type_clingo_ast_script_type_lua => {
    //             Script::Lua(script)
    //         }
    //         clingo_ast_script_type_clingo_ast_script_type_python => {
    //             Script::Python(script)
    //         }
    //         x => panic!("Failed to match clclingo_ast_script_type : {}.", x),
    //     }
    // }
    fn into(self) -> clingo_ast_script {
        match self {
            Script::Lua(script) => script,
            Script::Python(script) => script,
        }
    }
    pub fn code(&self) -> Result<&str, Utf8Error> {
        let script = Script::into(*self);
        if script.code.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(script.code) };
            c_str.to_str()
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Program(clingo_ast_program);
impl Program {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.0.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.name) };
            c_str.to_str()
        }
    }
    pub fn parameters(&self) -> &[Id] {
        unsafe { std::slice::from_raw_parts(self.0.parameters as *const Id, self.0.size) }
    }
}
#[derive(Copy, Clone)]
pub struct BodyLiteral<'a> {
    data: clingo_ast_body_literal_t,
    _lifetime: PhantomData<&'a ()>,
}
impl<'a> BodyLiteral<'a> {
    pub fn from_literal(sign: Sign, lit: &'a Literal) -> BodyLiteral<'a> {
        BodyLiteral {
            data: clingo_ast_body_literal_t {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_literal as i32,
                __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 { literal: &lit.data },
            },
            _lifetime: PhantomData,
        }
    }
    // pub fn from_conditional(sign: Sign, con: &ConditionalLiteral) -> BodyLiteral {
    //     let con = ConditionalLiteral::into(*con);
    //     BodyLiteral::Conditional(clingo_ast_body_literal_t {
    //         location: Location::default(),
    //         sign: sign as clingo_ast_sign_t,
    //         type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_conditional as i32,
    //         __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
    //             conditional: &con as *const clingo_ast_conditional_literal,
    //         },
    //     })
    // }
    // pub fn from_aggregate(sign: Sign, agg: &Aggregate) -> BodyLiteral {
    //     let agg = Aggregate::into(*agg);
    //     BodyLiteral::Aggregate(clingo_ast_body_literal_t {
    //         location: Location::default(),
    //         sign: sign as clingo_ast_sign_t,
    //         type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_aggregate as i32,
    //         __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
    //             aggregate: &agg as *const clingo_ast_aggregate,
    //         },
    //     })
    // }
    // pub fn from_body_aggregate(sign: Sign, agg: &BodyAggregate) -> BodyLiteral {
    //     let agg = BodyAggregate::into(*agg);
    //     BodyLiteral::BodyAggregate(clingo_ast_body_literal_t {
    //         location: Location::default(),
    //         sign: sign as clingo_ast_sign_t,
    //         type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_body_aggregate as i32,
    //         __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
    //             body_aggregate: &agg as *const clingo_ast_body_aggregate,
    //         },
    //     })
    // }
    // pub fn from_theory_atom(sign: Sign, atom: &TheoryAtom) -> BodyLiteral {
    //     let atom = TheoryAtom::into(*atom);
    //     BodyLiteral::TheoryAtom(clingo_ast_body_literal_t {
    //         location: Location::default(),
    //         sign: sign as clingo_ast_sign_t,
    //         type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_theory_atom as i32,
    //         __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
    //             theory_atom: &atom as *const clingo_ast_theory_atom,
    //         },
    //     })
    // }
    // pub fn from_disjoint(sign: Sign, dis: &Disjoint) -> BodyLiteral {
    //     let dis = Disjoint::into(*dis);
    //     BodyLiteral::Disjoint(clingo_ast_body_literal_t {
    //         location: Location::default(),
    //         sign: sign as clingo_ast_sign_t,
    //         type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_disjoint as i32,
    //         __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
    //             disjoint: &dis as *const clingo_ast_disjoint,
    //         },
    //     })
    // }
    // pub fn location(&self) -> Location {
    //     let lit = BodyLiteral::into(*self);
    //     Location(lit.location)
    // }
    // pub fn sign(&self) -> Sign {
    //     let lit = BodyLiteral::into(*self);

    //     match lit.sign as u32 {
    //         clingo_ast_sign_clingo_ast_sign_double_negation => Sign::DoubleNegation,
    //         clingo_ast_sign_clingo_ast_sign_negation => Sign::Negation,
    //         clingo_ast_sign_clingo_ast_sign_none => Sign::None,
    //         x => panic!("Failed to match clingo_ast_sign: {}.", x),
    //     }
    // }
}
#[derive(Copy, Clone)]
pub struct External<'a> {
    data: clingo_ast_external_t,
    _lifetime: PhantomData<&'a u32>,
}
impl<'a> External<'a> {
    /// Create an external atom default initialization with false
    pub fn new(term: Term, body: &'a [BodyLiteral]) -> External<'a> {
        let mut symbol = 0 as clingo_symbol_t;
        let name = CString::new("false").unwrap();
        if !unsafe { clingo_symbol_create_id(name.as_ptr(), true, &mut symbol) } {
            panic!("Failed to create false symbol");
        }
        let ext = clingo_ast_external {
            atom: term.data,
            body: body.as_ptr() as *const clingo_ast_body_literal_t,
            size: body.len(),
            type_: clingo_sys::clingo_ast_term {
                location: Location::default(),
                type_: clingo_ast_term_type_clingo_ast_term_type_symbol as i32,
                __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 { symbol },
            },
        };
        External {
            data: ext,
            _lifetime: PhantomData,
        }
    }
    // /// Create an external atom initialization with the flag term
    // pub fn new_with_flag(term: &Term, body: &[BodyLiteral], flag: &Term) -> External {
    //     let term = Term::into(*term);
    //     let flag = Term::into(*flag);
    //     let ext = clingo_ast_external {
    //         atom: term,
    //         body: body.as_ptr() as *const clingo_ast_body_literal_t,
    //         size: body.len(),
    //         type_: flag,
    //     };
    //     External(ext)
    // }
    // pub fn term(&self) -> Term {
    //     Term::from(self.0.atom)
    // }
    pub fn body(&self) -> &[BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.size) }
    }
    /// Create a statement for the external.
    pub fn ast_statement(&'a self) -> Option<AstStatement<'a>> {
        Some(AstStatement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: StatementType::External as clingo_ast_statement_type_t,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    external: &self.data,
                },
            },
            _lifetime: PhantomData,
        })
    }
}
#[derive(Copy, Clone)]
pub struct Edge(clingo_ast_edge);
impl Edge {
    // Create an edge
    // pub fn new(u: &Term, v: &Term, body: &[BodyLiteral]) -> Edge {
    //     let u = Term::into(*u);
    //     let v = Term::into(*v);
    //     Edge(clingo_ast_edge {
    //         u,
    //         v,
    //         body: body.as_ptr() as *const clingo_ast_body_literal_t,
    //         size: body.len(),
    //     })
    // }
    // pub fn u(&self) -> Term {
    //     Term::from(self.0.u)
    // }
    // pub fn v(&self) -> Term {
    //     Term::from(self.0.v)
    // }
    // pub fn body(&self) -> &[BodyLiteral] {
    //     unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    // }
}
#[derive(Copy, Clone)]
pub struct Heuristic(clingo_ast_heuristic);
impl Heuristic {
    // Create an heuristic
    // pub fn new(
    //     atom: &Term,
    //     body: &[BodyLiteral],
    //     bias: &Term,
    //     priority: &Term,
    //     modifier: &Term,
    // ) -> Heuristic {
    //     let atom = Term::into(*atom);
    //     let bias = Term::into(*bias);
    //     let priority = Term::into(*priority);
    //     let modifier = Term::into(*modifier);
    //     Heuristic(clingo_ast_heuristic {
    //         atom,
    //         body: body.as_ptr() as *const clingo_ast_body_literal_t,
    //         size: body.len(),
    //         bias,
    //         priority,
    //         modifier,
    //     })
    // }
    // pub fn atom(&self) -> Term {
    //     Term::from(self.0.atom)
    // }
    // pub fn body(&self) -> &[BodyLiteral] {
    //     unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    // }
    // pub fn bias(&self) -> Term {
    //     Term::from(self.0.bias)
    // }
    // pub fn priority(&self) -> Term {
    //     Term::from(self.0.priority)
    // }
    // pub fn modifier(&self) -> Term {
    //     Term::from(self.0.modifier)
    // }
}
#[derive(Copy, Clone)]
pub struct Project(clingo_ast_project);
impl Project {
    // Create a project
    // pub fn new(atom: &Term, body: &[BodyLiteral]) -> Project {
    //     let atom = Term::into(*atom);
    //     Project(clingo_ast_project {
    //         atom,
    //         body: body.as_ptr() as *const clingo_ast_body_literal_t,
    //         size: body.len(),
    //     })
    // }
    // pub fn atom(&self) -> Term {
    //     Term::from(self.0.atom)
    // }
    // pub fn body(&self) -> &[BodyLiteral] {
    //     unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    // }
}
// pub enum Term {
//     Symbol(clingo_ast_term_t),
//     Variable(clingo_ast_term_t),
//     UnaryOperation(clingo_ast_term_t),
//     BinaryOperation(clingo_ast_term_t),
//     Interval(clingo_ast_term_t),
//     Function(clingo_ast_term_t),
//     ExternalFunction(clingo_ast_term_t),
//     Pool(clingo_ast_term_t),
// }
pub struct Term<'a> {
    data: clingo_ast_term_t,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data.type_ as u32 {
            clingo_ast_term_type_clingo_ast_term_type_symbol => {
                write!(f, "Term.symbol: {}", unsafe {
                    self.data.__bindgen_anon_1.symbol
                })
            }
            _ => unimplemented!(),
        }
    }
}
impl<'a> Term<'a> {
    // fn from(term: clingo_ast_term_t) -> Term {
    //     match term.type_ as u32 {
    //         clingo_ast_term_type_clingo_ast_term_type_symbol => Term::Symbol(term),
    //         clingo_ast_term_type_clingo_ast_term_type_variable => Term::Variable(term),
    //         clingo_ast_term_type_clingo_ast_term_type_unary_operation => Term::UnaryOperation(term),
    //         clingo_ast_term_type_clingo_ast_term_type_binary_operation => {
    //             Term::BinaryOperation(term)
    //         }
    //         clingo_ast_term_type_clingo_ast_term_type_interval => Term::Interval(term),
    //         clingo_ast_term_type_clingo_ast_term_type_function => Term::Function(term),
    //         clingo_ast_term_type_clingo_ast_term_type_external_function => {
    //             Term::ExternalFunction(term)
    //         }
    //         clingo_ast_term_type_clingo_ast_term_type_pool => Term::Pool(term),
    //         x => panic!("Failed to match clingo_ast_term_type: {}.", x),
    //     }
    // }
    // pub fn location(&self) -> Location {
    //     let term = Term::into(*self);
    //     Location(term.location)
    // }
    // /// Create a variable term
    // ///
    // /// # Errors
    // ///
    // /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `string` contains a nul byte
    // pub fn new_variable(string: &str) -> Result<Term, NulError> {
    //     let cstr = CString::new(string)?;
    //     let _bg_union_1 = clingo_ast_term__bindgen_ty_1 {
    //         variable: cstr.as_ptr(),
    //     };
    //     let term = clingo_ast_term_t {
    //         location: Location::default(),
    //         type_: clingo_ast_term_type_clingo_ast_term_type_variable as i32,
    //         __bindgen_anon_1: _bg_union_1,
    //     };
    //     Ok(Term::Variable(term))
    // }
    // pub fn external_function(function: Function) -> Self {
    //     let function = Function::into(function);
    //     let term = clingo_ast_term_t {
    //         location: Location::default(),
    //         type_: clingo_ast_term_type_clingo_ast_term_type_symbol as i32,
    //         __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 {
    //             external_function: &function,
    //         },
    //     };
    //     Term::ExternalFunction(term)
    // }
}
impl<'a> From<Symbol> for Term<'a> {
    fn from(Symbol(symbol): Symbol) -> Term<'a> {
        Term {
            data: clingo_ast_term {
                location: Location::default(),
                type_: clingo_ast_term_type_clingo_ast_term_type_symbol as i32,
                __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 { symbol },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a UnaryOperation<'_>> for Term<'a> {
    fn from(op: &'a UnaryOperation) -> Self {
        Term {
            data: clingo_ast_term_t {
                location: Location::default(),
                type_: clingo_ast_term_type_clingo_ast_term_type_unary_operation as i32,
                __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 {
                    unary_operation: &op.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a BinaryOperation<'_>> for Term<'a> {
    fn from(op: &'a BinaryOperation) -> Self {
        Term {
            data: clingo_ast_term_t {
                location: Location::default(),
                type_: clingo_ast_term_type_clingo_ast_term_type_binary_operation as i32,
                __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 {
                    binary_operation: &op.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a Interval<'_>> for Term<'a> {
    fn from(interval: &'a Interval) -> Self {
        Term {
            data: clingo_ast_term_t {
                location: Location::default(),
                type_: clingo_ast_term_type_clingo_ast_term_type_interval as i32,
                __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 {
                    interval: &interval.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a Function<'_>> for Term<'a> {
    fn from(fun: &'a Function) -> Self {
        Term {
            data: clingo_ast_term_t {
                location: Location::default(),
                type_: clingo_ast_term_type_clingo_ast_term_type_function as i32,
                __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 {
                    function: &fun.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a Pool<'_>> for Term<'a> {
    fn from(pool: &'a Pool) -> Self {
        Term {
            data: clingo_ast_term_t {
                location: Location::default(),
                type_: clingo_ast_term_type_clingo_ast_term_type_pool as i32,
                __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 { pool: &pool.data },
            },
            _lifetime: PhantomData,
        }
    }
}
pub struct Literal<'a> {
    data: clingo_ast_literal_t,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data.type_ as u32 {
            clingo_ast_literal_type_clingo_ast_literal_type_symbolic => {
                let term = unsafe { self.data.__bindgen_anon_1.symbol } as *const Term;
                let term = unsafe { term.as_ref() }.unwrap();
                write!(f, "Literal.symbol: {:?}", term)
            }
            _ => unimplemented!(),
        }
    }
}
impl<'a> Literal<'a> {
    // fn into(self) -> clingo_ast_literal_t {
    //     match self {
    //         Literal::Boolean(lit) => lit,
    //         Literal::Symbolic(lit) => lit,
    //         Literal::Comparison(lit) => lit,
    //         Literal::CSP(lit) => lit,
    //     }
    // }
    // fn from(lit: clingo_ast_literal_t) -> Literal {
    //     match lit.type_ as u32 {
    //         clingo_ast_literal_type_clingo_ast_literal_type_boolean => Literal::Boolean(lit),
    //         clingo_ast_literal_type_clingo_ast_literal_type_symbolic => Literal::Symbolic(lit),
    //         clingo_ast_literal_type_clingo_ast_literal_type_comparison => Literal::Comparison(lit),
    //         clingo_ast_literal_type_clingo_ast_literal_type_csp => Literal::CSP(lit),
    //         x => panic!("Failed to match clingo_ast_head_literal_t: {}.", x),
    //     }
    // }
    // pub fn from_bool(sign: Sign, boolean: bool) -> Literal {
    //     Literal::Boolean(clingo_ast_literal_t {
    //         location: Location::default(),
    //         type_: clingo_ast_literal_type_clingo_ast_literal_type_boolean as i32,
    //         sign: sign as clingo_ast_sign_t,
    //         __bindgen_anon_1: clingo_ast_literal__bindgen_ty_1 { boolean },
    //     })
    // }

    /// Create a literal from a term and sign.
    pub fn from_term(sign: Sign, term: &'a Term) -> Literal<'a> {
        Literal {
            data: clingo_ast_literal {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_literal_type_clingo_ast_literal_type_symbolic as i32,
                __bindgen_anon_1: clingo_ast_literal__bindgen_ty_1 { symbol: &term.data },
            },
            _lifetime: PhantomData,
        }
    }
    // pub fn from_comparison(sign: Sign, comp: &Comparison) -> Literal {
    //     let comp = Comparison::into(*comp);
    //     Literal::Symbolic(clingo_ast_literal_t {
    //         location: Location::default(),
    //         type_: clingo_ast_literal_type_clingo_ast_literal_type_comparison as i32,
    //         sign: sign as clingo_ast_sign_t,
    //         __bindgen_anon_1: clingo_ast_literal__bindgen_ty_1 {
    //             comparison: &comp as *const clingo_sys::clingo_ast_comparison,
    //         },
    //     })
    // }
    // pub fn from_csp_literal(sign: Sign, lit: &CspLiteral) -> Literal {
    //     let lit = CspLiteral::into(*lit);
    //     Literal::Symbolic(clingo_ast_literal_t {
    //         location: Location::default(),
    //         type_: clingo_ast_literal_type_clingo_ast_literal_type_csp as i32,
    //         sign: sign as clingo_ast_sign_t,
    //         __bindgen_anon_1: clingo_ast_literal__bindgen_ty_1 {
    //             csp_literal: &lit as *const clingo_sys::clingo_ast_csp_literal,
    //         },
    //     })
    // }
    // pub fn location(&self) -> Location {
    //     let lit = Literal::into(*self);
    //     Location(lit.location)
    // }
    // pub fn sign(&self) -> Sign {
    //     let lit = Literal::into(*self);

    //     match lit.sign as u32 {
    //         clingo_ast_sign_clingo_ast_sign_double_negation => Sign::DoubleNegation,
    //         clingo_ast_sign_clingo_ast_sign_negation => Sign::Negation,
    //         clingo_ast_sign_clingo_ast_sign_none => Sign::None,
    //         x => panic!("Failed to match clingo_ast_sign: {}.", x),
    //     }
    // }
}
pub struct UnaryOperation<'a> {
    data: clingo_ast_unary_operation_t,
    _lifetime: PhantomData<&'a ()>,
}
impl<'a> UnaryOperation<'a> {
    pub fn minus(term: Term<'a>) -> UnaryOperation<'a> {
        UnaryOperation {
            data: clingo_ast_unary_operation {
                unary_operator: clingo_ast_unary_operator_clingo_ast_unary_operator_minus as i32,
                argument: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn negation(term: Term<'a>) -> UnaryOperation<'a> {
        UnaryOperation {
            data: clingo_ast_unary_operation {
                unary_operator: clingo_ast_unary_operator_clingo_ast_unary_operator_negation as i32,
                argument: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn absolute(term: Term<'a>) -> UnaryOperation<'a> {
        UnaryOperation {
            data: clingo_ast_unary_operation {
                unary_operator: clingo_ast_unary_operator_clingo_ast_unary_operator_absolute as i32,
                argument: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn argument(self) -> Term<'a> {
        Term {
            data: self.data.argument,
            _lifetime: PhantomData,
        }
    }
}
pub struct BinaryOperation<'a> {
    data: clingo_ast_binary_operation_t,
    _lifetime: PhantomData<&'a ()>,
}
impl<'l, 'r, 'a: 'l + 'r> BinaryOperation<'a> {
    pub fn xor(left: Term<'l>, right: Term<'r>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_xor as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn or(left: Term<'l>, right: Term<'r>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_or as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn and(left: Term<'l>, right: Term<'r>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_and as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn plus(left: Term<'l>, right: Term<'r>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_plus as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn minus(left: Term<'l>, right: Term<'r>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_minus as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn multiplication(left: Term<'l>, right: Term<'r>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator:
                    clingo_ast_binary_operator_clingo_ast_binary_operator_multiplication as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn division(left: Term<'l>, right: Term<'r>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_division
                    as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn modulo(left: Term<'l>, right: Term<'r>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_modulo
                    as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn power(left: Term<'l>, right: Term<'r>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_power as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    // pub fn left(&self) -> Term {
    //     let op = BinaryOperation::into(*self);
    //     Term::from(op.left)
    // }
    // pub fn right(&self) -> Term {
    //     let op = BinaryOperation::into(*self);
    //     Term::from(op.right)
    // }
}
#[derive(Copy, Clone)]
pub struct Interval<'a> {
    data: clingo_ast_interval,
    _lifetime: PhantomData<&'a ()>,
}
impl<'l, 'r, 'a: 'r + 'l> Interval<'a> {
    pub fn new(left: Term<'l>, right: Term<'r>) -> Interval<'a> {
        Interval {
            data: clingo_ast_interval {
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn left(self: Interval<'a>) -> Term<'a> {
        Term {
            data: self.data.left,
            _lifetime: PhantomData,
        }
    }
    pub fn right(self: Interval<'a>) -> Term<'a> {
        Term {
            data: self.data.right,
            _lifetime: PhantomData,
        }
    }
}
pub struct Function<'a> {
    data: clingo_ast_function,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Function<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().expect("Cant get function name!");

        let arguments = unsafe {
            std::slice::from_raw_parts(self.data.arguments as *const Term, self.data.size)
        };
        let mut comma_separated = String::new();

        for num in &arguments[0..arguments.len() - 1] {
            comma_separated.push_str(&format!("{:?}", num));
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&format!("{:?}", &arguments[arguments.len() - 1]));
        write!(f, "Function.name: {} args: {}", name, comma_separated)
    }
}
impl<'b, 'c, 'a: 'b + 'c> Function<'a> {
    pub fn new(name: &'b str, arguments: &'c mut [Term]) -> Result<Function<'a>, NulError> {
        let name = CString::new(name)?;
        Ok(Function {
            data: clingo_ast_function {
                name: name.into_raw(),
                arguments: arguments.as_mut_ptr() as *mut clingo_ast_term_t,
                size: arguments.len(),
            },
            _lifetime: PhantomData,
        })
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.data.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.data.name) };
            c_str.to_str()
        }
    }
    pub fn arguments(&'a self) -> &'a [Term] {
        unsafe { std::slice::from_raw_parts(self.data.arguments as *const Term, self.data.size) }
    }
}
pub struct Pool<'a> {
    data: clingo_ast_pool,
    _lifetime: PhantomData<&'a ()>,
}
impl<'a> Pool<'a> {
    // pub fn new(arguments: &mut [Term]) -> Pool {
    //     Pool(clingo_ast_pool {
    //         arguments: arguments.as_mut_ptr() as *mut clingo_ast_term_t,
    //         size: arguments.len(),
    //     })
    // }
    // pub fn arguments(&self) -> &[Term] {
    //     unsafe { std::slice::from_raw_parts(self.0.arguments as *const Term, self.0.size) }
    // }
}
pub struct CspProductTerm<'a> {
    data: clingo_ast_csp_product_term,
    _lifetime: PhantomData<&'a ()>,
}
impl<'a> CspProductTerm<'a> {
    // pub fn new(coefficient: &Term, variable: &[Term]) -> CspProductTerm {
    //     let coefficient = Term::into(*coefficient);
    //     CspProductTerm(clingo_ast_csp_product_term {
    //         location: Location::default(),
    //         coefficient: coefficient,
    //         variable: variable.as_ptr() as *const clingo_ast_term_t,
    //     })
    // }
    pub fn location(&self) -> Location {
        Location(self.data.location)
    }
    // pub fn coefficient(&self) -> Term {
    //     Term::from(self.0.coefficient)
    // }
    // pub fn variable(&self) -> Result<&Term, ClingoError> {
    //     match unsafe { (self.0.variable as *const Term).as_ref() } {
    //         Some(x) => Ok(x),
    //         None => Err(ClingoError::new("tried casting a null pointer to &Term.")),
    //     }
    // }
}
#[derive(Debug, Copy, Clone)]
pub struct CspSumTerm(clingo_ast_csp_sum_term);
impl CspSumTerm {
    fn into(self) -> clingo_ast_csp_sum_term {
        let CspSumTerm(term) = self;
        term
    }
    pub fn new(terms: &[CspProductTerm]) -> CspSumTerm {
        CspSumTerm(clingo_ast_csp_sum_term {
            location: Location::default(),
            terms: terms.as_ptr() as *const clingo_ast_csp_product_term_t,
            size: terms.len(),
        })
    }
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn terms(&self) -> &[CspProductTerm] {
        unsafe { std::slice::from_raw_parts(self.0.terms as *const CspProductTerm, self.0.size) }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CspGuard {
    GreaterThan(clingo_ast_csp_guard),
    LessThan(clingo_ast_csp_guard),
    LessEqual(clingo_ast_csp_guard),
    GreaterEqual(clingo_ast_csp_guard),
    NotEqual(clingo_ast_csp_guard),
    Equal(clingo_ast_csp_guard),
}
impl CspGuard {
    fn into(self) -> clingo_ast_csp_guard {
        match self {
            CspGuard::GreaterThan(guard) => guard,
            CspGuard::LessThan(guard) => guard,
            CspGuard::LessEqual(guard) => guard,
            CspGuard::GreaterEqual(guard) => guard,
            CspGuard::NotEqual(guard) => guard,
            CspGuard::Equal(guard) => guard,
        }
    }
    pub fn gt(term: &CspSumTerm) -> CspGuard {
        let term = CspSumTerm::into(*term);
        CspGuard::GreaterThan(clingo_ast_csp_guard {
            comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than
                as i32,
            term,
        })
    }
    pub fn lt(term: &CspSumTerm) -> CspGuard {
        let term = CspSumTerm::into(*term);
        CspGuard::LessThan(clingo_ast_csp_guard {
            comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than
                as i32,
            term,
        })
    }
    pub fn le(term: &CspSumTerm) -> CspGuard {
        let term = CspSumTerm::into(*term);
        CspGuard::LessEqual(clingo_ast_csp_guard {
            comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal
                as i32,
            term,
        })
    }
    pub fn ge(term: &CspSumTerm) -> CspGuard {
        let term = CspSumTerm::into(*term);
        CspGuard::GreaterEqual(clingo_ast_csp_guard {
            comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal
                as i32,
            term,
        })
    }
    pub fn ne(term: &CspSumTerm) -> CspGuard {
        let term = CspSumTerm::into(*term);
        CspGuard::NotEqual(clingo_ast_csp_guard {
            comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal
                as i32,
            term,
        })
    }
    pub fn eq(term: &CspSumTerm) -> CspGuard {
        let term = CspSumTerm::into(*term);
        CspGuard::Equal(clingo_ast_csp_guard {
            comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal as i32,
            term,
        })
    }
    pub fn term(&self) -> CspSumTerm {
        let guard = CspGuard::into(*self);
        CspSumTerm(guard.term)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct CspLiteral(clingo_ast_csp_literal);
impl CspLiteral {
    pub fn new(term: &CspSumTerm, guards: &[CspGuard]) -> CspLiteral {
        let term = CspSumTerm::into(*term);
        CspLiteral(clingo_ast_csp_literal {
            term,
            guards: guards.as_ptr() as *const clingo_ast_csp_guard_t,
            size: guards.len(),
        })
    }
    pub fn term(&self) -> CspSumTerm {
        CspSumTerm(self.0.term)
    }
    pub fn guards(&self) -> &[CspGuard] {
        unsafe { std::slice::from_raw_parts(self.0.guards as *const CspGuard, self.0.size) }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Id(clingo_ast_id);
impl Id {
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn id(&self) -> Result<&str, Utf8Error> {
        if self.0.id.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.id) };
            c_str.to_str()
        }
    }
}
pub enum Comparison {
    GreaterThan(clingo_ast_comparison),
    LessThan(clingo_ast_comparison),
    LessEqual(clingo_ast_comparison),
    GreaterEqual(clingo_ast_comparison),
    NotEqual(clingo_ast_comparison),
    Equal(clingo_ast_comparison),
}
impl Comparison {
    // pub fn gt(left: &Term, right: &Term) -> Comparison {
    //     let left = Term::into(*left);
    //     let right = Term::into(*right);
    //     Comparison::GreaterThan(clingo_ast_comparison {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than
    //             as i32,
    //         left,
    //         right,
    //     })
    // }
    // pub fn lt(left: &Term, right: &Term) -> Comparison {
    //     let left = Term::into(*left);
    //     let right = Term::into(*right);
    //     Comparison::LessThan(clingo_ast_comparison {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than
    //             as i32,
    //         left,
    //         right,
    //     })
    // }
    // pub fn le(left: &Term, right: &Term) -> Comparison {
    //     let left = Term::into(*left);
    //     let right = Term::into(*right);
    //     Comparison::LessEqual(clingo_ast_comparison {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal
    //             as i32,
    //         left,
    //         right,
    //     })
    // }
    // pub fn ge(left: &Term, right: &Term) -> Comparison {
    //     let left = Term::into(*left);
    //     let right = Term::into(*right);
    //     Comparison::GreaterEqual(clingo_ast_comparison {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal
    //             as i32,
    //         left,
    //         right,
    //     })
    // }
    // pub fn ne(left: &Term, right: &Term) -> Comparison {
    //     let left = Term::into(*left);
    //     let right = Term::into(*right);
    //     Comparison::NotEqual(clingo_ast_comparison {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal
    //             as i32,
    //         left,
    //         right,
    //     })
    // }
    // pub fn eq(left: &Term, right: &Term) -> Comparison {
    //     let left = Term::into(*left);
    //     let right = Term::into(*right);
    //     Comparison::Equal(clingo_ast_comparison {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal as i32,
    //         left,
    //         right,
    //     })
    // }
    // pub fn left(&self) -> Term {
    //     let comp = Comparison::into(*self);
    //     Term::from(comp.left)
    // }
    // pub fn right(&self) -> Term {
    //     let comp = Comparison::into(*self);
    //     Term::from(comp.left)
    // }
}
#[derive(Copy, Clone)]
pub struct AggregateGuard<'a> {
    data: clingo_ast_aggregate_guard,
    _lifetime: PhantomData<&'a ()>,
}
impl AggregateGuard<'_> {
    // fn into(self) -> clingo_ast_aggregate_guard {
    //     match self {
    //         AggregateGuard::GreaterThan(guard) => guard,
    //         AggregateGuard::LessThan(guard) => guard,
    //         AggregateGuard::LessEqual(guard) => guard,
    //         AggregateGuard::GreaterEqual(guard) => guard,
    //         AggregateGuard::NotEqual(guard) => guard,
    //         AggregateGuard::Equal(guard) => guard,
    //     }
    // }
    // pub fn gt(term: &Term) -> AggregateGuard {
    //     let term = Term::into(*term);
    //     AggregateGuard::GreaterThan(clingo_ast_aggregate_guard {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than
    //             as i32,
    //         term,
    //     })
    // }
    // pub fn lt(term: &Term) -> AggregateGuard {
    //     let term = Term::into(*term);
    //     AggregateGuard::LessThan(clingo_ast_aggregate_guard {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than
    //             as i32,
    //         term,
    //     })
    // }
    // pub fn le(term: &Term) -> AggregateGuard {
    //     let term = Term::into(*term);
    //     AggregateGuard::LessEqual(clingo_ast_aggregate_guard {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal
    //             as i32,
    //         term,
    //     })
    // }
    // pub fn ge(term: &Term) -> AggregateGuard {
    //     let term = Term::into(*term);
    //     AggregateGuard::GreaterEqual(clingo_ast_aggregate_guard {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal
    //             as i32,
    //         term,
    //     })
    // }
    // pub fn ne(term: &Term) -> AggregateGuard {
    //     let term = Term::into(*term);
    //     AggregateGuard::NotEqual(clingo_ast_aggregate_guard {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal
    //             as i32,
    //         term,
    //     })
    // }
    // pub fn eq(term: &Term) -> AggregateGuard {
    //     let term = Term::into(*term);
    //     AggregateGuard::Equal(clingo_ast_aggregate_guard {
    //         comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal as i32,
    //         term,
    //     })
    // }
    // pub fn term(&self) -> Term {
    //     let guard = AggregateGuard::into(*self);
    //     Term::from(guard.term)
    // }
}
// #[derive(Copy, Clone)]
pub struct ConditionalLiteral(clingo_ast_conditional_literal);
impl ConditionalLiteral {
    // pub fn new(literal: &Literal, condition: &[Literal]) -> ConditionalLiteral {
    //     let literal = Literal::into(*literal);
    //     ConditionalLiteral(clingo_ast_conditional_literal {
    //         literal: literal,
    //         condition: condition.as_ptr() as *const clingo_ast_literal_t,
    //         size: condition.len(),
    //     })
    // }
    // pub fn literal(&self) -> Literal {
    //     Literal::from(self.0.literal)
    // }
    // pub fn condition(&self) -> &[Literal] {
    //     unsafe { std::slice::from_raw_parts(self.0.condition as *const Literal, self.0.size) }
    // }
}
#[derive(Debug)]
pub struct Aggregate(clingo_ast_aggregate);
impl Aggregate {
    pub fn new(
        elements: &[ConditionalLiteral],
        left_guard: &AggregateGuard,
        right_guard: &AggregateGuard,
    ) -> Aggregate {
        Aggregate(clingo_ast_aggregate {
            elements: elements.as_ptr() as *const clingo_ast_conditional_literal_t,
            size: elements.len(),
            left_guard: &left_guard.data,
            right_guard: &right_guard.data,
        })
    }
    pub fn elements(&self) -> &[ConditionalLiteral] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const ConditionalLiteral, self.0.size)
        }
    }
    pub fn left_guard(&self) -> Result<&AggregateGuard, ClingoError> {
        match unsafe { (self.0.left_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::new(
                "tried casting a null pointer to &AggregateGuard.",
            )),
        }
    }
    pub fn right_guard(&self) -> Result<&AggregateGuard, ClingoError> {
        match unsafe { (self.0.right_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::new(
                "tried casting a null pointer to &AggregateGuard.",
            )),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BodyAggregateElement(clingo_ast_body_aggregate_element);
impl BodyAggregateElement {
    // pub fn new(tuple: &[Term], condition: &[Literal]) -> BodyAggregateElement {
    //     BodyAggregateElement(clingo_ast_body_aggregate_element {
    //         tuple: tuple.as_ptr() as *mut clingo_ast_term_t,
    //         tuple_size: tuple.len(),
    //         condition: condition.as_ptr() as *const clingo_ast_literal_t,
    //         condition_size: condition.len(),
    //     })
    // }
    // pub fn tuple(&self) -> &[Term] {
    //     unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    // }
    // pub fn condition(&self) -> &[Literal] {
    //     unsafe {
    //         std::slice::from_raw_parts(self.0.condition as *const Literal, self.0.condition_size)
    //     }
    // }
}
#[derive(Debug, Copy, Clone)]
pub struct BodyAggregate(clingo_ast_body_aggregate);
impl BodyAggregate {
    pub fn new(
        function: AggregateFunction,
        elements: &[BodyAggregateElement],
        left_guard: &AggregateGuard,
        right_guard: &AggregateGuard,
    ) -> BodyAggregate {
        BodyAggregate(clingo_ast_body_aggregate {
            function: function as i32,
            elements: elements.as_ptr() as *const clingo_ast_body_aggregate_element_t,
            size: elements.len(),
            left_guard: &left_guard.data,
            right_guard: &right_guard.data,
        })
    }
    pub fn function(&self) -> AggregateFunction {
        match self.0.function as u32 {
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_count => {
                AggregateFunction::Count
            }
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_sum => {
                AggregateFunction::Sum
            }
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_sump => {
                AggregateFunction::Sump
            }
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_min => {
                AggregateFunction::Min
            }
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_max => {
                AggregateFunction::Max
            }
            x => panic!("Failed to match clingo_ast_theory_term_type: {}.", x),
        }
    }
    pub fn elements(&self) -> &[BodyAggregateElement] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const BodyAggregateElement, self.0.size)
        }
    }
    pub fn left_guard(&self) -> Result<&AggregateGuard, ClingoError> {
        match unsafe { (self.0.left_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::new(
                "tried casting a null pointer to &AggregateGuard.",
            )),
        }
    }
    pub fn right_guard(&self) -> Result<&AggregateGuard, ClingoError> {
        match unsafe { (self.0.right_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::new(
                "tried casting a null pointer to &AggregateGuard.",
            )),
        }
    }
}
#[derive(Copy, Clone)]
pub struct HeadAggregateElement(clingo_ast_head_aggregate_element);
impl HeadAggregateElement {
    // pub fn new(tuple: &[Term], conditional_literal: &ConditionalLiteral) -> HeadAggregateElement {
    //     let conditional_literal = ConditionalLiteral::into(*conditional_literal);
    //     HeadAggregateElement(clingo_ast_head_aggregate_element {
    //         tuple: tuple.as_ptr() as *const clingo_ast_term_t,
    //         tuple_size: tuple.len(),
    //         conditional_literal,
    //     })
    // }
    // pub fn tuple(&self) -> &[Term] {
    //     unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    // }
    // pub fn conditional_literal(&self) -> ConditionalLiteral {
    //     ConditionalLiteral(self.0.conditional_literal)
    // }
}
#[derive(Debug, Copy, Clone)]
pub struct HeadAggregate(clingo_ast_head_aggregate);
impl HeadAggregate {
    pub fn new(
        function: AggregateFunction,
        elements: &[HeadAggregateElement],
        left_guard: &AggregateGuard,
        right_guard: &AggregateGuard,
    ) -> HeadAggregate {
        HeadAggregate(clingo_ast_head_aggregate {
            function: function as i32,
            elements: elements.as_ptr() as *const clingo_ast_head_aggregate_element_t,
            size: elements.len(),
            left_guard: &left_guard.data,
            right_guard: &right_guard.data,
        })
    }
    pub fn function(&self) -> AggregateFunction {
        match self.0.function as u32 {
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_count => {
                AggregateFunction::Count
            }
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_sum => {
                AggregateFunction::Sum
            }
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_sump => {
                AggregateFunction::Sump
            }
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_min => {
                AggregateFunction::Min
            }
            clingo_ast_aggregate_function_clingo_ast_aggregate_function_max => {
                AggregateFunction::Max
            }
            x => panic!("Failed to match clingo_ast_theory_term_type: {}.", x),
        }
    }
    pub fn elements(&self) -> &[HeadAggregateElement] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const HeadAggregateElement, self.0.size)
        }
    }
    pub fn left_guard(&self) -> Result<&AggregateGuard, ClingoError> {
        match unsafe { (self.0.left_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::new(
                "tried casting a null pointer to &AggregateGuard.",
            )),
        }
    }
    pub fn right_guard(&self) -> Result<&AggregateGuard, ClingoError> {
        match unsafe { (self.0.right_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::new(
                "tried casting a null pointer to &AggregateGuard.",
            )),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Disjunction(clingo_ast_disjunction);
impl Disjunction {
    pub fn new(elements: &[ConditionalLiteral]) -> Disjunction {
        Disjunction(clingo_ast_disjunction {
            elements: elements.as_ptr() as *const clingo_ast_conditional_literal_t,
            size: elements.len(),
        })
    }
    pub fn elements(&self) -> &[ConditionalLiteral] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const ConditionalLiteral, self.0.size)
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct DisjointElement(clingo_ast_disjoint_element);
impl DisjointElement {
    // pub fn new(tuple: &[Term], term: &CspSumTerm, condition: &[Literal]) -> DisjointElement {
    //     let term = CspSumTerm::into(*term);
    //     DisjointElement(clingo_ast_disjoint_element {
    //         location: Location::default(),
    //         tuple: tuple.as_ptr() as *const clingo_ast_term_t,
    //         tuple_size: tuple.len(),
    //         term,
    //         condition: condition.as_ptr() as *const clingo_ast_literal_t,
    //         condition_size: condition.len(),
    //     })
    // }
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    // pub fn tuple(&self) -> &[Term] {
    //     unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    // }
    pub fn term(&self) -> CspSumTerm {
        CspSumTerm(self.0.term)
    }
    // pub fn condition(&self) -> &[Literal] {
    //     unsafe {
    //         std::slice::from_raw_parts(self.0.condition as *const Literal, self.0.condition_size)
    //     }
    // }
}
#[derive(Debug, Copy, Clone)]
pub struct Disjoint(clingo_ast_disjoint);
impl Disjoint {
    pub fn new(elements: &[DisjointElement]) -> Disjoint {
        Disjoint(clingo_ast_disjoint {
            elements: elements.as_ptr() as *const clingo_ast_disjoint_element,
            size: elements.len(),
        })
    }
    pub fn elements(&self) -> &[DisjointElement] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const DisjointElement, self.0.size)
        }
    }
}
#[derive(Copy, Clone)]
pub enum TheoryTerm {
    Symbol(clingo_ast_theory_term),
    Variable(clingo_ast_theory_term),
    Tuple(clingo_ast_theory_term),
    List(clingo_ast_theory_term),
    Set(clingo_ast_theory_term),
    Function(clingo_ast_theory_term),
    UnparsedTerm(clingo_ast_theory_term),
}

impl TheoryTerm {
    fn into(self) -> clingo_ast_theory_term {
        match self {
            TheoryTerm::Symbol(term) => term,
            TheoryTerm::Variable(term) => term,
            TheoryTerm::Tuple(term) => term,
            TheoryTerm::List(term) => term,
            TheoryTerm::Set(term) => term,
            TheoryTerm::Function(term) => term,
            TheoryTerm::UnparsedTerm(term) => term,
        }
    }
    fn from(term: clingo_ast_theory_term) -> TheoryTerm {
        match term.type_ as u32 {
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_symbol => {
                TheoryTerm::Symbol(term)
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_variable => {
                TheoryTerm::Variable(term)
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_tuple => {
                TheoryTerm::Tuple(term)
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_list => TheoryTerm::List(term),
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_set => TheoryTerm::Set(term),
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_function => {
                TheoryTerm::Function(term)
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_unparsed_term => {
                TheoryTerm::UnparsedTerm(term)
            }
            x => panic!("Failed to match clingo_ast_theory_term_type: {}.", x),
        }
    }
    pub fn location(&self) -> Location {
        let term = TheoryTerm::into(*self);
        Location(term.location)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryTermArray(clingo_ast_theory_term_array);
impl TheoryTermArray {
    pub fn terms(&self) -> &[TheoryTerm] {
        unsafe { std::slice::from_raw_parts(self.0.terms as *const TheoryTerm, self.0.size) }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryFunction(clingo_ast_theory_function);
impl TheoryFunction {
    pub fn new(name: &str, arguments: &[TheoryTerm]) -> Result<TheoryFunction, NulError> {
        let name = CString::new(name)?;
        Ok(TheoryFunction(clingo_ast_theory_function {
            name: name.as_ptr(),
            arguments: arguments.as_ptr() as *const clingo_ast_theory_term_t,
            size: arguments.len(),
        }))
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.0.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.name) };
            c_str.to_str()
        }
    }
    pub fn arguments(&self) -> &[TheoryTerm] {
        unsafe { std::slice::from_raw_parts(self.0.arguments as *const TheoryTerm, self.0.size) }
    }
}
#[derive(Copy, Clone)]
pub struct TheoryUnparsedTermElement(clingo_ast_theory_unparsed_term_element);
impl TheoryUnparsedTermElement {
    pub fn operators(&self) -> Result<Vec<&str>, Utf8Error> {
        let s1 = unsafe {
            std::slice::from_raw_parts(
                self.0.operators as *const ::std::os::raw::c_char,
                self.0.size,
            )
        };
        let mut akku = vec![];
        for char_ptr in s1.iter() {
            akku.push(unsafe { CStr::from_ptr(char_ptr) }.to_str()?);
        }
        Ok(akku)
    }
    pub fn term(&self) -> TheoryTerm {
        TheoryTerm::from(self.0.term)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryUnparsedTerm(clingo_ast_theory_unparsed_term);
impl TheoryUnparsedTerm {
    pub fn new(elements: &[TheoryUnparsedTermElement]) -> TheoryUnparsedTerm {
        TheoryUnparsedTerm(clingo_ast_theory_unparsed_term {
            elements: elements.as_ptr() as *const clingo_ast_theory_unparsed_term_element_t,
            size: elements.len(),
        })
    }
    pub fn elements(&self) -> &[TheoryUnparsedTermElement] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.elements as *const TheoryUnparsedTermElement,
                self.0.size,
            )
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryAtomElement(clingo_ast_theory_atom_element);
impl TheoryAtomElement {
    // pub fn new(tuple: &[TheoryTerm], condition: &[Literal]) -> TheoryAtomElement {
    //     TheoryAtomElement(clingo_ast_theory_atom_element {
    //         tuple: tuple.as_ptr() as *const clingo_ast_theory_term_t,
    //         tuple_size: tuple.len(),
    //         condition: condition.as_ptr() as *const clingo_ast_literal_t,
    //         condition_size: condition.len(),
    //     })
    // }
    // pub fn tuple(&self) -> &[Term] {
    //     unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    // }
    // pub fn condition(&self) -> &[Literal] {
    //     unsafe {
    //         std::slice::from_raw_parts(self.0.condition as *const Literal, self.0.condition_size)
    //     }
    // }
}
#[derive(Copy, Clone)]
pub struct TheoryGuard(clingo_ast_theory_guard);
impl TheoryGuard {
    pub fn new(operator_name: &str, term: &TheoryTerm) -> Result<TheoryGuard, NulError> {
        let operator_name = CString::new(operator_name)?;
        let term = TheoryTerm::into(*term);
        Ok(TheoryGuard(clingo_ast_theory_guard {
            operator_name: operator_name.as_ptr(),
            term,
        }))
    }
    pub fn operator_name(&self) -> Result<&str, Utf8Error> {
        if self.0.operator_name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.operator_name) };
            c_str.to_str()
        }
    }
    pub fn term(&self) -> TheoryTerm {
        TheoryTerm::from(self.0.term)
    }
}
pub struct TheoryAtom(clingo_ast_theory_atom);
impl TheoryAtom {
    // pub fn new(term: &Term, elements: &[TheoryAtomElement], guard: &TheoryGuard) -> TheoryAtom {
    //     let term = Term::into(*term);
    //     let guard = TheoryGuard::into(*guard);
    //     TheoryAtom(clingo_ast_theory_atom {
    //         term,
    //         elements: elements.as_ptr() as *const clingo_ast_theory_atom_element_t,
    //         size: elements.len(),
    //         guard: &guard as *const clingo_ast_theory_guard_t,
    //     })
    // }
    // pub fn term(&self) -> Term {
    //     Term::from(self.0.term)
    // }
    pub fn elements(&self) -> &[TheoryAtomElement] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const TheoryAtomElement, self.0.size)
        }
    }
    pub fn guard(&self) -> Result<&TheoryGuard, ClingoError> {
        match unsafe { (self.0.guard as *const TheoryGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::new(
                "tried casting a null pointer to &TheoryGuard.",
            )),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum TheoryOperatorDefinition {
    Unary(clingo_ast_theory_operator_definition),
    BinaryLeft(clingo_ast_theory_operator_definition),
    BinaryRight(clingo_ast_theory_operator_definition),
}
impl TheoryOperatorDefinition {
    fn into(self) -> clingo_ast_theory_operator_definition {
        match self {
            TheoryOperatorDefinition::Unary(operator) => operator,
            TheoryOperatorDefinition::BinaryLeft(operator) => operator,
            TheoryOperatorDefinition::BinaryRight(operator) => operator,
        }
    }
    // fn from(operator: clingo_ast_theory_operator_definition) -> TheoryOperatorDefinition {
    //     match operator.type_ as u32 {
    //         clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_unary => {
    //             TheoryOperatorDefinition::Unary(operator)
    //         }
    //         clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_left => {
    //             TheoryOperatorDefinition::BinaryLeft(operator)
    //         }
    //         clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_right => {
    //             TheoryOperatorDefinition::BinaryRight(operator)
    //         }
    //         x => panic!("Failed to match clingo_ast_theory_operator_type: {}.", x),
    //     }
    // }
    pub fn location(&self) -> Location {
        let operator = TheoryOperatorDefinition::into(*self);
        Location(operator.location)
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        let operator = TheoryOperatorDefinition::into(*self);
        if operator.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(operator.name) };
            c_str.to_str()
        }
    }
    pub fn priority(&self) -> u32 {
        let operator = TheoryOperatorDefinition::into(*self);
        operator.priority
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryTermDefinition(clingo_ast_theory_term_definition);
impl TheoryTermDefinition {
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn new(
        name: &str,
        operators: &[TheoryOperatorDefinition],
    ) -> Result<TheoryTermDefinition, NulError> {
        let name = CString::new(name)?;
        Ok(TheoryTermDefinition(clingo_ast_theory_term_definition {
            location: Location::default(),
            name: name.as_ptr(),
            operators: operators.as_ptr() as *const clingo_ast_theory_operator_definition_t,
            size: operators.len(),
        }))
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.0.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.name) };
            c_str.to_str()
        }
    }
    pub fn operators(&self) -> &[TheoryOperatorDefinition] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.operators as *const TheoryOperatorDefinition,
                self.0.size,
            )
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryGuardDefinition(clingo_ast_theory_guard_definition);
impl TheoryGuardDefinition {
    pub fn term(&self) -> Result<&str, Utf8Error> {
        if self.0.term.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.term) };
            c_str.to_str()
        }
    }
    pub fn operators(&self) -> Result<Vec<&str>, Utf8Error> {
        let s1 = unsafe {
            std::slice::from_raw_parts(
                self.0.operators as *const ::std::os::raw::c_char,
                self.0.size,
            )
        };
        let mut akku = vec![];
        for char_ptr in s1.iter() {
            akku.push(unsafe { CStr::from_ptr(char_ptr) }.to_str()?);
        }
        Ok(akku)
    }
}
#[derive(Debug, Copy, Clone)]
pub enum TheoryAtomDefinition {
    Head(clingo_ast_theory_atom_definition),
    Body(clingo_ast_theory_atom_definition),
    Any(clingo_ast_theory_atom_definition),
    Directive(clingo_ast_theory_atom_definition),
}
impl TheoryAtomDefinition {
    fn into(self) -> clingo_ast_theory_atom_definition {
        match self {
            TheoryAtomDefinition::Head(atom) => atom,
            TheoryAtomDefinition::Body(atom) => atom,
            TheoryAtomDefinition::Any(atom) => atom,
            TheoryAtomDefinition::Directive(atom) => atom,
        }
    }
    // fn from(atom: clingo_ast_theory_atom_definition) -> TheoryAtomDefinition {
    //     match atom.type_ as u32 {
    //         clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_head => {
    //             TheoryAtomDefinition::Head(atom)
    //         }
    //         clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_body => {
    //             TheoryAtomDefinition::Body(atom)
    //         }
    //         clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_any => {
    //             TheoryAtomDefinition::Any(atom)
    //         }
    //         clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_directive => {
    //             TheoryAtomDefinition::Directive(atom)
    //         }
    //         x => panic!(
    //             "Failed to match clingo_ast_theory_atom_definition_type: {}.",
    //             x
    //         ),
    //     }
    // }
    pub fn location(&self) -> Location {
        let atom = TheoryAtomDefinition::into(*self);
        Location(atom.location)
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        let atom = TheoryAtomDefinition::into(*self);
        if atom.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(atom.name) };
            c_str.to_str()
        }
    }
    pub fn arity(&self) -> u32 {
        let atom = TheoryAtomDefinition::into(*self);
        atom.arity
    }
    pub fn elements(&self) -> Result<&str, Utf8Error> {
        let atom = TheoryAtomDefinition::into(*self);
        if atom.elements.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(atom.elements) };
            c_str.to_str()
        }
    }
    pub fn guard(&self) -> Result<&TheoryGuardDefinition, ClingoError> {
        let atom = TheoryAtomDefinition::into(*self);
        match unsafe { (atom.guard as *const TheoryGuardDefinition).as_ref() } {
            Some(x) => Ok(x),
            None => Err(ClingoError::new(
                "tried casting a null pointer to &TheoryGuardDefinition.",
            )),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryDefinition(clingo_ast_theory_definition);
impl TheoryDefinition {
    pub fn new(
        name: &str,
        terms: &[TheoryTermDefinition],
        atoms: &[TheoryAtomDefinition],
    ) -> Result<TheoryDefinition, NulError> {
        let name = CString::new(name)?;
        Ok(TheoryDefinition(clingo_ast_theory_definition {
            name: name.as_ptr(),
            terms: terms.as_ptr() as *const clingo_ast_theory_term_definition_t,
            terms_size: terms.len(),
            atoms: atoms.as_ptr() as *const clingo_ast_theory_atom_definition_t,
            atoms_size: atoms.len(),
        }))
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.0.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.name) };
            c_str.to_str()
        }
    }
    pub fn terms(&self) -> &[TheoryTermDefinition] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.terms as *const TheoryTermDefinition,
                self.0.terms_size,
            )
        }
    }
    pub fn atoms(&self) -> &[TheoryTermDefinition] {
        unsafe {
            std::slice::from_raw_parts(
                self.0.atoms as *const TheoryTermDefinition,
                self.0.atoms_size,
            )
        }
    }
}
