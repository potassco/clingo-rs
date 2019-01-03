use clingo::*;

#[test]
fn version_test() {
    let (ma, mi, re) = version();
    assert!(ma == 5);
    assert!(mi == 3);
    assert!(re == 0);
}
#[test]
fn parse_program_test() {
    let mut sym = Symbol::create_number(42);
    assert!(42 == sym.number().unwrap());
    sym = Symbol::create_infimum();
    assert!(SymbolType::Infimum == sym.symbol_type());
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
    assert!(SymbolType::Infimum == sym2.symbol_type());
    // sup
    let sym3 = Symbol::create_supremum();
    assert!(SymbolType::Supremum == sym3.symbol_type());
    // str
    let sym4 = Symbol::create_string("x").unwrap();
    assert!("x" == sym4.string().unwrap());
    // id
    let sym5 = Symbol::create_id("x", false).unwrap();
    assert!(SymbolType::Function == sym5.symbol_type());
    assert!(sym5.is_negative().unwrap());
    assert!(!sym5.is_positive().unwrap());
    assert!("x" == sym5.name().unwrap());
    let args = vec![sym, sym2, sym3, sym4, sym5];
    // fun
    let sym6 = Symbol::create_function("f", &args, true).unwrap();
    assert!(SymbolType::Function == sym6.symbol_type());
    assert!(!sym6.is_negative().unwrap());
    assert!("f" == sym6.name().unwrap());
    assert!("f(42,#inf,#sup,\"x\",-x)" == sym6.to_string().unwrap());
    assert!(args.len() == sym6.arguments().unwrap().len());
    assert_eq!(args, sym6.arguments().unwrap());
    if let Err(e) = sym6.number() {
        assert!(e.to_string() == "ErrorType::Runtime: unexpected");
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
    //     assert!(a.hash() == a.hash());
    //     assert!(a.hash() != b.hash());
}
