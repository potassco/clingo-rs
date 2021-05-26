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
    let mut ctl = control(vec![]).unwrap();
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
    let mut ctl = control(vec![]).unwrap();
    ctl.add("base", &[], "{a; b; c}.").unwrap();

    let part = Part::new("base", vec![]).unwrap();
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
    let mut ctl = control(vec![]).unwrap();
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

    let part = Part::new("base", vec![]).unwrap();
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
    let string2 = format!("{}", stmt);
    assert_eq!(string2, string);

    let mut ctl = control(vec![]).unwrap();

    // get the program builder
    let mut builder = ast::ProgramBuilder::from(&mut ctl).unwrap();

    builder
        .add(&stmt)
        .expect("Failed to add statement to ProgramBuilder.");

    // finish building a program
    builder.end().expect("Failed to finish building a program.");
    let string2 = format!("{}", stmt);
    assert_eq!(string2, string);

    // ground the base part
    let part = Part::new("base", vec![]).unwrap();
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
    let loc = Location::default();
    let sym1 = Symbol::create_number(42);
    let sym2 = Symbol::create_string("test").unwrap();

    let symbols = [sym1, sym2];
    let sym3 = Symbol::create_function("fun1", &symbols, true).unwrap();
    let sym4 = Symbol::create_supremum();
    let sym5 = Symbol::create_infimum();

    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let term1 = Term::from(term1);
    assert_eq!(format!("{}", term1), "42");

    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let term2 = Term::from(term2);
    assert_eq!(format!("{}", term2), "\"test\"");

    let term = symbolic_term(&loc, &sym3).unwrap();
    let term = Term::from(term);
    assert_eq!(format!("{}", term), "fun1(42,\"test\")");

    let term = symbolic_term(&loc, &sym4).unwrap();
    let term = Term::from(term);
    assert_eq!(format!("{}", term), "#sup");

    let term = symbolic_term(&loc, &sym5).unwrap();
    let term = Term::from(term);
    assert_eq!(format!("{}", term), "#inf");

    let term = variable(&loc, "Var").unwrap();
    assert_eq!(format!("{}", term), "Var");

    let negation = UnaryOperator::Negation;
    let uop = unary_operation(&loc, negation, term1.clone()).unwrap();
    // let op = uop.unary_operator();
    // assert_eq!(format!("{:?}", op), "Negation");
    // let arg = uop.argument();
    // assert_eq!(format!("{:?}", arg), "Term { symbol: 42 }");
    let uop = Term::from(uop);
    assert_eq!(format!("{}", uop), "~42");

    let xor = BinaryOperator::Xor;
    let bop = binary_operation(&loc, xor, term1.clone(), term2.clone()).unwrap();
    // let op = bop.binary_operator();
    // assert_eq!(format!("{:?}", op), "Xor");
    // let arg = bop.left();
    // assert_eq!(format!("{:?}", arg), "Term { symbol: 42 }");
    // let arg = bop.right();
    // assert_eq!(format!("{:?}", arg), "Term { symbol: \"test\" }");
    let bop = Term::from(bop);
    assert_eq!(format!("{}", bop), "(42^\"test\")");

    let interval = interval(&loc, term1.clone(), term2.clone()).unwrap();
    // let arg = interval.left();
    // assert_eq!(format!("{:?}", arg), "Term { symbol: 42 }");
    // let arg = interval.right();
    // assert_eq!(format!("{:?}", arg), "Term { symbol: \"test\" }");
    let interval = Term::from(interval);
    assert_eq!(format!("{}", interval), "(42..\"test\")");

    let args = vec![term1, term2];
    let fun = function(&loc, "fun2", &args, false).unwrap();
    let fun = Term::from(fun);
    assert_eq!(format!("{}", fun), "fun2(42,\"test\")");
    let external_function = function(&loc, "fun2", &args, true).unwrap();
    let external_function = Term::from(external_function);
    assert_eq!(format!("{}", external_function), "@fun2(42,\"test\")");
    let pool = pool(&loc, &args).unwrap();
    let pool = Term::from(pool);
    assert_eq!(format!("{}", pool), "(42;\"test\")");
}

#[test]
fn ast_literal() {
    let loc = Location::default();
    let sym1 = Symbol::create_number(42);
    let sym2 = Symbol::create_string("test").unwrap();

    let symbols = [sym1, sym2];
    let sym3 = Symbol::create_function("fun1", &symbols, true).unwrap();

    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let term3 = symbolic_term(&loc, &sym3).unwrap();

    let lit = ast::basic_literal_from_boolean_constant(&loc, Sign::NoSign, true).unwrap();
    assert_eq!(format!("{}", lit), "#true");
    let sterm1 = ast::symbolic_atom(term1.clone()).unwrap();
    let lit = ast::basic_literal_from_symbolic_atom(&loc, Sign::NoSign, sterm1).unwrap();
    assert_eq!(format!("{}", lit), "42");

    let gt = ComparisonOperator::GreaterThan;
    let comp = comparison(gt, term2.clone(), term3.clone()).unwrap();
    let lit = ast::basic_literal_from_comparison(&loc, Sign::NoSign, comp).unwrap();
    assert_eq!(format!("{}", lit), "\"test\" > fun1(42,\"test\")");

    // TODO: activate these test when the bug in libclingo is fixed
    // let sym4 = Symbol::create_supremum();
    // let term4 = symbolic_term(&loc, &sym4).unwrap();
    // let csp_prod_term1 = csp_product(&loc, term1, Some(term2)).unwrap();
    // let csp_prod_term2 = csp_product(&loc, term3, Some(term4)).unwrap();
    // let csp_prod_terms1 = vec![csp_prod_term1];
    // let csp_prod_terms2 = vec![csp_prod_term2];

    // let csp_sum_term1 = csp_sum(&loc,csp_prod_terms1);
    // let csp_sum_term2 = csp_sum(&loc,csp_prod_terms2);

    // let csp_guard = CspGuard::gt(csp_sum_term1);
    // let csp_guards = vec![csp_guard];
    // let csp_lit = CspLiteral::new(csp_sum_term2, &csp_guards);

    // let lit = ast::Literal::from_csp_literal(Sign::NoSign, &csp_lit);
    // assert_eq!(format!("{:?}",lit), "Literal { sign: NoSign csp_literal: CspLiteral { term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: fun1(42,\"test\") } variable: Term { symbol: #sup } }] } guards: [CspGuard { comparison: GreaterThan term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: 42 } variable: Term { symbol: \"test\" } }] } }] } }");
    // let lt = lit.literal_type();
    // assert_eq!(format!("{:?}", lt), "CSP(CspLiteral { term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: fun1(42,\"test\") } variable: Term { symbol: #sup } }] } guards: [CspGuard { comparison: GreaterThan term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: 42 } variable: Term { symbol: \"test\" } }] } }] })");
}

#[test]
fn ast_head_literal() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = symbolic_term(&loc, &sym).unwrap();
    let atom1 = symbolic_atom(term1.clone()).unwrap();
    let term2 = symbolic_term(&loc, &sym).unwrap();
    let atom2 = symbolic_atom(term2).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom1).unwrap();
    let lit2 = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom2).unwrap();
    let condition: Vec<ast::Literal> = vec![lit2.into()];
    let cond = conditional_literal(&loc, lit.clone(), &condition).unwrap();
    let elements = vec![cond.clone()];
    let dis = disjunction(&loc, &elements).unwrap();

    let term3 = symbolic_term(&loc, &sym).unwrap();
    let gt = ComparisonOperator::GreaterThan;
    let guard = aggregate_guard(gt, term3).unwrap();
    let agg = aggregate(&loc, Some(guard.clone()), &elements, Some(guard.clone())).unwrap();

    let tuple = vec![term1.clone().into()];
    let helem = head_aggregate_element(&tuple, cond).unwrap();
    let elements = vec![helem];
    let hagg = head_aggregate(
        &loc,
        Some(guard.clone()),
        ast::AggregateFunction::Count,
        &elements,
        Some(guard),
    )
    .unwrap();

    let tuple = vec![term1.clone().into()];
    let element = theory_atom_element(&tuple, &condition).unwrap();
    let elements = vec![element];
    let guard = theory_guard("theory_operator", term1.clone()).unwrap();
    let tatom = theory_atom(&loc, term1, &elements, Some(guard)).unwrap();

    let hlit: ast::Head = lit.into();
    assert_eq!(format!("{}", hlit), "test");

    let hlit: ast::Head = dis.into();
    assert_eq!(format!("{}", hlit), "test: test");

    let hlit: ast::Head = agg.into();
    assert_eq!(format!("{}", hlit), "test > { test: test } > test");

    let hlit: Head = hagg.into();
    assert_eq!(
        format!("{}", hlit),
        "test > #count { test: test: test } > test"
    );

    let hlit: Head = tatom.into();
    assert_eq!(
        format!("{}", hlit),
        "&test { test: test } theory_operator test"
    );
}
#[test]
fn ast_body_literal() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term1 = symbolic_term(&loc, &sym).unwrap();
    let atom1 = symbolic_atom(term1.clone()).unwrap();
    let term2 = symbolic_term(&loc, &sym).unwrap();
    let atom2 = symbolic_atom(term2.clone()).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom1).unwrap();
    let lit2 = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom2).unwrap();
    let condition = vec![lit2.into()];
    let cond = conditional_literal(&loc, lit.clone(), &condition).unwrap();
    let elements = vec![cond.clone()];

    let term3 = symbolic_term(&loc, &sym).unwrap();
    let gt = ComparisonOperator::GreaterThan;
    let guard = aggregate_guard(gt, term3).unwrap();
    let agg = aggregate(&loc, Some(guard.clone()), &elements, Some(guard.clone())).unwrap();

    let tuple = vec![term1.clone().into()];
    let element = body_aggregate_element(&tuple, &condition).unwrap();
    let elements = vec![element];
    let bagg = body_aggregate(
        &loc,
        Some(guard.clone()),
        AggregateFunction::Count,
        &elements,
        Some(guard),
    )
    .unwrap();

    let th_term = symbolic_term(&loc, &sym).unwrap();
    let tuple = vec![th_term.clone().into()];
    let element = theory_atom_element(&tuple, &condition).unwrap();
    let elements = vec![element.into()];
    let guard = theory_guard("theory_operator", th_term).unwrap();
    let tatom = theory_atom(&loc, term1.clone(), &elements, Some(guard)).unwrap();
    let blit: BodyLiteral = lit.into();
    assert_eq!(format!("{}", blit), "test");

    let blit: BodyLiteral = cond.into();
    assert_eq!(format!("{}", blit), "test: test");

    let blit = atomic_literal_from_body_atom(&loc, Sign::NoSign, agg.into()).unwrap();
    let blit: BodyLiteral = blit.into();
    assert_eq!(format!("{}", blit), "test > { test: test } > test");

    let blit = atomic_literal_from_body_atom(&loc, Sign::NoSign, bagg.into()).unwrap();
    let blit: BodyLiteral = blit.into();
    assert_eq!(format!("{}", blit), "test > #count { test: test } > test");

    let blit: BodyLiteral = tatom.into();
    assert_eq!(
        format!("{}", blit),
        "&test { test: test } theory_operator test"
    );
    // TODO activate test when clingo bug fixed
    // let tuple = vec![term1.clone()];
    // let csp_prod_term1 = csp_product(&loc, term1, Some(term2));
    // let csp_prod_terms1 = vec![csp_prod_term1];

    // let csp_sum_term1 = csp_sum(&loc, &csp_prod_terms1);
    // let element = DisjointElement::new(&tuple, csp_sum_term1, &condition);
    // let elements = vec![element];
    // let dis = disjoint(&loc,&elements);

    // let blit : BodyLiteral= dis.into();
    // assert_eq!(
    //     format!("{}", blit),
    //     "BodyLiteral { sign: NoSign disjoint: Disjoint { elements: [DisjointElement { tuple: [Term { symbol: test }] term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: test } variable: Term { symbol: test } }] } condition: [Literal { sign: NoSign symbol: Term { symbol: test } }] }] } }"
    // );
}
#[test]
fn ast_theory_term() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();

    let th_term1 = symbolic_term(&loc, &sym).unwrap();
    let th_term1: TheoryTerm = th_term1.into();
    assert_eq!(format!("{}", th_term1), "test");

    let arr = vec![th_term1];
    let th_fun = theory_function(&loc, "fun1", &arr).unwrap();

    let th_term = theory_sequence(&loc, TheoryTermSequenceType::Tuple, &arr).unwrap();
    assert_eq!(format!("{}", th_term), "(test,)");

    let th_term = theory_sequence(&loc, TheoryTermSequenceType::List, &arr).unwrap();
    assert_eq!(format!("{}", th_term), "[test]");

    let th_term = theory_sequence(&loc, TheoryTermSequenceType::Set, &arr).unwrap();
    assert_eq!(format!("{}", th_term), "{test}");

    let th_term: TheoryTerm = th_fun.into();
    assert_eq!(format!("{}", th_term), "fun1(test)");

    let th_term: TheoryTerm = variable(&loc, "Var").unwrap().into();
    assert_eq!(format!("{}", th_term), "Var");
}
#[test]
fn ast_edge() {
    let loc = Location::default();
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let sym2 = Symbol::create_id("test2", true).unwrap();
    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let atom2 = symbolic_atom(term2.clone()).unwrap();
    let lit = ast::basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom2).unwrap();
    let body = vec![lit.into()];
    let edge = edge(&loc, term1, term2, &body).unwrap();
    let stm: Statement = edge.into();
    test_statement(&stm, "#edge (test1,test2) : test2.");
}
#[test]
fn ast_minimize() {
    let loc = Location::default();
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let weight = symbolic_term(&loc, &sym1).unwrap();
    let weight_atom = symbolic_atom(weight.clone()).unwrap();
    let sym2 = Symbol::create_id("test2", true).unwrap();
    let priority = symbolic_term(&loc, &sym2).unwrap();
    let tuple = vec![weight.clone().into(), priority.clone().into()];
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, weight_atom).unwrap();
    let body = vec![lit.into()];
    let mini = minimize(&loc, weight, priority, &tuple, &body).unwrap();
    let stm: Statement = mini.into();
    test_statement(&stm, ":~ test1. [test1@test2,test1,test2]");
}
#[test]
fn ast_show_term() {
    let loc = Location::default();
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let sym2 = Symbol::create_id("test2", true).unwrap();
    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let atom2 = symbolic_atom(term2).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom2).unwrap();
    let body = vec![lit.into()];
    let term = show_term(&loc, term1, &body, true).unwrap();
    let stm: Statement = term.into();
    test_statement(&stm, "#show $test1 : test2.");
}
#[test]
fn ast_show_signature() {
    let loc = Location::default();
    let ssig = show_signature(&loc, "signame", 3, true, false).unwrap();
    let stm: Statement = ssig.into();
    test_statement(&stm, "#show signame/3.");
}
#[test]
fn ast_project_signature() {
    let loc = Location::default();
    let sig = project_signature(&loc, "signame", 3, false).unwrap();
    let stm: Statement = sig.into();
    test_statement(&stm, "#project -signame/3.");
}
#[test]
fn ast_defined() {
    let loc = Location::default();
    let def = defined(&loc, "signame", 3, false).unwrap();
    let stm: Statement = def.into();
    test_statement(&stm, "#defined -signame/3.");
}
#[test]
fn ast_const_definition() {
    let loc = Location::default();
    let sym1 = Symbol::create_id("test1", true).unwrap();
    let value = symbolic_term(&loc, &sym1).unwrap();
    let def = definition(&loc, "constname", value, true).unwrap();
    let stm: Statement = def.into();
    test_statement(&stm, "#const constname = test1.");
}
#[test]
fn ast_theory_definition() {
    let loc = Location::default();
    let op_def =
        theory_operator_definition(&loc, "operator_name", 2, TheoryOperatorType::Unary).unwrap();
    let operators = vec![op_def];
    let termdef = theory_term_definition(&loc, "def_name", &operators).unwrap();
    let terms = vec![termdef];
    let op1 = "operator1";
    let op2 = "operator2";
    let operators = vec![op1, op2];
    let guard = theory_guard_definition(&operators, "guard_term").unwrap();
    let atom_def = theory_atom_definition(
        &loc,
        TheoryAtomType::Head,
        "atom_def_name",
        2,
        "bla",
        Some(guard),
    )
    .unwrap();
    let atoms = vec![atom_def];
    // let atoms = vec![];
    let def = theory_definition(&loc, "theory_name", &terms, &atoms).unwrap();
    let stm: Statement = def.into();
    test_statement(
            &stm,
            "#theory theory_name {\n  def_name {\n    operator_name : 2, unary\n  };\n  &atom_def_name/2: bla, { operator1, operator2 }, guard_term, head\n}.",
        );
}
#[test]
fn ast_external() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term).unwrap();
    let ext = external(&loc, atom.into(), &[], ExternalType::False).unwrap();
    let stm = ext.into();
    test_statement(&stm, "#external test. [false]");
}
#[test]
fn ast_rule_head_literal() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let rule = rule(&loc, lit, &[]).unwrap();
    let stm = rule.into();
    test_statement(&stm, "test.");
}
#[test]
fn ast_rule_head_aggregate() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term.clone()).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let condition = vec![lit.clone().into()];
    let cond = conditional_literal(&loc, lit, &condition).unwrap();
    let elements = vec![cond];
    let gt = ComparisonOperator::GreaterThan;
    let left_guard = aggregate_guard(gt, term.clone()).unwrap();
    let lt = ComparisonOperator::LessThan;
    let right_guard = aggregate_guard(lt, term).unwrap();
    let agg = aggregate(&loc, Some(left_guard), &elements, Some(right_guard)).unwrap();
    // let hlit = HeadLiteral::from(&agg);
    let rule = rule(&loc, agg, &[]).unwrap();
    let stm = rule.into();
    test_statement(&stm, "test > { test: test } < test.");
}
#[test]
fn ast_rule_head_head_aggregate() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term.clone()).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let condition = vec![lit.clone().into()];
    let cond = conditional_literal(&loc, lit, &condition).unwrap();
    let tuple = vec![term.clone().into()];
    let helem = head_aggregate_element(&tuple, cond).unwrap();
    let elements = vec![helem];
    let gt = ComparisonOperator::GreaterThan;
    let guard = aggregate_guard(gt, term).unwrap();
    let hagg = head_aggregate(
        &loc,
        Some(guard.clone()),
        AggregateFunction::Count,
        &elements,
        Some(guard),
    )
    .unwrap();
    let rule = rule(&loc, hagg, &[]).unwrap();
    let stm = rule.into();
    test_statement(&stm, "test > #count { test: test: test } > test.");
}
#[test]
fn ast_rule() {
    let loc = Location::default();
    let id1 = String::from("test1");
    let id2 = String::from("test2");
    let id3 = String::from("test3");
    let id8 = String::from("test8");
    let id9 = String::from("test9");
    let id10 = String::from("test10");

    let sym1 = Symbol::create_id(&id1, true).unwrap();
    let sym2 = Symbol::create_id(&id2, true).unwrap();
    let sym3 = Symbol::create_id(&id3, true).unwrap();
    let sym8 = Symbol::create_id(&id8, true).unwrap();
    let sym9 = Symbol::create_id(&id9, true).unwrap();
    let sym10 = Symbol::create_id(&id10, true).unwrap();

    let term1 = symbolic_term(&loc, &sym1).unwrap();
    let atom1 = symbolic_atom(term1).unwrap();
    let term2 = symbolic_term(&loc, &sym2).unwrap();
    let term3 = symbolic_term(&loc, &sym3).unwrap();
    let term8 = symbolic_term(&loc, &sym8).unwrap();
    let term9 = symbolic_term(&loc, &sym9).unwrap();
    let term10 = symbolic_term(&loc, &sym10).unwrap();

    let minus = UnaryOperator::Minus;
    let uop1 = unary_operation(&loc, minus, term8).unwrap();

    let xor = BinaryOperator::Xor;
    let bop1 = binary_operation(&loc, xor, term9, term10).unwrap();

    let mut args = vec![bop1.into()];
    let fun1 = function(&loc, "fun1", &mut args, false).unwrap();

    let gt = ComparisonOperator::GreaterThan;
    let comp = comparison(gt, term2, term3).unwrap();

    // TODO activate one the bug in clingo is fixed
    // let id4 = String::from("test4");
    // let id5 = String::from("test5");
    // let id6 = String::from("test6");
    // let id7 = String::from("test7");
    // let sym4 = Symbol::create_id(&id4, true).unwrap();
    // let sym5 = Symbol::create_id(&id5, true).unwrap();
    // let sym6 = Symbol::create_id(&id6, true).unwrap();
    // let sym7 = Symbol::create_id(&id7, true).unwrap();
    // let term4 = symbolic_term(&loc, &sym4).unwrap();
    // let term5 = symbolic_term(&loc, &sym5).unwrap();
    // let term6 = symbolic_term(&loc, &sym6).unwrap();
    // let term7 = symbolic_term(&loc, &sym7).unwrap();
    // let csp_prod_term1 = csp_product(&loc, term4, Some(term5)).unwrap();
    // let csp_prod_term2 = csp_product(&loc, term6, Some(term7)).unwrap();
    // let csp_prod_terms1 = vec![csp_prod_term1];
    // let csp_prod_terms2 = vec![csp_prod_term2];

    // let csp_sum_term1 = csp_sum(&loc, &csp_prod_terms1);
    // let csp_sum_term2 = csp_sum(&loc, &csp_prod_terms2);
    //
    // let csp_guard = csp_guard(&loc, gt, csp_sum_term1);
    // let csp_guards = vec![csp_guard];
    // let csp_lit = CspLiteral::new(csp_sum_term2, &csp_guards);
    // let lit4: ast::Literal = csp_lit.into();

    // let rule4 = rule(&loc, lit4.into(), &[]).unwrap();
    // let stm = rule4.into();
    // test_statement(&stm, "Statement { rule: Rule { head: HeadLiteral { literal: Literal { sign: NoSign csp_literal: CspLiteral { term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: test6 } variable: Term { symbol: test7 } }] } guards: [CspGuard { comparison: GreaterThan term: CspSumTerm { terms: [CspProductTerm { coefficient: Term { symbol: test4 } variable: Term { symbol: test5 } }] } }] } } }, body: [] } }");

    let lit1 = basic_literal_from_boolean_constant(&loc, Sign::NoSign, true).unwrap();
    let lit2 = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom1).unwrap();
    let lit3 = basic_literal_from_comparison(&loc, Sign::NoSign, comp).unwrap();

    let lit5 = symbolic_atom(uop1).unwrap();
    let lit5: ast::Literal = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, lit5)
        .unwrap()
        .into();

    let lit6 = symbolic_atom(fun1).unwrap();
    let lit6: ast::Literal = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, lit6)
        .unwrap()
        .into();

    let rule1 = rule(&loc, lit1, &[]).unwrap();
    let rule2 = rule(&loc, lit2, &[]).unwrap();
    let rule3 = rule(&loc, lit3, &[]).unwrap();
    let rule5 = rule(&loc, lit5, &[]).unwrap();
    let rule6 = rule(&loc, lit6, &[]).unwrap();

    let stm = rule1.clone().into();
    test_statement(&stm, "#true.");
    let head = rule1.head();
    assert_eq!(format!("{}", head), "#true");

    let stm = rule2.into();
    test_statement(&stm, "test1.");
    let stm = rule3.into();
    test_statement(&stm, "test2 > test3.");
    let stm = rule5.into();
    test_statement(&stm, "-test8.");
    let stm = rule6.into();
    test_statement(&stm, "fun1((test9^test10)).");
}
#[test]
fn ast_rule_body() {
    let loc = Location::default();
    let sym = Symbol::create_id("test", true).unwrap();
    let term = symbolic_term(&loc, &sym).unwrap();
    let atom = symbolic_atom(term).unwrap();
    let lit = basic_literal_from_symbolic_atom(&loc, Sign::NoSign, atom).unwrap();
    let body = vec![lit.clone().into()];
    let rule = rule(&loc, lit, &body).unwrap();
    // drop(body);
    drop(sym);
    // on windows the body disappears
    let h = rule.head();
    assert_eq!(format!("{}", h), "test");
    drop(h);
    let stm = rule.into();
    test_statement(&stm, "test :- test.");
}
#[test]
fn ast_program() {
    let loc = Location::default();
    let parameters = vec![];
    let prg = program(&loc, "base", &parameters).unwrap();
    let stm = prg.into();
    test_statement(&stm, "#program base.");
}
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    //     // t.compile_fail("tests/ui/ast_term_from_symbol.rs");
    // t.compile_fail("tests/ui/ast_term_from_unary_operation.rs");
    //     t.compile_fail("tests/ui/ast_term_from_binary_operation.rs");
    //     t.compile_fail("tests/ui/ast_term_from_function.rs");
    //     t.compile_fail("tests/ui/ast_term_from_pool.rs");
    //     t.compile_fail("tests/ui/ast_csp_product_term.rs");
    //     // t.compile_fail("tests/ui/ast_comparison.rs"); // terms are copied
    //     // t.compile_fail("tests/ui/ast_unary_operation.rs"); // terms are copied
    //     // t.compile_fail("tests/ui/ast_binary_operation.rs");// terms are copied
    //     // t.compile_fail("tests/ui/ast_aggregate_guard.rs");// terms are copied
    //     t.compile_fail("tests/ui/ast_function.rs");
    //     // t.compile_fail("tests/ui/ast_interval.rs");// terms are copied
    //     t.compile_fail("tests/ui/ast_pool.rs");
    //     // t.compile_fail("tests/ui/ast_literal_from_boolean.rs"); //bool is copy
    //     t.compile_fail("tests/ui/ast_literal_from_term.rs");
    //     t.compile_fail("tests/ui/ast_literal_from_comparison.rs");
    //     t.compile_fail("tests/ui/ast_aggregate.rs");
    //     t.compile_fail("tests/ui/ast_conditional_literal.rs");
    //     t.compile_fail("tests/ui/ast_head_aggregate.rs");
    //     t.compile_fail("tests/ui/ast_head_aggregate_element.rs");
    //     t.compile_fail("tests/ui/ast_disjunction.rs");
    //     t.compile_fail("tests/ui/ast_head_literal.rs");
    //     t.compile_fail("tests/ui/ast_body_literal_from_term.rs");
    t.compile_fail("tests/ui/ast_rule.rs");
    //     // t.compile_fail("tests/ui/ast_external.rs");// terms are copied
    //     t.compile_fail("tests/ui/ast_statement_from_external.rs");
    //     t.compile_fail("tests/ui/ast_statement_from_rule.rs");
    //     //check builder.add(stmt)
}
