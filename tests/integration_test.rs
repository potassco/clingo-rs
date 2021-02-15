use clingo::ast::*;
use clingo::*;

#[test]
fn version() {
    let (ma, mi, re) = clingo::version();
    assert!(ma == 5);
    assert!(mi == 5);
    assert!(re == 0);
}
#[test]
fn signature() {
    let a = Signature::new("a", 2, false).unwrap();
    let b = Signature::new("a", 2, false).unwrap();
    let c = Signature::new("a", 2, true).unwrap();
    assert_eq!(a.name(), Ok("a"));
    assert_eq!(a.arity(), 2);
    assert!(a.is_negative());
    assert!(!a.is_positive());
    assert_eq!(b, a);
    assert_ne!(c, a);
    assert!(c < a);
    assert!(c <= a);
    assert!(a <= b);
    assert!(!(a <= c));
    assert!(a > c);
    assert!(a >= c);
    assert!(a >= b);
    assert!(!(c >= a));
    // assert!(c.hash() != a.hash());
    // assert!(b.hash() == a.hash());
}
#[test]
fn symbol() {
    // numbers
    let sym = Symbol::create_number(42);
    assert!(42 == sym.number().unwrap());
    // inf
    let sym2 = Symbol::create_infimum();
    assert!(SymbolType::Infimum == sym2.symbol_type().unwrap());
    // sup
    let sym3 = Symbol::create_supremum();
    assert!(SymbolType::Supremum == sym3.symbol_type().unwrap());
    // str
    let sym4 = Symbol::create_string("x").unwrap();
    assert!("x" == sym4.string().unwrap());
    // id
    let sym5 = Symbol::create_id("x", false).unwrap();
    assert!(SymbolType::Function == sym5.symbol_type().unwrap());
    assert!(sym5.is_negative().unwrap());
    assert!(!sym5.is_positive().unwrap());
    assert!("x" == sym5.name().unwrap());
    let args = vec![sym, sym2, sym3, sym4, sym5];
    // fun
    let sym6 = Symbol::create_function("f", &args, true).unwrap();
    assert!(SymbolType::Function == sym6.symbol_type().unwrap());
    assert!(!sym6.is_negative().unwrap());
    assert!("f" == sym6.name().unwrap());
    assert!("f(42,#inf,#sup,\"x\",-x)" == sym6.to_string());
    assert!(args.len() == sym6.arguments().unwrap().len());
    assert_eq!(args, sym6.arguments().unwrap());
    if let Err(e) = sym6.number() {
        assert!(e.to_string() == "InternalError: Call to clingo_symbol_number() failed, code: Runtime, last: unexpected");
    }
    // comparison
    let a = Symbol::create_number(1);
    let b = Symbol::create_number(2);
    assert!(a < b);
    assert!(!(a < a));
    assert!(!(b < a));
    assert!(b > a);
    assert!(!(a > a));
    assert!(!(a > b));
    assert!(a <= a);
    assert!(a <= b);
    assert!(!(b <= a));
    assert!(a >= a);
    assert!(b >= a);
    assert!(!(a >= b));
    assert!(a == a);
    assert!(!(a == b));
    assert!(a != b);
    assert!(!(a != a));
    // assert!(a.hash() == a.hash());
    // assert!(a.hash() != b.hash());
}
#[test]
fn configuration() {
    let mut ctl = Control::new(vec![]).unwrap();
    // get the configuration object and its root key
    let conf = ctl.configuration_mut().unwrap();
    let root_key = conf.root().unwrap();
    let sub_key = conf.map_at(root_key, "solve.models").unwrap();
    conf.value_set(sub_key, "0").unwrap();
    let res = conf.value_get(sub_key).unwrap();
    assert_eq!(res, "0");
    let desc = conf.description(sub_key).unwrap();
    assert_eq!(desc, "Compute at most %A models (0 for all)\n");
}
#[test]
fn backend() {
    let mut ctl = Control::new(vec![]).unwrap();
    ctl.add("base", &[], "{a; b; c}.").unwrap();

    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts).unwrap();

    let sa = ctl.symbolic_atoms().unwrap();
    let bla = sa.signatures().unwrap();
    for b in bla {
        println!("bbla: {:?}", b);
    }
}
#[test]
fn symbols() {
    let number_symbol = Symbol::create_number(42);
    let identifier_symbol = Symbol::create_id("x", true).unwrap();

    let symbols = [number_symbol, identifier_symbol];
    let function_symbol = Symbol::create_function("x", &symbols, true).unwrap();

    // retrieve argument symbols of a symbol
    let symbols2 = function_symbol.arguments().unwrap();
    assert_eq!(symbols.to_vec(), symbols2);
}
#[test]
fn theory_atoms() {
    let mut ctl = Control::new(vec![]).unwrap();
    ctl.add(
        "base",
        &[],
        "#theory t {\
         term   { + : 1, binary, left };\
         &a/0 : term, any;\
         &b/1 : term, {=}, term, any\
         }.\
         y :- &b(a) { } = 17.",
    )
    .unwrap();

    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts).unwrap();

    let atoms = ctl.theory_atoms().unwrap();
    for atom in atoms.iter() {
        let term = atoms.atom_term(atom).unwrap();
        let string = atoms.atom_to_string(atom).unwrap();
        assert_eq!("&b(a){}=17", string);
        let string = atoms.term_to_string(term).unwrap();
        assert_eq!("b(a)", string);
    }
}

fn test_statement(stmt: &Statement, string: &str) {
    let string2 = format!("{:?}", stmt);
    assert_eq!(string2, string);

    let mut ctl = Control::new(vec![]).unwrap();

    // get the program builder
    let mut builder = ast::ProgramBuilder::from(&mut ctl).unwrap();

    builder
        .add(&stmt)
        .expect("Failed to add statement to ProgramBuilder.");

    // finish building a program
    builder.end().expect("Failed to finish building a program.");
    let string2 = format!("{:?}", stmt);
    assert_eq!(string2, string);

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let atoms = ctl.symbolic_atoms().unwrap();
    let mut atoms_iterator = atoms.iter().unwrap();
    while let Some(item) = atoms_iterator.next() {
        let symbol = item.symbol().unwrap();
        let string2 = symbol.to_string();
        println!("{}", string2);
    }
}

#[test]
fn ast_term() {
    let sym1 = Symbol::create_number(42);
    let sym2 = Symbol::create_string("test").unwrap();

    let symbols = [sym1, sym2];
    let sym3 = Symbol::create_function("fun1", &symbols, true).unwrap();
    let sym4 = Symbol::create_supremum();
    let sym5 = Symbol::create_infimum();

    let term1 = Term::from(sym1);
    assert_eq!(format!("{:?}", term1), "Term { symbol: 42 }");
    // let tt = term1.term_type();
    // assert_eq!(format!("{:?}", tt), "Symbol(Symbol(281474976710698))");

    let term2 = Term::from(sym2);
    assert_eq!(format!("{:?}", term2), "Term { symbol: \"test\" }");
    // let tt = term2.term_type();
    // assert_eq!(format!("{:?}", tt), "Symbol(Symbol(281474976710698))");

    let term = Term::from(sym3);
    assert_eq!(format!("{:?}", term), "Term { symbol: fun1(42,\"test\") }");

    let term = Term::from(sym4);
    assert_eq!(format!("{:?}", term), "Term { symbol: #sup }");

    let term = Term::from(sym5);
    assert_eq!(format!("{:?}", term), "Term { symbol: #inf }");

    let term = Term::variable("Var").unwrap();
    assert_eq!(format!("{:?}", term), "Term { variable: \"Var\" }");
    let tt = term.term_type();
    assert_eq!(format!("{:?}", tt), "Variable(\"Var\")");

    let uop = UnaryOperation::negation(term1);
    let op = uop.unary_operator();
    assert_eq!(format!("{:?}", op), "Negation");
    let arg = uop.argument();
    assert_eq!(format!("{:?}", arg), "Term { symbol: 42 }");

    let bop = BinaryOperation::xor(term1, term2);
    let op = bop.binary_operator();
    assert_eq!(format!("{:?}", op), "Xor");
    let arg = bop.left();
    assert_eq!(format!("{:?}", arg), "Term { symbol: 42 }");
    let arg = bop.right();
    assert_eq!(format!("{:?}", arg), "Term { symbol: \"test\" }");

    let interval = Interval::new(term1, term2);
    let arg = interval.left();
    assert_eq!(format!("{:?}", arg), "Term { symbol: 42 }");
    let arg = interval.right();
    assert_eq!(format!("{:?}", arg), "Term { symbol: \"test\" }");

    let args = vec![term1, term2];
    let function = Function::new("fun2", &args).unwrap();
    let pool = Pool::new(&args);

    let term = Term::from(&uop);
    assert_eq!(format!("{:?}",term), "Term { unary_operation: UnaryOperation { unary_operator: Negation argument: Term { symbol: 42 } } }");
    let tt = term.term_type();
    assert_eq!(
        format!("{:?}", tt),
        "UnaryOperation(UnaryOperation { unary_operator: Negation argument: Term { symbol: 42 } })"
    );

    let term = Term::from(&bop);
    assert_eq!(format!("{:?}",term), "Term { binary_operation: BinaryOperation { binary_operator: Xor left: Term { symbol: 42 } right: Term { symbol: \"test\" } } }");
    let tt = term.term_type();
    assert_eq!(format!("{:?}", tt), "BinaryOperation(BinaryOperation { binary_operator: Xor left: Term { symbol: 42 } right: Term { symbol: \"test\" } })");

    let term = Term::from(&interval);
    assert_eq!(format!("{:?}",term), "Term { interval: Interval { left: Term { symbol: 42 } right: Term { symbol: \"test\" } } }");
    let tt = term.term_type();
    assert_eq!(
        format!("{:?}", tt),
        "Interval(Interval { left: Term { symbol: 42 } right: Term { symbol: \"test\" } })"
    );

    let term = Term::from(&function);
    assert_eq!(format!("{:?}",term), "Term { function: Function { name: fun2 args: [Term { symbol: 42 }, Term { symbol: \"test\" }] } }");
    let tt = term.term_type();
    assert_eq!(
        format!("{:?}", tt),
        "Function(Function { name: fun2 args: [Term { symbol: 42 }, Term { symbol: \"test\" }] })"
    );

    let term = Term::external_function(&function);
    assert_eq!(format!("{:?}",term), "Term { external_function: Function { name: fun2 args: [Term { symbol: 42 }, Term { symbol: \"test\" }] } }");
    let tt = term.term_type();
    assert_eq!(format!("{:?}", tt), "ExternalFunction(Function { name: fun2 args: [Term { symbol: 42 }, Term { symbol: \"test\" }] })");

    let term = Term::from(&pool);
    assert_eq!(
        format!("{:?}", term),
        "Term { pool: Pool { args: [Term { symbol: 42 }, Term { symbol: \"test\" }] } }"
    );
    let tt = term.term_type();
    assert_eq!(
        format!("{:?}", tt),
        "Pool(Pool { args: [Term { symbol: 42 }, Term { symbol: \"test\" }] })"
    );
}

#[test]
fn ast_literal() {
    let sym1 = Symbol::create_number(42);
    let sym2 = Symbol::create_string("test").unwrap();

    let symbols = [sym1, sym2];
    let sym3 = Symbol::create_function("fun1", &symbols, true).unwrap();
    let sym4 = Symbol::create_supremum();

    let term1 = Term::from(sym1);
    let term2 = Term::from(sym2);
    let term3 = Term::from(sym3);
    let term4 = Term::from(sym4);

    let comp = Comparison::gt(term2, term3);

    let csp_prod_term1 = CspProductTerm::new(term1, &term2);
    let csp_prod_term2 = CspProductTerm::new(term3, &term4);
    let csp_prod_terms1 = vec![csp_prod_term1];
    let csp_prod_terms2 = vec![csp_prod_term2];

    let csp_sum_term1 = CspSumTerm::new(&csp_prod_terms1);
    let csp_sum_term2 = CspSumTerm::new(&csp_prod_terms2);

    let csp_guard = CspGuard::gt(csp_sum_term1);
    let csp_guards = vec![csp_guard];
    let csp_lit = CspLiteral::new(csp_sum_term2, &csp_guards);

    let lit = ast::Literal::from_bool(Sign::NoSign, true);
    assert_eq!(
        format!("{:?}", lit),
        "Literal { sign: NoSign boolean: true }"
    );
    let lt = lit.literal_type();
    assert_eq!(format!("{:?}", lt), "Boolean(true)");

    let lit = ast::Literal::from_term(Sign::NoSign, &term1);
    assert_eq!(
        format!("{:?}", lit),
        "Literal { sign: NoSign symbol: Term { symbol: 42 } }"
    );
    let lt = lit.literal_type();
    assert_eq!(format!("{:?}", lt), "Symbolic(Term { symbol: 42 })");

    let lit = ast::Literal::from_comparison(Sign::NoSign, &comp);
    assert_eq!(format!("{:?}",lit), "Literal { sign: NoSign comparison: Comparison { op: GreaterThan left: Term { symbol: \"test\" } right: Term { symbol: fun1(42,\"test\") } } }");
    let lt = lit.literal_type();
    assert_eq!(format!("{:?}", lt), "Comparison(Comparison { op: GreaterThan left: Term { symbol: \"test\" } right: Term { symbol: fun1(42,\"test\") } })");

    let lit = ast::Literal::from_csp_literal(Sign::NoSign, &csp_lit);
    assert_eq!(format!("{:?}",lit), "Literal { sign: NoSign csp_literal: CspLiteral { term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: fun1(42,\"test\") } variable: Term { symbol: #sup } }] } guards: [CspGuard { comparison: GreaterThan term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: 42 } variable: Term { symbol: \"test\" } }] } }] } }");
    let lt = lit.literal_type();
    assert_eq!(format!("{:?}", lt), "CSP(CspLiteral { term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: fun1(42,\"test\") } variable: Term { symbol: #sup } }] } guards: [CspGuard { comparison: GreaterThan term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: 42 } variable: Term { symbol: \"test\" } }] } }] })");
}

#[test]
fn ast_head_literal() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = Term::from(sym);
    let term2 = Term::from(sym);
    let lit = ast::Literal::from_term(Sign::NoSign, &term1);
    let lit2 = ast::Literal::from_term(Sign::NoSign, &term2);
    let condition = vec![lit2];
    let cond = ConditionalLiteral::new(&lit, &condition);
    let elements = vec![cond];
    let dis = Disjunction::new(&elements);

    let term3 = Term::from(sym);
    let guard = AggregateGuard::gt(term3);
    let agg = Aggregate::new(&elements, &guard, &guard);

    let tuple = vec![term1];
    let helem = HeadAggregateElement::new(&tuple, cond);
    let elements = vec![helem];
    let hagg = HeadAggregate::new(ast::AggregateFunction::Count, &elements, &guard, &guard);

    let th_term = TheoryTerm::from(sym);
    let tuple = vec![th_term];
    let element = TheoryAtomElement::new(&tuple, &condition);
    let elements = vec![element];
    let operator_name = String::from("theory_operator");
    let guard = TheoryGuard::new(&operator_name, th_term).unwrap();
    let tatom = TheoryAtom::new(term1, &elements, &guard);

    let hlit = ast::HeadLiteral::from(&lit);
    assert_eq!(
        format!("{:?}", hlit),
        "HeadLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } } }"
    );

    let hlit = ast::HeadLiteral::from(&dis);
    assert_eq!(format!("{:?}",hlit), "HeadLiteral { disjunction: Disjunction { elements: [ConditionalLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } }, condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] }] } }");

    let hlit = ast::HeadLiteral::from(&agg);
    assert_eq!(format!("{:?}",hlit), "HeadLiteral { aggregate: Aggregate { elements: [ConditionalLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } }, condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] }], left_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } }, right_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } } } }");

    let hlit = ast::HeadLiteral::from(&hagg);
    assert_eq!(format!("{:?}",hlit), "HeadLiteral { head_aggregate: HeadAggregate { function: Count elements: [HeadAggregateElement { tuple: [Term { symbol: test }], conditional_literal: ConditionalLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } }, condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] } }], left_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } }, right_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } } } }");

    let hlit = ast::HeadLiteral::from(&tatom);
    assert_eq!(format!("{:?}",hlit), "HeadLiteral { theory_atom: TheoryAtom { term: Term { symbol: test } elements: [TheoryAtomElement { tuple: [TheoryTerm { symbol: test }] condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] }] guard: TheoryGuard { operator_name: \"theory_operator\" term: TheoryTerm { symbol: test } } } }");
}
#[test]
fn ast_body_literal() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = Term::from(sym);
    let term2 = Term::from(sym);
    let lit = ast::Literal::from_term(Sign::NoSign, &term1);
    let lit2 = ast::Literal::from_term(Sign::NoSign, &term2);
    let condition = vec![lit2];
    let cond = ConditionalLiteral::new(&lit, &condition);
    let elements = vec![cond];

    let term3 = Term::from(sym);
    let guard = AggregateGuard::gt(term3);
    let agg = Aggregate::new(&elements, &guard, &guard);

    let tuple = vec![term1];
    let element = BodyAggregateElement::new(&tuple, &condition);
    let elements = vec![element];
    let bagg = BodyAggregate::new(AggregateFunction::Count, &elements, &guard, &guard);

    let th_term = TheoryTerm::from(sym);
    let tuple = vec![th_term];
    let element = TheoryAtomElement::new(&tuple, &condition);
    let elements = vec![element];
    let operator_name = String::from("theory_operator");
    let guard = TheoryGuard::new(&operator_name, th_term).unwrap();
    let tatom = TheoryAtom::new(term1, &elements, &guard);

    let tuple = vec![term1];
    let csp_prod_term1 = CspProductTerm::new(term1, &term2);
    let csp_prod_terms1 = vec![csp_prod_term1];
    let csp_sum_term1 = CspSumTerm::new(&csp_prod_terms1);
    let element = DisjointElement::new(&tuple, csp_sum_term1, &condition);
    let elements = vec![element];
    let dis = Disjoint::new(&elements);

    let blit = ast::BodyLiteral::from_literal(Sign::NoSign, &lit);
    assert_eq!(
        format!("{:?}", blit),
        "BodyLiteral { sign: NoSign literal: Literal { sign: NoSign symbol: Term { symbol: test } } }"
    );

    let blit = ast::BodyLiteral::from_conditional(Sign::NoSign, &cond);
    assert_eq!(
        format!("{:?}", blit),
        "BodyLiteral { sign: NoSign conditional: ConditionalLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } }, condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] } }"
    );

    let blit = ast::BodyLiteral::from_aggregate(Sign::NoSign, &agg);
    assert_eq!(
        format!("{:?}", blit),
        "BodyLiteral { sign: NoSign aggregate: Aggregate { elements: [ConditionalLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } }, condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] }], left_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } }, right_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } } } }"
    );

    let blit = ast::BodyLiteral::from_body_aggregate(Sign::NoSign, &bagg);
    assert_eq!(
        format!("{:?}", blit),
        "BodyLiteral { sign: NoSign body_aggregate: BodyAggregate { function: Count elements: [BodyAggregateElement { tuple: [Term { symbol: test }], condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] }], left_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } }, right_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } } } }"
    );

    let blit = ast::BodyLiteral::from_theory_atom(Sign::NoSign, &tatom);
    assert_eq!(
        format!("{:?}", blit),
        "BodyLiteral { sign: NoSign theory_atom: TheoryAtom { term: Term { symbol: test } elements: [TheoryAtomElement { tuple: [TheoryTerm { symbol: test }] condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] }] guard: TheoryGuard { operator_name: \"theory_operator\" term: TheoryTerm { symbol: test } } } }"
    );

    let blit = ast::BodyLiteral::from_disjoint(Sign::NoSign, &dis);
    assert_eq!(
        format!("{:?}", blit),
        "BodyLiteral { sign: NoSign disjoint: Disjoint { elements: [DisjointElement { tuple: [Term { symbol: test }] term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: test } variable: Term { symbol: test } }] } condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] }] } }"
    );
}
#[test]
fn ast_theory_term() {
    let sym = Symbol::create_id("test", true).unwrap();

    let th_term1 = TheoryTerm::from(sym);
    assert_eq!(format!("{:?}", th_term1), "TheoryTerm { symbol: test }");

    let arr = vec![th_term1];
    let b: &[TheoryTerm] = &arr;
    let th_arr = TheoryTermArray::from(b);

    let name = String::from("fun1");
    let th_fun = TheoryFunction::new(&name, &arr).unwrap();

    let th_term = TheoryTerm::tuple(&th_arr);
    assert_eq!(
        format!("{:?}", th_term),
        "TheoryTerm { tuple: TheoryTermArray { terms: [TheoryTerm { symbol: test }] } }"
    );

    let th_term = TheoryTerm::list(&th_arr);
    assert_eq!(
        format!("{:?}", th_term),
        "TheoryTerm { list: TheoryTermArray { terms: [TheoryTerm { symbol: test }] } }"
    );

    let th_term = TheoryTerm::set(&th_arr);
    assert_eq!(
        format!("{:?}", th_term),
        "TheoryTerm { set: TheoryTermArray { terms: [TheoryTerm { symbol: test }] } }"
    );

    let th_term = TheoryTerm::from(&th_fun);
    assert_eq!(
        format!("{:?}", th_term),
        "TheoryTerm { theory_function: TheoryFunction { name: \"fun1\" arguments: [TheoryTerm { symbol: test }] } }"
    );

    let th_term = TheoryTerm::variable("Var").unwrap();
    assert_eq!(format!("{:?}", th_term), "TheoryTerm { variable: \"Var\" }");
}
#[test]
fn ast_edge() {
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let term1 = Term::from(sym1);
    let sym2 = Symbol::create_id("test2", true).unwrap();
    let term2 = Term::from(sym2);
    let lit = ast::Literal::from_term(Sign::NoSign, &term1);
    let blit1 = BodyLiteral::from_literal(Sign::NoSign, &lit);
    let body = vec![blit1];
    let edge = Edge::new(term1, term2, &body);
    let stm = Statement::from(&edge);
    test_statement(
        &stm,
        "Statement { edge: Edge { u: Term { symbol: test1 } v: Term { symbol: test2 } body: [BodyLiteral { sign: NoSign literal: Literal { sign: NoSign symbol: Term { symbol: test1 } } }] } }",
    );
}
#[test]
fn ast_minimize() {
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let weight = Term::from(sym1);
    let sym2 = Symbol::create_id("test2", true).unwrap();
    let priority = Term::from(sym2);
    let tuple = vec![weight, priority];
    let lit = ast::Literal::from_term(Sign::NoSign, &weight);
    let blit1 = BodyLiteral::from_literal(Sign::NoSign, &lit);
    let body = vec![blit1];
    let mini = Minimize::new(weight, priority, &tuple, &body);
    let stm = Statement::from(&mini);
    test_statement(
        &stm,
        "Statement { minimize: Minimize { weight: Term { symbol: test1 } priority: Term { symbol: test2 } tuple: [Term { symbol: test1 }, Term { symbol: test2 }] body: [BodyLiteral { sign: NoSign literal: Literal { sign: NoSign symbol: Term { symbol: test1 } } }] } }",
    );
}
#[test]
fn ast_show_term() {
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let term1 = Term::from(sym1);
    let sym2 = Symbol::create_id("test2", true).unwrap();
    let term2 = Term::from(sym2);
    let lit = ast::Literal::from_term(Sign::NoSign, &term2);
    let blit1 = BodyLiteral::from_literal(Sign::NoSign, &lit);
    let body = vec![blit1];
    let term = ShowTerm::new(term1, &body, true);
    let stm = Statement::from(&term);
    test_statement(
        &stm,
        "Statement { show_term: ShowTerm { term: Term { symbol: test1 } body: [BodyLiteral { sign: NoSign literal: Literal { sign: NoSign symbol: Term { symbol: test2 } } }] csp: true } }",
    );
}
// #[test]
// fn ast_show_signature() {
//     let sig = Signature::new("signame",3, false).unwrap();
//     let ssig = ShowSignature::new(sig, true);
//     let stm = Statement::from(&ssig);
//     test_statement(
//         &stm,
//         "Statement { show_signature: ShowSignature { data: clingo_ast_show_signature { signature: 984814660390081, csp: true } } }",
//     );
// }
// #[test]
// fn ast_project_signature() {
//     let sig = Signature::new("signame",3, false).unwrap();
//     let stm = Statement::from(sig);
//     test_statement(
//         &stm,
//         "Statement { project_atom_signature: Signature(985118797729569) }",
//     );
// }
// #[test]
// fn ast_defined() {
//     let sig = Signature::new("signame",3, false).unwrap();
//     let def = Defined::new(sig);
//     let stm = Statement::from(&def);
//     test_statement(
//         &stm,
//         "Statement { defined: Defined { signature: Signature(984058746113217) } }",
//     );
// }
#[test]
fn ast_const_definition() {
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let value = Term::from(sym1);
    let def = Definition::new("constname", value, true).unwrap();
    let stm = Statement::from(&def);
    test_statement(
        &stm,
        "Statement { const: Definition { head: \"constname\", value: Term { symbol: test1 } is_default: true } }",
    );
}
#[test]
fn ast_theory_definition() {
    let op_def = TheoryOperatorDefinition::new("operator_name", 2, TheoryOperatorType::Unary);
    let operators = vec![op_def];
    let termdef = TheoryTermDefinition::new("term_Def_name", &operators).unwrap();
    let terms = vec![termdef];
    // let op1 = "operator1";
    // let op2 = "operator2";
    // let operators = vec![internalize_string(op1), op2];
    // let guard = TheoryGuardDefinition::new("guard_term", &operators).unwrap();
    // let atom_def = TheoryAtomDefinition::new("atom_def_name",TheoryAtomType::Head, 2, "bla", &guard).unwrap();
    // let atoms = vec![atom_def];
    let atoms = vec![];
    let def = TheoryDefinition::new("theory_name", &terms, &atoms).unwrap();
    let stm = Statement::from(&def);
    test_statement(
        &stm,
        "Statement { theory_definition: TheoryDefinition { name: \"theory_name\" terms: [TheoryTermDefinition { name: \"term_Def_name\" operators: [TheoryOperatorDefinition { name: \"operator_name\" priority: 2 type: Unary }] }] atoms: [] } }",
    );
}
#[test]
fn ast_external() {
    let sym = Symbol::create_id("test", true).unwrap();
    let atom = Term::from(sym);
    let ext = External::new(atom, &[]);
    let stm = ext.ast_statement();
    test_statement(
        &stm,
        "Statement { external: External { atom: Term { symbol: test }, body: [] } }",
    );
}
#[test]
fn ast_rule_head_literal() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = Term::from(sym);
    let lit = ast::Literal::from_term(Sign::NoSign, &term);
    let hlit = HeadLiteral::from(&lit);
    let rule = Rule::new(hlit, &[]);
    let stm = rule.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } } }, body: [] } }");
}
#[test]
fn ast_rule_head_aggregate() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = Term::from(sym);
    let lit = ast::Literal::from_term(Sign::NoSign, &term);
    let condition = vec![lit];
    let cond = ConditionalLiteral::new(&lit, &condition);
    let elements = vec![cond];
    let left_guard = AggregateGuard::gt(term);
    let right_guard = AggregateGuard::lt(term);
    let agg = Aggregate::new(&elements, &left_guard, &right_guard);
    let hlit = HeadLiteral::from(&agg);
    let rule = Rule::new(hlit, &[]);
    let stm = rule.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { aggregate: Aggregate { elements: [ConditionalLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } }, condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] }], left_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } }, right_guard: AggregateGuard { comparison: LessThan, term: Term { symbol: test } } } }, body: [] } }");
}
#[test]
fn ast_rule_head_head_aggregate() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = Term::from(sym);
    let lit = ast::Literal::from_term(Sign::NoSign, &term);
    let condition = vec![lit];
    let cond = ConditionalLiteral::new(&lit, &condition);
    let tuple = vec![term];
    let helem = HeadAggregateElement::new(&tuple, cond);
    let elements = vec![helem];
    let guard = AggregateGuard::gt(term);
    let hagg = HeadAggregate::new(AggregateFunction::Count, &elements, &guard, &guard);
    let hlit = HeadLiteral::from(&hagg);
    let rule = Rule::new(hlit, &[]);
    let stm = rule.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { head_aggregate: HeadAggregate { function: Count elements: [HeadAggregateElement { tuple: [Term { symbol: test }], conditional_literal: ConditionalLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } }, condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] } }], left_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } }, right_guard: AggregateGuard { comparison: GreaterThan, term: Term { symbol: test } } } }, body: [] } }");
}
#[test]
fn ast_rule() {
    let id1 = String::from("test1");
    let id2 = String::from("test2");
    let id3 = String::from("test3");
    let id4 = String::from("test4");
    let id5 = String::from("test5");
    let id6 = String::from("test6");
    let id7 = String::from("test7");
    let id8 = String::from("test8");
    let id9 = String::from("test9");
    let id10 = String::from("test10");

    let sym1 = Symbol::create_id(&id1, true).unwrap();
    let sym2 = Symbol::create_id(&id2, true).unwrap();
    let sym3 = Symbol::create_id(&id3, true).unwrap();
    let sym4 = Symbol::create_id(&id4, true).unwrap();
    let sym5 = Symbol::create_id(&id5, true).unwrap();
    let sym6 = Symbol::create_id(&id6, true).unwrap();
    let sym7 = Symbol::create_id(&id7, true).unwrap();
    let sym8 = Symbol::create_id(&id8, true).unwrap();
    let sym9 = Symbol::create_id(&id9, true).unwrap();
    let sym10 = Symbol::create_id(&id10, true).unwrap();

    let term1 = Term::from(sym1);
    let term2 = Term::from(sym2);
    let term3 = Term::from(sym3);
    let term4 = Term::from(sym4);
    let term5 = Term::from(sym5);
    let term6 = Term::from(sym6);
    let term7 = Term::from(sym7);
    let term8 = Term::from(sym8);
    let term9 = Term::from(sym9);
    let term10 = Term::from(sym10);

    let uop1 = UnaryOperation::minus(term8);

    let bop1 = BinaryOperation::xor(term9, term10);

    let term11 = Term::from(&uop1);
    let term12 = Term::from(&bop1);

    let mut args = vec![term12];
    let fun1 = Function::new("fun1", &mut args).unwrap();

    let term13 = Term::from(&fun1);

    let comp = Comparison::gt(term2, term3);

    let csp_prod_term1 = CspProductTerm::new(term4, &term5);
    let csp_prod_term2 = CspProductTerm::new(term6, &term7);
    let csp_prod_terms1 = vec![csp_prod_term1];
    let csp_prod_terms2 = vec![csp_prod_term2];

    let csp_sum_term1 = CspSumTerm::new(&csp_prod_terms1);
    let csp_sum_term2 = CspSumTerm::new(&csp_prod_terms2);

    let csp_guard = CspGuard::gt(csp_sum_term1);
    let csp_guards = vec![csp_guard];
    let csp_lit = CspLiteral::new(csp_sum_term2, &csp_guards);

    let lit1 = ast::Literal::from_bool(Sign::NoSign, true);
    let lit2 = ast::Literal::from_term(Sign::NoSign, &term1);
    let lit3 = ast::Literal::from_comparison(Sign::NoSign, &comp);
    let lit4 = ast::Literal::from_csp_literal(Sign::NoSign, &csp_lit);
    let lit5 = ast::Literal::from_term(Sign::NoSign, &term11);
    let lit6 = ast::Literal::from_term(Sign::NoSign, &term13);

    let hlit1 = HeadLiteral::from(&lit1);
    let hlit2 = HeadLiteral::from(&lit2);
    let hlit3 = HeadLiteral::from(&lit3);
    let hlit4 = HeadLiteral::from(&lit4);
    let hlit5 = HeadLiteral::from(&lit5);
    let hlit6 = HeadLiteral::from(&lit6);

    let rule1 = Rule::new(hlit1, &[]);
    let rule2 = Rule::new(hlit2, &[]);
    let rule3 = Rule::new(hlit3, &[]);
    let rule4 = Rule::new(hlit4, &[]);
    let rule5 = Rule::new(hlit5, &[]);
    let rule6 = Rule::new(hlit6, &[]);

    let stm = rule1.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { literal: Literal { sign: NoSign boolean: true } }, body: [] } }");
    let head = rule1.head();
    assert_eq!(
        format!("{:?}", head),
        "HeadLiteral { literal: Literal { sign: NoSign boolean: true } }"
    );

    let stm = rule2.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test1 } } }, body: [] } }");
    let stm = rule3.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { literal: Literal { sign: NoSign comparison: Comparison { op: GreaterThan left: Term { symbol: test2 } right: Term { symbol: test3 } } } }, body: [] } }");
    let stm = rule4.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { literal: Literal { sign: NoSign csp_literal: CspLiteral { term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: test6 } variable: Term { symbol: test7 } }] } guards: [CspGuard { comparison: GreaterThan term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: test4 } variable: Term { symbol: test5 } }] } }] } } }, body: [] } }");
    let stm = rule5.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { literal: Literal { sign: NoSign symbol: Term { unary_operation: UnaryOperation { unary_operator: Minus argument: Term { symbol: test8 } } } } }, body: [] } }");
    let stm = rule6.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { literal: Literal { sign: NoSign symbol: Term { function: Function { name: fun1 args: [Term { binary_operation: BinaryOperation { binary_operator: Xor left: Term { symbol: test9 } right: Term { symbol: test10 } } }] } } } }, body: [] } }");
}
#[test]
fn ast_rule_body() {
    let sym = Symbol::create_id("test", true).unwrap();
    let term = Term::from(sym);
    let lit = ast::Literal::from_term(Sign::NoSign, &term);
    let hlit = HeadLiteral::from(&lit);
    let blit1 = BodyLiteral::from_literal(Sign::NoSign, &lit);
    let body = vec![blit1];
    let rule = Rule::new(hlit, &body);
    let stm = rule.ast_statement();
    test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { literal: Literal { sign: NoSign symbol: Term { symbol: test } } }, body: [BodyLiteral { sign: NoSign literal: Literal { sign: NoSign symbol: Term { symbol: test } } }] } }");
}
#[test]
fn ast_program() {
    let parameters = vec![];
    let prg = Program::new("a:-b. b.", &parameters).unwrap();
    let stm = Statement::from(&prg);
    test_statement(
        &stm,
        "Statement { program: Program { name: \"a:-b. b.\" parameters: [] } }",
    );
}
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    // t.compile_fail("tests/ui/ast_term_from_symbol.rs");
    t.compile_fail("tests/ui/ast_term_from_unary_operation.rs");
    t.compile_fail("tests/ui/ast_term_from_binary_operation.rs");
    t.compile_fail("tests/ui/ast_term_from_function.rs");
    t.compile_fail("tests/ui/ast_term_from_pool.rs");
    t.compile_fail("tests/ui/ast_csp_product_term.rs");
    // t.compile_fail("tests/ui/ast_comparison.rs"); // terms are copied
    // t.compile_fail("tests/ui/ast_unary_operation.rs"); // terms are copied
    // t.compile_fail("tests/ui/ast_binary_operation.rs");// terms are copied
    // t.compile_fail("tests/ui/ast_aggregate_guard.rs");// terms are copied
    t.compile_fail("tests/ui/ast_function.rs");
    // t.compile_fail("tests/ui/ast_interval.rs");// terms are copied
    t.compile_fail("tests/ui/ast_pool.rs");
    // t.compile_fail("tests/ui/ast_literal_from_boolean.rs"); //bool is copy
    t.compile_fail("tests/ui/ast_literal_from_term.rs");
    t.compile_fail("tests/ui/ast_literal_from_comparison.rs");
    t.compile_fail("tests/ui/ast_aggregate.rs");
    t.compile_fail("tests/ui/ast_conditional_literal.rs");
    t.compile_fail("tests/ui/ast_head_aggregate.rs");
    t.compile_fail("tests/ui/ast_head_aggregate_element.rs");
    t.compile_fail("tests/ui/ast_disjunction.rs");
    t.compile_fail("tests/ui/ast_head_literal.rs");
    t.compile_fail("tests/ui/ast_body_literal_from_term.rs");
    t.compile_fail("tests/ui/ast_rule.rs");
    // t.compile_fail("tests/ui/ast_external.rs");// terms are copied
    t.compile_fail("tests/ui/ast_statement_from_external.rs");
    t.compile_fail("tests/ui/ast_statement_from_rule.rs");
    //check builder.add(stmt)
}
