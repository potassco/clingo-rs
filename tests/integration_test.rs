use clingo::*;

#[test]
fn version_test() {
    let (ma, mi, re) = version();
    assert!(ma == 5);
    assert!(mi == 4);
    assert!(re == 0);
}
#[test]
fn signature_test() {
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
fn symbol_test() {
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
    assert!("f(42,#inf,#sup,\"x\",-x)" == sym6.to_string().unwrap());
    assert!(args.len() == sym6.arguments().unwrap().len());
    assert_eq!(args, sym6.arguments().unwrap());
    if let Err(e) = sym6.number() {
        assert!(e.to_string() == "ClingoError: Call to clingo_symbol_number() failed.");
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
fn configuration_test() {
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
fn backend_test() {
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
fn symbols_test() {
    let number_symbol = Symbol::create_number(42);
    let identifier_symbol = Symbol::create_id("x", true).unwrap();

    let symbols = [number_symbol, identifier_symbol];
    let function_symbol = Symbol::create_function("x", &symbols, true).unwrap();

    // retrieve argument symbols of a symbol
    let symbols2 = function_symbol.arguments().unwrap();
    assert_eq!(symbols.to_vec(), symbols2);
}
#[test]
fn theory_atoms_test() {
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
