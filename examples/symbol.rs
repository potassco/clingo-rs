use clingo::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    // create a number, identifier (function without arguments), and a function symbol
    let number_symbol = Symbol::create_number(42);
    let identifier_symbol = Symbol::create_id("x", true).unwrap();

    let mut symbols = vec![number_symbol, identifier_symbol];
    let function_symbol = Symbol::create_function("x", &symbols, true).unwrap();
    symbols.push(function_symbol);

    // print the symbols along with their hash values
    let mut hasher = DefaultHasher::new();
    for symbol in &symbols {
        symbol.hash(&mut hasher);
        println!("the hash of {} is {}", symbol, hasher.finish());
    }

    // retrieve argument symbols of a symbol
    let symbols2 = function_symbol.arguments().unwrap();

    // equal to comparison
    for symbol in symbols2 {
        print!("{} is ", symbols[0]);
        if symbols[0] == symbol {
            print!("equal");
        } else {
            print!("not equal");
        }
        println!(" to {}", symbol);
    }

    // less than comparison
    print!("{} is ", symbols[0]);
    if symbols[0] < symbols[1] {
        print!("less");
    } else {
        print!("not less");
    }
    println!(" than {}", symbols[1]);
}
