#[derive(Debug, Copy, Clone)]
pub struct Term(Ast);
impl Term {
    pub fn get_tterm(&self) -> Result<TTerm, ClingoError> {
        match self.0.get_type()? {
            AstType::Variable => Ok(TTerm::Variable(Variable(self.0))),
            AstType::SymbolicTerm => Ok(TTerm::SymbolicTerm(SymbolicTerm(self.0))),
            AstType::UnaryOperation => Ok(TTerm::UnaryOperation(UnaryOperation(self.0))),
            AstType::BinaryOperation => Ok(TTerm::BinaryOperation(BinaryOperation(self.0))),
            AstType::Interval => Ok(TTerm::Interval(Interval(self.0))),
            AstType::Function => Ok(TTerm::Function(Function(self.0))),
            AstType::Pool => Ok(TTerm::Pool(Pool(self.0))),
            x => panic!("unexpected AstType: {:?}", x),
        }
    }
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.0.to_string()
    }
}
impl From<Variable> for Term {
    fn from(var: Variable) -> Self {
        Term(var.0)
    }
}
impl From<&Variable> for Term {
    fn from(var: &Variable) -> Self {
        Term(var.0)
    }
}
impl From<SymbolicTerm> for Term {
    fn from(term: SymbolicTerm) -> Self {
        Term(term.0)
    }
}
impl From<Function> for Term {
    fn from(fun: Function) -> Self {
        Term(fun.0)
    }
}
impl From<UnaryOperation> for Term {
    fn from(op: UnaryOperation) -> Self {
        Term(op.0)
    }
}
impl From<BinaryOperation> for Term {
    fn from(op: BinaryOperation) -> Self {
        Term(op.0)
    }
}
impl From<Interval> for Term {
    fn from(interval: Interval) -> Self {
        Term(interval.0)
    }
}
impl From<Pool> for Term {
    fn from(pool: Pool) -> Self {
        Term(pool.0)
    }
}
#[derive(Debug, Copy, Clone)]
pub enum TTerm {
    Variable(Variable),
    SymbolicTerm(SymbolicTerm),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Interval(Interval),
    Function(Function),
    Pool(Pool),
}
#[derive(Debug, Copy, Clone)]
pub struct BodyLiteral(Ast);
impl From<Literal> for BodyLiteral {
    fn from(lit: Literal) -> Self {
        BodyLiteral(lit.0)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryTerm(Ast);

#[derive(Debug, Copy, Clone)]
pub struct Statement(Ast);
impl Statement {
    pub fn get_tterm(&self) -> Result<TStatement, ClingoError> {
        match self.0.get_type()? {
            AstType::Program => Ok(TStatement::Program(Program(self.0))),
            AstType::Rule => Ok(TStatement::Rule(Rule(self.0))),
            AstType::External => Ok(TStatement::External(External(self.0))),
            x => panic!("unexpected AstType: {:?}", x),
        }
    }
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.0.to_string()
    }
}
#[derive(Debug, Copy, Clone)]
pub enum TStatement {
    Program(Program),
    Rule(Rule),
    External(External),
}
impl From<Program> for Statement {
    fn from(prg: Program) -> Self {
        Statement(prg.0)
    }
}
impl From<Rule> for Statement {
    fn from(rule: Rule) -> Self {
        Statement(rule.0)
    }
}
impl From<External> for Statement {
    fn from(ext: External) -> Self {
        Statement(ext.0)
    }
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

#[derive(Debug, Copy, Clone)]
pub struct Id(Ast);
impl Id {
    /// Construct an AST node of type `ASTType.Id`.
    pub fn new(location: &Location, name: &str) -> Result<Id, ClingoError> {
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
            Some(ast) => Ok(Id(Ast(ast))),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Variable(Ast);
impl Variable {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.0.to_string()
    }
    /// Construct an AST node of type `ASTType.Variable`.
    pub fn variable(location: &Location, name: &str) -> Result<Variable, ClingoError> {
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
            Some(ast) => Ok(Variable(Ast(ast))),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct SymbolicTerm(Ast);
/// Construct an AST node of type `ASTType.SymbolicTerm`.
pub fn symbolic_term(location: &Location, symbol: &Symbol) -> Result<SymbolicTerm, ClingoError> {
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
        Some(ast) => Ok(SymbolicTerm(Ast(ast))),

        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}
impl SymbolicTerm {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.0.to_string()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Function(Ast);
impl Function {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.0.to_string()
    }
    /// Construct an AST node of type `ASTType.Function`.
    pub fn function(
        location: &Location,
        name: &str,
        arguments: &[Term],
        external: bool,
    ) -> Result<Function, ClingoError> {
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
        println!("A {}", ast_type);
        match NonNull::new(ast) {
            Some(ast) => {
                // let x = Ast(ast_ref);
                // println!("B {:?}", x.get_type());
                Ok(Function(Ast(ast)))
            }
            None => Err(ClingoError::FFIError {
                msg: "tried casting a null pointer to &mut clingo_ast.",
            }),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct UnaryOperation(Ast);
impl UnaryOperation {
    /// Construct an AST node of type `ASTType.UnaryOperation`.
    pub fn unary_operation(
        location: &Location,
        operator_type: UnaryOperator,
        argument: Term,
    ) -> Result<UnaryOperation, ClingoError> {
        let mut ast = std::ptr::null_mut();

        if !unsafe {
            clingo_ast_build(
                clingo_ast_type_e_clingo_ast_type_unary_operation as i32,
                &mut ast,
                location,
                operator_type as i32,
                argument.0,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_build() failed.",
            ));
        }
        match NonNull::new(ast) {
            Some(ast) => Ok(UnaryOperation(Ast(ast))),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
    pub fn unary_operator(self) -> UnaryOperator {
        unimplemented!()
    }
    pub fn argument(self) -> Term {
        unimplemented!()
    }
}
#[derive(Debug, Copy, Clone)]
pub struct BinaryOperation(Ast);
impl BinaryOperation {
    /// Construct an AST node of type `ASTType.BinaryOperation`.
    pub fn binary_operation(
        location: &Location,
        operator_type: BinaryOperator,
        left: Term,
        right: Term,
    ) -> Result<BinaryOperation, ClingoError> {
        let mut ast = std::ptr::null_mut();
        if !unsafe {
            clingo_ast_build(
                clingo_ast_type_e_clingo_ast_type_binary_operation as i32,
                &mut ast,
                location,
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
            Some(ast) => Ok(BinaryOperation(Ast(ast))),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
    pub fn operator_type(&self) -> BinaryOperator {
        unimplemented!()
    }
    pub fn left(&self) -> Term {
        unimplemented!()
    }
    pub fn right(&self) -> Term {
        unimplemented!()
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Interval(Ast);
impl Interval {
    /// Construct an AST node of type `ASTType.Interval`.
    pub fn interval(location: &Location, left: Term, right: Term) -> Result<Interval, ClingoError> {
        let mut ast = std::ptr::null_mut();
        if !unsafe {
            clingo_ast_build(
                clingo_ast_type_e_clingo_ast_type_interval as i32,
                &mut ast,
                location,
                left.0,
                right.0,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_build() failed.",
            ));
        }
        match NonNull::new(ast) {
            Some(ast) => Ok(Interval(Ast(ast))),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
    pub fn left(&self) -> Term {
        unimplemented!()
    }
    pub fn right(&self) -> Term {
        unimplemented!()
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Pool(Ast);
impl Pool {
    /// Construct an AST node of type `ASTType.Pool`.
    pub fn pool(location: &Location, arguments: &[Term]) -> Result<Pool, ClingoError> {
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
        eprintln!("yeah");
        match NonNull::new(ast) {
            Some(ast) => Ok(Pool(Ast(ast))),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct CspProduct(Ast);
/// Construct an AST node of type `ASTType.CspProduct`.
pub fn csp_product(
    location: &Location,
    coefficient: Term,
    variable: Option<Term>,
) -> Result<CspProduct, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if let Some(variable) = variable {
        if !unsafe {
            clingo_ast_build(
                clingo_ast_type_e_clingo_ast_type_csp_product as i32,
                &mut ast,
                location,
                coefficient.0,
                variable.0,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_build() failed.",
            ));
        }
    } else {
        if !unsafe {
            clingo_ast_build(
                clingo_ast_type_e_clingo_ast_type_csp_product as i32,
                &mut ast,
                location,
                coefficient.0,
                std::ptr::null() as *const clingo_ast_t,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_build() failed.",
            ));
        }
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(CspProduct(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CspSum(Ast);
/// Construct an AST node of type `ASTType.CspSum`.
pub fn csp_sum(
    location: &Location,
    coefficient: Term,
    variable: Term,
) -> Result<CspSum, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_csp_sum as i32,
            &mut ast,
            location,
            coefficient.0,
            variable.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(CspSum(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
pub struct CspTerm(Ast);
impl From<CspSum> for CspTerm {
    fn from(csp_sum: CspSum) -> Self {
        CspTerm(csp_sum.0)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct CspGuard(Ast);
/// Construct an AST node of type `ASTType.CspGuard`.
pub fn csp_guard(
    location: &Location,
    comparison: ComparisonOperator,
    term: &CspTerm,
) -> Result<CspGuard, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_csp_guard as i32,
            &mut ast,
            location,
            comparison as i32,
            term.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(CspGuard(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BooleanConstant(Ast);
/// Construct an AST node of type `ASTType.BooleanConstant`.
pub fn boolean_constant(value: bool) -> Result<BooleanConstant, ClingoError> {
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
        Some(ast) => Ok(BooleanConstant(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SymbolicAtom(Ast);
/// Construct an AST node of type `ASTType.SymbolicAtom`.
pub fn symbolic_atom(symbol: Term) -> Result<SymbolicAtom, ClingoError> {
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
        Some(ast) => Ok(SymbolicAtom(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
impl SymbolicAtom {
    pub fn to_string(&self) -> Result<String, ClingoError> {
        self.0.to_string()
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Comparison(Ast);
/// Construct an AST node of type `ASTType.Comparison`.
pub fn comparison(
    comparison: ComparisonOperator,
    left: Term,
    right: Term,
) -> Result<Comparison, ClingoError> {
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
        Some(ast) => Ok(Comparison(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CspLiteral(Ast);
/// Construct an AST node of type `ASTType.CspLiteral`.
pub fn csp_literal(
    location: &Location,
    term: CspTerm,
    guards: &[CspGuard],
) -> Result<CspLiteral, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_csp_literal as i32,
            &mut ast,
            location,
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
        Some(ast) => Ok(CspLiteral(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct AggregateGuard(Ast);
/// Construct an AST node of type `ASTType.AggregateGuard`.
pub fn aggregate_guard(
    comparison: ComparisonOperator,
    term: Term,
) -> Result<AggregateGuard, ClingoError> {
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
        Some(ast) => Ok(AggregateGuard(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ConditionalLiteral(Ast);
/// Construct an AST node of type `ASTType.ConditionalLiteral`.
pub fn conditional_literal(
    location: &Location,
    literal: Literal,
    condition: &[Literal],
) -> Result<ConditionalLiteral, ClingoError> {
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
        Some(ast) => Ok(ConditionalLiteral(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Aggregate(Ast);
/// Construct an AST node of type `ASTType.Aggregate`.
pub fn aggregate(
    location: &Location,
    left_guard: Option<AggregateGuard>,
    elements: &[ConditionalLiteral],
    right_guard: Option<AggregateGuard>,
) -> Result<Aggregate, ClingoError> {
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
        Some(ast) => Ok(Aggregate(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BodyAggregateElement(Ast);
/// Construct an AST node of type `ASTType.BodyAggregateElement`.
pub fn body_aggregate_element(
    terms: &[Term],
    condition: &[Literal],
) -> Result<BodyAggregateElement, ClingoError> {
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
        Some(ast) => Ok(BodyAggregateElement(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BodyAggregate(Ast);
/// Construct an AST node of type `ASTType.BodyAggregate`.
pub fn body_aggregate(
    location: &Location,
    left_guard: Option<AggregateGuard>,
    function: i32,
    elements: &[BodyAggregateElement],
    right_guard: Option<AggregateGuard>,
) -> Result<BodyAggregate, ClingoError> {
    let mut ast = std::ptr::null_mut();

    let left_guard = if let Some(left_guard) = left_guard {
        left_guard.0 .0.as_ptr()
    } else {
        std::ptr::null()
    };
    let right_guard = if let Some(right_guard) = right_guard {
        right_guard.0 .0.as_ptr()
    } else {
        std::ptr::null()
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
        Some(ast_ref) => Ok(BodyAggregate(Ast(ast_ref))),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HeadAggregateElement(Ast);
/// Construct an AST node of type `ASTType.HeadAggregateElement`.
pub fn head_aggregate_element(
    terms: &[Term],
    condition: ConditionalLiteral,
) -> Result<HeadAggregateElement, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_head_aggregate_element as i32,
            &mut ast,
            terms.as_ptr() as *const clingo_ast_t,
            terms.len(),
            condition.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast_ref) => Ok(HeadAggregateElement(Ast(ast_ref))),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

#[derive(Debug, Copy, Clone)]
pub struct HeadAggregate(Ast);
/// Construct an AST node of type `ASTType.HeadAggregate`.
pub fn head_aggregate(
    location: &Location,
    left_guard: Option<AggregateGuard>,
    function: AggregateFunction,
    elements: &[HeadAggregateElement],
    right_guard: Option<AggregateGuard>,
) -> Result<HeadAggregate, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let left_guard = if let Some(left_guard) = left_guard {
        left_guard.0 .0.as_ptr()
    } else {
        std::ptr::null()
    };
    let right_guard = if let Some(right_guard) = right_guard {
        right_guard.0 .0.as_ptr()
    } else {
        std::ptr::null()
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
        Some(ast_ref) => Ok(HeadAggregate(Ast(ast_ref))),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Disjunction(Ast);
/// Construct an AST node of type `ASTType.Disjunction`.
pub fn disjunction(
    location: &Location,
    elements: &[ConditionalLiteral],
) -> Result<Disjunction, ClingoError> {
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
        Some(ast) => Ok(Disjunction(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DisjointElement(Ast);
/// Construct an AST node of type `ASTType.DisjointElement`.
pub fn disjoint_element(
    location: &Location,
    terms: &[Term],
    term: CspTerm,
    condition: &[Literal],
) -> Result<DisjointElement, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_disjoint_element as i32,
            &mut ast,
            location,
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
        Some(ast) => Ok(DisjointElement(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Disjoint(Ast);
/// Construct an AST node of type `ASTType.Disjoint`.
pub fn disjoint(
    location: &Location,
    elements: &[DisjointElement],
) -> Result<Disjoint, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_disjoint as i32,
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
        Some(ast) => Ok(Disjoint(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheorySequence(Ast);
/// Construct an AST node of type `ASTType.TheorySequence`.
pub fn theory_sequence(
    location: &Location,
    sequence_type: TheoryTermSequenceType,
    terms: &[TheoryTerm],
) -> Result<TheorySequence, ClingoError> {
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
        Some(ast) => Ok(TheorySequence(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheoryFunction(Ast);
/// Construct an AST node of type `ASTType.TheoryFunction`.
pub fn theory_function(
    location: &Location,
    name: &str,
    arguments: &[TheoryTerm],
) -> Result<TheoryFunction, ClingoError> {
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
        Some(ast) => Ok(TheoryFunction(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheoryUnparsedTermElement(Ast);
/// Construct an AST node of type `ASTType.TheoryUnparsedTermElement`.
pub fn theory_unparsed_term_element(
    operators: &[&str],
    term: TheoryTerm,
) -> Result<TheoryUnparsedTermElement, ClingoError> {
    let mut ast = std::ptr::null_mut();
    // c_operators = [ _ffi.new('char[]', x.encode()) for x in operators ]
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_unparsed_term_element as i32,
            &mut ast,
            operators,
            operators.len(),
            term.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(TheoryUnparsedTermElement(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "tried casting a null pointer to &mut clingo_ast.",
        }),
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryUnparsedTerm(Ast);
/// Construct an AST node of type `ASTType.TheoryUnparsedTerm`.
pub fn theory_unparsed_term(
    location: &Location,
    elements: &[TheoryUnparsedTermElement],
) -> Result<TheoryUnparsedTerm, ClingoError> {
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
        Some(ast) => Ok(TheoryUnparsedTerm(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheoryGuard(Ast);
/// Construct an AST node of type `ASTType.TheoryGuard`.
pub fn theory_guard(operator_name: &str, term: TheoryTerm) -> Result<TheoryGuard, ClingoError> {
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
        Some(ast) => Ok(TheoryGuard(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheoryAtomElement(Ast);
/// Construct an AST node of type `ASTType.TheoryAtomElement`.
pub fn theory_atom_element(
    terms: &[TheoryTerm],
    condition: &[Literal],
) -> Result<TheoryAtomElement, ClingoError> {
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
        Some(ast) => Ok(TheoryAtomElement(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheoryAtom(Ast);
/// Construct an AST node of type `ASTType.TheoryAtom`.
pub fn theory_atom(
    location: &Location,
    term: Term,
    elements: &[TheoryAtomElement],
    guard: Option<TheoryGuard>,
) -> Result<TheoryAtom, ClingoError> {
    let mut ast = std::ptr::null_mut();

    let guard = match &guard {
        Some(guard) => &guard.0,
        None => std::ptr::null(),
    };
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_theory_atom as i32,
            &mut ast,
            location,
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
        Some(ast) => Ok(TheoryAtom(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Literal(Ast);
impl Literal {
    /// Construct an AST node of type `ASTType.Literal`.
    pub fn literal_from_symbolic_atom(
        location: &Location,
        sign: Sign,
        atom: &SymbolicAtom,
    ) -> Result<Literal, ClingoError> {
        let mut ast = std::ptr::null_mut();

        if !unsafe {
            clingo_ast_build(
                clingo_ast_type_e_clingo_ast_type_literal as i32,
                &mut ast,
                location,
                sign as i32,
                atom.0 .0,
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_build() failed.",
            ));
        }
        match NonNull::new(ast) {
            Some(ast) => Ok(Literal(Ast(ast))),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryOperatorDefinition(Ast);
/// Construct an AST node of type `ASTType.TheoryOperatorDefinition`.
pub fn theory_operator_definition(
    location: &Location,
    name: &str,
    priority: u32,
    operator_type: u32,
) -> Result<TheoryOperatorDefinition, ClingoError> {
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
        Some(ast) => Ok(TheoryOperatorDefinition(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheoryTermDefinition(Ast);
/// Construct an AST node of type `ASTType.TheoryTermDefinition`.
pub fn theory_term_definition(
    location: &Location,
    name: &str,
    operators: &[TheoryOperatorDefinition],
) -> Result<TheoryTermDefinition, ClingoError> {
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
        Some(ast) => Ok(TheoryTermDefinition(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheoryGuardDefinition(Ast);
/// Construct an AST node of type `ASTType.TheoryGuardDefinition`.
pub fn theory_guard_definition(
    operators: &[&str],
    term: &str,
) -> Result<TheoryGuardDefinition, ClingoError> {
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
        Some(ast) => Ok(TheoryGuardDefinition(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheoryAtomDefinition(Ast);
/// Construct an AST node of type `ASTType.TheoryAtomDefinition`.
pub fn theory_atom_definition(
    location: &Location,
    atom_type: TheoryAtomType,
    name: &str,
    arity: u32,
    term: &str,
    guard: Option<TheoryGuardDefinition>,
) -> Result<TheoryAtomDefinition, ClingoError> {
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
        Some(ast) => Ok(TheoryAtomDefinition(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Head(Ast);
impl Head {
    pub fn head(lit: Literal) -> Head {
        Head(lit.0)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rule(Ast);

    /// Construct an AST node of type `ASTType.Rule`.
    pub fn rule(
        location: &Location,
        head: &Head,
        body: &[BodyLiteral],
    ) -> Result<Rule, ClingoError> {
        let mut ast = std::ptr::null_mut();

        if !unsafe {
            clingo_ast_build(
                clingo_ast_type_e_clingo_ast_type_rule as i32,
                &mut ast,
                location,
                head.0,
                body,
                body.len(),
            )
        } {
            return Err(ClingoError::new_internal(
                "Call to clingo_ast_build() failed.",
            ));
        }

        match NonNull::new(ast) {
            Some(ast) => Ok(Rule(Ast(ast))),
            None => Err(ClingoError::FFIError {
                msg: "Tried creating NonNull from a null pointer.",
            })?,
        }
    }
impl Rule {
    pub fn body(&self) -> Body {
        Body {
            ast: &self.0,
            attribute: AstAttribute::Body,
        }
    }
    pub fn head(&self) -> Head {
        let ast = self.0.get_attribute_ast(AstAttribute::Head).unwrap();
        Head(ast)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Definition(Ast);
/// Construct an AST node of type `ASTType.Definition`.
pub fn definition(
    location: &Location,
    name: &str,
    value: Term,
    is_default: bool,
) -> Result<Definition, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let name = internalize_string(name)?;

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_definition as i32,
            &mut ast,
            location,
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
        Some(ast) => Ok(Definition(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ShowSignature(Ast);
/// Construct an AST node of type `ASTType.ShowSignature`.
pub fn show_signature(
    location: &Location,
    name: &str,
    arity: u32,
    positive: bool,
    csp: bool,
) -> Result<ShowSignature, ClingoError> {
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
        Some(ast) => Ok(ShowSignature(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ShowTerm(Ast);
/// Construct an AST node of type `ASTType.ShowTerm`.
pub fn show_term(
    location: &Location,
    term: Term,
    body: &[BodyLiteral],
    csp: bool,
) -> Result<ShowTerm, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_show_term as i32,
            &mut ast,
            location,
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
        Some(ast) => Ok(ShowTerm(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Minimize(Ast);
/// Construct an AST node of type `ASTType.Minimize`.
pub fn minimize(
    location: &Location,
    weight: Term,
    priority: Term,
    terms: &[Term],
    body: &[BodyLiteral],
) -> Result<Minimize, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_minimize as i32,
            &mut ast,
            location,
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
        Some(ast) => Ok(Minimize(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Script(Ast);
/// Construct an AST node of type `ASTType.Script`.
pub fn script(location: &Location, script_type: ScriptType, code: &str) -> Result<Script, ClingoError> {
    let mut ast = std::ptr::null_mut();
    let code = internalize_string(code);

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_script as i32,
            &mut ast,
            location,
            script_type as i32,
            code,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(Script(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Program(Ast);
/// Construct an AST node of type `ASTType.Program`.
pub fn program(location: &Location, name: &str, parameters: &[Id]) -> Result<Program, ClingoError> {
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
        Some(ast) => Ok(Program(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct External(Ast);
pub fn external(
    location: &Location,
    atom: SymbolicAtom,
    body: &[BodyLiteral],
    external_type: Term,
) -> Result<External, ClingoError> {
    let mut ast = std::ptr::null_mut();
    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_external as i32,
            &mut ast,
            location,
            atom.0,
            body.as_ptr() as *const clingo_ast_t,
            body.len(),
            external_type.0,
        )
    } {
        return Err(ClingoError::new_internal(
            "Call to clingo_ast_build() failed.",
        ));
    }
    match NonNull::new(ast) {
        Some(ast) => Ok(External(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Edge(Ast);
/// Construct an AST node of type `ASTType.Edge`.
pub fn edge(
    location: &Location,
    node_u: Term,
    node_v: Term,
    body: &[BodyLiteral],
) -> Result<Edge, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_edge as i32,
            &mut ast,
            location,
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
        Some(ast) => Ok(Edge(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Heuristic(Ast);
/// Construct an AST node of type `ASTType.Heuristic`.
pub fn heuristic(
    location: &Location,
    atom: SymbolicAtom,
    body: &[BodyLiteral],
    bias: Term,
    priority: Term,
    modifier: Term,
) -> Result<Heuristic, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_heuristic as i32,
            &mut ast,
            location,
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
        Some(ast) => Ok(Heuristic(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ProjectAtom(Ast);
/// Construct an AST node of type `ASTType.ProjectAtom`.
pub fn project_atom(location: &Location, atom: SymbolicAtom, body: &[BodyLiteral]) -> Result<ProjectAtom, ClingoError> {
    let mut ast = std::ptr::null_mut();

    if !unsafe {
        clingo_ast_build(
            clingo_ast_type_e_clingo_ast_type_project_atom as i32,
            &mut ast,
            location,
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
        Some(ast) => Ok(ProjectAtom(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ProjectSignature(Ast);
/// Construct an AST node of type `ASTType.ProjectSignature`.
pub fn project_signature(
    location: &Location,
    name: &str,
    arity: u32,
    positive: bool,
) -> Result<ProjectSignature, ClingoError> {
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
        Some(ast) => Ok(ProjectSignature(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Defined(Ast);
/// Construct an AST node of type `ASTType.Defined`.
pub fn defined(
    location: &Location,
    name: &str,
    arity: u32,
    positive: bool,
) -> Result<Defined, ClingoError> {
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
        Some(ast) => Ok(Defined(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TheoryDefinition(Ast);
/// Construct an AST node of type `ASTType.TheoryDefinition`.
pub fn theory_definition(
    location: &Location,
    name: &str,
    terms: &[TheoryTermDefinition],
    atoms: &[TheoryAtomDefinition],
) -> Result<TheoryDefinition, ClingoError> {
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
        Some(ast) => Ok(TheoryDefinition(Ast(ast))),
        None => Err(ClingoError::FFIError {
            msg: "Tried creating NonNull from a null pointer.",
        })?,
    }
}
