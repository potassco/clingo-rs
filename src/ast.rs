#![allow(clippy::needless_lifetimes)]
use crate::*;
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sign {
    /// For positive literals.
    NoSign = clingo_ast_sign_clingo_ast_sign_no_sign as isize,
    ///  For negative literals (prefix `not`s).
    Negation = clingo_ast_sign_clingo_ast_sign_negation as isize,
    /// For double negated literals (prefix `not not`).
    DoubleNegation = clingo_ast_sign_clingo_ast_sign_double_negation as isize,
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of comparison relations
pub enum ComparisonOperator {
    /// Operator `>`.
    GreaterThan =
        clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_than as isize,
    /// Operator `<`.
    LessThan = clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_than as isize,
    /// Operator `<=`.
    LessEqual = clingo_ast_comparison_operator_clingo_ast_comparison_operator_less_equal as isize,
    /// Operator `>=`.
    GreaterEqual =
        clingo_ast_comparison_operator_clingo_ast_comparison_operator_greater_equal as isize,
    /// Operator `!=`.
    NotEqual = clingo_ast_comparison_operator_clingo_ast_comparison_operator_not_equal as isize,
    /// Operator `==`.
    Equal = clingo_ast_comparison_operator_clingo_ast_comparison_operator_equal as isize,
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of unary operators.
pub enum UnaryOperator {
    /// Operator `-`.
    Minus = clingo_ast_unary_operator_clingo_ast_unary_operator_minus as isize,
    /// Operator `~`.
    Negation = clingo_ast_unary_operator_clingo_ast_unary_operator_negation as isize,
    /// Operator `|.|`.
    Absolute = clingo_ast_unary_operator_clingo_ast_unary_operator_absolute as isize,
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of binary operators.
pub enum BinaryOperator {
    /// Operator `^`.
    Xor = clingo_ast_binary_operator_clingo_ast_binary_operator_xor as isize,
    /// Operator `?`.
    Or = clingo_ast_binary_operator_clingo_ast_binary_operator_or as isize,
    /// Operator `&`.
    And = clingo_ast_binary_operator_clingo_ast_binary_operator_and as isize,
    /// Operator `+`.
    Plus = clingo_ast_binary_operator_clingo_ast_binary_operator_plus as isize,
    /// Operator `-`.
    Minus = clingo_ast_binary_operator_clingo_ast_binary_operator_minus as isize,
    /// Operator `*`.
    Multiplication = clingo_ast_binary_operator_clingo_ast_binary_operator_multiplication as isize,
    /// Operator `/`.
    Division = clingo_ast_binary_operator_clingo_ast_binary_operator_division as isize,
    /// Operator `\`.
    Modulo = clingo_ast_binary_operator_clingo_ast_binary_operator_modulo as isize,
    /// Operator `**`.
    Power = clingo_ast_binary_operator_clingo_ast_binary_operator_power as isize,
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of aggregate functions.
pub enum AggregateFunction {
    Count = clingo_ast_aggregate_function_clingo_ast_aggregate_function_count as isize,
    Sum = clingo_ast_aggregate_function_clingo_ast_aggregate_function_sum as isize,
    Sump = clingo_ast_aggregate_function_clingo_ast_aggregate_function_sump as isize,
    Min = clingo_ast_aggregate_function_clingo_ast_aggregate_function_min as isize,
    Max = clingo_ast_aggregate_function_clingo_ast_aggregate_function_max as isize,
}
#[derive(Debug, Copy, Clone)]
enum HeadLiteralType {
    Literal = clingo_ast_head_literal_type_clingo_ast_head_literal_type_literal as isize,
    Disjunction = clingo_ast_head_literal_type_clingo_ast_head_literal_type_disjunction as isize,
    Aggregate = clingo_ast_head_literal_type_clingo_ast_head_literal_type_aggregate as isize,
    HeadAggregate =
        clingo_ast_head_literal_type_clingo_ast_head_literal_type_head_aggregate as isize,
    TheoryAtom = clingo_ast_head_literal_type_clingo_ast_head_literal_type_theory_atom as isize,
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of script types.
pub enum ScriptType {
    /// For Lua scripts.
    Lua = clingo_ast_script_type_clingo_ast_script_type_lua as isize,
    /// For Python scripts.
    Python = clingo_ast_script_type_clingo_ast_script_type_python as isize,
}

#[derive(Debug, Copy, Clone)]
pub enum StatementType<'a> {
    Rule(&'a Rule<'a>),
    Const(&'a Definition<'a>),
    ShowSignature(&'a ShowSignature),
    ShowTerm(&'a ShowTerm<'a>),
    Minimize(&'a Minimize<'a>),
    Script(&'a Script),
    Program(&'a Program<'a>),
    External(&'a External<'a>),
    Edge(&'a Edge<'a>),
    Heuristic(&'a Heuristic<'a>),
    ProjectAtom(&'a Project<'a>),
    ProjectAtomSignature(&'a Signature),
    TheoryDefinition(&'a TheoryDefinition<'a>),
    Defined(&'a Defined),
}
/// Representation of a program statement.
pub struct Statement<'a> {
    pub(crate) data: clingo_ast_statement_t,
    _lifetime: PhantomData<&'a ()>,
}
impl<'a> From<&'a Edge<'a>> for Statement<'a> {
    fn from(edge: &'a Edge<'a>) -> Self {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_edge as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    edge: &edge.data as *const clingo_ast_edge,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a TheoryDefinition<'a>> for Statement<'a> {
    fn from(def: &'a TheoryDefinition<'a>) -> Self {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_theory_definition as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    theory_definition: &def.data as *const clingo_ast_theory_definition,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<Signature> for Statement<'a> {
    fn from(Signature(project_signature): Signature) -> Self {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_project_atom_signature
                    as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 { project_signature },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a Definition<'a>> for Statement<'a> {
    fn from(def: &'a Definition<'a>) -> Self {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_const as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    definition: &def.data as *const clingo_ast_definition,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a ShowTerm<'a>> for Statement<'a> {
    fn from(term: &'a ShowTerm<'a>) -> Self {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_show_term as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    show_term: &term.data as *const clingo_ast_show_term,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a ShowSignature> for Statement<'a> {
    fn from(sig: &'a ShowSignature) -> Self {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_show_signature as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    show_signature: &sig.data as *const clingo_ast_show_signature,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a Defined> for Statement<'a> {
    fn from(def: &'a Defined) -> Self {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_defined as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    defined: &def.data as *const clingo_ast_defined,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a Minimize<'a>> for Statement<'a> {
    fn from(min: &'a Minimize<'a>) -> Self {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_minimize as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    minimize: &min.data as *const clingo_ast_minimize,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl<'a> From<&'a Program<'a>> for Statement<'a> {
    fn from(prg: &'a Program<'a>) -> Self {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_program as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    program: &prg.data as *const clingo_ast_program,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
impl fmt::Debug for Statement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.statement_type() {
            StatementType::Rule(rule) => write!(f, "Statement {{ rule: {:?} }}", rule),
            StatementType::Const(def) => write!(f, "Statement {{ const: {:?} }}", def),
            StatementType::ShowSignature(sig) => {
                write!(f, "Statement {{ show_signature: {:?} }}", sig)
            }
            StatementType::ShowTerm(term) => write!(f, "Statement {{ show_term: {:?} }}", term),
            StatementType::Minimize(stm) => write!(f, "Statement {{ minimize: {:?} }}", stm),
            StatementType::Script(script) => write!(f, "Statement {{ script: {:?} }}", script),
            StatementType::Program(prg) => write!(f, "Statement {{ program: {:?} }}", prg),
            StatementType::External(ext) => write!(f, "Statement {{ external: {:?} }}", ext),
            StatementType::Edge(edge) => write!(f, "Statement {{ edge: {:?} }}", edge),
            StatementType::Heuristic(heu) => write!(f, "Statement {{ heuristic: {:?} }}", heu),
            StatementType::ProjectAtom(atom) => {
                write!(f, "Statement {{ project_atom: {:?} }}", atom)
            }
            StatementType::ProjectAtomSignature(sig) => {
                write!(f, "Statement {{ project_atom_signature: {:?} }}", sig)
            }
            StatementType::TheoryDefinition(def) => {
                write!(f, "Statement {{ theory_definition: {:?} }}", def)
            }
            StatementType::Defined(def) => write!(f, "Statement {{ defined: {:?} }}", def),
        }
    }
}
impl<'a> Statement<'a> {
    /// Get the type of the statement.
    pub fn statement_type(&self) -> StatementType {
        match self.data.type_ as u32 {
            clingo_ast_statement_type_clingo_ast_statement_type_rule => StatementType::Rule(
                unsafe { (self.data.__bindgen_anon_1.rule as *const Rule).as_ref() }.unwrap(),
            ),
            clingo_ast_statement_type_clingo_ast_statement_type_const => StatementType::Const(
                unsafe { (self.data.__bindgen_anon_1.definition as *const Definition).as_ref() }
                    .unwrap(),
            ),
            clingo_ast_statement_type_clingo_ast_statement_type_show_signature => {
                StatementType::ShowSignature(
                    unsafe {
                        (self.data.__bindgen_anon_1.show_signature as *const ShowSignature).as_ref()
                    }
                    .unwrap(),
                )
            }
            clingo_ast_statement_type_clingo_ast_statement_type_show_term => {
                StatementType::ShowTerm(
                    unsafe { (self.data.__bindgen_anon_1.show_term as *const ShowTerm).as_ref() }
                        .unwrap(),
                )
            }
            clingo_ast_statement_type_clingo_ast_statement_type_minimize => {
                StatementType::Minimize(
                    unsafe { (self.data.__bindgen_anon_1.minimize as *const Minimize).as_ref() }
                        .unwrap(),
                )
            }
            clingo_ast_statement_type_clingo_ast_statement_type_script => StatementType::Script(
                unsafe { (self.data.__bindgen_anon_1.script as *const Script).as_ref() }.unwrap(),
            ),
            clingo_ast_statement_type_clingo_ast_statement_type_program => StatementType::Program(
                unsafe { (self.data.__bindgen_anon_1.program as *const Program).as_ref() }.unwrap(),
            ),
            clingo_ast_statement_type_clingo_ast_statement_type_external => {
                StatementType::External(
                    unsafe { (self.data.__bindgen_anon_1.external as *const External).as_ref() }
                        .unwrap(),
                )
            }
            clingo_ast_statement_type_clingo_ast_statement_type_edge => StatementType::Edge(
                unsafe { (self.data.__bindgen_anon_1.edge as *const Edge).as_ref() }.unwrap(),
            ),
            clingo_ast_statement_type_clingo_ast_statement_type_heuristic => {
                StatementType::Heuristic(
                    unsafe { (self.data.__bindgen_anon_1.heuristic as *const Heuristic).as_ref() }
                        .unwrap(),
                )
            }
            clingo_ast_statement_type_clingo_ast_statement_type_project_atom => {
                StatementType::ProjectAtom(
                    unsafe { (self.data.__bindgen_anon_1.project_atom as *const Project).as_ref() }
                        .unwrap(),
                )
            }
            clingo_ast_statement_type_clingo_ast_statement_type_project_atom_signature => {
                StatementType::ProjectAtomSignature(
                    unsafe {
                        (&self.data.__bindgen_anon_1.project_signature as *const clingo_signature_t
                            as *const Signature)
                            .as_ref()
                    }
                    .unwrap(),
                )
            }
            clingo_ast_statement_type_clingo_ast_statement_type_theory_definition => {
                StatementType::TheoryDefinition(
                    unsafe {
                        (self.data.__bindgen_anon_1.theory_definition as *const TheoryDefinition)
                            .as_ref()
                    }
                    .unwrap(),
                )
            }
            clingo_ast_statement_type_clingo_ast_statement_type_defined => StatementType::Defined(
                unsafe { (self.data.__bindgen_anon_1.defined as *const Defined).as_ref() }.unwrap(),
            ),
            x => panic!("Failed to match clingo_ast_statement_type: {}", x),
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
            x => panic!("Failed to match clingo_ast_head_literal_type: {}!", x),
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
    pub fn ast_statement(&'a self) -> Statement<'a> {
        Statement {
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
impl fmt::Debug for Definition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().unwrap();
        write!(
            f,
            "Definition {{ head: {:?}, value: {:?} is_default: {} }}",
            name,
            self.value(),
            self.is_default()
        )
    }
}
impl<'a> Definition<'a> {
    pub fn new(
        name: &str,
        value: Term<'a>,
        is_default: bool,
    ) -> Result<Definition<'a>, ClingoError> {
        let name = internalize_string(name)?;
        Ok(Definition {
            data: clingo_ast_definition {
                name,
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
    pub fn value(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.value as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn is_default(&self) -> bool {
        self.data.is_default
    }
}
#[derive(Debug, Copy, Clone)]
pub struct ShowSignature {
    data: clingo_ast_show_signature,
}
impl ShowSignature {
    pub fn new(Signature(signature): Signature, csp: bool) -> ShowSignature {
        ShowSignature {
            data: clingo_ast_show_signature { signature, csp },
        }
    }
    pub fn signature(&self) -> &Signature {
        unsafe { (&self.data.signature as *const clingo_signature_t as *const Signature).as_ref() }
            .unwrap()
    }
    pub fn csp(&self) -> bool {
        self.data.csp
    }
}
#[derive(Copy, Clone)]
pub struct ShowTerm<'a> {
    data: clingo_ast_show_term,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for ShowTerm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ShowTerm {{ term: {:?} body: {:?} csp: {} }}",
            self.term(),
            self.body(),
            self.csp()
        )
    }
}
impl<'a> ShowTerm<'a> {
    pub fn new(term: Term<'a>, body: &'a [BodyLiteral<'a>], csp: bool) -> ShowTerm<'a> {
        ShowTerm {
            data: clingo_ast_show_term {
                term: term.data,
                body: body.as_ptr() as *const clingo_ast_body_literal_t,
                size: body.len(),
                csp,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn term(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.term as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn body(&self) -> &'a [BodyLiteral<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.size) }
    }
    pub fn csp(&self) -> bool {
        self.data.csp
    }
}
#[derive(Copy, Clone)]
pub struct Defined {
    data: clingo_ast_defined,
}
impl fmt::Debug for Defined {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Defined {{ signature: {:?} }}", self.signature())
    }
}
impl Defined {
    pub fn new(Signature(signature): Signature) -> Defined {
        Defined {
            data: clingo_ast_defined { signature },
        }
    }
    pub fn signature(&self) -> &Signature {
        unsafe { (&self.data.signature as *const clingo_signature_t as *const Signature).as_ref() }
            .unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct Minimize<'a> {
    data: clingo_ast_minimize,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Minimize<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Minimize {{ weight: {:?} priority: {:?} tuple: {:?} body: {:?} }}",
            self.weight(),
            self.priority(),
            self.tuple(),
            self.body()
        )
    }
}
impl<'a> Minimize<'a> {
    pub fn new(
        weight: Term,
        priority: Term,
        tuple: &'a [Term<'a>],
        body: &'a [BodyLiteral<'a>],
    ) -> Minimize<'a> {
        Minimize {
            data: clingo_ast_minimize {
                weight: weight.data,
                priority: priority.data,
                tuple: tuple.as_ptr() as *const clingo_ast_term_t,
                tuple_size: tuple.len(),
                body: body.as_ptr() as *const clingo_ast_body_literal_t,
                body_size: body.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn weight(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.weight as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn priority(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.priority as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn tuple(&self) -> &'a [Term<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.tuple as *const Term, self.data.tuple_size) }
    }
    pub fn body(&self) -> &'a [BodyLiteral<'a>] {
        unsafe {
            std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.body_size)
        }
    }
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
    // pub fn code(&self) -> Result<&str, Utf8Error> {
    //     if self.0.code.is_null() {
    //         Ok("")
    //     } else {
    //         let c_str = unsafe { CStr::from_ptr(self.0.code) };
    //         c_str.to_str()
    //     }
    // }
}
#[derive(Copy, Clone)]
pub struct Program<'a> {
    data: clingo_ast_program,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for Program<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().unwrap();
        write!(
            f,
            "Program {{ name: {:?} parameters: {:?} }}",
            name,
            self.parameters(),
        )
    }
}
impl<'a> Program<'a> {
    pub fn new(name: &str, parameters: &'a [Id]) -> Result<Program<'a>, ClingoError> {
        let name = internalize_string(name)?;
        Ok(Program {
            data: clingo_ast_program {
                name,
                parameters: parameters.as_ptr() as *const clingo_ast_id,
                size: parameters.len(),
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
    pub fn parameters(&self) -> &'a [Id] {
        unsafe { std::slice::from_raw_parts(self.data.parameters as *const Id, self.data.size) }
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
    pub fn sign(&self) -> Sign {
        match self.data.sign as u32 {
            clingo_ast_sign_clingo_ast_sign_double_negation => Sign::DoubleNegation,
            clingo_ast_sign_clingo_ast_sign_negation => Sign::Negation,
            clingo_ast_sign_clingo_ast_sign_no_sign => Sign::NoSign,
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
        write!(
            f,
            "External {{ atom: {:?}, body: {:?} }}",
            self.atom(),
            self.body()
        )
    }
}
impl<'a> External<'a> {
    /// Create an external atom default initialization with false
    pub fn new(term: Term<'a>, body: &'a [BodyLiteral<'a>]) -> External<'a> {
        let sym = Symbol::create_id("false", true).unwrap();
        let atom = Term::from(sym);
        External {
            data: clingo_ast_external {
                atom: term.data,
                body: body.as_ptr() as *const clingo_ast_body_literal_t,
                size: body.len(),
                type_: atom.data,
            },
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
    pub fn atom(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.atom as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn body(&self) -> &'a [BodyLiteral<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.size) }
    }
    /// Create a statement for the external.
    pub fn ast_statement(&'a self) -> Statement<'a> {
        Statement {
            data: clingo_ast_statement_t {
                location: Location::default(),
                type_: clingo_ast_statement_type_clingo_ast_statement_type_external as i32,
                __bindgen_anon_1: clingo_ast_statement__bindgen_ty_1 {
                    external: &self.data,
                },
            },
            _lifetime: PhantomData,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Edge<'a> {
    data: clingo_ast_edge,
    _lifetime: PhantomData<&'a u32>,
}
impl fmt::Debug for Edge<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Edge {{ u: {:?} v: {:?} body: {:?} }}",
            self.u(),
            self.v(),
            self.body()
        )
    }
}
impl<'a> Edge<'a> {
    /// Create an edge
    pub fn new(u: Term<'a>, v: Term<'a>, body: &'a [BodyLiteral<'a>]) -> Edge<'a> {
        Edge {
            data: clingo_ast_edge {
                u: u.data,
                v: v.data,
                body: body.as_ptr() as *const clingo_ast_body_literal_t,
                size: body.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn u(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.u as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn v(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.v as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn body(&self) -> &'a [BodyLiteral<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.size) }
    }
}
#[derive(Copy, Clone)]
pub struct Heuristic<'a> {
    data: clingo_ast_heuristic,
    _lifetime: PhantomData<&'a u32>,
}
impl fmt::Debug for Heuristic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Heuristic {{ atom: {:?} body: {:?} bias: {:?} priority: {:?} modifier: {:?} }}",
            self.atom(),
            self.body(),
            self.bias(),
            self.priority(),
            self.modifier(),
        )
    }
}
impl<'a> Heuristic<'a> {
    // Create an heuristic
    pub fn new(
        atom: Term<'a>,
        body: &'a [BodyLiteral<'a>],
        bias: Term<'a>,
        priority: Term<'a>,
        modifier: Term<'a>,
    ) -> Heuristic<'a> {
        Heuristic {
            data: clingo_ast_heuristic {
                atom: atom.data,
                body: body.as_ptr() as *const clingo_ast_body_literal_t,
                size: body.len(),
                bias: bias.data,
                priority: priority.data,
                modifier: modifier.data,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn atom(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.atom as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn body(&self) -> &'a [BodyLiteral<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.size) }
    }
    pub fn bias(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.bias as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn priority(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.priority as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn modifier(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.modifier as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
}
#[derive(Copy, Clone)]
pub struct Project<'a> {
    data: clingo_ast_project,
    _lifetime: PhantomData<&'a u32>,
}
impl fmt::Debug for Project<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Project {{ atom: {:?} body: {:?} }}",
            self.atom(),
            self.body()
        )
    }
}
impl<'a> Project<'a> {
    // Create a project
    pub fn new(atom: Term<'a>, body: &'a [BodyLiteral<'a>]) -> Project<'a> {
        Project {
            data: clingo_ast_project {
                atom: atom.data,
                body: body.as_ptr() as *const clingo_ast_body_literal_t,
                size: body.len(),
            },
            _lifetime: PhantomData,
        }
    }
    pub fn atom(&'a self) -> &'a Term<'a> {
        unsafe { (&self.data.atom as *const clingo_ast_term as *const Term).as_ref() }.unwrap()
    }
    pub fn body(&self) -> &'a [BodyLiteral<'a>] {
        unsafe { std::slice::from_raw_parts(self.data.body as *const BodyLiteral, self.data.size) }
    }
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
                write!(f, "Term {{ symbol: {} }}", sym)
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
    pub fn variable(name: &str) -> Result<Term<'a>, ClingoError> {
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
    pub fn sign(&self) -> Sign {
        match self.data.sign as u32 {
            clingo_ast_sign_clingo_ast_sign_double_negation => Sign::DoubleNegation,
            clingo_ast_sign_clingo_ast_sign_negation => Sign::Negation,
            clingo_ast_sign_clingo_ast_sign_no_sign => Sign::NoSign,
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
    pub fn new(name: &str, arguments: &'a [Term<'a>]) -> Result<Function<'a>, ClingoError> {
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
        left_guard: Option<&'a AggregateGuard<'a>>,
        right_guard: Option<&'a AggregateGuard<'a>>,
    ) -> Aggregate<'a> {
        let left_guard = match &left_guard {
            Some(left_guard) => &left_guard.data,
            None => std::ptr::null(),
        };
        let right_guard = match &right_guard {
            Some(right_guard) => &right_guard.data,
            None => std::ptr::null(),
        };
        Aggregate {
            data: clingo_ast_aggregate {
                elements: elements.as_ptr() as *const clingo_ast_conditional_literal_t,
                size: elements.len(),
                left_guard,
                right_guard,
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
    pub fn left_guard(&self) -> Option<&'a AggregateGuard<'a>> {
        let pointer =
            self.data.left_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { pointer.as_ref() }.unwrap())
        }
    }
    pub fn right_guard(&self) -> Option<&'a AggregateGuard<'a>> {
        let pointer =
            self.data.right_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { pointer.as_ref() }.unwrap())
        }
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
        left_guard: Option<&'a AggregateGuard<'a>>,
        right_guard: Option<&'a AggregateGuard<'a>>,
    ) -> BodyAggregate<'a> {
        let left_guard = match &left_guard {
            Some(left_guard) => &left_guard.data,
            None => std::ptr::null(),
        };
        let right_guard = match &right_guard {
            Some(right_guard) => &right_guard.data,
            None => std::ptr::null(),
        };
        BodyAggregate {
            data: clingo_ast_body_aggregate {
                function: function as i32,
                elements: elements.as_ptr() as *const clingo_ast_body_aggregate_element_t,
                size: elements.len(),
                left_guard,
                right_guard,
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
    pub fn left_guard(&self) -> Option<&'a AggregateGuard<'a>> {
        let pointer =
            self.data.left_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { pointer.as_ref() }.unwrap())
        }
    }
    pub fn right_guard(&self) -> Option<&'a AggregateGuard<'a>> {
        let pointer =
            self.data.right_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { pointer.as_ref() }.unwrap())
        }
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
        left_guard: Option<&'a AggregateGuard<'a>>,
        right_guard: Option<&'a AggregateGuard<'a>>,
    ) -> HeadAggregate<'a> {
        let left_guard = match &left_guard {
            Some(left_guard) => &left_guard.data,
            None => std::ptr::null(),
        };
        let right_guard = match &right_guard {
            Some(right_guard) => &right_guard.data,
            None => std::ptr::null(),
        };
        HeadAggregate {
            data: clingo_ast_head_aggregate {
                function: function as i32,
                elements: elements.as_ptr() as *const clingo_ast_head_aggregate_element_t,
                size: elements.len(),
                left_guard,
                right_guard,
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
    pub fn left_guard(&self) -> Option<&'a AggregateGuard<'a>> {
        let pointer =
            self.data.left_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { pointer.as_ref() }.unwrap())
        }
    }
    pub fn right_guard(&self) -> Option<&'a AggregateGuard<'a>> {
        let pointer =
            self.data.right_guard as *const clingo_ast_aggregate_guard as *const AggregateGuard;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { pointer.as_ref() }.unwrap())
        }
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
pub enum TheoryTermSequenceType {
    /// For theory tuples `(t1,...,tn)`.
    Tuple = clingo_ast_theory_sequence_type_clingo_ast_theory_sequence_type_tuple as isize,
    /// For theory lists `[t1,...,tn]`.
    List = clingo_ast_theory_sequence_type_clingo_ast_theory_sequence_type_list as isize,
    /// for theory sets `{t1,...,tn}`.
    Set = clingo_ast_theory_sequence_type_clingo_ast_theory_sequence_type_set as isize,
}
#[derive(Debug, Copy, Clone)]
pub enum TheoryTermSequence<'a> {
    /// Theory tuples `(t1,...,tn)`.
    Tuple(&'a TheoryTermArray<'a>),
    /// Theory lists `[t1,...,tn]`.
    List(&'a TheoryTermArray<'a>),
    /// Theory sets `{t1,...,tn}`.
    Set(&'a TheoryTermArray<'a>),
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
                write!(f, "TheoryTerm {{ symbol: {} }}", sym)
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
    pub fn variable(name: &str) -> Result<TheoryTerm<'a>, ClingoError> {
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
        name: &str,
        arguments: &'a [TheoryTerm<'a>],
    ) -> Result<TheoryFunction<'a>, ClingoError> {
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
    pub fn new(operator_name: &str, term: TheoryTerm<'a>) -> Result<TheoryGuard<'a>, ClingoError> {
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
        guard: Option<&'a TheoryGuard<'a>>,
    ) -> TheoryAtom<'a> {
        let guard = match &guard {
            Some(guard) => &guard.data,
            None => std::ptr::null(),
        };
        TheoryAtom {
            data: clingo_ast_theory_atom {
                term: term.data,
                elements: elements.as_ptr() as *const clingo_ast_theory_atom_element_t,
                size: elements.len(),
                guard,
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
    pub fn guard(&self) -> Option<&'a TheoryGuard<'a>> {
        let pointer = self.data.guard as *const clingo_ast_theory_guard as *const TheoryGuard;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { pointer.as_ref() }.unwrap())
        }
    }
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of theory operators.
pub enum TheoryOperatorType {
    // An unary theory operator.
    Unary = clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_unary as isize,
    /// A left associative binary operator.
    BinaryLeft =
        clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_left as isize,
    /// A right associative binary operator.
    BinaryRight =
        clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_right as isize,
}
#[derive(Copy, Clone)]
pub struct TheoryOperatorDefinition<'a> {
    data: clingo_ast_theory_operator_definition,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryOperatorDefinition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().unwrap();
        write!(
            f,
            "TheoryOperatorDefinition {{ name: {:?} priority: {:?} type: {:?} }}",
            name,
            self.priority(),
            self.operator_type()
        )
    }
}
impl<'a> TheoryOperatorDefinition<'a> {
    pub fn new(
        name: &str,
        priority: u32,
        operator_type: TheoryOperatorType,
    ) -> TheoryOperatorDefinition {
        let name = internalize_string(name).unwrap();
        TheoryOperatorDefinition {
            data: clingo_ast_theory_operator_definition {
                location: Location::default(),
                name,
                priority,
                type_: operator_type as i32,
            },
            _lifetime: PhantomData,
        }
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.data.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.data.name) };
            c_str.to_str()
        }
    }
    pub fn priority(&self) -> u32 {
        self.data.priority
    }
    /// Get the type of the operator.
    pub fn operator_type(&self) -> TheoryOperatorType {
        match self.data.type_ as u32 {
            clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_unary => {
                TheoryOperatorType::Unary
            }
            clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_left => {
                TheoryOperatorType::BinaryLeft
            }
            clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_right => {
                TheoryOperatorType::BinaryRight
            }
            x => panic!("Failed to match clingo_ast_theory_operator_type: {} ", x),
        }
    }
}
#[derive(Copy, Clone)]
pub struct TheoryTermDefinition<'a> {
    data: clingo_ast_theory_term_definition,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryTermDefinition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().unwrap();
        write!(
            f,
            "TheoryTermDefinition {{ name: {:?} operators: {:?} }}",
            name,
            self.operators()
        )
    }
}
impl<'a> TheoryTermDefinition<'a> {
    pub fn new(
        name: &str,
        operators: &'a [TheoryOperatorDefinition<'a>],
    ) -> Result<TheoryTermDefinition<'a>, ClingoError> {
        let name = internalize_string(name)?;
        Ok(TheoryTermDefinition {
            data: clingo_ast_theory_term_definition {
                location: Location::default(),
                name,
                operators: operators.as_ptr() as *const clingo_ast_theory_operator_definition_t,
                size: operators.len(),
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
    pub fn operators(&self) -> &'a [TheoryOperatorDefinition<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.operators as *const TheoryOperatorDefinition,
                self.data.size,
            )
        }
    }
}
pub struct TheoryGuardDefinition<'a> {
    data: clingo_ast_theory_guard_definition,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryGuardDefinition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let term = self.term().unwrap();
        let operators = self.operators().unwrap();
        write!(
            f,
            "TheoryGuardDefinition {{ term: {:?} operators: {:?} }}",
            term, operators
        )
    }
}
impl<'a> TheoryGuardDefinition<'a> {
    pub fn new(
        term: &str,
        operators: &'a [*const c_char],
    ) -> Result<TheoryGuardDefinition<'a>, ClingoError> {
        let term = internalize_string(term)?;
        Ok(TheoryGuardDefinition {
            data: clingo_ast_theory_guard_definition {
                term,
                operators: operators.as_ptr() as *const *const c_char,
                size: operators.len(),
            },
            _lifetime: PhantomData,
        })
    }
    pub fn term(&self) -> Result<&str, Utf8Error> {
        if self.data.term.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.data.term) };
            c_str.to_str()
        }
    }
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
}
#[derive(Debug, Copy, Clone)]
/// Enumeration of the theory atom types.
pub enum TheoryAtomType {
    /// For theory atoms that can appear in the head.
    Head =
        clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_head as isize,
    /// For theory atoms that can appear in the body.
    Body =
        clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_body as isize,
    /// For theory atoms that can appear in both head and body.
    Any =
        clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_any as isize,
    /// For theory atoms that must not have a body.
    Directive =
        clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_directive
            as isize,
}
#[derive(Copy, Clone)]
pub struct TheoryAtomDefinition<'a> {
    data: clingo_ast_theory_atom_definition,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryAtomDefinition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().unwrap();
        write!(
            f,
            "TheoryAtomDefinition {{ type: {:?} name: {:?} arity: {:?} elements: {:?} guard: {:?} }}",
            self.atom_type(),
            name,
            self.arity(),
            self.elements(),
            self.guard(),
        )
    }
}
impl<'a> TheoryAtomDefinition<'a> {
    pub fn new(
        name: &str,
        atom_type: TheoryAtomType,
        arity: u32,
        elements: &str,
        guard: Option<&'a TheoryGuardDefinition<'a>>,
    ) -> Result<TheoryAtomDefinition<'a>, ClingoError> {
        let name = internalize_string(name)?;
        let elements = internalize_string(elements)?;
        let guard = match &guard {
            Some(guard) => &guard.data,
            None => std::ptr::null(),
        };
        Ok(TheoryAtomDefinition {
            data: clingo_ast_theory_atom_definition_t {
                location: Location::default(),
                type_: atom_type as i32,
                name,
                arity,
                elements,
                guard,
            },
            _lifetime: PhantomData,
        })
    }
    fn atom_type(&self) -> TheoryAtomType {
        match self.data.type_ as u32 {
            clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_head => {
                TheoryAtomType::Head
            }
            clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_body => {
                TheoryAtomType::Body
            }
            clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_any => {
                TheoryAtomType::Any
            }
            clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_directive => {
                TheoryAtomType::Directive
            }
            x => panic!(
                "Failed to match clingo_ast_theory_atom_definition_type: {}.",
                x
            ),
        }
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.data.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.data.name) };
            c_str.to_str()
        }
    }
    pub fn arity(&self) -> u32 {
        self.data.arity
    }
    pub fn elements(&self) -> Result<&str, Utf8Error> {
        if self.data.elements.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.data.elements) };
            c_str.to_str()
        }
    }
    pub fn guard(&self) -> Option<&'a TheoryGuardDefinition<'a>> {
        let pointer = self.data.guard as *const clingo_ast_theory_guard_definition
            as *const TheoryGuardDefinition;
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { pointer.as_ref() }.unwrap())
        }
    }
}
#[derive(Copy, Clone)]
pub struct TheoryDefinition<'a> {
    data: clingo_ast_theory_definition,
    _lifetime: PhantomData<&'a ()>,
}
impl fmt::Debug for TheoryDefinition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name().unwrap();
        write!(
            f,
            "TheoryDefinition {{ name: {:?} terms: {:?} atoms: {:?} }}",
            name,
            self.terms(),
            self.atoms()
        )
    }
}
impl<'a> TheoryDefinition<'a> {
    pub fn new(
        name: &str,
        terms: &'a [TheoryTermDefinition<'a>],
        atoms: &'a [TheoryAtomDefinition<'a>],
    ) -> Result<TheoryDefinition<'a>, ClingoError> {
        let name = internalize_string(name)?;
        Ok(TheoryDefinition {
            data: clingo_ast_theory_definition {
                name,
                terms: terms.as_ptr() as *const clingo_ast_theory_term_definition_t,
                terms_size: terms.len(),
                atoms: atoms.as_ptr() as *const clingo_ast_theory_atom_definition_t,
                atoms_size: atoms.len(),
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
    pub fn terms(&self) -> &'a [TheoryTermDefinition<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.terms as *const TheoryTermDefinition,
                self.data.terms_size,
            )
        }
    }
    pub fn atoms(&self) -> &'a [TheoryAtomDefinition<'a>] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.atoms as *const TheoryAtomDefinition,
                self.data.atoms_size,
            )
        }
    }
}
/// Object to build non-ground programs.
pub struct ProgramBuilder<'a> {
    pub(crate) theref: &'a mut clingo_program_builder_t,
}
impl<'a> ProgramBuilder<'a> {
    /// Get an object to add non-ground directives to the program.
    pub fn from(ctl: &'a mut Control) -> Result<ProgramBuilder<'a>, ClingoError> {
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
    pub fn add(&mut self, stm: &'a Statement<'a>) -> Result<(), ClingoError> {
        if !unsafe { clingo_program_builder_add(self.theref, &stm.data) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_program_builder_add() failed",
            ));
        }
        Ok(())
    }

    // for ASTv2
    // extern "C" {
    //     #[doc = "! Adds a statement to the program."]
    //     #[doc = "!"]
    //     #[doc = "! @attention @ref clingo_program_builder_begin() must be called before adding statements and @ref clingo_program_builder_end() must be called after all statements have been added."]
    //     #[doc = "! @param[in] builder the target program builder"]
    //     #[doc = "! @param[in] ast the AST node to add"]
    //     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
    //     #[doc = "! - ::clingo_error_runtime for statements of invalid form or AST nodes that do not represent statements"]
    //     #[doc = "! - ::clingo_error_bad_alloc"]
    //     pub fn clingo_program_builder_add_ast(
    //         builder: *mut clingo_program_builder_t,
    //         ast: *mut clingo_ast_t,
    //     ) -> bool;

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
    Id = clingo_ast_type_clingo_ast_type_id as isize,
    Variable = clingo_ast_type_clingo_ast_type_variable as isize,
    SymbolicTerm = clingo_ast_type_clingo_ast_type_symbolic_term as isize,
    UnaryOperation = clingo_ast_type_clingo_ast_type_unary_operation as isize,
    BinaryOperation = clingo_ast_type_clingo_ast_type_binary_operation as isize,
    Interval = clingo_ast_type_clingo_ast_type_interval as isize,
    Function = clingo_ast_type_clingo_ast_type_function as isize,
    Pool = clingo_ast_type_clingo_ast_type_pool as isize,
    CspProduct = clingo_ast_type_clingo_ast_type_csp_product as isize,
    CspSum = clingo_ast_type_clingo_ast_type_csp_sum as isize,
    CspGuard = clingo_ast_type_clingo_ast_type_csp_guard as isize,
    BooleanConstant = clingo_ast_type_clingo_ast_type_boolean_constant as isize,
    SymbolicAtom = clingo_ast_type_clingo_ast_type_symbolic_atom as isize,
    Comparison = clingo_ast_type_clingo_ast_type_comparison as isize,
    CspLiteral = clingo_ast_type_clingo_ast_type_csp_literal as isize,
    AggregateGuard = clingo_ast_type_clingo_ast_type_aggregate_guard as isize,
    ConditionalLiteral = clingo_ast_type_clingo_ast_type_conditional_literal as isize,
    Aggregate = clingo_ast_type_clingo_ast_type_aggregate as isize,
    BodyAggregateElement = clingo_ast_type_clingo_ast_type_body_aggregate_element as isize,
    BodyAggregate = clingo_ast_type_clingo_ast_type_body_aggregate as isize,
    HeadAggregateElement = clingo_ast_type_clingo_ast_type_head_aggregate_element as isize,
    HeadAggregate = clingo_ast_type_clingo_ast_type_head_aggregate as isize,
    Disjunction = clingo_ast_type_clingo_ast_type_disjunction as isize,
    DisjointElement = clingo_ast_type_clingo_ast_type_disjoint_element as isize,
    Disjoint = clingo_ast_type_clingo_ast_type_disjoint as isize,
    TheorySequence = clingo_ast_type_clingo_ast_type_theory_sequence as isize,
    TheoryFunction = clingo_ast_type_clingo_ast_type_theory_function as isize,
    TheoryUnparsedTermElement =
        clingo_ast_type_clingo_ast_type_theory_unparsed_term_element as isize,
    TheoryUnparsedTerm = clingo_ast_type_clingo_ast_type_theory_unparsed_term as isize,
    TheoryGuard = clingo_ast_type_clingo_ast_type_theory_guard as isize,
    TheoryAtomElement = clingo_ast_type_clingo_ast_type_theory_atom_element as isize,
    TheoryAtom = clingo_ast_type_clingo_ast_type_theory_atom as isize,
    Literal = clingo_ast_type_clingo_ast_type_literal as isize,
    TheoryOperatorDefinition = clingo_ast_type_clingo_ast_type_theory_operator_definition as isize,
    TheoryTermDefinition = clingo_ast_type_clingo_ast_type_theory_term_definition as isize,
    TheoryGuardDefinition = clingo_ast_type_clingo_ast_type_theory_guard_definition as isize,
    TheoryAtomDefinition = clingo_ast_type_clingo_ast_type_theory_atom_definition as isize,
    Rule = clingo_ast_type_clingo_ast_type_rule as isize,
    Definition = clingo_ast_type_clingo_ast_type_definition as isize,
    ShowSignature = clingo_ast_type_clingo_ast_type_show_signature as isize,
    ShowTerm = clingo_ast_type_clingo_ast_type_show_term as isize,
    Minimize = clingo_ast_type_clingo_ast_type_minimize as isize,
    Script = clingo_ast_type_clingo_ast_type_script as isize,
    Program = clingo_ast_type_clingo_ast_type_program as isize,
    External = clingo_ast_type_clingo_ast_type_external as isize,
    Edge = clingo_ast_type_clingo_ast_type_edge as isize,
    Heuristic = clingo_ast_type_clingo_ast_type_heuristic as isize,
    ProjectAtom = clingo_ast_type_clingo_ast_type_project_atom as isize,
    ProjectAtomSignature = clingo_ast_type_clingo_ast_type_project_signature as isize,
    Defined = clingo_ast_type_clingo_ast_type_defined as isize,
    TheoryDefinition = clingo_ast_type_clingo_ast_type_theory_definition as isize,
}

#[derive(Debug, Copy, Clone)]
/// Enumeration of attributes types used by the AST.
pub enum AstAttributeType {
    /// For an attribute of type `int`.
    Number = clingo_ast_attribute_type_clingo_ast_attribute_type_number as isize,
    /// For an attribute of type `clingo_ast_symbol_t`.
    Symbol = clingo_ast_attribute_type_clingo_ast_attribute_type_symbol as isize,
    /// For an attribute of type `clingo_location_t`.
    Location = clingo_ast_attribute_type_clingo_ast_attribute_type_location as isize,
    /// For an attribute of type `char const *`.
    String = clingo_ast_attribute_type_clingo_ast_attribute_type_string as isize,
    /// For an attribute of type `clingo_ast_t *`.
    Ast = clingo_ast_attribute_type_clingo_ast_attribute_type_ast as isize,
    /// For an attribute of type `clingo_ast_t *` that can be NULL.
    OptionalAst = clingo_ast_attribute_type_clingo_ast_attribute_type_optional_ast as isize,
    /// For an attribute of type `char const **`.
    StringArray = clingo_ast_attribute_type_clingo_ast_attribute_type_string_array as isize,
    /// For an attribute of type `clingo_ast_t **`.
    AstArray = clingo_ast_attribute_type_clingo_ast_attribute_type_ast_array as isize,
}

#[derive(Debug, Copy, Clone)]
/// Enumeration of attributes used by the AST.
pub enum AstAttribute {
    Argument = clingo_ast_attribute_clingo_ast_attribute_argument as isize,
    Arguments = clingo_ast_attribute_clingo_ast_attribute_arguments as isize,
    Arity = clingo_ast_attribute_clingo_ast_attribute_arity as isize,
    Atom = clingo_ast_attribute_clingo_ast_attribute_atom as isize,
    Atoms = clingo_ast_attribute_clingo_ast_attribute_atoms as isize,
    AtomType = clingo_ast_attribute_clingo_ast_attribute_atom_type as isize,
    Bias = clingo_ast_attribute_clingo_ast_attribute_bias as isize,
    Body = clingo_ast_attribute_clingo_ast_attribute_body as isize,
    Code = clingo_ast_attribute_clingo_ast_attribute_code as isize,
    Coefficient = clingo_ast_attribute_clingo_ast_attribute_coefficient as isize,
    Comparison = clingo_ast_attribute_clingo_ast_attribute_comparison as isize,
    Condition = clingo_ast_attribute_clingo_ast_attribute_condition as isize,
    Csp = clingo_ast_attribute_clingo_ast_attribute_csp as isize,
    Elements = clingo_ast_attribute_clingo_ast_attribute_elements as isize,
    External = clingo_ast_attribute_clingo_ast_attribute_external as isize,
    ExternalType = clingo_ast_attribute_clingo_ast_attribute_external_type as isize,
    Function = clingo_ast_attribute_clingo_ast_attribute_function as isize,
    Guard = clingo_ast_attribute_clingo_ast_attribute_guard as isize,
    Guards = clingo_ast_attribute_clingo_ast_attribute_guards as isize,
    Head = clingo_ast_attribute_clingo_ast_attribute_head as isize,
    IsDefault = clingo_ast_attribute_clingo_ast_attribute_is_default as isize,
    Left = clingo_ast_attribute_clingo_ast_attribute_left as isize,
    LeftGuard = clingo_ast_attribute_clingo_ast_attribute_left_guard as isize,
    Literal = clingo_ast_attribute_clingo_ast_attribute_literal as isize,
    Location = clingo_ast_attribute_clingo_ast_attribute_location as isize,
    Modifier = clingo_ast_attribute_clingo_ast_attribute_modifier as isize,
    Name = clingo_ast_attribute_clingo_ast_attribute_name as isize,
    NodeU = clingo_ast_attribute_clingo_ast_attribute_node_u as isize,
    NodeV = clingo_ast_attribute_clingo_ast_attribute_node_v as isize,
    OperatorName = clingo_ast_attribute_clingo_ast_attribute_operator_name as isize,
    OperatorType = clingo_ast_attribute_clingo_ast_attribute_operator_type as isize,
    Operators = clingo_ast_attribute_clingo_ast_attribute_operators as isize,
    Parameters = clingo_ast_attribute_clingo_ast_attribute_parameters as isize,
    Positive = clingo_ast_attribute_clingo_ast_attribute_positive as isize,
    Priority = clingo_ast_attribute_clingo_ast_attribute_priority as isize,
    Right = clingo_ast_attribute_clingo_ast_attribute_right as isize,
    RightGuard = clingo_ast_attribute_clingo_ast_attribute_right_guard as isize,
    ScriptType = clingo_ast_attribute_clingo_ast_attribute_script_type as isize,
    SequenceType = clingo_ast_attribute_clingo_ast_attribute_sequence_type as isize,
    Sign = clingo_ast_attribute_clingo_ast_attribute_sign as isize,
    Symbol = clingo_ast_attribute_clingo_ast_attribute_symbol as isize,
    Term = clingo_ast_attribute_clingo_ast_attribute_term as isize,
    Terms = clingo_ast_attribute_clingo_ast_attribute_terms as isize,
    Value = clingo_ast_attribute_clingo_ast_attribute_value as isize,
    Variable = clingo_ast_attribute_clingo_ast_attribute_variable as isize,
    Weight = clingo_ast_attribute_clingo_ast_attribute_weight as isize,
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
pub struct Ast(clingo_ast_t);

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
// extern "C" {
//     #[doc = "! Get the type of an AST node."]
//     #[doc = "!"]
//     #[doc = "! @param[in] ast the target AST"]
//     #[doc = "! @param[out] type the resulting type"]
//     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
//     #[doc = "! - ::clingo_error_runtime"]
//     pub fn clingo_ast_get_type(ast: *mut clingo_ast_t, type_: *mut clingo_ast_type_t) -> bool;
// }
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
// extern "C" {
//     #[doc = "! Parse the given program and return an abstract syntax tree for each statement via a callback."]
//     #[doc = "!"]
//     #[doc = "! @param[in] program the program in gringo syntax"]
//     #[doc = "! @param[in] callback the callback reporting statements"]
//     #[doc = "! @param[in] callback_data user data for the callback"]
//     #[doc = "! @param[in] logger callback to report messages during parsing"]
//     #[doc = "! @param[in] logger_data user data for the logger"]
//     #[doc = "! @param[in] message_limit the maximum number of times the logger is called"]
//     #[doc = "! @return whether the call was successful; might set one of the following error codes:"]
//     #[doc = "! - ::clingo_error_runtime if parsing fails"]
//     #[doc = "! - ::clingo_error_bad_alloc"]
//     pub fn clingo_ast_parse_string(
//         program: *const ::std::os::raw::c_char,
//         callback: clingo_ast_callback_v2_t,
//         callback_data: *mut ::std::os::raw::c_void,
//         logger: clingo_logger_t,
//         logger_data: *mut ::std::os::raw::c_void,
//         message_limit: ::std::os::raw::c_uint,
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
    Condition = clingo_ast_unpool_type_clingo_ast_unpool_type_condition as isize,
    /// To unpool everything except conditions of conditional literals.
    Other = clingo_ast_unpool_type_clingo_ast_unpool_type_other as isize,
    /// To unpool everything.
    All = clingo_ast_unpool_type_clingo_ast_unpool_type_all as isize,
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
