use ::*;

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
#[derive(Copy, Clone)]
pub struct HeadLiteral(clingo_ast_head_literal_t);
impl HeadLiteral {
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn head_literal_type(&self) -> HeadLiteralType {
        match self.0.type_ as u32 {
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_literal => {
                HeadLiteralType::Literal
            }
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_disjunction => {
                HeadLiteralType::Disjuction
            }
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_aggregate => {
                HeadLiteralType::Aggregate
            }
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_head_aggregate => {
                HeadLiteralType::HeadAggregate
            }
            clingo_ast_head_literal_type_clingo_ast_head_literal_type_theory_atom => {
                HeadLiteralType::TheoryAtom
            }
            x => panic!("Failed to match clingo_ast_head_literal_type: {}.", x),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Rule(clingo_ast_rule_t);
impl Rule {
    pub fn new(HeadLiteral(head): HeadLiteral, body: &[BodyLiteral]) -> Rule {
        let rule = clingo_ast_rule {
            head: head,
            body: body.as_ptr() as *const clingo_ast_body_literal_t,
            size: body.len(),
        };
        Rule(rule)
    }
    pub fn head(&self) -> HeadLiteral {
        HeadLiteral(self.0.head)
    }
    pub fn body(&self) -> &[BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    }

    /// Create a statement for the rule.
    pub fn ast_statement(&self, Location(loc): Location) -> AstStatement<ast::Rule> {
        let _bg_union_2 = clingo_ast_statement__bindgen_ty_1 {
            rule: &self.0 as *const clingo_ast_rule,
        };
        let stm = clingo_ast_statement_t {
            location: loc,
            type_: ast::StatementType::Rule as clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        AstStatement {
            data: stm,
            phantom: PhantomData,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Definition(clingo_ast_definition);
impl Definition {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.0.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.name) };
            c_str.to_str()
        }
    }
    pub fn value(&self) -> Term {
        Term(self.0.value)
    }
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
    pub fn term(&self) -> Term {
        Term(self.0.term)
    }
    pub fn body(&self) -> &[BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    }
    pub fn csp(&self) -> bool {
        self.0.csp
    }
}
#[derive(Copy, Clone)]
pub struct Minimize(clingo_ast_minimize);
impl Minimize {
    pub fn weight(&self) -> Term {
        Term(self.0.weight)
    }
    pub fn priority(&self) -> Term {
        Term(self.0.priority)
    }
    pub fn tuple(&self) -> &[Term] {
        unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    }
    pub fn body(&self) -> &[BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.body_size) }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Script(clingo_ast_script);
impl Script {
    pub fn script_type(&self) -> ScriptType {
        match self.0.type_ as u32 {
            clingo_ast_script_type_clingo_ast_script_type_lua => ScriptType::Lua,
            clingo_ast_script_type_clingo_ast_script_type_python => ScriptType::Python,
            x => panic!("Failed to match clingo_ast_script_type: {}.", x),
        }
    }
    pub fn code(&self) -> Result<&str, Utf8Error> {
        if self.0.code.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.code) };
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
pub struct BodyLiteral(clingo_ast_body_literal_t);
impl BodyLiteral {
    pub fn new(
        Location(location): Location,
        sign: Sign,
        type_: BodyLiteralType,
        lit_ref: &Literal,
    ) -> BodyLiteral {
        let _bg_union_2 = clingo_ast_body_literal__bindgen_ty_1 {
            literal: (lit_ref as *const Literal) as *const clingo_ast_literal,
        };
        BodyLiteral(clingo_ast_body_literal_t {
            location: location,
            sign: sign as clingo_ast_sign_t,
            type_: type_ as clingo_ast_body_literal_type_t,
            __bindgen_anon_1: _bg_union_2,
        })
    }

    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn sign(&self) -> Sign {
        match self.0.sign as u32 {
            clingo_ast_sign_clingo_ast_sign_double_negation => Sign::DoubleNegation,
            clingo_ast_sign_clingo_ast_sign_negation => Sign::Negation,
            clingo_ast_sign_clingo_ast_sign_none => Sign::None,
            x => panic!("Failed to match clingo_ast_sign: {}.", x),
        }
    }
    pub fn body_literal_type(&self) -> BodyLiteralType {
        match self.0.type_ as u32 {
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_aggregate => {
                BodyLiteralType::Aggregate
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_body_aggregate => {
                BodyLiteralType::BodyAggregate
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_conditional => {
                BodyLiteralType::Conditional
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_disjoint => {
                BodyLiteralType::Disjoint
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_literal => {
                BodyLiteralType::Literal
            }
            clingo_ast_body_literal_type_clingo_ast_body_literal_type_theory_atom => {
                BodyLiteralType::TheoryAtom
            }
            x => panic!("Failed to match clingo_ast_body_literal_type: {}.", x),
        }
    }
}
#[derive(Copy, Clone)]
pub struct External(clingo_ast_external_t);
impl External {
    pub fn new(Atom(atom): Atom, body: &[BodyLiteral]) -> External {
        let ext = clingo_ast_external {
            atom: atom,
            body: body.as_ptr() as *const clingo_ast_body_literal_t,
            size: body.len(),
        };
        External(ext)
    }
    pub fn atom(&self) -> Atom {
        Atom(self.0.atom)
    }
    pub fn body(&self) -> &[BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    }
    /// Create a statement for the external.
    pub fn ast_statement<'a>(&'a self, Location(loc): Location) -> AstStatement<'a, ast::External> {
        let _bg_union_2 = clingo_ast_statement__bindgen_ty_1 {
            external: &self.0 as *const clingo_ast_external,
        };
        let stm = clingo_ast_statement_t {
            location: loc,
            type_: StatementType::External as clingo_ast_statement_type_t,
            __bindgen_anon_1: _bg_union_2,
        };
        AstStatement {
            data: stm,
            phantom: PhantomData,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Edge(clingo_ast_edge);
impl Edge {
    pub fn u(&self) -> Term {
        Term(self.0.u)
    }
    pub fn v(&self) -> Term {
        Term(self.0.v)
    }
    pub fn body(&self) -> &[BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    }
}
#[derive(Copy, Clone)]
pub struct Heuristic(clingo_ast_heuristic);
impl Heuristic {
    pub fn atom(&self) -> Term {
        Term(self.0.atom)
    }
    pub fn body(&self) -> &[BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    }
    pub fn bias(&self) -> Term {
        Term(self.0.bias)
    }
    pub fn priority(&self) -> Term {
        Term(self.0.priority)
    }
    pub fn modifier(&self) -> Term {
        Term(self.0.modifier)
    }
}
#[derive(Copy, Clone)]
pub struct Project(clingo_ast_project);
impl Project {
    pub fn atom(&self) -> Term {
        Term(self.0.atom)
    }
    pub fn body(&self) -> &[BodyLiteral] {
        unsafe { std::slice::from_raw_parts(self.0.body as *const BodyLiteral, self.0.size) }
    }
}

#[derive(Copy, Clone)]
pub struct Atom(clingo_ast_term_t);
impl Atom {
    pub fn from_symbol(Location(location): Location, Symbol(symbol): Symbol) -> Atom {
        let _bg_union_1 = clingo_ast_term__bindgen_ty_1 { symbol: symbol };
        let term = clingo_ast_term_t {
            location: location,
            type_: TermType::Symbol as clingo_ast_term_type_t,
            __bindgen_anon_1: _bg_union_1,
        };
        Atom(term)
    }
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
}
#[derive(Copy, Clone)]
pub struct Term(clingo_ast_term_t);
impl Term {
    pub fn new_symbol(Location(location): Location, Symbol(symbol): Symbol) -> Term {
        let _bg_union_1 = clingo_ast_term__bindgen_ty_1 { symbol: symbol };
        let term = clingo_ast_term_t {
            location: location,
            type_: TermType::Symbol as clingo_ast_term_type_t,
            __bindgen_anon_1: _bg_union_1,
        };
        Term(term)
    }

    /// Create a variable term
    ///
    /// # Errors
    ///
    /// - [`NulError`](https://doc.rust-lang.org/std/ffi/struct.NulError.html) - if `string` contains a nul byte
    pub fn new_variable(Location(location): Location, string: &str) -> Result<Term, NulError> {
        let cstr = CString::new(string)?;
        let _bg_union_1 = clingo_ast_term__bindgen_ty_1 {
            variable: cstr.as_ptr(),
        };
        let term = clingo_ast_term_t {
            location: location,
            type_: TermType::Variable as clingo_ast_term_type_t,
            __bindgen_anon_1: _bg_union_1,
        };
        Ok(Term(term))
    }

    pub fn location(&self) -> Location {
        Location(self.0.location)
    }

    pub fn term_type(&self) -> TermType {
        match self.0.type_ as u32 {
            clingo_ast_term_type_clingo_ast_term_type_symbol => TermType::Symbol,
            clingo_ast_term_type_clingo_ast_term_type_variable => TermType::Variable,
            clingo_ast_term_type_clingo_ast_term_type_unary_operation => TermType::UnaryOperation,
            clingo_ast_term_type_clingo_ast_term_type_binary_operation => TermType::BinaryOperation,
            clingo_ast_term_type_clingo_ast_term_type_interval => TermType::Interval,
            clingo_ast_term_type_clingo_ast_term_type_function => TermType::Function,
            clingo_ast_term_type_clingo_ast_term_type_external_function => {
                TermType::ExternalFunction
            }
            clingo_ast_term_type_clingo_ast_term_type_pool => TermType::Pool,
            x => panic!("Failed to match clingo_ast_term_type: {}.", x),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Literal(clingo_ast_literal_t);
impl Literal {
    pub fn from_atom(Location(location): Location, sign: Sign, Atom(atom): &Atom) -> Literal {
        let _bg_union_2 = clingo_ast_literal__bindgen_ty_1 {
            symbol: atom as *const clingo_sys::clingo_ast_term,
        };
        let lit = clingo_ast_literal_t {
            location: location,
            type_: LiteralType::Symbolic as clingo_ast_literal_type_t,
            sign: sign as clingo_ast_sign_t,
            __bindgen_anon_1: _bg_union_2,
        };
        Literal(lit)
    }

    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn sign(&self) -> Sign {
        match self.0.sign as u32 {
            clingo_ast_sign_clingo_ast_sign_double_negation => Sign::DoubleNegation,
            clingo_ast_sign_clingo_ast_sign_negation => Sign::Negation,
            clingo_ast_sign_clingo_ast_sign_none => Sign::None,
            x => panic!("Failed to match clingo_ast_sign: {}.", x),
        }
    }
    pub fn literal_type(&self) -> LiteralType {
        match self.0.type_ as u32 {
            clingo_ast_literal_type_clingo_ast_literal_type_boolean => LiteralType::Boolean,
            clingo_ast_literal_type_clingo_ast_literal_type_symbolic => LiteralType::Symbolic,
            clingo_ast_literal_type_clingo_ast_literal_type_comparison => LiteralType::Comparison,
            clingo_ast_literal_type_clingo_ast_literal_type_csp => LiteralType::CSP,
            x => panic!("Failed to match clingo_ast_literal_type: {}.", x),
        }
    }
}
#[derive(Copy, Clone)]
pub struct UnaryOperation(clingo_ast_unary_operation_t);
impl UnaryOperation {
    pub fn unary_operator(&self) -> UnaryOperator {
        match self.0.unary_operator as u32 {
            clingo_ast_unary_operator_clingo_ast_unary_operator_minus => UnaryOperator::Minus,
            clingo_ast_unary_operator_clingo_ast_unary_operator_negation => UnaryOperator::Negation,
            clingo_ast_unary_operator_clingo_ast_unary_operator_absolute => UnaryOperator::Absolute,
            x => panic!("Failed to match clingo_ast_unary_operator: {}.", x),
        }
    }
    pub fn argument(&self) -> Term {
        Term(self.0.argument)
    }
}
#[derive(Copy, Clone)]
pub struct BinaryOperation(clingo_ast_binary_operation);
impl BinaryOperation {
    pub fn binary_operator(&self) -> BinaryOperator {
        match self.0.binary_operator as u32 {
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
            x => panic!("Failed to match clingo_ast_binary_operator: {}.", x),
        }
    }
    pub fn left(&self) -> Term {
        Term(self.0.left)
    }
    pub fn right(&self) -> Term {
        Term(self.0.right)
    }
}
#[derive(Copy, Clone)]
pub struct Interval(clingo_ast_interval);
impl Interval {
    pub fn left(&self) -> Term {
        Term(self.0.left)
    }
    pub fn right(&self) -> Term {
        Term(self.0.right)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Function(clingo_ast_function);
impl Function {
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.0.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.name) };
            c_str.to_str()
        }
    }
    pub fn arguments(&self) -> &[Term] {
        unsafe { std::slice::from_raw_parts(self.0.arguments as *const Term, self.0.size) }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Pool(clingo_ast_pool);
impl Pool {
    pub fn arguments(&self) -> &[Term] {
        unsafe { std::slice::from_raw_parts(self.0.arguments as *const Term, self.0.size) }
    }
}
#[derive(Copy, Clone)]
pub struct CspProductTerm(clingo_ast_csp_product_term);
impl CspProductTerm {
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn coefficient(&self) -> Term {
        Term(self.0.coefficient)
    }
    pub fn variable(&self) -> Result<&Term, WrapperError> {
        match unsafe { (self.0.variable as *const Term).as_ref() } {
            Some(x) => Ok(x),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &Term.",
            }),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct CspSumTerm(clingo_ast_csp_sum_term);
impl CspSumTerm {
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn terms(&self) -> &[CspProductTerm] {
        unsafe { std::slice::from_raw_parts(self.0.terms as *const CspProductTerm, self.0.size) }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct CspGuard(clingo_ast_csp_guard);
impl CspGuard {
    pub fn comparison(&self) -> ComparisonOperator {
        match self.0.comparison as u32 {
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
                ComparisonOperator::GreaterEqual
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
    pub fn term(&self) -> CspSumTerm {
        CspSumTerm(self.0.term)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct CspLiteral(clingo_ast_csp_literal);
impl CspLiteral {
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
#[derive(Copy, Clone)]
pub struct AggregateGuard(clingo_ast_aggregate_guard);
impl AggregateGuard {
    pub fn comparison(&self) -> ComparisonOperator {
        match self.0.comparison as u32 {
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
                ComparisonOperator::GreaterEqual
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
    pub fn term(&self) -> Term {
        Term(self.0.term)
    }
}
#[derive(Copy, Clone)]
pub struct ConditionalLiteral(clingo_ast_conditional_literal);
impl ConditionalLiteral {
    pub fn literal(&self) -> Literal {
        Literal(self.0.literal)
    }
    pub fn condition(&self) -> &[Literal] {
        unsafe { std::slice::from_raw_parts(self.0.condition as *const Literal, self.0.size) }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Aggregate(clingo_ast_aggregate);
impl Aggregate {
    pub fn elements(&self) -> &[ConditionalLiteral] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const ConditionalLiteral, self.0.size)
        }
    }
    pub fn left_guard(&self) -> Result<&AggregateGuard, WrapperError> {
        match unsafe { (self.0.left_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &AggregateGuard.",
            }),
        }
    }
    pub fn right_guard(&self) -> Result<&AggregateGuard, WrapperError> {
        match unsafe { (self.0.right_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &AggregateGuard.",
            }),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BodyAggregateElement(clingo_ast_body_aggregate_element);
impl BodyAggregateElement {
    pub fn tuple(&self) -> &[Term] {
        unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    }
    pub fn condition(&self) -> &[Literal] {
        unsafe {
            std::slice::from_raw_parts(self.0.condition as *const Literal, self.0.condition_size)
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct BodyAggregate(clingo_ast_body_aggregate);
impl BodyAggregate {
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
    pub fn left_guard(&self) -> Result<&AggregateGuard, WrapperError> {
        match unsafe { (self.0.left_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &AggregateGuard.",
            }),
        }
    }
    pub fn right_guard(&self) -> Result<&AggregateGuard, WrapperError> {
        match unsafe { (self.0.right_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &AggregateGuard.",
            }),
        }
    }
}
#[derive(Copy, Clone)]
pub struct HeadAggregateElement(clingo_ast_head_aggregate_element);
impl HeadAggregateElement {
    pub fn tuple(&self) -> &[Term] {
        unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    }
    pub fn conditional_literal(&self) -> ConditionalLiteral {
        ConditionalLiteral(self.0.conditional_literal)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct HeadAggregate(clingo_ast_head_aggregate);
impl HeadAggregate {
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
    pub fn left_guard(&self) -> Result<&AggregateGuard, WrapperError> {
        match unsafe { (self.0.left_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &AggregateGuard.",
            }),
        }
    }
    pub fn right_guard(&self) -> Result<&AggregateGuard, WrapperError> {
        match unsafe { (self.0.right_guard as *const AggregateGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &AggregateGuard.",
            }),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Disjunction(clingo_ast_disjunction);
impl Disjunction {
    pub fn elements(&self) -> &[ConditionalLiteral] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const ConditionalLiteral, self.0.size)
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct DisjointElement(clingo_ast_disjoint_element);
impl DisjointElement {
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn tuple(&self) -> &[Term] {
        unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    }
    pub fn term(&self) -> CspSumTerm {
        CspSumTerm(self.0.term)
    }
    pub fn condition(&self) -> &[Literal] {
        unsafe {
            std::slice::from_raw_parts(self.0.condition as *const Literal, self.0.condition_size)
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Disjoint(clingo_ast_disjoint);
impl Disjoint {
    pub fn elements(&self) -> &[DisjointElement] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const DisjointElement, self.0.size)
        }
    }
}
#[derive(Copy, Clone)]
pub struct TheoryTerm(clingo_ast_theory_term);
impl TheoryTerm {
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn term_type(&self) -> TheoryTermType {
        match self.0.type_ as u32 {
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_symbol => {
                TheoryTermType::Symbol
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_variable => {
                TheoryTermType::Variable
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_tuple => TheoryTermType::Tuple,
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_list => TheoryTermType::List,
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_set => TheoryTermType::Set,
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_function => {
                TheoryTermType::Function
            }
            clingo_ast_theory_term_type_clingo_ast_theory_term_type_unparsed_term => {
                TheoryTermType::UnparsedTerm
            }
            x => panic!("Failed to match clingo_ast_theory_term_type: {}.", x),
        }
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
        TheoryTerm(self.0.term)
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryUnparsedTerm(clingo_ast_theory_unparsed_term);
impl TheoryUnparsedTerm {
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
    pub fn tuple(&self) -> &[Term] {
        unsafe { std::slice::from_raw_parts(self.0.tuple as *const Term, self.0.tuple_size) }
    }
    pub fn condition(&self) -> &[Literal] {
        unsafe {
            std::slice::from_raw_parts(self.0.condition as *const Literal, self.0.condition_size)
        }
    }
}
#[derive(Copy, Clone)]
pub struct TheoryGuard(clingo_ast_theory_guard);
impl TheoryGuard {
    pub fn operator_name(&self) -> Result<&str, Utf8Error> {
        if self.0.operator_name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.operator_name) };
            c_str.to_str()
        }
    }
    pub fn term(&self) -> TheoryTerm {
        TheoryTerm(self.0.term)
    }
}
#[derive(Copy, Clone)]
pub struct TheoryAtom(clingo_ast_theory_atom);
impl TheoryAtom {
    pub fn term(&self) -> Term {
        Term(self.0.term)
    }
    pub fn elements(&self) -> &[TheoryAtomElement] {
        unsafe {
            std::slice::from_raw_parts(self.0.elements as *const TheoryAtomElement, self.0.size)
        }
    }
    pub fn guard(&self) -> Result<&TheoryGuard, WrapperError> {
        match unsafe { (self.0.guard as *const TheoryGuard).as_ref() } {
            Some(x) => Ok(x),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &TheoryGuard.",
            }),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryOperatorDefinition(clingo_ast_theory_operator_definition);
impl TheoryOperatorDefinition {
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.0.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.name) };
            c_str.to_str()
        }
    }
    pub fn priority(&self) -> u32 {
        self.0.priority
    }
    pub fn theory_operator_type(&self) -> TheoryOperatorType {
        match self.0.type_ as u32 {
            clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_unary => {
                TheoryOperatorType::Unary
            }
            clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_left => {
                TheoryOperatorType::BinaryLeft
            }
            clingo_ast_theory_operator_type_clingo_ast_theory_operator_type_binary_right => {
                TheoryOperatorType::BinaryRight
            }
            x => panic!("Failed to match clingo_ast_theory_operator_type: {}.", x),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryTermDefinition(clingo_ast_theory_term_definition);
impl TheoryTermDefinition {
    pub fn location(&self) -> Location {
        Location(self.0.location)
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
pub enum TheoryAtomDefinitionType {
    Head =
        clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_head as isize,
    Body =
        clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_body as isize,
    Any =
        clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_any as isize,
    Directive =
        clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_directive
            as isize,
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryAtomDefinition(clingo_ast_theory_atom_definition);
impl TheoryAtomDefinition {
    pub fn location(&self) -> Location {
        Location(self.0.location)
    }
    pub fn definition_type(&self) -> TheoryAtomDefinitionType {
        match self.0.type_ as u32 {
            clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_any => {
                TheoryAtomDefinitionType::Any
            }
            clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_body => {
                TheoryAtomDefinitionType::Body
            }
            clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_directive => {
                TheoryAtomDefinitionType::Directive
            }
            clingo_ast_theory_atom_definition_type_clingo_ast_theory_atom_definition_type_head => {
                TheoryAtomDefinitionType::Head
            }
            x => panic!(
                "Failed to match clingo_ast_theory_atom_definition_type: {}.",
                x
            ),
        }
    }
    pub fn name(&self) -> Result<&str, Utf8Error> {
        if self.0.name.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.name) };
            c_str.to_str()
        }
    }
    pub fn arity(&self) -> u32 {
        self.0.arity
    }
    pub fn elements(&self) -> Result<&str, Utf8Error> {
        if self.0.elements.is_null() {
            Ok("")
        } else {
            let c_str = unsafe { CStr::from_ptr(self.0.elements) };
            c_str.to_str()
        }
    }
    pub fn guard(&self) -> Result<&TheoryGuardDefinition, WrapperError> {
        match unsafe { (self.0.guard as *const TheoryGuardDefinition).as_ref() } {
            Some(x) => Ok(x),
            None => Err(WrapperError {
                msg: "tried casting a null pointer to &TheoryGuardDefinition.",
            }),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TheoryDefinition(clingo_ast_theory_definition);
impl TheoryDefinition {
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
