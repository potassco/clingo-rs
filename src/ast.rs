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
pub enum AggregateFunction {
    Count = clingo_ast_aggregate_function_clingo_ast_aggregate_function_count as isize,
    Sum = clingo_ast_aggregate_function_clingo_ast_aggregate_function_sum as isize,
    Sump = clingo_ast_aggregate_function_clingo_ast_aggregate_function_sump as isize,
    Min = clingo_ast_aggregate_function_clingo_ast_aggregate_function_min as isize,
    Max = clingo_ast_aggregate_function_clingo_ast_aggregate_function_max as isize,
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
impl fmt::Debug for AstStatement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.data.type_ as u32 {
            clingo_ast_statement_type_clingo_ast_statement_type_rule => {
                let rule = unsafe { self.data.__bindgen_anon_1.rule } as *const Rule;
                let rule = unsafe { rule.as_ref() }.unwrap();
                write!(f, "AstStatement {{ rule: {:?} }}", rule)
            }
            clingo_ast_statement_type_clingo_ast_statement_type_external => {
                let ext = unsafe { self.data.__bindgen_anon_1.external } as *const External;
                let ext = unsafe { ext.as_ref() }.unwrap();
                write!(f, "AstStatement {{ external: {:?} }}", ext)
            }
            _ => unimplemented!(),
        }
    }
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
#[derive(Copy, Clone)]
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
                write!(f, "HeadLiteral {{ literal: {:?} }}", literal)
            }
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_disjunction => {
                let dis = unsafe { self.data.__bindgen_anon_1.disjunction } as *const Disjunction;
                let dis = unsafe { dis.as_ref() }.unwrap();
                write!(f, "HeadLiteral {{ disjunction: {:?} }}", dis)
            }
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_aggregate => {
                let agg = unsafe { self.data.__bindgen_anon_1.aggregate } as *const Aggregate;
                let agg = unsafe { agg.as_ref() }.unwrap();
                write!(f, "HeadLiteral {{ aggregate: {:?} }}", agg)
            }
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_head_aggregate => {
                let hagg =
                    unsafe { self.data.__bindgen_anon_1.head_aggregate } as *const HeadAggregate;
                let hagg = unsafe { hagg.as_ref() }.unwrap();
                write!(f, "HeadLiteral {{ head_aggregate: {:?} }}", hagg)
            }
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_theory_atom => {
                let atom = unsafe { self.data.__bindgen_anon_1.theory_atom } as *const TheoryAtom;
                let atom = unsafe { atom.as_ref() }.unwrap();
                write!(f, "HeadLiteral {{ theory_atom: {:?} }}", atom)
            }
            x => panic!("Unknown head_literal_type: {}!", x),
        }
    }
}
impl<'a> From<&'a Literal<'a>> for HeadLiteral<'a> {
    fn from(lit: &'a Literal<'a>) -> HeadLiteral<'a> {
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
impl<'a> From<&'a Disjunction<'a>> for HeadLiteral<'a> {
    fn from(dis: &'a Disjunction) -> HeadLiteral<'a> {
        HeadLiteral {
            data: clingo_ast_head_literal_t {
                location: Location::default(),
                type_: clingo_ast_head_literal_type_clingo_ast_head_literal_type_disjunction as i32,
                __bindgen_anon_1: clingo_ast_head_literal__bindgen_ty_1 {
                    disjunction: &dis.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a Aggregate<'a>> for HeadLiteral<'a> {
    fn from(agg: &'a Aggregate) -> HeadLiteral<'a> {
        HeadLiteral {
            data: clingo_ast_head_literal_t {
                location: Location::default(),
                type_: clingo_ast_head_literal_type_clingo_ast_head_literal_type_aggregate as i32,
                __bindgen_anon_1: clingo_ast_head_literal__bindgen_ty_1 {
                    aggregate: &agg.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a HeadAggregate<'a>> for HeadLiteral<'a> {
    fn from(agg: &'a HeadAggregate) -> HeadLiteral<'a> {
        HeadLiteral {
            data: clingo_ast_head_literal_t {
                location: Location::default(),
                type_: clingo_ast_head_literal_type_clingo_ast_head_literal_type_head_aggregate
                    as i32,
                __bindgen_anon_1: clingo_ast_head_literal__bindgen_ty_1 {
                    head_aggregate: &agg.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a TheoryAtom<'a>> for HeadLiteral<'a> {
    fn from(atom: &'a TheoryAtom<'a>) -> HeadLiteral<'a> {
        HeadLiteral {
            data: clingo_ast_head_literal_t {
                location: Location::default(),
                type_: clingo_ast_head_literal_type_clingo_ast_head_literal_type_theory_atom as i32,
                __bindgen_anon_1: clingo_ast_head_literal__bindgen_ty_1 {
                    theory_atom: &atom.data,
                },
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

#[derive(Copy, Clone)]
pub struct Rule<'a> {
    data: clingo_ast_rule_t,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Rule<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let head = self.head();
        let body = self.body();
        write!(f, "Rule {{ head: {:?}, body: {:?} }}", head, &body)
    }
}
impl<'a> Rule<'a> {
    pub fn new(head: HeadLiteral<'a>, body: &'a [BodyLiteral<'a>]) -> Rule<'a> {
        Rule {
            data: clingo_ast_rule {
                head: head.data,
                body: body.as_ptr() as *const clingo_ast_body_literal_t,
                size: body.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn head(&'a self) -> &'a HeadLiteral<'a> {
        unsafe {
            (&self.data.head as *const clingo_ast_head_literal_t as *const HeadLiteral).as_ref()
        }
        .unwrap()
    }
    pub fn body(&'a self) -> &'a [BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.size) }
    }
    /// Create a statement for the rule.
    pub fn ast_statement(&'a self) -> AstStatement<'a> {
        AstStatement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_rule as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    rule: &self.data as *const clingo_ast_rule,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
pub struct Definition<'a> {
    data: clingo_ast_definition,
    _lifetime: PhantomData<&'a ()>,
}
impl<'a> Definition<'a> {
    pub fn new(
        name: &'a str,
        value: &'a Term<'a>,
        is_default: bool,
    ) -> Result<Definition<'a>, NulError> {
        let name = CString::new(name)?;
        Ok(Definition {
            data: clingo_ast_definition {
                name: name.as_ptr(),
                value: value.data,
                is_default,
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
    // pub fn value(&self) -> Term {
    //     Term::from(self.data.value)
    // }
    pub fn is_default(&self) -> bool {
        self.data.is_default
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
#[derive(Debug, Copy, Clone)]
pub enum BodyLiteralType<'a> {
    Literal(&'a Literal<'a>),
    Conditional(&'a ConditionalLiteral<'a>),
    Aggregate(&'a Aggregate<'a>),
    BodyAggregate(&'a BodyAggregate<'a>),
    TheoryAtom(&'a TheoryAtom<'a>),
    Disjoint(&'a Disjoint<'a>),
}
#[derive(Copy, Clone)]
pub struct BodyLiteral<'a> {
    data: clingo_ast_body_literal_t,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for BodyLiteral<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = self.sign();
        match self.body_literal_type() {
            BodyLiteralType::Literal(lit) => {
                write!(f, "BodyLiteral {{ sign: {:?} literal: {:?} }}", sign, lit)
            }
            BodyLiteralType::Conditional(lit) => write!(
                f,
                "BodyLiteral {{ sign: {:?} conditional: {:?} }}",
                sign, lit
            ),
            BodyLiteralType::Aggregate(agg) => {
                write!(f, "BodyLiteral {{ sign: {:?} aggregate: {:?} }}", sign, agg)
            }
            BodyLiteralType::BodyAggregate(agg) => write!(
                f,
                "BodyLiteral {{ sign: {:?} body_aggregate: {:?} }}",
                sign, agg
            ),
            BodyLiteralType::TheoryAtom(atom) => write!(
                f,
                "BodyLiteral {{ sign: {:?} theory_atom: {:?} }}",
                sign, atom
            ),
            BodyLiteralType::Disjoint(dis) => {
                write!(f, "BodyLiteral {{ sign: {:?} disjoint: {:?} }}", sign, dis)
            }
        }
    }
}
impl<'a> BodyLiteral<'a> {
    pub fn from_literal(sign: Sign, lit: &'a Literal<'a>) -> BodyLiteral<'a> {
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
    pub fn from_conditional(sign: Sign, lit: &'a ConditionalLiteral<'a>) -> BodyLiteral<'a> {
        BodyLiteral {
            data: clingo_ast_body_literal_t {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_conditional as i32,
                __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
                    conditional: &lit.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
    pub fn from_aggregate(sign: Sign, agg: &'a Aggregate<'a>) -> BodyLiteral<'a> {
        BodyLiteral {
            data: clingo_ast_body_literal_t {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_aggregate as i32,
                __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
                    aggregate: &agg.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
    pub fn from_body_aggregate(sign: Sign, agg: &'a BodyAggregate<'a>) -> BodyLiteral<'a> {
        BodyLiteral {
            data: clingo_ast_body_literal_t {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_body_aggregate
                    as i32,
                __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
                    body_aggregate: &agg.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
    pub fn from_theory_atom(sign: Sign, atom: &'a TheoryAtom<'a>) -> BodyLiteral<'a> {
        BodyLiteral {
            data: clingo_ast_body_literal_t {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_theory_atom as i32,
                __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
                    theory_atom: &atom.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
    pub fn from_disjoint(sign: Sign, dis: &'a Disjoint<'a>) -> BodyLiteral<'a> {
        BodyLiteral {
            data: clingo_ast_body_literal_t {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_body_literal_type_clingo_ast_body_literal_type_disjoint as i32,
                __bindgen_anon_1: clingo_ast_body_literal__bindgen_ty_1 {
                    disjoint: &dis.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
    // pub fn location(&self) -> Location {
    //     let lit = BodyLiteral::into(*self);
    //     Location(lit.location)
    // }
    pub fn sign(&self) -> Sign {
        match self.data.sign as u32 {
            clingo_ast_sign_clingo_ast_sign_double_negation => Sign::DoubleNegation,
            clingo_ast_sign_clingo_ast_sign_negation => Sign::Negation,
            clingo_ast_sign_clingo_ast_sign_none => Sign::None,
            x => panic!("Failed to match clingo_ast_sign: {}.", x),
        }
    }
    pub fn body_literal_type(&self) -> BodyLiteralType {
        match self.data.type_ as u32 {
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_literal => {
                BodyLiteralType::Literal(
                    unsafe { (self.data.__bindgen_anon_1.literal as *const Literal).as_ref() }
                        .unwrap(),
                )
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_conditional => {
                BodyLiteralType::Conditional(
                    unsafe {
                        (self.data.__bindgen_anon_1.conditional as *const ConditionalLiteral)
                            .as_ref()
                    }
                    .unwrap(),
                )
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_aggregate => {
                BodyLiteralType::Aggregate(
                    unsafe { (self.data.__bindgen_anon_1.aggregate as *const Aggregate).as_ref() }
                        .unwrap(),
                )
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_body_aggregate => {
                BodyLiteralType::BodyAggregate(
                    unsafe {
                        (self.data.__bindgen_anon_1.body_aggregate as *const BodyAggregate).as_ref()
                    }
                    .unwrap(),
                )
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_theory_atom => {
                BodyLiteralType::TheoryAtom(
                    unsafe {
                        (self.data.__bindgen_anon_1.theory_atom as *const TheoryAtom).as_ref()
                    }
                    .unwrap(),
                )
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_disjoint => {
                BodyLiteralType::Disjoint(
                    unsafe { (self.data.__bindgen_anon_1.disjoint as *const Disjoint).as_ref() }
                        .unwrap(),
                )
            }
            x => panic!("Failed to match clingo_ast_body_literal_type: {}.", x),
        }
    }
}
#[derive(Copy, Clone)]
pub struct External<'a> {
    data: clingo_ast_external_t,
    _lifetime: PhantomData<&'a u32>,
}
impl fmt::Debug for External<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let atom = Term {
            data: self.data.atom,
            _lifetime: PhantomData,
        };
        let body = unsafe {
            std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.size)
        };
        write!(f, "External {{ atom: {:?}, body: {:?} }}", atom, &body)
    }
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
    pub fn ast_statement(&'a self) -> AstStatement<'a> {
        AstStatement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: StatementType::External as clingo_ast_statement_type_t,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    external: &self.data,
                },
            },
            _lifetime: PhantomData,
        }
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

#[derive(Debug, Clone)]
pub enum TermType<'a> {
    Symbol(Symbol),
    Variable(&'a str),
    UnaryOperation(&'a UnaryOperation<'a>),
    BinaryOperation(&'a BinaryOperation<'a>),
    Interval(&'a Interval<'a>),
    Function(&'a Function<'a>),
    ExternalFunction(&'a Function<'a>),
    Pool(&'a Pool<'a>),
}
#[derive(Copy, Clone)]
pub struct Term<'a> {
    data: clingo_ast_term_t,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.term_type() {
            TermType::Symbol(sym) => {
                let string = sym.to_string().unwrap();
                write!(f, "Term {{ symbol: {} }}", string)
            }
            TermType::Variable(var) => write!(f, "Term {{ variable: {:?} }}", var),
            TermType::UnaryOperation(uop) => write!(f, "Term {{ unary_operation: {:?} }}", uop),
            TermType::BinaryOperation(bop) => write!(f, "Term {{ binary_operation: {:?} }}", bop),
            TermType::Interval(interval) => write!(f, "Term {{ interval: {:?} }}", interval),
            TermType::Function(fun) => write!(f, "Term {{ function: {:?} }}", fun),
            TermType::ExternalFunction(fun) => write!(f, "Term {{ external_function: {:?} }}", fun),
            TermType::Pool(pool) => write!(f, "Term {{ pool: {:?} }}", pool),
        }
    }
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
impl<'a> From<&'a UnaryOperation<'a>> for Term<'a> {
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
impl<'a> From<&'a BinaryOperation<'a>> for Term<'a> {
    fn from(op: &'a BinaryOperation<'a>) -> Self {
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
impl<'a> From<&'a Interval<'a>> for Term<'a> {
    fn from(interval: &'a Interval<'a>) -> Self {
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
impl<'a> From<&'a Function<'a>> for Term<'a> {
    fn from(fun: &'a Function<'a>) -> Self {
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
impl<'a> From<&'a Pool<'a>> for Term<'a> {
    fn from(pool: &'a Pool<'a>) -> Self {
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
impl<'a> Term<'a> {
    /// Create a variable term
    ///
    /// # Errors
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `string` contains a nul byte
    pub fn variable(name: &'a str) -> Result<Term<'a>, Error> {
        let variable = internalize_string(name)?;
        Ok(Term {
            data: clingo_ast_term {
                location: Location::default(),
                type_: clingo_ast_term_type_clingo_ast_term_type_variable as i32,
                __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 { variable },
            },
            _lifetime: PhantomData,
        })
    }
    /// Create a term from an external function
    pub fn external_function(fun: &'a Function<'a>) -> Self {
        Term {
            data: clingo_ast_term_t {
                location: Location::default(),
                type_: clingo_ast_term_type_clingo_ast_term_type_external_function as i32,
                __bindgen_anon_1: clingo_ast_term__bindgen_ty_1 {
                    function: &fun.data,
                },
            },
            _lifetime: PhantomData,
        }
    }

    pub fn term_type(&self) -> TermType {
        match self.data.type_ as u32 {
            clingo_ast_term_type_clingo_ast_term_type_symbol => {
                TermType::Symbol(Symbol(unsafe { self.data.__bindgen_anon_1.symbol }))
            }
            clingo_ast_term_type_clingo_ast_term_type_variable => TermType::Variable(
                if unsafe { self.data.__bindgen_anon_1.variable.is_null() } {
                    ""
                } else {
                    let c_str = unsafe { CStr::from_ptr(self.data.__bindgen_anon_1.variable) };
                    c_str.to_str().unwrap()
                },
            ),
            clingo_ast_term_type_clingo_ast_term_type_unary_operation => TermType::UnaryOperation(
                unsafe {
                    (self.data.__bindgen_anon_1.unary_operation as *const UnaryOperation).as_ref()
                }
                .unwrap(),
            ),
            clingo_ast_term_type_clingo_ast_term_type_binary_operation => {
                TermType::BinaryOperation(
                    unsafe {
                        (self.data.__bindgen_anon_1.binary_operation as *const BinaryOperation)
                            .as_ref()
                    }
                    .unwrap(),
                )
            }
            clingo_ast_term_type_clingo_ast_term_type_interval => TermType::Interval(
                unsafe { (self.data.__bindgen_anon_1.interval as *const Interval).as_ref() }
                    .unwrap(),
            ),
            clingo_ast_term_type_clingo_ast_term_type_function => TermType::Function(
                unsafe { (self.data.__bindgen_anon_1.function as *const Function).as_ref() }
                    .unwrap(),
            ),
            clingo_ast_term_type_clingo_ast_term_type_external_function => {
                TermType::ExternalFunction(
                    unsafe {
                        (self.data.__bindgen_anon_1.external_function as *const Function).as_ref()
                    }
                    .unwrap(),
                )
            }
            clingo_ast_term_type_clingo_ast_term_type_pool => TermType::Pool(
                unsafe { (self.data.__bindgen_anon_1.pool as *const Pool).as_ref() }.unwrap(),
            ),
            x => panic!("Failed to match clingo_ast_term_type: {}.", x),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum LiteralType<'a> {
    Boolean(bool),
    Comparison(&'a Comparison<'a>),
    CSP(&'a CspLiteral<'a>),
    Symbolic(&'a Term<'a>),
}
#[derive(Copy, Clone)]
pub struct Literal<'a> {
    data: clingo_ast_literal_t,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = self.sign();
        match self.literal_type() {
            LiteralType::Boolean(boolean) => {
                write!(f, "Literal {{ sign: {:?} boolean: {:?} }}", sign, boolean)
            }
            LiteralType::Symbolic(term) => {
                write!(f, "Literal {{ sign: {:?} symbol: {:?} }}", sign, term)
            }
            LiteralType::Comparison(comp) => {
                write!(f, "Literal {{ sign: {:?} comparison: {:?} }}", sign, comp)
            }
            LiteralType::CSP(csp) => {
                write!(f, "Literal {{ sign: {:?} csp_literal: {:?} }}", sign, csp)
            }
        }
    }
}
impl<'a> Literal<'a> {
    /// Create a literal from a boolean
    pub fn from_bool(sign: Sign, boolean: bool) -> Literal<'a> {
        Literal {
            data: clingo_ast_literal {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_literal_type_clingo_ast_literal_type_boolean as i32,
                __bindgen_anon_1: clingo_ast_literal__bindgen_ty_1 { boolean },
            },
            _lifetime: PhantomData,
        }
    }
    /// Create a literal from a term.
    pub fn from_term(sign: Sign, term: &'a Term<'a>) -> Literal<'a> {
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
    /// Create a literal from a comparison.
    pub fn from_comparison(sign: Sign, comp: &'a Comparison<'a>) -> Literal<'a> {
        Literal {
            data: clingo_ast_literal {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_literal_type_clingo_ast_literal_type_comparison as i32,
                __bindgen_anon_1: clingo_ast_literal__bindgen_ty_1 {
                    comparison: &comp.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
    pub fn from_csp_literal(sign: Sign, csp: &'a CspLiteral<'a>) -> Literal<'a> {
        Literal {
            data: clingo_ast_literal {
                location: Location::default(),
                sign: sign as i32,
                type_: clingo_ast_literal_type_clingo_ast_literal_type_csp as i32,
                __bindgen_anon_1: clingo_ast_literal__bindgen_ty_1 {
                    csp_literal: &csp.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
    // pub fn location(&self) -> Location {
    //     let lit = Literal::into(*self);
    //     Location(lit.location)
    // }
    pub fn sign(&self) -> Sign {
        match self.data.sign as u32 {
            clingo_ast_sign_clingo_ast_sign_double_negation => Sign::DoubleNegation,
            clingo_ast_sign_clingo_ast_sign_negation => Sign::Negation,
            clingo_ast_sign_clingo_ast_sign_none => Sign::None,
            x => panic!("Failed to match clingo_ast_sign: {}.", x),
        }
    }
    pub fn literal_type(&self) -> LiteralType {
        match self.data.type_ as u32 {
            clingo_ast_literal_type_clingo_ast_literal_type_boolean => {
                LiteralType::Boolean(unsafe { self.data.__bindgen_anon_1.boolean })
            }
            clingo_ast_literal_type_clingo_ast_literal_type_comparison => LiteralType::Comparison(
                unsafe { (self.data.__bindgen_anon_1.comparison as *const Comparison).as_ref() }
                    .unwrap(),
            ),
            clingo_ast_literal_type_clingo_ast_literal_type_csp => LiteralType::CSP(
                unsafe { (self.data.__bindgen_anon_1.csp_literal as *const CspLiteral).as_ref() }
                    .unwrap(),
            ),
            clingo_ast_literal_type_clingo_ast_literal_type_symbolic => LiteralType::Symbolic(
                unsafe { (self.data.__bindgen_anon_1.symbol as *const Term).as_ref() }.unwrap(),
            ),
            x => panic!("Failed to match clingo_ast_literal_type: {}.", x),
        }
    }
}
pub struct UnaryOperation<'a> {
    data: clingo_ast_unary_operation_t,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for UnaryOperation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UnaryOperation {{ unary_operator: {:?} argument: {:?} }}",
            self.unary_operator(),
            self.argument()
        )
    }
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
    pub fn unary_operator(&self) -> UnaryOperator {
        match self.data.unary_operator as u32 {
            clingo_ast_unary_operator_clingo_ast_unary_operator_minus => UnaryOperator::Minus,
            clingo_ast_unary_operator_clingo_ast_unary_operator_negation => UnaryOperator::Negation,
            clingo_ast_unary_operator_clingo_ast_unary_operator_absolute => UnaryOperator::Absolute,
            x => panic!("Failed to match clingo_ast_unary_operator: {}.", x),
        }
    }
    pub fn argument(&self) -> &'a Term<'a> {
        unsafe { (&self.data.argument as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
}
pub struct BinaryOperation<'a> {
    data: clingo_ast_binary_operation_t,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for BinaryOperation<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BinaryOperation {{ binary_operator: {:?} left: {:?} right: {:?} }}",
            self.binary_operator(),
            self.left(),
            self.right()
        )
    }
}
impl<'a> BinaryOperation<'a> {
    pub fn xor(left: Term<'a>, right: Term<'a>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_xor as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn or(left: Term<'a>, right: Term<'a>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_or as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn and(left: Term<'a>, right: Term<'a>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_and as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn plus(left: Term<'a>, right: Term<'a>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_plus as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn minus(left: Term<'a>, right: Term<'a>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_minus as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn multiplication(left: Term<'a>, right: Term<'a>) -> BinaryOperation<'a> {
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
    pub fn division(left: Term<'a>, right: Term<'a>) -> BinaryOperation<'a> {
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
    pub fn modulo(left: Term<'a>, right: Term<'a>) -> BinaryOperation<'a> {
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
    pub fn power(left: Term<'a>, right: Term<'a>) -> BinaryOperation<'a> {
        BinaryOperation {
            data: clingo_ast_binary_operation {
                binary_operator: clingo_ast_binary_operator_clingo_ast_binary_operator_power as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn binary_operator(&self) -> BinaryOperator {
        match self.data.binary_operator as u32 {
            clingo_ast_binary_operator_clingo_ast_binary_operator_xor => BinaryOperator::Xor,
            clingo_ast_binary_operator_clingo_ast_binary_operator_or => BinaryOperator::Or,
            clingo_ast_binary_operator_clingo_ast_binary_operator_and => BinaryOperator::And,
            clingo_ast_binary_operator_clingo_ast_binary_operator_plus => BinaryOperator::Plus,
            clingo_ast_binary_operator_clingo_ast_binary_operator_minus => BinaryOperator::Minus,
            clingo_ast_binary_operator_clingo_ast_binary_operator_multiplication => {
                BinaryOperator::Multiplication
            }
            clingo_ast_binary_operator_clingo_ast_binary_operator_division => {
                BinaryOperator::Division
            }
            clingo_ast_binary_operator_clingo_ast_binary_operator_modulo => BinaryOperator::Modulo,
            clingo_ast_binary_operator_clingo_ast_binary_operator_power => BinaryOperator::Power,
            x => panic!("Failed to match clingo_ast_binary_operator: {}.", x),
        }
    }
    pub fn left(&self) -> &'a Term<'a> {
        unsafe { (&self.data.left as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn right(&self) -> &'a Term<'a> {
        unsafe { (&self.data.right as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct Interval<'a> {
    data: clingo_ast_interval,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Interval<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Interval {{ left: {:?} right: {:?} }}",
            self.left(),
            self.right()
        )
    }
}
impl<'a> Interval<'a> {
    pub fn new(left: Term<'a>, right: Term<'a>) -> Interval<'a> {
        Interval {
            data: clingo_ast_interval {
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn left(&self) -> &'a Term<'a> {
        unsafe { (&self.data.left as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn right(&self) -> &'a Term<'a> {
        unsafe { (&self.data.right as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct Function<'a> {
    data: clingo_ast_function,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Function<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().expect("Cant get function name!");
        write!(
            f,
            "Function {{ name: {} args: {:?} }}",
            name,
            self.arguments()
        )
    }
}
impl<'a> Function<'a> {
    pub fn new(name: &'a str, arguments: &'a [Term<'a>]) -> Result<Function<'a>, Error> {
        let name = internalize_string(name)?;
        Ok(Function {
            data: clingo_ast_function {
                name,
                arguments: arguments.as_ptr() as *const clingo_ast_term_t,
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
#[derive(Copy, Clone)]
pub struct Pool<'a> {
    data: clingo_ast_pool,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Pool<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pool {{ args: {:?} }}", self.arguments())
    }
}
impl<'a> Pool<'a> {
    pub fn new(arguments: &'a [Term<'a>]) -> Pool<'a> {
        Pool {
            data: clingo_ast_pool {
                arguments: arguments.as_ptr() as *const clingo_ast_term_t,
                size: arguments.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn arguments(&self) -> &'a [Term<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.arguments as *const Term, self.data.size) }
    }
}
#[derive(Copy, Clone)]
pub struct CspProductTerm<'a> {
    data: clingo_ast_csp_product_term,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for CspProductTerm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CspProductTerm {{ coefficient: {:?} variable: {:?} }}",
            self.coefficient(),
            self.variable()
        )
    }
}
impl<'a> CspProductTerm<'a> {
    pub fn new(coefficient: Term<'a>, variable: &'a Term) -> CspProductTerm<'a> {
        CspProductTerm {
            data: clingo_ast_csp_product_term {
                location: Location::default(),
                coefficient: coefficient.data,
                variable: &variable.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn location(&self) -> Location {
        Location(self.data.location)
    }
    pub fn coefficient(&self) -> &'a Term<'a> {
        unsafe { (&self.data.coefficient as *const clingo_ast_term as *const Term).as_ref() }
            .unwrap()
    }
    pub fn variable(&self) -> &'a Term<'a> {
        unsafe { (self.data.variable as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct CspSumTerm<'a> {
    data: clingo_ast_csp_sum_term,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for CspSumTerm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CspSumTerm {{ terms: {:?} }}", self.terms())
    }
}
impl<'a> CspSumTerm<'a> {
    pub fn new(terms: &'a [CspProductTerm<'a>]) -> CspSumTerm<'a> {
        CspSumTerm {
            data: clingo_ast_csp_sum_term {
                location: Location::default(),
                terms: terms.as_ptr() as *const clingo_ast_csp_product_term_t,
                size: terms.len(),
            },
            _lifetime: PhantomData,
        }
    }
    // pub fn location(&self) -> Location {
    //     Location(self.data.location)
    // }
    pub fn terms(&self) -> &'a [CspProductTerm<'a>] {
        unsafe {
            std::slice::from_raw_parts(self.data.terms as *const CspProductTerm, self.data.size)
        }
    }
}

#[derive(Copy, Clone)]
pub struct CspGuard<'a> {
    data: clingo_ast_csp_guard,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for CspGuard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CspGuard {{ comparison: {:?} term: {:?} }}",
            self.comparison_type(),
            self.term()
        )
    }
}
impl<'a> CspGuard<'a> {
    pub fn gt(term: CspSumTerm<'a>) -> CspGuard<'a> {
        CspGuard {
            data: clingo_ast_csp_guard {
                comparison:
                    clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than
                        as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn lt(term: CspSumTerm<'a>) -> CspGuard<'a> {
        CspGuard {
            data: clingo_ast_csp_guard {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than
                    as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn le(term: CspSumTerm<'a>) -> CspGuard<'a> {
        CspGuard {
            data: clingo_ast_csp_guard {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal
                    as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn ge(term: CspSumTerm<'a>) -> CspGuard<'a> {
        CspGuard {
            data: clingo_ast_csp_guard {
                comparison:
                    clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal
                        as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn ne(term: CspSumTerm<'a>) -> CspGuard<'a> {
        CspGuard {
            data: clingo_ast_csp_guard {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal
                    as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn eq(term: CspSumTerm<'a>) -> CspGuard<'a> {
        CspGuard {
            data: clingo_ast_csp_guard {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal
                    as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn comparison_type(&self) -> ComparisonOperator {
        match self.data.comparison as u32 {
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than => {
                ComparisonOperator::GreaterThan
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than => {
                ComparisonOperator::LessThan
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal => {
                ComparisonOperator::LessEqual
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal => {
                ComparisonOperator::GreaterThan
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal => {
                ComparisonOperator::NotEqual
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal => {
                ComparisonOperator::Equal
            }
            x => panic!("Failed to match clingo_ast_comparison_operator: {}.", x),
        }
    }
    pub fn term(&self) -> &'a CspSumTerm<'a> {
        unsafe { (&self.data.term as *const clingo_ast_csp_sum_term as *const CspSumTerm).as_ref() }
            .unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct CspLiteral<'a> {
    data: clingo_ast_csp_literal,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for CspLiteral<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term = self.term();
        let guards = self.guards();
        write!(f, "CspLiteral {{ term: {:?} guards: {:?} }}", term, guards)
    }
}
impl<'a> CspLiteral<'a> {
    pub fn new(term: CspSumTerm<'a>, guards: &'a [CspGuard<'a>]) -> CspLiteral<'a> {
        CspLiteral {
            data: clingo_ast_csp_literal {
                term: term.data,
                guards: guards.as_ptr() as *const clingo_ast_csp_guard_t,
                size: guards.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn term(&self) -> &'a CspSumTerm<'a> {
        unsafe { (&self.data.term as *const clingo_ast_csp_sum_term as *const CspSumTerm).as_ref() }
            .unwrap()
    }
    pub fn guards(&self) -> &'a [CspGuard<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.guards as *const CspGuard, self.data.size) }
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
#[derive(Copy, Clone)]
pub struct Comparison<'a> {
    data: clingo_ast_comparison,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Comparison<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Comparison {{ op: {:?} left: {:?} right: {:?} }}",
            self.comparison_type(),
            self.left(),
            self.right()
        )
    }
}
impl<'a> Comparison<'a> {
    pub fn gt(left: Term<'a>, right: Term<'a>) -> Comparison<'a> {
        Comparison {
            data: clingo_ast_comparison {
                comparison:
                    clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than
                        as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn lt(left: Term<'a>, right: Term<'a>) -> Comparison<'a> {
        Comparison {
            data: clingo_ast_comparison {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than
                    as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn le(left: Term<'a>, right: Term<'a>) -> Comparison<'a> {
        Comparison {
            data: clingo_ast_comparison {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal
                    as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn ge(left: Term<'a>, right: Term<'a>) -> Comparison<'a> {
        Comparison {
            data: clingo_ast_comparison {
                comparison:
                    clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal
                        as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn ne(left: Term<'a>, right: Term<'a>) -> Comparison<'a> {
        Comparison {
            data: clingo_ast_comparison {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal
                    as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn eq(left: Term<'a>, right: Term<'a>) -> Comparison<'a> {
        Comparison {
            data: clingo_ast_comparison {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal
                    as i32,
                left: left.data,
                right: right.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn comparison_type(&self) -> ComparisonOperator {
        match self.data.comparison as u32 {
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than => {
                ComparisonOperator::GreaterThan
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than => {
                ComparisonOperator::LessThan
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal => {
                ComparisonOperator::LessEqual
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal => {
                ComparisonOperator::GreaterThan
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal => {
                ComparisonOperator::NotEqual
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal => {
                ComparisonOperator::Equal
            }
            x => panic!("Failed to match clingo_ast_comparison_operator: {}.", x),
        }
    }

    pub fn left(&self) -> &'a Term<'a> {
        unsafe { (&self.data.left as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn right(&self) -> &'a Term<'a> {
        unsafe { (&self.data.right as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct AggregateGuard<'a> {
    data: clingo_ast_aggregate_guard,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for AggregateGuard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AggregateGuard {{ comparison: {:?}, term: {:?} }}",
            self.comparison_type(),
            self.term()
        )
    }
}
impl<'a> AggregateGuard<'a> {
    pub fn gt(term: Term<'a>) -> AggregateGuard<'a> {
        AggregateGuard {
            data: clingo_ast_aggregate_guard {
                comparison:
                    clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than
                        as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn lt(term: Term<'a>) -> AggregateGuard<'a> {
        AggregateGuard {
            data: clingo_ast_aggregate_guard {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than
                    as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn le(term: Term<'a>) -> AggregateGuard<'a> {
        AggregateGuard {
            data: clingo_ast_aggregate_guard {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal
                    as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn ge(term: Term<'a>) -> AggregateGuard<'a> {
        AggregateGuard {
            data: clingo_ast_aggregate_guard {
                comparison:
                    clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal
                        as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn ne(term: Term<'a>) -> AggregateGuard<'a> {
        AggregateGuard {
            data: clingo_ast_aggregate_guard {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal
                    as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn eq(term: Term<'a>) -> AggregateGuard<'a> {
        AggregateGuard {
            data: clingo_ast_aggregate_guard {
                comparison: clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal
                    as i32,
                term: term.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn comparison_type(&self) -> ComparisonOperator {
        match self.data.comparison as u32 {
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than => {
                ComparisonOperator::GreaterThan
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than => {
                ComparisonOperator::LessThan
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal => {
                ComparisonOperator::LessEqual
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal => {
                ComparisonOperator::GreaterThan
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal => {
                ComparisonOperator::NotEqual
            }
            clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal => {
                ComparisonOperator::Equal
            }
            x => panic!("Failed to match clingo_ast_comparison_operator: {}.", x),
        }
    }
    pub fn term(&self) -> &'a Term<'a> {
        unsafe { (&self.data.term as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct ConditionalLiteral<'a> {
    data: clingo_ast_conditional_literal,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for ConditionalLiteral<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConditionalLiteral {{ literal: {:?}, condition: {:?} }}",
            self.literal(),
            self.condition()
        )
    }
}
impl<'a> ConditionalLiteral<'a> {
    pub fn new(literal: &'a Literal<'a>, condition: &'a [Literal<'a>]) -> ConditionalLiteral<'a> {
        ConditionalLiteral {
            data: clingo_ast_conditional_literal {
                literal: literal.data,
                condition: condition.as_ptr() as *const clingo_ast_literal_t,
                size: condition.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn literal(&self) -> &'a Literal<'a> {
        unsafe { (&self.data.literal as *const clingo_ast_literal_t as *const Literal).as_ref() }
            .unwrap()
    }
    pub fn condition(&self) -> &'a [Literal<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.condition as *const Literal, self.data.size) }
    }
}
pub struct Aggregate<'a> {
    data: clingo_ast_aggregate,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Aggregate<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Aggregate {{ elements: {:?}, left_guard: {:?}, right_guard: {:?} }}",
            self.elements(),
            self.left_guard(),
            self.right_guard()
        )
    }
}
impl<'a> Aggregate<'a> {
    pub fn new(
        elements: &'a [ConditionalLiteral<'a>],
        left_guard: &'a AggregateGuard<'a>,
        right_guard: &'a AggregateGuard<'a>,
    ) -> Aggregate<'a> {
        Aggregate {
            data: clingo_ast_aggregate {
                elements: elements.as_ptr() as *const clingo_ast_conditional_literal_t,
                size: elements.len(),
                left_guard: &left_guard.data,
                right_guard: &right_guard.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn elements(&self) -> &[ConditionalLiteral] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.elements as *const ConditionalLiteral,
                self.data.size,
            )
        }
    }
    pub fn left_guard(&self) -> &'a AggregateGuard<'a> {
        unsafe {
            (self.data.left_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard)
                .as_ref()
        }
        .unwrap()
    }
    pub fn right_guard(&self) -> &'a AggregateGuard<'a> {
        unsafe {
            (self.data.right_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard)
                .as_ref()
        }
        .unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct BodyAggregateElement<'a> {
    data: clingo_ast_body_aggregate_element,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for BodyAggregateElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BodyAggregateElement {{ tuple: {:?}, condition: {:?} }}",
            self.tuple(),
            self.condition()
        )
    }
}
impl<'a> BodyAggregateElement<'a> {
    pub fn new(tuple: &'a [Term<'a>], condition: &'a [Literal<'a>]) -> BodyAggregateElement<'a> {
        BodyAggregateElement {
            data: clingo_ast_body_aggregate_element {
                tuple: tuple.as_ptr() as *const clingo_ast_term_t,
                tuple_size: tuple.len(),
                condition: condition.as_ptr() as *const clingo_ast_literal_t,
                condition_size: condition.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn tuple(&self) -> &'a [Term<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.tuple as *const Term, self.data.tuple_size) }
    }
    pub fn condition(&self) -> &'a [Literal<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.condition as *const Literal,
                self.data.condition_size,
            )
        }
    }
}
#[derive(Copy, Clone)]
pub struct BodyAggregate<'a> {
    data: clingo_ast_body_aggregate,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for BodyAggregate<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BodyAggregate {{ function: {:?} elements: {:?}, left_guard: {:?}, right_guard: {:?} }}",
            self.aggregate_function(), self.elements(), self.left_guard(), self.right_guard()
        )
    }
}
impl<'a> BodyAggregate<'a> {
    pub fn new(
        function: AggregateFunction,
        elements: &'a [BodyAggregateElement<'a>],
        left_guard: &'a AggregateGuard<'a>,
        right_guard: &'a AggregateGuard<'a>,
    ) -> BodyAggregate<'a> {
        BodyAggregate {
            data: clingo_ast_body_aggregate {
                function: function as i32,
                elements: elements.as_ptr() as *const clingo_ast_body_aggregate_element_t,
                size: elements.len(),
                left_guard: &left_guard.data,
                right_guard: &right_guard.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn aggregate_function(&self) -> AggregateFunction {
        match self.data.function as u32 {
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
    pub fn elements(&self) -> &'a [BodyAggregateElement<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.elements as *const BodyAggregateElement,
                self.data.size,
            )
        }
    }
    pub fn left_guard(&self) -> &'a AggregateGuard<'a> {
        unsafe {
            (self.data.left_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard)
                .as_ref()
        }
        .unwrap()
    }
    pub fn right_guard(&self) -> &'a AggregateGuard<'a> {
        unsafe {
            (self.data.right_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard)
                .as_ref()
        }
        .unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct HeadAggregateElement<'a> {
    data: clingo_ast_head_aggregate_element,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for HeadAggregateElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HeadAggregateElement {{ tuple: {:?}, conditional_literal: {:?} }}",
            self.tuple(),
            self.conditional_literal()
        )
    }
}
impl<'a> HeadAggregateElement<'a> {
    pub fn new(
        tuple: &'a [Term<'a>],
        conditional_literal: ConditionalLiteral<'a>,
    ) -> HeadAggregateElement<'a> {
        HeadAggregateElement {
            data: clingo_ast_head_aggregate_element {
                tuple: tuple.as_ptr() as *const clingo_ast_term_t,
                tuple_size: tuple.len(),
                conditional_literal: conditional_literal.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn tuple(&self) -> &'a [Term<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.tuple as *const Term, self.data.tuple_size) }
    }
    pub fn conditional_literal(&self) -> &'a ConditionalLiteral<'a> {
        unsafe {
            (&self.data.conditional_literal as *const clingo_ast_conditional_literal
                as *const ConditionalLiteral)
                .as_ref()
        }
        .unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct HeadAggregate<'a> {
    data: clingo_ast_head_aggregate,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for HeadAggregate<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HeadAggregate {{ function: {:?} elements: {:?}, left_guard: {:?}, right_guard: {:?} }}",
            self.aggregate_function(),
            self.elements(),
            self.left_guard(),
            self.right_guard()
        )
    }
}
impl<'a> HeadAggregate<'a> {
    pub fn new(
        function: AggregateFunction,
        elements: &'a [HeadAggregateElement<'a>],
        left_guard: &'a AggregateGuard<'a>,
        right_guard: &'a AggregateGuard<'a>,
    ) -> HeadAggregate<'a> {
        HeadAggregate {
            data: clingo_ast_head_aggregate {
                function: function as i32,
                elements: elements.as_ptr() as *const clingo_ast_head_aggregate_element_t,
                size: elements.len(),
                left_guard: &left_guard.data,
                right_guard: &right_guard.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn aggregate_function(&self) -> AggregateFunction {
        match self.data.function as u32 {
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
            x => panic!("Failed to match clingo_ast_aggregate_function: {}.", x),
        }
    }
    pub fn elements(&self) -> &'a [HeadAggregateElement<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.elements as *const HeadAggregateElement,
                self.data.size,
            )
        }
    }
    pub fn left_guard(&self) -> &'a AggregateGuard<'a> {
        unsafe {
            (self.data.left_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard)
                .as_ref()
        }
        .unwrap()
    }
    pub fn right_guard(&self) -> &'a AggregateGuard<'a> {
        unsafe {
            (self.data.right_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard)
                .as_ref()
        }
        .unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct Disjunction<'a> {
    data: clingo_ast_disjunction,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Disjunction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Disjunction {{ elements: {:?} }}", self.elements())
    }
}
impl<'a> Disjunction<'a> {
    pub fn new(elements: &'a [ConditionalLiteral<'a>]) -> Disjunction<'a> {
        Disjunction {
            data: clingo_ast_disjunction {
                elements: elements.as_ptr() as *const clingo_ast_conditional_literal_t,
                size: elements.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn elements(&'a self) -> &'a [ConditionalLiteral<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.elements as *const ConditionalLiteral,
                self.data.size,
            )
        }
    }
}
#[derive(Copy, Clone)]
pub struct DisjointElement<'a> {
    data: clingo_ast_disjoint_element,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for DisjointElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DisjointElement {{ tuple: {:?} term: {:?} condition: {:?} }}",
            self.tuple(),
            self.term(),
            self.condition()
        )
    }
}
impl<'a> DisjointElement<'a> {
    pub fn new(
        tuple: &'a [Term<'a>],
        term: CspSumTerm<'a>,
        condition: &'a [Literal<'a>],
    ) -> DisjointElement<'a> {
        DisjointElement {
            data: clingo_ast_disjoint_element {
                location: Location::default(),
                tuple: tuple.as_ptr() as *const clingo_ast_term_t,
                tuple_size: tuple.len(),
                term: term.data,
                condition: condition.as_ptr() as *const clingo_ast_literal_t,
                condition_size: condition.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn location(&self) -> Location {
        Location(self.data.location)
    }
    pub fn tuple(&self) -> &'a [Term<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.tuple as *const Term, self.data.tuple_size) }
    }
    pub fn term(&self) -> &'a CspSumTerm<'a> {
        unsafe { (&self.data.term as *const clingo_ast_csp_sum_term as *const CspSumTerm).as_ref() }
            .unwrap()
    }
    pub fn condition(&self) -> &'a [Literal<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.condition as *const Literal,
                self.data.condition_size,
            )
        }
    }
}
#[derive(Copy, Clone)]
pub struct Disjoint<'a> {
    data: clingo_ast_disjoint,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Disjoint<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Disjoint {{ elements: {:?} }}", self.elements())
    }
}
impl<'a> Disjoint<'a> {
    pub fn new(elements: &'a [DisjointElement<'a>]) -> Disjoint<'a> {
        Disjoint {
            data: clingo_ast_disjoint {
                elements: elements.as_ptr() as *const clingo_ast_disjoint_element,
                size: elements.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn elements(&self) -> &'a [DisjointElement<'a>] {
        unsafe {
            std::slice::from_raw_parts(self.data.elements as *const DisjointElement, self.data.size)
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum TheoryTermType<'a> {
    Symbol(Symbol),
    Variable(&'a str),
    Tuple(&'a TheoryTermArray<'a>),
    List(&'a TheoryTermArray<'a>),
    Set(&'a TheoryTermArray<'a>),
    Function(&'a TheoryFunction<'a>),
    UnparsedTerm(&'a TheoryUnparsedTerm<'a>),
}
#[derive(Copy, Clone)]
pub struct TheoryTerm<'a> {
    data: clingo_ast_theory_term,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryTerm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.term_type() {
            TheoryTermType::Symbol(sym) => {
                let string = sym.to_string().unwrap();
                write!(f, "TheoryTerm {{ symbol: {} }}", string)
            }
            TheoryTermType::Variable(var) => write!(f, "TheoryTerm {{ variable: {:?} }}", var),
            TheoryTermType::Tuple(tuple) => write!(f, "TheoryTerm {{ tuple: {:?} }}", tuple),
            TheoryTermType::List(list) => write!(f, "TheoryTerm {{ list: {:?} }}", list),
            TheoryTermType::Set(set) => write!(f, "TheoryTerm {{ set: {:?} }}", set),
            TheoryTermType::Function(fun) => {
                write!(f, "TheoryTerm {{ theory_function: {:?} }}", fun)
            }
            TheoryTermType::UnparsedTerm(term) => {
                write!(f, "TheoryTerm {{ uparsed_term: {:?} }}", term)
            }
        }
    }
}
impl<'a> From<Symbol> for TheoryTerm<'a> {
    fn from(Symbol(symbol): Symbol) -> TheoryTerm<'a> {
        TheoryTerm {
            data: clingo_ast_theory_term {
                location: Location::default(),
                type_: clingo_ast_theory_term_type_clingo_ast_theory_term_type_symbol as i32,
                __bindgen_anon_1: clingo_ast_theory_term__bindgen_ty_1 { symbol },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a TheoryFunction<'a>> for TheoryTerm<'a> {
    fn from(fun: &'a TheoryFunction<'a>) -> Self {
        TheoryTerm {
            data: clingo_ast_theory_term {
                location: Location::default(),
                type_: clingo_ast_theory_term_type_clingo_ast_theory_term_type_function as i32,
                __bindgen_anon_1: clingo_ast_theory_term__bindgen_ty_1 {
                    function: &fun.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a TheoryUnparsedTerm<'a>> for TheoryTerm<'a> {
    fn from(term: &'a TheoryUnparsedTerm<'a>) -> TheoryTerm<'a> {
        TheoryTerm {
            data: clingo_ast_theory_term {
                location: Location::default(),
                type_: clingo_ast_theory_term_type_clingo_ast_theory_term_type_unparsed_term as i32,
                __bindgen_anon_1: clingo_ast_theory_term__bindgen_ty_1 {
                    unparsed_term: &term.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> TheoryTerm<'a> {
    pub fn variable(name: &'a str) -> Result<TheoryTerm<'a>, Error> {
        let variable = internalize_string(name)?;
        Ok(TheoryTerm {
            data: clingo_ast_theory_term {
                location: Location::default(),
                type_: clingo_ast_theory_term_type_clingo_ast_theory_term_type_variable as i32,
                __bindgen_anon_1: clingo_ast_theory_term__bindgen_ty_1 { variable },
            },
            _lifetime: PhantomData,
        })
    }

    pub fn tuple(tuple: &'a TheoryTermArray<'a>) -> TheoryTerm<'a> {
        TheoryTerm {
            data: clingo_ast_theory_term {
                location: Location::default(),
                type_: clingo_ast_theory_term_type_clingo_ast_theory_term_type_tuple as i32,
                __bindgen_anon_1: clingo_ast_theory_term__bindgen_ty_1 { tuple: &tuple.data },
            },
            _lifetime: PhantomData,
        }
    }
    pub fn list(list: &'a TheoryTermArray<'a>) -> TheoryTerm<'a> {
        TheoryTerm {
            data: clingo_ast_theory_term {
                location: Location::default(),
                type_: clingo_ast_theory_term_type_clingo_ast_theory_term_type_list as i32,
                __bindgen_anon_1: clingo_ast_theory_term__bindgen_ty_1 { list: &list.data },
            },
            _lifetime: PhantomData,
        }
    }
    pub fn set(set: &'a TheoryTermArray<'a>) -> TheoryTerm<'a> {
        TheoryTerm {
            data: clingo_ast_theory_term {
                location: Location::default(),
                type_: clingo_ast_theory_term_type_clingo_ast_theory_term_type_set as i32,
                __bindgen_anon_1: clingo_ast_theory_term__bindgen_ty_1 { set: &set.data },
            },
            _lifetime: PhantomData,
        }
    }
    pub fn term_type(&self) -> TheoryTermType {
        match self.data.type_ as u32 {
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_symbol => {
                TheoryTermType::Symbol(Symbol(unsafe { self.data.__bindgen_anon_1.symbol }))
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_variable => {
                TheoryTermType::Variable(
                    if unsafe { self.data.__bindgen_anon_1.variable.is_null() } {
                        ""
                    } else {
                        let c_str = unsafe { CStr::from_ptr(self.data.__bindgen_anon_1.variable) };
                        c_str.to_str().unwrap()
                    },
                )
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_tuple => TheoryTermType::Tuple(
                unsafe { (self.data.__bindgen_anon_1.tuple as *const TheoryTermArray).as_ref() }
                    .unwrap(),
            ),
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_list => TheoryTermType::List(
                unsafe { (self.data.__bindgen_anon_1.list as *const TheoryTermArray).as_ref() }
                    .unwrap(),
            ),
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_set => TheoryTermType::Set(
                unsafe { (self.data.__bindgen_anon_1.set as *const TheoryTermArray).as_ref() }
                    .unwrap(),
            ),
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_function => {
                TheoryTermType::Function(
                    unsafe {
                        (self.data.__bindgen_anon_1.function as *const TheoryFunction).as_ref()
                    }
                    .unwrap(),
                )
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_unparsed_term => {
                TheoryTermType::UnparsedTerm(
                    unsafe {
                        (self.data.__bindgen_anon_1.unparsed_term as *const TheoryUnparsedTerm)
                            .as_ref()
                    }
                    .unwrap(),
                )
            }
            x => panic!("Failed to match theory term type: {}!", x),
        }
    }
    // pub fn location(&self) -> Location {
    //     let term = TheoryTerm::into(*self);
    //     Location(term.location)
    // }
}
#[derive(Copy, Clone)]
pub struct TheoryTermArray<'a> {
    data: clingo_ast_theory_term_array,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryTermArray<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TheoryTermArray {{ terms: {:?} }}", self.terms())
    }
}
impl<'a> From<&'a [TheoryTerm<'a>]> for TheoryTermArray<'a> {
    fn from(terms: &'a [TheoryTerm<'a>]) -> TheoryTermArray<'a> {
        TheoryTermArray {
            data: clingo_ast_theory_term_array {
                terms: terms.as_ptr() as *const clingo_ast_theory_term,
                size: terms.len(),
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> TheoryTermArray<'a> {
    pub fn terms(&self) -> &'a [TheoryTerm<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.terms as *const TheoryTerm, self.data.size) }
    }
}
#[derive(Copy, Clone)]
pub struct TheoryFunction<'a> {
    data: clingo_ast_theory_function,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryFunction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().expect("Cant get function name!");
        write!(
            f,
            "TheoryFunction {{ name: {:?} arguments: {:?} }}",
            name,
            self.arguments()
        )
    }
}
impl<'a> TheoryFunction<'a> {
    pub fn new(
        name: &'a str,
        arguments: &'a [TheoryTerm<'a>],
    ) -> Result<TheoryFunction<'a>, Error> {
        let name = internalize_string(name)?;
        Ok(TheoryFunction {
            data: clingo_ast_theory_function {
                name,
                arguments: arguments.as_ptr() as *const clingo_ast_theory_term_t,
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
    pub fn arguments(&self) -> &'a [TheoryTerm<'a>] {
        unsafe {
            std::slice::from_raw_parts(self.data.arguments as *const TheoryTerm, self.data.size)
        }
    }
}
#[derive(Copy, Clone)]
pub struct TheoryUnparsedTermElement<'a> {
    data: clingo_ast_theory_unparsed_term_element,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryUnparsedTermElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operators = self.operators().unwrap();
        write!(
            f,
            "TheoryUnparsedTermElement {{ operators: {:?} term: {:?} }}",
            operators,
            self.term()
        )
    }
}
impl<'a> TheoryUnparsedTermElement<'a> {
    pub fn operators(&self) -> Result<Vec<&str>, Utf8Error> {
        let s1 = unsafe {
            std::slice::from_raw_parts(
                self.data.operators as *const ::std::os::raw::c_char,
                self.data.size,
            )
        };
        let mut akku = vec![];
        for char_ptr in s1.iter() {
            akku.push(unsafe { CStr::from_ptr(char_ptr) }.to_str()?);
        }
        Ok(akku)
    }
    pub fn term(&self) -> &'a TheoryTerm<'a> {
        unsafe { (&self.data.term as *const clingo_ast_theory_term as *const TheoryTerm).as_ref() }
            .unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct TheoryUnparsedTerm<'a> {
    data: clingo_ast_theory_unparsed_term,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryUnparsedTerm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TheoryUnparsedTerm {{ elements: {:?} }}",
            self.elements()
        )
    }
}
impl<'a> TheoryUnparsedTerm<'a> {
    pub fn new(elements: &'a [TheoryUnparsedTermElement<'a>]) -> TheoryUnparsedTerm<'a> {
        TheoryUnparsedTerm {
            data: clingo_ast_theory_unparsed_term {
                elements: elements.as_ptr() as *const clingo_ast_theory_unparsed_term_element_t,
                size: elements.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn elements(&self) -> &'a [TheoryUnparsedTermElement<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.elements as *const TheoryUnparsedTermElement,
                self.data.size,
            )
        }
    }
}
#[derive(Copy, Clone)]
pub struct TheoryAtomElement<'a> {
    data: clingo_ast_theory_atom_element,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryAtomElement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TheoryAtomElement {{ tuple: {:?} condition: {:?} }}",
            self.tuple(),
            self.condition()
        )
    }
}
impl<'a> TheoryAtomElement<'a> {
    pub fn new(tuple: &'a [TheoryTerm<'a>], condition: &'a [Literal<'a>]) -> TheoryAtomElement<'a> {
        TheoryAtomElement {
            data: clingo_ast_theory_atom_element {
                tuple: tuple.as_ptr() as *const clingo_ast_theory_term_t,
                tuple_size: tuple.len(),
                condition: condition.as_ptr() as *const clingo_ast_literal_t,
                condition_size: condition.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn tuple(&self) -> &'a [TheoryTerm<'a>] {
        unsafe {
            std::slice::from_raw_parts(self.data.tuple as *const TheoryTerm, self.data.tuple_size)
        }
    }
    pub fn condition(&self) -> &'a [Literal<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.condition as *const Literal,
                self.data.condition_size,
            )
        }
    }
}
#[derive(Copy, Clone)]
pub struct TheoryGuard<'a> {
    data: clingo_ast_theory_guard,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryGuard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.operator_name().unwrap();
        write!(
            f,
            "TheoryGuard {{ operator_name: {:?} term: {:?} }}",
            name,
            self.term()
        )
    }
}
impl<'a> TheoryGuard<'a> {
    pub fn new(operator_name: &'a str, term: TheoryTerm<'a>) -> Result<TheoryGuard<'a>, Error> {
        let operator_name = internalize_string(operator_name)?;
        Ok(TheoryGuard {
            data: clingo_ast_theory_guard {
                operator_name,
                term: term.data,
            },
            _lifetime: PhantomData,
        })
    }
    pub fn operator_name(&self) -> Result<&str, Utf8Error> {
        if self.data.operator_name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.data.operator_name) };
            c_str.to_str()
        }
    }
    pub fn term(&self) -> &'a TheoryTerm<'a> {
        unsafe { (&self.data.term as *const clingo_ast_theory_term as *const TheoryTerm).as_ref() }
            .unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct TheoryAtom<'a> {
    data: clingo_ast_theory_atom,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryAtom<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TheoryAtom {{ term: {:?} elements: {:?} guard: {:?} }}",
            self.term(),
            self.elements(),
            self.guard()
        )
    }
}
impl<'a> TheoryAtom<'a> {
    pub fn new(
        term: Term<'a>,
        elements: &'a [TheoryAtomElement<'a>],
        guard: &'a TheoryGuard<'a>,
    ) -> TheoryAtom<'a> {
        TheoryAtom {
            data: clingo_ast_theory_atom {
                term: term.data,
                elements: elements.as_ptr() as *const clingo_ast_theory_atom_element_t,
                size: elements.len(),
                guard: &guard.data as *const clingo_ast_theory_guard_t,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn term(&self) -> &'a Term<'a> {
        unsafe { (&self.data.term as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn elements(&self) -> &'a [TheoryAtomElement<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.elements as *const TheoryAtomElement,
                self.data.size,
            )
        }
    }
    pub fn guard(&self) -> &'a TheoryGuard<'a> {
        unsafe {
            (self.data.guard as *const clingo_ast_theory_guard as *const TheoryGuard).as_ref()
        }
        .unwrap()
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
